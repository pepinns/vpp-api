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
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
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
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type InterfaceIndex=u32; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2PluginGetVersion { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Ikev2PluginGetVersion { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_plugin_get_version_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2PluginGetVersionReply { 
	pub context : u32, 
	pub major : u32, 
	pub minor : u32, 
} 
impl Ikev2PluginGetVersionReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_plugin_get_version_reply_9b32cf86") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Ikev2ProfileDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileDetails { 
	pub context : u32, 
	pub profile : Ikev2Profile, 
} 
impl Ikev2ProfileDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_details_670d01d9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Ikev2SaDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_sa_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaDetails { 
	pub context : u32, 
	pub retval : i32, 
	pub sa : Ikev2Sa, 
} 
impl Ikev2SaDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_sa_details_937c22d5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ChildSaDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sa_index : u32, 
} 
impl Ikev2ChildSaDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_child_sa_dump_01eab609") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ChildSaDetails { 
	pub context : u32, 
	pub retval : i32, 
	pub child_sa : Ikev2ChildSa, 
} 
impl Ikev2ChildSaDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_child_sa_details_ff67741f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2NonceGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_initiator : bool, 
	pub sa_index : u32, 
} 
impl Ikev2NonceGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_nonce_get_7fe9ad51") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2NonceGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub data_len : u32, 
	pub nonce : u8, 
} 
impl Ikev2NonceGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_nonce_get_reply_1b37a342") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2TrafficSelectorDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_initiator : bool, 
	pub sa_index : u32, 
	pub child_sa_index : u32, 
} 
impl Ikev2TrafficSelectorDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_traffic_selector_dump_a7385e33") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2TrafficSelectorDetails { 
	pub context : u32, 
	pub retval : i32, 
	pub ts : Ikev2Ts, 
} 
impl Ikev2TrafficSelectorDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_traffic_selector_details_518cb06f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub is_add : bool, 
} 
impl Ikev2ProfileAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_add_del_2c925b55") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetAuth { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub auth_method : u8, 
	pub is_hex : bool, 
	pub data_len : u32, 
	pub data : u8, 
} 
impl Ikev2ProfileSetAuth { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_auth_642c97cd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetAuthReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetAuthReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_auth_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetId { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub is_local : bool, 
	pub id_type : u8, 
	pub data_len : u32, 
	pub data : u8, 
} 
impl Ikev2ProfileSetId { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_id_4d7e2418") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetIdReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetIdReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_id_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileDisableNatt { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
} 
impl Ikev2ProfileDisableNatt { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_disable_natt_ebf79a66") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileDisableNattReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileDisableNattReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_disable_natt_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetTs { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub ts : Ikev2Ts, 
} 
impl Ikev2ProfileSetTs { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_ts_8eb8cfd1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetTsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetTsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_ts_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetLocalKey { 
	pub client_index : u32, 
	pub context : u32, 
	pub key_file : FixedSizeString<U256>, 
} 
impl Ikev2SetLocalKey { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_local_key_799b69ec") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetLocalKeyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetLocalKeyReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_local_key_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetTunnelInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Ikev2SetTunnelInterface { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_tunnel_interface_ca67182c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetTunnelInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetTunnelInterfaceReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_tunnel_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetResponder { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub responder : Ikev2Responder, 
} 
impl Ikev2SetResponder { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_responder_a2055df1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetResponderReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetResponderReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_responder_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetIkeTransforms { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub tr : Ikev2IkeTransforms, 
} 
impl Ikev2SetIkeTransforms { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_ike_transforms_076d7378") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetIkeTransformsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetIkeTransformsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_ike_transforms_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetEspTransforms { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub tr : Ikev2EspTransforms, 
} 
impl Ikev2SetEspTransforms { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_esp_transforms_a63dc205") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetEspTransformsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetEspTransformsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_esp_transforms_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetSaLifetime { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
	pub lifetime : u64, 
	pub lifetime_jitter : u32, 
	pub handover : u32, 
	pub lifetime_maxdata : u64, 
} 
impl Ikev2SetSaLifetime { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_sa_lifetime_7039feaa") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SetSaLifetimeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2SetSaLifetimeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_set_sa_lifetime_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateSaInit { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
} 
impl Ikev2InitiateSaInit { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_sa_init_ebf79a66") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateSaInitReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2InitiateSaInitReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_sa_init_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateDelIkeSa { 
	pub client_index : u32, 
	pub context : u32, 
	pub ispi : u64, 
} 
impl Ikev2InitiateDelIkeSa { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_del_ike_sa_8d125bdd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateDelIkeSaReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2InitiateDelIkeSaReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_del_ike_sa_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateDelChildSa { 
	pub client_index : u32, 
	pub context : u32, 
	pub ispi : u32, 
} 
impl Ikev2InitiateDelChildSa { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_del_child_sa_7f004d2e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateDelChildSaReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2InitiateDelChildSaReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_del_child_sa_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateRekeyChildSa { 
	pub client_index : u32, 
	pub context : u32, 
	pub ispi : u32, 
} 
impl Ikev2InitiateRekeyChildSa { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_rekey_child_sa_7f004d2e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2InitiateRekeyChildSaReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2InitiateRekeyChildSaReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_initiate_rekey_child_sa_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetUdpEncap { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<U64>, 
} 
impl Ikev2ProfileSetUdpEncap { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_udp_encap_ebf79a66") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetUdpEncapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetUdpEncapReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_udp_encap_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetIpsecUdpPort { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_set : u8, 
	pub port : u16, 
	pub name : FixedSizeString<U64>, 
} 
impl Ikev2ProfileSetIpsecUdpPort { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_ipsec_udp_port_615ce758") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetIpsecUdpPortReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetIpsecUdpPortReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_ipsec_udp_port_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetLiveness { 
	pub client_index : u32, 
	pub context : u32, 
	pub period : u32, 
	pub max_retries : u32, 
} 
impl Ikev2ProfileSetLiveness { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_liveness_6bdf4d65") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ProfileSetLivenessReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ikev2ProfileSetLivenessReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ikev2_profile_set_liveness_reply_e8d4e804") 
	 } 
} 
