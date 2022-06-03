use bincode;
use bincode::Options;
use bytes::{Buf, BufMut, BytesMut};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{self, Write},
    usize,
};
use tokio::net::UnixStream;
use tokio_util::codec::{Decoder, Encoder, Framed};

fn get_encoder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}
pub type Result<T> = std::result::Result<T, CodecError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgHeader {
    _q: u64,
    msglen: u32,
    gc_mark: u32,
}
impl MsgHeader {
    pub(crate) const LENGTH: usize = (3 * std::mem::size_of::<u32>()) + std::mem::size_of::<u64>();
    pub fn read_from(buf: &mut BytesMut) -> Result<Self> {
        let h = get_encoder().deserialize(buf).unwrap();
        Ok(h)
    }
}

pub struct MsgFrame {
    pub header: MsgHeader,
    pub message: Vec<u8>,
}
pub struct Codec {
    cur_header: Option<MsgHeader>,
}

impl Decoder for Codec {
    type Item = MsgFrame;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        trace!("try decode new frame. len: {:?}", src.len());
        if src.len() < 1 {
            return Ok(None);
        }

        let h = if let Some(h) = self.cur_header.take() {
            h
        } else if src.len() >= MsgHeader::LENGTH {
            // read_from advances src MsgHeader::LENGTH bytes
            MsgHeader::read_from(src)?
        } else {
            return Ok(None);
        };

        let remaining = h.msglen as usize;

        if src.len() < remaining {
            src.reserve(remaining as usize);
            self.cur_header = Some(h);
            return Ok(None);
        }
        trace!("split_to: at {:?}  total_len {:?}", remaining, src.len());
        let msg_src = &mut src.split_to(remaining as usize);
        let frame = MsgFrame {
            header: h,
            message: msg_src.to_vec(),
        };

        Ok(Some(frame))
    }
}
impl Encoder<MsgFrame> for Codec {
    type Error = CodecError;

    fn encode(&mut self, item: MsgFrame, dst: &mut BytesMut) -> Result<()> {
        let enc = get_encoder();
        let mut writer = dst.writer();
        enc.serialize_into(&mut writer, &item.header).unwrap();
        writer.write_all(&item.message)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CodecError {
    /// The maximum line length was exceeded.
    MaxLineLengthExceeded,
    /// An IO error occured.
    Io(io::Error),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::MaxLineLengthExceeded => write!(f, "max line length exceeded"),
            CodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for CodecError {
    fn from(e: io::Error) -> CodecError {
        CodecError::Io(e)
    }
}

impl std::error::Error for CodecError {}

pub async fn framed(vpp_api_sock: String) -> Result<Framed<UnixStream, Codec>> {
    let codec = Codec { cur_header: None };
    let socket = UnixStream::connect(vpp_api_sock).await?;

    Ok(codec.framed(socket))
}
