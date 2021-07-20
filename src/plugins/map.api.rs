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
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapAddDomain { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip6_prefix : Ip6Prefix, 
	pub ip4_prefix : Ip4Prefix, 
	pub ip6_src : Ip6Prefix, 
	pub ea_bits_len : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub mtu : u16, 
	pub tag : FixedSizeString<U64>, 
} 
impl MapAddDomain { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_add_domain_7a5a18c9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapAddDomainReply { 
	pub context : u32, 
	pub index : u32, 
	pub retval : i32, 
} 
impl MapAddDomainReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_add_domain_reply_3e6d4e2c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDelDomain { 
	pub client_index : u32, 
	pub context : u32, 
	pub index : u32, 
} 
impl MapDelDomain { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_del_domain_8ac76db6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDelDomainReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapDelDomainReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_del_domain_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapAddDelRule { 
	pub client_index : u32, 
	pub context : u32, 
	pub index : u32, 
	pub is_add : bool, 
	pub ip6_dst : Ip6Address, 
	pub psid : u16, 
} 
impl MapAddDelRule { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_add_del_rule_c65b32f7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapAddDelRuleReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapAddDelRuleReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_add_del_rule_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDomainsGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub cursor : u32, 
} 
impl MapDomainsGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_domains_get_f75ba505") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDomainsGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub cursor : u32, 
} 
impl MapDomainsGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_domains_get_reply_53b48f5d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDomainDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl MapDomainDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_domain_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapDomainDetails { 
	pub context : u32, 
	pub domain_index : u32, 
	pub ip6_prefix : Ip6Prefix, 
	pub ip4_prefix : Ip4Prefix, 
	pub ip6_src : Ip6Prefix, 
	pub ea_bits_len : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub flags : u8, 
	pub mtu : u16, 
	pub tag : FixedSizeString<U64>, 
} 
impl MapDomainDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_domain_details_fc1859dd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapRuleDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub domain_index : u32, 
} 
impl MapRuleDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_rule_dump_e43e6ff6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapRuleDetails { 
	pub context : u32, 
	pub ip6_dst : Ip6Address, 
	pub psid : u16, 
} 
impl MapRuleDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_rule_details_c7cbeea5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapIfEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_enable : bool, 
	pub is_translation : bool, 
} 
impl MapIfEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_if_enable_disable_59bb32f4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapIfEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapIfEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_if_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapSummaryStats { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl MapSummaryStats { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_summary_stats_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapSummaryStatsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub total_bindings : u64, 
	pub total_pkts : u64, 
	pub total_bytes : u64, 
	pub total_ip4_fragments : u64, 
	pub total_security_check : u64, 
} 
impl MapSummaryStatsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_summary_stats_reply_0e4ace0e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetFragmentation { 
	pub client_index : u32, 
	pub context : u32, 
	pub inner : bool, 
	pub ignore_df : bool, 
} 
impl MapParamSetFragmentation { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_fragmentation_9ff54d90") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetFragmentationReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetFragmentationReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_fragmentation_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetIcmp { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip4_err_relay_src : Ip4Address, 
} 
impl MapParamSetIcmp { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_icmp_58210cbf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetIcmpReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetIcmpReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_icmp_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetIcmp6 { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_unreachable : bool, 
} 
impl MapParamSetIcmp6 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_icmp6_5d01f8c1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetIcmp6Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetIcmp6Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_icmp6_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamAddDelPreResolve { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip4_nh_address : Ip4Address, 
	pub ip6_nh_address : Ip6Address, 
} 
impl MapParamAddDelPreResolve { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_add_del_pre_resolve_17008c66") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamAddDelPreResolveReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamAddDelPreResolveReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_add_del_pre_resolve_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetSecurityCheck { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub fragments : bool, 
} 
impl MapParamSetSecurityCheck { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_security_check_6abe9836") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetSecurityCheckReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetSecurityCheckReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_security_check_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetTrafficClass { 
	pub client_index : u32, 
	pub context : u32, 
	pub copy : bool, 
	pub tc_class : u8, 
} 
impl MapParamSetTrafficClass { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_traffic_class_9cac455c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetTrafficClassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetTrafficClassReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_traffic_class_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetTcp { 
	pub client_index : u32, 
	pub context : u32, 
	pub tcp_mss : u16, 
} 
impl MapParamSetTcp { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_tcp_87a825d9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamSetTcpReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl MapParamSetTcpReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_set_tcp_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl MapParamGet { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_get_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct MapParamGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub frag_inner : u8, 
	pub frag_ignore_df : u8, 
	pub icmp_ip4_err_relay_src : Ip4Address, 
	pub icmp6_enable_unreachable : bool, 
	pub ip4_nh_address : Ip4Address, 
	pub ip6_nh_address : Ip6Address, 
	pub ip4_lifetime_ms : u16, 
	pub ip4_pool_size : u16, 
	pub ip4_buffers : u32, 
	pub ip4_ht_ratio : f64, 
	pub sec_check_enable : bool, 
	pub sec_check_fragments : bool, 
	pub tc_copy : bool, 
	pub tc_class : u8, 
} 
impl MapParamGetReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("map_param_get_reply_28092156") 
	 } 
} 
