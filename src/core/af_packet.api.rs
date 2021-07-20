/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfStatusFlags { 
	 IF_STATUS_API_FLAG_ADMIN_UP=1, 
	 IF_STATUS_API_FLAG_LINK_UP=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum MtuProto { 
	 MTU_PROTO_API_L3=0, 
	 MTU_PROTO_API_IP4=1, 
	 MTU_PROTO_API_IP6=2, 
	 MTU_PROTO_API_MPLS=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LinkDuplex { 
	 LINK_DUPLEX_API_UNKNOWN=0, 
	 LINK_DUPLEX_API_HALF=1, 
	 LINK_DUPLEX_API_FULL=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SubIfFlags { 
	 SUB_IF_API_FLAG_NO_TAGS=1, 
	 SUB_IF_API_FLAG_ONE_TAG=2, 
	 SUB_IF_API_FLAG_TWO_TAGS=4, 
	 SUB_IF_API_FLAG_DOT1AD=8, 
	 SUB_IF_API_FLAG_EXACT_MATCH=16, 
	 SUB_IF_API_FLAG_DEFAULT=32, 
	 SUB_IF_API_FLAG_OUTER_VLAN_ID_ANY=64, 
	 SUB_IF_API_FLAG_INNER_VLAN_ID_ANY=128, 
	 SUB_IF_API_FLAG_MASK_VNET=254, 
	 SUB_IF_API_FLAG_DOT1AH=256, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum RxMode { 
	 RX_MODE_API_UNKNOWN=0, 
	 RX_MODE_API_POLLING=1, 
	 RX_MODE_API_INTERRUPT=2, 
	 RX_MODE_API_ADAPTIVE=3, 
	 RX_MODE_API_DEFAULT=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfType { 
	 IF_API_TYPE_HARDWARE=0, 
	 IF_API_TYPE_SUB=1, 
	 IF_API_TYPE_P2P=2, 
	 IF_API_TYPE_PIPE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum Direction { 
	 RX=0, 
	 TX=1, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub hw_addr : MacAddress, 
	pub use_random_hw_addr : bool, 
	pub host_if_name : FixedSizeString<U64>, 
} 
impl AfPacketCreate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_create_a190415f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl AfPacketCreateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_create_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub host_if_name : FixedSizeString<U64>, 
} 
impl AfPacketDelete { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_delete_863fa648") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AfPacketDeleteReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_delete_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketSetL4CksumOffload { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub set : bool, 
} 
impl AfPacketSetL4CksumOffload { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_set_l4_cksum_offload_319cd5c8") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketSetL4CksumOffloadReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AfPacketSetL4CksumOffloadReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_set_l4_cksum_offload_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl AfPacketDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AfPacketDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub host_if_name : FixedSizeString<U64>, 
} 
impl AfPacketDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("af_packet_details_58c7c042") 
	 } 
} 
