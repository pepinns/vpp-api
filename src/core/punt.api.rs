/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntL4 { 
	pub af : AddressFamily, 
	pub protocol : IpProto, 
	pub port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntIpProto { 
	pub af : AddressFamily, 
	pub protocol : IpProto, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntException { 
	pub id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Punt { 
	pub typ : PuntType, 
	pub punt : PuntUnion, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReason { 
	pub id : u32, 
	pub name : String, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union punt_union { 
	 exception : PuntException, 
	 l4 : PuntL4, 
	 ip_proto : PuntIpProto, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum AddressFamily { 
	 ADDRESS_IP4=0, 
	 ADDRESS_IP6=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpFeatureLocation { 
	 IP_API_FEATURE_INPUT=0, 
	 IP_API_FEATURE_OUTPUT=1, 
	 IP_API_FEATURE_LOCAL=2, 
	 IP_API_FEATURE_PUNT=3, 
	 IP_API_FEATURE_DROP=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpEcn { 
	 IP_API_ECN_NONE=0, 
	 IP_API_ECN_ECT0=1, 
	 IP_API_ECN_ECT1=2, 
	 IP_API_ECN_CE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpDscp { 
	 IP_API_DSCP_CS0=0, 
	 IP_API_DSCP_CS1=8, 
	 IP_API_DSCP_AF11=10, 
	 IP_API_DSCP_AF12=12, 
	 IP_API_DSCP_AF13=14, 
	 IP_API_DSCP_CS2=16, 
	 IP_API_DSCP_AF21=18, 
	 IP_API_DSCP_AF22=20, 
	 IP_API_DSCP_AF23=22, 
	 IP_API_DSCP_CS3=24, 
	 IP_API_DSCP_AF31=26, 
	 IP_API_DSCP_AF32=28, 
	 IP_API_DSCP_AF33=30, 
	 IP_API_DSCP_CS4=32, 
	 IP_API_DSCP_AF41=34, 
	 IP_API_DSCP_AF42=36, 
	 IP_API_DSCP_AF43=38, 
	 IP_API_DSCP_CS5=40, 
	 IP_API_DSCP_EF=46, 
	 IP_API_DSCP_CS6=48, 
	 IP_API_DSCP_CS7=50, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpProto { 
	 IP_API_PROTO_HOPOPT=0, 
	 IP_API_PROTO_ICMP=1, 
	 IP_API_PROTO_IGMP=2, 
	 IP_API_PROTO_TCP=6, 
	 IP_API_PROTO_UDP=17, 
	 IP_API_PROTO_GRE=47, 
	 IP_API_PROTO_ESP=50, 
	 IP_API_PROTO_AH=51, 
	 IP_API_PROTO_ICMP6=58, 
	 IP_API_PROTO_EIGRP=88, 
	 IP_API_PROTO_OSPF=89, 
	 IP_API_PROTO_SCTP=132, 
	 IP_API_PROTO_RESERVED=255, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum PuntType { 
	 PUNT_API_TYPE_L4=1, 
	 PUNT_API_TYPE_IP_PROTO=2, 
	 PUNT_API_TYPE_EXCEPTION=3, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetPunt { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub punt : Punt, 
} 
impl SetPunt { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_punt_83799618") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetPuntReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SetPuntReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_punt_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketRegister { 
	pub client_index : u32, 
	pub context : u32, 
	pub header_version : u32, 
	pub punt : Punt, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketRegister { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_register_c8cd10fa") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketRegisterReply { 
	pub context : u32, 
	pub retval : i32, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketRegisterReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_register_reply_bd30ae90") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : PuntType, 
} 
impl PuntSocketDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_dump_52974935") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDetails { 
	pub context : u32, 
	pub punt : Punt, 
	pub pathname : FixedSizeString<U108>, 
} 
impl PuntSocketDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_details_1de0ce75") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDeregister { 
	pub client_index : u32, 
	pub context : u32, 
	pub punt : Punt, 
} 
impl PuntSocketDeregister { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_deregister_98a444f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntSocketDeregisterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl PuntSocketDeregisterReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_socket_deregister_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReasonDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub reason : PuntReason, 
} 
impl PuntReasonDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_reason_dump_5c0dd4fe") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PuntReasonDetails { 
	pub context : u32, 
	pub reason : PuntReason, 
} 
impl PuntReasonDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("punt_reason_details_2c9d4a40") 
	 } 
} 
