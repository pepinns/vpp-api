/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44LbAddrPort { 
	pub addr : Ip4Address, 
	pub port : u16, 
	pub probability : u8, 
	pub vrf_id : u32, 
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
pub enum NatLogLevel { 
	 NAT_LOG_NONE=0, 
	 NAT_LOG_ERROR=1, 
	 NAT_LOG_WARNING=2, 
	 NAT_LOG_NOTICE=3, 
	 NAT_LOG_INFO=4, 
	 NAT_LOG_DEBUG=5, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum NatConfigFlags { 
	 NAT_IS_NONE=0, 
	 NAT_IS_TWICE_NAT=1, 
	 NAT_IS_SELF_TWICE_NAT=2, 
	 NAT_IS_OUT2IN_ONLY=4, 
	 NAT_IS_ADDR_ONLY=8, 
	 NAT_IS_OUTSIDE=16, 
	 NAT_IS_INSIDE=32, 
	 NAT_IS_STATIC=64, 
	 NAT_IS_EXT_HOST_VALID=128, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum Nat44ConfigFlags { 
	 NAT44_IS_ENDPOINT_INDEPENDENT=0, 
	 NAT44_IS_ENDPOINT_DEPENDENT=1, 
	 NAT44_IS_STATIC_MAPPING_ONLY=2, 
	 NAT44_IS_CONNECTION_TRACKING=4, 
	 NAT44_IS_OUT2IN_DPO=8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44PluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub users : u32, 
	pub user_memory : u32, 
	pub sessions : u32, 
	pub session_memory : u32, 
	pub user_sessions : u32, 
	pub enable : bool, 
	pub flags : Nat44ConfigFlags, 
} 
impl Nat44PluginEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_plugin_enable_disable_dea0d501") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44PluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44PluginEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_plugin_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdPluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub sessions : u32, 
	pub session_memory : u32, 
	pub enable : bool, 
	pub flags : Nat44ConfigFlags, 
} 
impl Nat44EdPluginEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_plugin_enable_disable_be17f8dd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdPluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44EdPluginEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_plugin_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatControlPing { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatControlPing { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_control_ping_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatControlPingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub client_index : u32, 
	pub vpe_pid : u32, 
} 
impl NatControlPingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_control_ping_reply_f6b0b8ca") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatShowConfig { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatShowConfig { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_show_config_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatShowConfigReply { 
	pub context : u32, 
	pub retval : i32, 
	pub static_mapping_only : bool, 
	pub static_mapping_connection_tracking : bool, 
	pub deterministic : bool, 
	pub endpoint_dependent : bool, 
	pub out2in_dpo : bool, 
	pub dslite_ce : bool, 
	pub translation_buckets : u32, 
	pub translation_memory_size : u32, 
	pub user_buckets : u32, 
	pub user_memory_size : u64, 
	pub max_translations_per_user : u32, 
	pub outside_vrf_id : u32, 
	pub inside_vrf_id : u32, 
	pub nat64_bib_buckets : u32, 
	pub nat64_bib_memory_size : u64, 
	pub nat64_st_buckets : u32, 
	pub nat64_st_memory_size : u64, 
} 
impl NatShowConfigReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_show_config_reply_7903ef06") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatShowConfig2 { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatShowConfig2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_show_config_2_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatShowConfig2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub static_mapping_only : bool, 
	pub static_mapping_connection_tracking : bool, 
	pub deterministic : bool, 
	pub endpoint_dependent : bool, 
	pub out2in_dpo : bool, 
	pub dslite_ce : bool, 
	pub translation_buckets : u32, 
	pub translation_memory_size : u64, 
	pub user_buckets : u32, 
	pub user_memory_size : u64, 
	pub max_translations_per_user : u32, 
	pub outside_vrf_id : u32, 
	pub inside_vrf_id : u32, 
	pub nat64_bib_buckets : u32, 
	pub nat64_bib_memory_size : u64, 
	pub nat64_st_buckets : u32, 
	pub nat64_st_memory_size : u64, 
	pub max_translations_per_thread : u32, 
	pub max_users_per_thread : u32, 
} 
impl NatShowConfig2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_show_config_2_reply_0404a5b4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ShowRunningConfig { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44ShowRunningConfig { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_show_running_config_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ShowRunningConfigReply { 
	pub context : u32, 
	pub retval : i32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub users : u32, 
	pub sessions : u32, 
	pub user_sessions : u32, 
	pub user_buckets : u32, 
	pub translation_buckets : u32, 
	pub forwarding_enabled : bool, 
	pub ipfix_logging_enabled : bool, 
	pub timeouts : NatTimeouts, 
	pub log_level : NatLogLevel, 
	pub flags : Nat44ConfigFlags, 
} 
impl Nat44ShowRunningConfigReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_show_running_config_reply_93d8e267") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44SessionCleanup { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44SessionCleanup { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_session_cleanup_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44SessionCleanupReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44SessionCleanupReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_session_cleanup_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44SetSessionLimit { 
	pub client_index : u32, 
	pub context : u32, 
	pub session_limit : u32, 
	pub vrf_id : u32, 
} 
impl Nat44SetSessionLimit { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_set_session_limit_8899bbb1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44SetSessionLimitReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44SetSessionLimitReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_set_session_limit_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetLogLevel { 
	pub client_index : u32, 
	pub context : u32, 
	pub log_level : NatLogLevel, 
} 
impl NatSetLogLevel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_log_level_70076bfe") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetLogLevelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatSetLogLevelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_log_level_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetWorkers { 
	pub client_index : u32, 
	pub context : u32, 
	pub worker_mask : u64, 
} 
impl NatSetWorkers { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_workers_da926638") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetWorkersReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatSetWorkersReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_workers_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatWorkerDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatWorkerDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_worker_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatWorkerDetails { 
	pub context : u32, 
	pub worker_index : u32, 
	pub lcore_id : u32, 
	pub name : FixedSizeString<U64>, 
} 
impl NatWorkerDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_worker_details_84bf06fc") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatIpfixEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
	pub enable : bool, 
} 
impl NatIpfixEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ipfix_enable_disable_9af4a2d2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatIpfixEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatIpfixEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ipfix_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl NatSetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_timeouts_d4746b16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatSetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_timeouts_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatGetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_timeouts_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl NatGetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_timeouts_reply_3c4df4e1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetAddrAndPortAllocAlg { 
	pub client_index : u32, 
	pub context : u32, 
	pub alg : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub psid : u16, 
	pub start_port : u16, 
	pub end_port : u16, 
} 
impl NatSetAddrAndPortAllocAlg { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_addr_and_port_alloc_alg_deeb746f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetAddrAndPortAllocAlgReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatSetAddrAndPortAllocAlgReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_addr_and_port_alloc_alg_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetAddrAndPortAllocAlg { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatGetAddrAndPortAllocAlg { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_addr_and_port_alloc_alg_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetAddrAndPortAllocAlgReply { 
	pub context : u32, 
	pub retval : i32, 
	pub alg : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub psid : u16, 
	pub start_port : u16, 
	pub end_port : u16, 
} 
impl NatGetAddrAndPortAllocAlgReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_addr_and_port_alloc_alg_reply_3607a7d0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetMssClamping { 
	pub client_index : u32, 
	pub context : u32, 
	pub mss_value : u16, 
	pub enable : bool, 
} 
impl NatSetMssClamping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_mss_clamping_25e90abb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatSetMssClampingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatSetMssClampingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_set_mss_clamping_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetMssClamping { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatGetMssClamping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_mss_clamping_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatGetMssClampingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub mss_value : u16, 
	pub enable : bool, 
} 
impl NatGetMssClampingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_get_mss_clamping_reply_1c0b2a78") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaSetListener { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub path_mtu : u32, 
} 
impl NatHaSetListener { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_set_listener_e4a8cb4e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaSetListenerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatHaSetListenerReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_set_listener_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaSetFailover { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub session_refresh_interval : u32, 
} 
impl NatHaSetFailover { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_set_failover_718246af") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaSetFailoverReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatHaSetFailoverReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_set_failover_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaGetListener { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatHaGetListener { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_get_listener_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaGetListenerReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub path_mtu : u32, 
} 
impl NatHaGetListenerReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_get_listener_reply_123ea41f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaGetFailover { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatHaGetFailover { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_get_failover_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaGetFailoverReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub session_refresh_interval : u32, 
} 
impl NatHaGetFailoverReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_get_failover_reply_a67d8752") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatHaFlush { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_flush_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatHaFlushReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_flush_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaResync { 
	pub client_index : u32, 
	pub context : u32, 
	pub want_resync_event : u8, 
	pub pid : u32, 
} 
impl NatHaResync { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_resync_c8ab9e03") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaResyncReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatHaResyncReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_resync_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatHaResyncCompletedEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub missed_count : u32, 
} 
impl NatHaResyncCompletedEvent { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_ha_resync_completed_event_fdc598fb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44DelUser { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub fib_index : u32, 
} 
impl Nat44DelUser { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_del_user_99a9f998") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44DelUserReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44DelUserReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_del_user_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelAddressRange { 
	pub client_index : u32, 
	pub context : u32, 
	pub first_ip_address : Ip4Address, 
	pub last_ip_address : Ip4Address, 
	pub vrf_id : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
} 
impl Nat44AddDelAddressRange { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_address_range_d4c7568c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelAddressRangeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelAddressRangeReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_address_range_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddressDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44AddressDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_address_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddressDetails { 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub flags : NatConfigFlags, 
	pub vrf_id : u32, 
} 
impl Nat44AddressDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_address_details_45410ac4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddDelFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat44InterfaceAddDelFeature { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_add_del_feature_f3699b83") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddDelFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44InterfaceAddDelFeatureReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_add_del_feature_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44InterfaceDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat44InterfaceDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_details_5d286289") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddDelOutputFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat44InterfaceAddDelOutputFeature { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_add_del_output_feature_f3699b83") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddDelOutputFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44InterfaceAddDelOutputFeatureReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_add_del_output_feature_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceOutputFeatureDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44InterfaceOutputFeatureDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_output_feature_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceOutputFeatureDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Nat44InterfaceOutputFeatureDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_output_feature_details_5d286289") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelStaticMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub local_ip_address : Ip4Address, 
	pub external_ip_address : Ip4Address, 
	pub protocol : u8, 
	pub local_port : u16, 
	pub external_port : u16, 
	pub external_sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<U64>, 
} 
impl Nat44AddDelStaticMapping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_static_mapping_e165e83b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelStaticMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelStaticMappingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_static_mapping_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelStaticMappingV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub match_pool : bool, 
	pub flags : NatConfigFlags, 
	pub pool_ip_address : Ip4Address, 
	pub local_ip_address : Ip4Address, 
	pub external_ip_address : Ip4Address, 
	pub protocol : u8, 
	pub local_port : u16, 
	pub external_port : u16, 
	pub external_sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<U64>, 
} 
impl Nat44AddDelStaticMappingV2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_static_mapping_v2_5e205f1a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelStaticMappingV2Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelStaticMappingV2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_static_mapping_v2_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44StaticMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44StaticMappingDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_static_mapping_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44StaticMappingDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub local_ip_address : Ip4Address, 
	pub external_ip_address : Ip4Address, 
	pub protocol : u8, 
	pub local_port : u16, 
	pub external_port : u16, 
	pub external_sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<U64>, 
} 
impl Nat44StaticMappingDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_static_mapping_details_1a433ef7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelIdentityMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub ip_address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<U64>, 
} 
impl Nat44AddDelIdentityMapping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_identity_mapping_8e12743f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelIdentityMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelIdentityMappingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_identity_mapping_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44IdentityMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44IdentityMappingDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_identity_mapping_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44IdentityMappingDetails { 
	pub context : u32, 
	pub flags : NatConfigFlags, 
	pub ip_address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<U64>, 
} 
impl Nat44IdentityMappingDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_identity_mapping_details_36d21351") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelInterfaceAddr { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub flags : NatConfigFlags, 
} 
impl Nat44AddDelInterfaceAddr { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_interface_addr_fc835325") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelInterfaceAddrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelInterfaceAddrReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_interface_addr_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddrDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44InterfaceAddrDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_addr_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44InterfaceAddrDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub flags : NatConfigFlags, 
} 
impl Nat44InterfaceAddrDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_interface_addr_details_3e687514") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44UserDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44UserDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_user_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44UserDetails { 
	pub context : u32, 
	pub vrf_id : u32, 
	pub ip_address : Ip4Address, 
	pub nsessions : u32, 
	pub nstaticsessions : u32, 
} 
impl Nat44UserDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_user_details_355896c2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44UserSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub vrf_id : u32, 
} 
impl Nat44UserSessionDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_user_session_dump_e1899c98") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44UserSessionDetails { 
	pub context : u32, 
	pub outside_ip_address : Ip4Address, 
	pub outside_port : u16, 
	pub inside_ip_address : Ip4Address, 
	pub inside_port : u16, 
	pub protocol : u16, 
	pub flags : NatConfigFlags, 
	pub last_heard : u64, 
	pub total_bytes : u64, 
	pub total_pkts : u32, 
	pub ext_host_address : Ip4Address, 
	pub ext_host_port : u16, 
	pub ext_host_nat_address : Ip4Address, 
	pub ext_host_nat_port : u16, 
} 
impl Nat44UserSessionDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_user_session_details_1965fd69") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelLbStaticMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub flags : NatConfigFlags, 
	pub external_addr : Ip4Address, 
	pub external_port : u16, 
	pub protocol : u8, 
	pub affinity : u32, 
	pub tag : FixedSizeString<U64>, 
	pub local_num : u32, 
	pub locals : Nat44LbAddrPort, 
} 
impl Nat44AddDelLbStaticMapping { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_lb_static_mapping_53b24611") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44AddDelLbStaticMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44AddDelLbStaticMappingReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_add_del_lb_static_mapping_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44LbStaticMappingAddDelLocal { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub external_addr : Ip4Address, 
	pub external_port : u16, 
	pub protocol : u8, 
	pub local : Nat44LbAddrPort, 
} 
impl Nat44LbStaticMappingAddDelLocal { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_lb_static_mapping_add_del_local_2910a151") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44LbStaticMappingAddDelLocalReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44LbStaticMappingAddDelLocalReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_lb_static_mapping_add_del_local_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44LbStaticMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44LbStaticMappingDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_lb_static_mapping_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44LbStaticMappingDetails { 
	pub context : u32, 
	pub external_addr : Ip4Address, 
	pub external_port : u16, 
	pub protocol : u8, 
	pub flags : NatConfigFlags, 
	pub affinity : u32, 
	pub tag : FixedSizeString<U64>, 
	pub local_num : u32, 
	pub locals : Nat44LbAddrPort, 
} 
impl Nat44LbStaticMappingDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_lb_static_mapping_details_2267b9e8") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44DelSession { 
	pub client_index : u32, 
	pub context : u32, 
	pub address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub vrf_id : u32, 
	pub flags : NatConfigFlags, 
	pub ext_host_address : Ip4Address, 
	pub ext_host_port : u16, 
} 
impl Nat44DelSession { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_del_session_4c49c387") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44DelSessionReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44DelSessionReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_del_session_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ForwardingEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
} 
impl Nat44ForwardingEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_forwarding_enable_disable_b3e225d2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ForwardingEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44ForwardingEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_forwarding_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ForwardingIsEnabled { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44ForwardingIsEnabled { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_forwarding_is_enabled_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44ForwardingIsEnabledReply { 
	pub context : u32, 
	pub enabled : bool, 
} 
impl Nat44ForwardingIsEnabledReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_forwarding_is_enabled_reply_46924a06") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdSetFqOptions { 
	pub client_index : u32, 
	pub context : u32, 
	pub frame_queue_nelts : u32, 
} 
impl Nat44EdSetFqOptions { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_set_fq_options_2399bd71") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdSetFqOptionsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Nat44EdSetFqOptionsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_set_fq_options_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdShowFqOptions { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Nat44EdShowFqOptions { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_show_fq_options_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Nat44EdShowFqOptionsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub frame_queue_nelts : u32, 
} 
impl Nat44EdShowFqOptionsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat44_ed_show_fq_options_reply_7213b545") 
	 } 
} 
