use clap::Clap;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};
use std::string::ToString;
extern crate strum;
#[macro_use]
extern crate strum_macros;
use env_logger;
use linked_hash_map::LinkedHashMap;

#[derive(Clap, Debug, Serialize, Deserialize, EnumString, Display)]
enum OptParseType {
    File,
    Tree,
    ApiType,
    ApiMessage,
}

/// Ingest the VPP API JSON definition file and output the Rust code
#[clap(version = "0.1", author = "Andrew Yourtchenko <ayourtch@gmail.com>")]
#[derive(Clap, Debug, Serialize, Deserialize)]
struct Opts {
    /// Input file name
    #[clap(short, long)]
    in_file: String,

    /// output file name
    #[clap(short, long, default_value = "dummy.rs")]
    out_file: String,

    /// parse type for the operation: Tree, File, ApiMessage or ApiType
    #[clap(short, long, default_value = "File")]
    parse_type: OptParseType,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Run various asserts and self-checks in the process
    #[clap(short, long)]
    asserts: bool,
}

#[derive(Debug)]
struct VppJsApiType {
    type_name: String,
    fields: Vec<VppJsApiMessageFieldDef>,
}

impl Serialize for VppJsApiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.fields.len()))?;
        seq.serialize_element(&self.type_name)?;
        for e in &self.fields {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

struct VppJsApiTypeVisitor;

impl<'de> Visitor<'de> for VppJsApiTypeVisitor {
    type Value = VppJsApiType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct VppJsApiType")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<VppJsApiType, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let type_name: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let mut fields: Vec<VppJsApiMessageFieldDef> = vec![];
        loop {
            let nxt = seq.next_element();
            log::debug!("Next: {:#?}", &nxt);
            if let Ok(Some(v)) = nxt {
                fields.push(v);
            } else {
                break;
            }
        }
        Ok(VppJsApiType { type_name, fields })
    }
}

impl<'de> Deserialize<'de> for VppJsApiType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VppJsApiTypeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VppJsApiDefaultValue {
    Str(String),
    Bool(bool),
    I64(i64),
    F64(f64),
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiFieldOptions {
    #[serde(default)]
    default: Option<VppJsApiDefaultValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VppJsApiFieldSize {
    Fixed(usize),
    Variable(Option<String>),
}

#[derive(Debug)]
struct VppJsApiMessageFieldDef {
    ctype: String,
    name: String,
    maybe_size: Option<VppJsApiFieldSize>,
    maybe_options: Option<VppJsApiFieldOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VppJsApiMessageFieldHelper {
    Str(String),
    Usize(usize),
    Map(VppJsApiFieldOptions),
}

impl Serialize for VppJsApiMessageFieldDef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::VppJsApiFieldSize::*;

        let mut len = 2;
        if self.maybe_options.is_some() {
            len = len + 1
        }
        len = len
            + match &self.maybe_size {
                None => 0,
                Some(Fixed(n)) => 1,
                Some(Variable(None)) => 1,
                Some(Variable(Some(x))) => 2,
            };
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.ctype)?;
        seq.serialize_element(&self.name)?;
        match &self.maybe_size {
            None => { /* do nothing */ }
            Some(Fixed(n)) => {
                seq.serialize_element(&n);
            }
            Some(Variable(None)) => {
                seq.serialize_element(&0u32);
            }
            Some(Variable(Some(x))) => {
                seq.serialize_element(&0u32);
                seq.serialize_element(&x);
            }
        }

        if let Some(o) = &self.maybe_options {
            seq.serialize_element(o);
        }
        seq.end()
    }
}

struct VppJsApiMessageFieldDefVisitor;

impl<'de> Visitor<'de> for VppJsApiMessageFieldDefVisitor {
    type Value = VppJsApiMessageFieldDef;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct VppJsApiMessageField")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<VppJsApiMessageFieldDef, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let ctype: String = if let Some(VppJsApiMessageFieldHelper::Str(s)) = seq.next_element()? {
            s
        } else {
            panic!("Error");
        };
        let name: String = if let Some(VppJsApiMessageFieldHelper::Str(s)) = seq.next_element()? {
            s
        } else {
            panic!("Error 2");
        };

        let mut maybe_sz: Option<usize> = None;
        let mut maybe_sz_name: Option<String> = None;
        let mut maybe_options: Option<VppJsApiFieldOptions> = None;

        loop {
            let nxt = seq.next_element();
            match nxt? {
                Some(VppJsApiMessageFieldHelper::Map(m)) => {
                    maybe_options = Some(m);
                    break;
                }
                Some(VppJsApiMessageFieldHelper::Str(o)) => {
                    maybe_sz_name = Some(o);
                }
                Some(VppJsApiMessageFieldHelper::Usize(o)) => {
                    maybe_sz = Some(o);
                }
                None => break,
            }
        }
        let maybe_size = match (maybe_sz, maybe_sz_name) {
            (None, None) => None,
            (Some(0), None) => Some(VppJsApiFieldSize::Variable(None)),
            (Some(0), Some(s)) => Some(VppJsApiFieldSize::Variable(Some(s))),
            (Some(x), None) => Some(VppJsApiFieldSize::Fixed(x)),
            (None, Some(s)) => panic!("Unexpected dependent field {} with no length", s),
            (Some(x), Some(s)) => panic!("Unexpected dependent field {} with length {}", s, x),
        };
        let ret = VppJsApiMessageFieldDef {
            ctype,
            name,
            maybe_size,
            maybe_options,
        };
        Ok(ret)
    }
}

