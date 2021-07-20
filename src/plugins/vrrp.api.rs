/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrKey { 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrConf { 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub priority : u8, 
	pub interval : u16, 
	pub flags : VrrpVrFlags, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTracking { 
	pub interfaces_dec : u32, 
	pub priority : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrRuntime { 
	pub state : VrrpVrState, 
	pub master_adv_int : u16, 
	pub skew : u16, 
	pub master_down_int : u16, 
	pub mac : MacAddress, 
	pub tracking : VrrpVrTracking, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTrackIf { 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
} 
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
pub enum VrrpVrFlags { 
	 VRRP_API_VR_PREEMPT=1, 
	 VRRP_API_VR_ACCEPT=2, 
	 VRRP_API_VR_UNICAST=4, 
	 VRRP_API_VR_IPV6=8, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VrrpVrState { 
	 VRRP_API_VR_STATE_INIT=0, 
	 VRRP_API_VR_STATE_BACKUP=1, 
	 VRRP_API_VR_STATE_MASTER=2, 
	 VRRP_API_VR_STATE_INTF_DOWN=3, 
} 
pub type InterfaceIndex=u32; 
pub type Ip4Address=[u8;4]; 
pub type Ip6Address=[u8;16]; 
pub type AddressWithPrefix=Prefix; 
pub type Ip4AddressWithPrefix=Ip4Prefix; 
pub type Ip6AddressWithPrefix=Ip6Prefix; 
pub type MacAddress=[u8;6]; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub priority : u8, 
	pub interval : u16, 
	pub flags : VrrpVrFlags, 
	pub n_addrs : u8, 
	pub addrs : Address, 
} 
impl VrrpVrAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_add_del_6dc4b881") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VrrpVrAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VrrpVrDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrDetails { 
	pub context : u32, 
	pub config : VrrpVrConf, 
	pub runtime : VrrpVrRuntime, 
	pub n_addrs : u8, 
	pub addrs : Address, 
} 
impl VrrpVrDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_details_0412fa71") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrStartStop { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub is_start : u8, 
} 
impl VrrpVrStartStop { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_start_stop_0662a3b7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrStartStopReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VrrpVrStartStopReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_start_stop_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrSetPeers { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_addrs : u8, 
	pub addrs : Address, 
} 
impl VrrpVrSetPeers { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_set_peers_baa2e52b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrSetPeersReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VrrpVrSetPeersReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_set_peers_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrPeerDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
} 
impl VrrpVrPeerDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_peer_dump_6fa3f7c4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrPeerDetails { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_peer_addrs : u8, 
	pub peer_addrs : Address, 
} 
impl VrrpVrPeerDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_peer_details_abd9145e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTrackIfAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
	pub is_add : u8, 
	pub n_ifs : u8, 
	pub ifs : VrrpVrTrackIf, 
} 
impl VrrpVrTrackIfAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_track_if_add_del_337f4ba4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTrackIfAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VrrpVrTrackIfAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_track_if_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTrackIfDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
	pub dump_all : u8, 
} 
impl VrrpVrTrackIfDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_track_if_dump_a34dfc6d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrTrackIfDetails { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_ifs : u8, 
	pub ifs : VrrpVrTrackIf, 
} 
impl VrrpVrTrackIfDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_track_if_details_99bcca9c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VrrpVrEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub vr : VrrpVrKey, 
	pub old_state : VrrpVrState, 
	pub new_state : VrrpVrState, 
} 
impl VrrpVrEvent { 
	 pub fn get_message_id() -> String { 
	 	 String::from("vrrp_vr_event_c1fea6a5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantVrrpVrEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub pid : u32, 
} 
impl WantVrrpVrEvents { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_vrrp_vr_events_c5e2af94") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct WantVrrpVrEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl WantVrrpVrEventsReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("want_vrrp_vr_events_reply_e8d4e804") 
	 } 
} 
