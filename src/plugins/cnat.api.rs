/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatEndpoint { 
	pub addr : Address, 
	pub sw_if_index : InterfaceIndex, 
	pub if_af : AddressFamily, 
	pub port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatEndpointTuple { 
	pub dst_ep : CnatEndpoint, 
	pub src_ep : CnatEndpoint, 
	pub flags : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslation { 
	pub vip : CnatEndpoint, 
	pub id : u32, 
	pub ip_proto : IpProto, 
	pub is_real_ip : u8, 
	pub flags : u8, 
	pub lb_type : CnatLbType, 
	pub n_paths : u32, 
	pub paths : CnatEndpointTuple, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSession { 
	pub src : CnatEndpoint, 
	pub dst : CnatEndpoint, 
	pub new : CnatEndpoint, 
	pub ip_proto : IpProto, 
	pub location : u8, 
	pub timestamp : f64, 
} 
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
pub enum FibPathNhProto { 
	 FIB_API_PATH_NH_PROTO_IP4=0, 
	 FIB_API_PATH_NH_PROTO_IP6=1, 
	 FIB_API_PATH_NH_PROTO_MPLS=2, 
	 FIB_API_PATH_NH_PROTO_ETHERNET=3, 
	 FIB_API_PATH_NH_PROTO_BIER=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathFlags { 
	 FIB_API_PATH_FLAG_NONE=0, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_ATTACHED=1, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_HOST=2, 
	 FIB_API_PATH_FLAG_POP_PW_CW=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathType { 
	 FIB_API_PATH_TYPE_NORMAL=0, 
	 FIB_API_PATH_TYPE_LOCAL=1, 
	 FIB_API_PATH_TYPE_DROP=2, 
	 FIB_API_PATH_TYPE_UDP_ENCAP=3, 
	 FIB_API_PATH_TYPE_BIER_IMP=4, 
	 FIB_API_PATH_TYPE_ICMP_UNREACH=5, 
	 FIB_API_PATH_TYPE_ICMP_PROHIBIT=6, 
	 FIB_API_PATH_TYPE_SOURCE_LOOKUP=7, 
	 FIB_API_PATH_TYPE_DVR=8, 
	 FIB_API_PATH_TYPE_INTERFACE_RX=9, 
	 FIB_API_PATH_TYPE_CLASSIFY=10, 
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
pub enum CnatTranslationFlags { 
	 CNAT_TRANSLATION_ALLOC_PORT=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum CnatEndpointTupleFlags { 
	 CNAT_EPT_NO_NAT=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum CnatLbType { 
	 CNAT_LB_TYPE_DEFAULT=0, 
	 CNAT_LB_TYPE_MAGLEV=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum CnatSnatPolicies { 
	 CNAT_SNAT_POLICY_NONE=1, 
} 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type InterfaceIndex=u32; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub translation : CnatTranslation, 
} 
impl CnatTranslationUpdate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_update_cd5aedf5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub id : u32, 
} 
impl CnatTranslationUpdateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_update_reply_e2fc8294") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u32, 
} 
impl CnatTranslationDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_del_3a91bde5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CnatTranslationDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationDetails { 
	pub context : u32, 
	pub translation : CnatTranslation, 
} 
impl CnatTranslationDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_details_347e1f16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatTranslationDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl CnatTranslationDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_translation_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSessionPurge { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl CnatSessionPurge { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_session_purge_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSessionPurgeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CnatSessionPurgeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_session_purge_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSessionDetails { 
	pub context : u32, 
	pub session : CnatSession, 
} 
impl CnatSessionDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_session_details_7e5017c7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl CnatSessionDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_session_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSetSnatAddresses { 
	pub client_index : u32, 
	pub context : u32, 
	pub snat_ip4 : Ip4Address, 
	pub snat_ip6 : Ip6Address, 
	pub sw_if_index : InterfaceIndex, 
} 
impl CnatSetSnatAddresses { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_set_snat_addresses_d997e96c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSetSnatAddressesReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CnatSetSnatAddressesReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_set_snat_addresses_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatGetSnatAddresses { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl CnatGetSnatAddresses { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_get_snat_addresses_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatGetSnatAddressesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub id : u32, 
	pub snat_ip4 : Ip4Address, 
	pub snat_ip6 : Ip6Address, 
	pub sw_if_index : InterfaceIndex, 
} 
impl CnatGetSnatAddressesReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_get_snat_addresses_reply_879513c1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatAddDelSnatPrefix { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub prefix : Prefix, 
} 
impl CnatAddDelSnatPrefix { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_add_del_snat_prefix_e26dd79a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatAddDelSnatPrefixReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CnatAddDelSnatPrefixReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_add_del_snat_prefix_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSetSnatPolicy { 
	pub client_index : u32, 
	pub context : u32, 
	pub policy : CnatSnatPolicies, 
} 
impl CnatSetSnatPolicy { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_set_snat_policy_5329b08d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CnatSetSnatPolicyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl CnatSetSnatPolicyReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("cnat_set_snat_policy_reply_e8d4e804") 
	 } 
} 