impl<'de> Deserialize<'de> for VppJsApiMessageFieldDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VppJsApiMessageFieldDefVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiMessageInfo {
    crc: String,
}

#[derive(Debug)]
struct VppJsApiMessage {
    name: String,
    fields: Vec<VppJsApiMessageFieldDef>,
    info: VppJsApiMessageInfo,
}

impl Serialize for VppJsApiMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.fields.len() + 1))?;
        seq.serialize_element(&self.name)?;
        for e in &self.fields {
            seq.serialize_element(e)?;
        }
        seq.serialize_element(&self.info)?;
        seq.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VppJsApiMessageHelper {
    Field(VppJsApiMessageFieldDef),
    Info(VppJsApiMessageInfo),
    Name(String),
}

struct VppJsApiMessageVisitor;

impl<'de> Visitor<'de> for VppJsApiMessageVisitor {
    type Value = VppJsApiMessage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct VppJsApiMessage")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<VppJsApiMessage, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let name: String = if let Some(VppJsApiMessageHelper::Name(s)) = seq.next_element()? {
            s
        } else {
            panic!("Error");
        };
        log::debug!("API message: {}", &name);
        let mut fields: Vec<VppJsApiMessageFieldDef> = vec![];
        let mut maybe_info: Option<VppJsApiMessageInfo> = None;
        loop {
            let nxt = seq.next_element();
            log::debug!("Next: {:#?}", &nxt);
            match nxt? {
                Some(VppJsApiMessageHelper::Field(f)) => fields.push(f),
                Some(VppJsApiMessageHelper::Info(i)) => {
                    if maybe_info.is_some() {
                        panic!("Info is already set!");
                    }
                    maybe_info = Some(i);
                    break;
                }
                x => panic!("Unexpected element {:?}", x),
            }
        }
        let info = maybe_info.unwrap();
        Ok(VppJsApiMessage { name, fields, info })
    }
}

impl<'de> Deserialize<'de> for VppJsApiMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VppJsApiMessageVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct VppJsApiAlias {
    #[serde(rename = "type")]
    ctype: String,
    length: Option<usize>,
}

impl Serialize for VppJsApiAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut len = 1;
        if self.length.is_some() {
            len = len + 1;
        }
        let mut map = serializer.serialize_map(Some(len))?;
        map.serialize_entry("type", &self.ctype)?;
        if let Some(s) = &self.length {
            map.serialize_entry("length", s);
        }
        map.end()
    }
}

#[derive(Debug, Deserialize)]
struct VppJsApiService {
    reply: String,
    stream: Option<bool>,
}

impl Serialize for VppJsApiService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut len = 1;
        if self.stream.is_some() {
            len = len + 1;
        }
        let mut map = serializer.serialize_map(Some(len))?;
        map.serialize_entry("reply", &self.reply)?;
        if let Some(s) = &self.stream {
            map.serialize_entry("stream", s);
        }
        map.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiOptions {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiEnumInfo {
    enumtype: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VppJsApiEnumValueDef {
    name: String,
    value: i64,
}

impl Serialize for VppJsApiEnumValueDef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.value)?;
        seq.end()
    }
}

#[derive(Debug)]
struct VppJsApiEnum {
    name: String,
    values: Vec<VppJsApiEnumValueDef>,
    info: VppJsApiEnumInfo,
}

impl Serialize for VppJsApiEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.values.len() + 1))?;
        seq.serialize_element(&self.name)?;
        for e in &self.values {
            seq.serialize_element(e)?;
        }
        seq.serialize_element(&self.info)?;
        seq.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum VppJsApiEnumHelper {
    Str(String),
    Val(VppJsApiEnumValueDef),
    Map(VppJsApiEnumInfo),
}

struct VppJsApiEnumVisitor;

