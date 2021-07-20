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
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44PluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub enable : bool, 
} 
impl Det44PluginEnableDisable { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_plugin_enable_disable_617b6bf8") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44PluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44PluginEnableDisableReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_plugin_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44InterfaceAddDelFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_inside : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Det44InterfaceAddDelFeature { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_interface_add_del_feature_dc17a836") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44InterfaceAddDelFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44InterfaceAddDelFeatureReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_interface_add_del_feature_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44InterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Det44InterfaceDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_interface_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44InterfaceDetails { 
	pub context : u32, 
	pub is_inside : bool, 
	pub is_outside : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Det44InterfaceDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_interface_details_e60cc5be") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44AddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
} 
impl Det44AddDelMap { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_add_del_map_1150a190") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44AddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44AddDelMapReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_add_del_map_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44Forward { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
} 
impl Det44Forward { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_forward_7f8a89cd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44ForwardReply { 
	pub context : u32, 
	pub retval : i32, 
	pub out_port_lo : u16, 
	pub out_port_hi : u16, 
	pub out_addr : Ip4Address, 
} 
impl Det44ForwardReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_forward_reply_a8ccbdc0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44Reverse { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_port : u16, 
	pub out_addr : Ip4Address, 
} 
impl Det44Reverse { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_reverse_a7573fe1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44ReverseReply { 
	pub context : u32, 
	pub retval : i32, 
	pub in_addr : Ip4Address, 
} 
impl Det44ReverseReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_reverse_reply_34066d48") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44MapDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Det44MapDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_map_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44MapDetails { 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
	pub sharing_ratio : u32, 
	pub ports_per_host : u16, 
	pub ses_num : u32, 
} 
impl Det44MapDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_map_details_ad91dc83") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44CloseSessionOut { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_addr : Ip4Address, 
	pub out_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
impl Det44CloseSessionOut { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_close_session_out_f6b259d1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44CloseSessionOutReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44CloseSessionOutReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_close_session_out_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44CloseSessionIn { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
impl Det44CloseSessionIn { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_close_session_in_3c68e073") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44CloseSessionInReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44CloseSessionInReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_close_session_in_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44SessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub user_addr : Ip4Address, 
} 
impl Det44SessionDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_session_dump_e45a3af7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44SessionDetails { 
	pub context : u32, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
	pub out_port : u16, 
	pub state : u8, 
	pub expire : u32, 
} 
impl Det44SessionDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_session_details_27f3c171") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44SetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl Det44SetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_set_timeouts_d4746b16") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44SetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Det44SetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_set_timeouts_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44GetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl Det44GetTimeouts { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_get_timeouts_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Det44GetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
impl Det44GetTimeoutsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("det44_get_timeouts_reply_3c4df4e1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetAddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
} 
impl NatDetAddDelMap { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_add_del_map_112fde05") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetAddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatDetAddDelMapReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_add_del_map_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetForward { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
} 
impl NatDetForward { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_forward_7f8a89cd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetForwardReply { 
	pub context : u32, 
	pub retval : i32, 
	pub out_port_lo : u16, 
	pub out_port_hi : u16, 
	pub out_addr : Ip4Address, 
} 
impl NatDetForwardReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_forward_reply_a8ccbdc0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetReverse { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_port : u16, 
	pub out_addr : Ip4Address, 
} 
impl NatDetReverse { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_reverse_a7573fe1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetReverseReply { 
	pub context : u32, 
	pub retval : i32, 
	pub in_addr : Ip4Address, 
} 
impl NatDetReverseReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_reverse_reply_34066d48") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetMapDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl NatDetMapDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_map_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetMapDetails { 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
	pub sharing_ratio : u32, 
	pub ports_per_host : u16, 
	pub ses_num : u32, 
} 
impl NatDetMapDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_map_details_88000ee1") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetCloseSessionOut { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_addr : Ip4Address, 
	pub out_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
impl NatDetCloseSessionOut { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_close_session_out_c1b6cbfb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetCloseSessionOutReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatDetCloseSessionOutReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_close_session_out_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetCloseSessionIn { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
impl NatDetCloseSessionIn { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_close_session_in_0a10ef64") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetCloseSessionInReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl NatDetCloseSessionInReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_close_session_in_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub user_addr : Ip4Address, 
} 
impl NatDetSessionDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_session_dump_e45a3af7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct NatDetSessionDetails { 
	pub context : u32, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
	pub out_port : u16, 
	pub state : u8, 
	pub expire : u32, 
} 
impl NatDetSessionDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("nat_det_session_details_27f3c171") 
	 } 
} 
