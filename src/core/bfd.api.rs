/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
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
pub enum BfdState { 
	 BFD_STATE_API_ADMIN_DOWN=0, 
	 BFD_STATE_API_DOWN=1, 
	 BFD_STATE_API_INIT=2, 
	 BFD_STATE_API_UP=3, 
} 
pub type InterfaceIndex=u32; 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSetEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl BfdUdpSetEchoSource { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_set_echo_source_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSetEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpSetEchoSourceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_set_echo_source_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpDelEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl BfdUdpDelEchoSource { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_del_echo_source_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpDelEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpDelEchoSourceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_del_echo_source_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpGetEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl BfdUdpGetEchoSource { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_get_echo_source_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpGetEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_set : bool, 
	pub have_usable_ip4 : bool, 
	pub ip4_addr : Ip4Address, 
	pub have_usable_ip6 : bool, 
	pub ip6_addr : Ip6Address, 
} 
impl BfdUdpGetEchoSourceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_get_echo_source_reply_1e00cfce") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub desired_min_tx : u32, 
	pub required_min_rx : u32, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub detect_mult : u8, 
	pub is_authenticated : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
} 
impl BfdUdpAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_add_7a6d1185") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_add_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpMod { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub desired_min_tx : u32, 
	pub required_min_rx : u32, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub detect_mult : u8, 
} 
impl BfdUdpMod { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_mod_783a3ff6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpModReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpModReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_mod_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
} 
impl BfdUdpDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_del_8096514d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl BfdUdpSessionDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_session_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSessionDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub state : BfdState, 
	pub is_authenticated : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
	pub required_min_rx : u32, 
	pub desired_min_tx : u32, 
	pub detect_mult : u8, 
} 
impl BfdUdpSessionDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_session_details_60653c02") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSessionSetFlags { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub flags : IfStatusFlags, 
} 
impl BfdUdpSessionSetFlags { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_session_set_flags_cf313851") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpSessionSetFlagsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpSessionSetFlagsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_session_set_flags_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantBfdEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub pid : u32, 
} 
impl WantBfdEvents { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_bfd_events_c5e2af94") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantBfdEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantBfdEventsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_bfd_events_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthSetKey { 
	pub client_index : u32, 
	pub context : u32, 
	pub conf_key_id : u32, 
	pub key_len : u8, 
	pub auth_type : u8, 
	pub key : u8, 
} 
impl BfdAuthSetKey { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_set_key_690b8877") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthSetKeyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdAuthSetKeyReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_set_key_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthDelKey { 
	pub client_index : u32, 
	pub context : u32, 
	pub conf_key_id : u32, 
} 
impl BfdAuthDelKey { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_del_key_65310b22") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthDelKeyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdAuthDelKeyReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_del_key_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthKeysDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl BfdAuthKeysDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_keys_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdAuthKeysDetails { 
	pub context : u32, 
	pub conf_key_id : u32, 
	pub use_count : u32, 
	pub auth_type : u8, 
} 
impl BfdAuthKeysDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_auth_keys_details_84130e9f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAuthActivate { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub is_delayed : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
} 
impl BfdUdpAuthActivate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_auth_activate_493ee0ec") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAuthActivateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpAuthActivateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_auth_activate_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAuthDeactivate { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub is_delayed : bool, 
} 
impl BfdUdpAuthDeactivate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_auth_deactivate_99978c32") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BfdUdpAuthDeactivateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl BfdUdpAuthDeactivateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("bfd_udp_auth_deactivate_reply_e8d4e804") 
	 } 
} 