impl<'de> Visitor<'de> for VppJsApiEnumVisitor {
    type Value = VppJsApiEnum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct VppJsApiEnum")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<VppJsApiEnum, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let name: String = if let Some(VppJsApiEnumHelper::Str(s)) = seq.next_element()? {
            s
        } else {
            panic!("Error");
        };
        log::debug!("API message: {}", &name);
        let mut values: Vec<VppJsApiEnumValueDef> = vec![];
        let mut maybe_info: Option<VppJsApiEnumInfo> = None;
        loop {
            let nxt = seq.next_element();
            log::debug!("Next: {:#?}", &nxt);
            match nxt? {
                Some(VppJsApiEnumHelper::Val(f)) => values.push(f),
                Some(VppJsApiEnumHelper::Map(i)) => {
                    if maybe_info.is_some() {
                        panic!("Info is already set!");
                    }
                    maybe_info = Some(i);
                    break;
                }
                x => panic!("Unexpected element {:?}", x),
            }
        }
        let info = maybe_info.unwrap();
        Ok(VppJsApiEnum { name, values, info })
    }
}

impl<'de> Deserialize<'de> for VppJsApiEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VppJsApiEnumVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiCounterElement {
    name: String,
    severity: String,
    #[serde(rename = "type")]
    typ: String,
    units: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiCounter {
    name: String,
    elements: Vec<VppJsApiCounterElement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiPath {
    path: String,
    counter: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VppJsApiFile {
    types: Vec<VppJsApiType>,
    messages: Vec<VppJsApiMessage>,
    unions: Vec<VppJsApiType>,
    enums: Vec<VppJsApiEnum>,
    #[serde(default)]
    enumflags: Vec<VppJsApiEnum>,
    services: LinkedHashMap<String, VppJsApiService>,
    options: VppJsApiOptions,
    aliases: LinkedHashMap<String, VppJsApiAlias>,
    vl_api_version: String,
    imports: Vec<String>,
    counters: Vec<VppJsApiCounter>,
    paths: Vec<Vec<VppJsApiPath>>,
}

impl VppJsApiFile {
    pub fn from_str(
        data: &str,
        asserts: bool,
    ) -> std::result::Result<VppJsApiFile, serde_json::Error> {
        let desc = serde_json::from_str::<VppJsApiFile>(&data);
        desc
    }
}

fn parse_api_tree(opts: &Opts, root: &str, map: &mut LinkedHashMap<String, VppJsApiFile>) {
    use std::fs;
    if opts.verbose > 2 {
        println!("parse tree: {:?}", root);
    }
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if opts.verbose > 2 {
            println!("Entry: {:?}", &entry);
        }

        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_file() {
            let res = std::fs::read_to_string(&path);
            if let Ok(data) = res {
                let desc = VppJsApiFile::from_str(&data, opts.asserts);
                if let Ok(d) = desc {
                    map.insert(path.to_str().unwrap().to_string(), d);
                } else {
                    eprintln!("Error loading {:?}: {:?}", &path, &desc);
                }
            } else {
                eprintln!("Error reading {:?}: {:?}", &path, &res);
            }
        }
        if metadata.is_dir() && entry.file_name() != "." && entry.file_name() != ".." {
            parse_api_tree(opts, &path.to_str().unwrap(), map);
        }
    }
}

fn main() {
    env_logger::init();
    let opts: Opts = Opts::parse();
    log::info!("Starting file {}", &opts.in_file);

    if let Ok(data) = std::fs::read_to_string(&opts.in_file) {
        match opts.parse_type {
            OptParseType::Tree => {
                panic!("Can't parse a tree out of file!");
            }
            OptParseType::File => {
                let desc = VppJsApiFile::from_str(&data, opts.asserts).unwrap();
                eprintln!(
                    "File: {} version: {} services: {} types: {} messages: {} aliases: {} imports: {} enums: {} unions: {}",
                    &opts.in_file,
                    &desc.vl_api_version,
                    desc.services.len(),
                    desc.types.len(),
                    desc.messages.len(),
                    desc.aliases.len(),
                    desc.imports.len(),
                    desc.enums.len(),
                    desc.unions.len()
                );
                if opts.verbose > 1 {
                    println!("Dump File: {:#?}", &desc);
                }
                let data = serde_json::to_string_pretty(&desc).unwrap();
                println!("{}", &data);
            }
            OptParseType::ApiType => {
                let desc: VppJsApiType = serde_json::from_str(&data).unwrap();
                println!("Dump Type: {:#?}", &desc);
            }
            OptParseType::ApiMessage => {
                let desc: VppJsApiMessage = serde_json::from_str(&data).unwrap();
                println!("Dump: {:#?}", &desc);
            }
        }
    } else {
        match opts.parse_type {
            OptParseType::Tree => {
                // it was a directory tree, descend downwards...
                let mut api_files: LinkedHashMap<String, VppJsApiFile> = LinkedHashMap::new();
                parse_api_tree(&opts, &opts.in_file, &mut api_files);
                println!("Loaded {} API definition files", api_files.len());
            }
            e => {
                panic!("inappropriate parse type {:?} for inexistent file", e);
            }
        }
    }
}
