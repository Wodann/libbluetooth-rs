use libc::{c_char, c_int};
pub const ETH_ALEN: usize = 6;
// BNEP UUIDs
pub const BNEP_BASE_UUID: u128 = 0x0000000000001000800000805F9B34FB;
pub const BNEP_UUID16: i32 = 0x02;
pub const BNEP_UUID32: i32 = 0x04;
pub const BNEP_UUID128: i32 = 0x16;
pub const BNEP_SVC_PANU: i32 = 0x1115;
pub const BNEP_SVC_NAP: i32 = 0x1116;
pub const BNEP_SVC_GN: i32 = 0x1117;
// BNEP packet types
pub const BNEP_GENERAL: i32 = 0x00;
pub const BNEP_CONTROL: i32 = 0x01;
pub const BNEP_COMPRESSED: i32 = 0x02;
pub const BNEP_COMPRESSED_SRC_ONLY: i32 = 0x03;
pub const BNEP_COMPRESSED_DST_ONLY: i32 = 0x04;
// BNEP control types
pub const BNEP_CMD_NOT_UNDERSTOOD: i32 = 0x00;
pub const BNEP_SETUP_CONN_REQ: i32 = 0x01;
pub const BNEP_SETUP_CONN_RSP: i32 = 0x02;
pub const BNEP_FILTER_NET_TYPE_SET: i32 = 0x03;
pub const BNEP_FILTER_NET_TYPE_RSP: i32 = 0x04;
pub const BNEP_FILTER_MULT_ADDR_SET: i32 = 0x05;
pub const BNEP_FILTER_MULT_ADDR_RSP: i32 = 0x06;
// BNEP response messages
pub const BNEP_SUCCESS: i32 = 0x00;
pub const BNEP_CONN_INVALID_DST: i32 = 0x01;
pub const BNEP_CONN_INVALID_SRC: i32 = 0x02;
pub const BNEP_CONN_INVALID_SVC: i32 = 0x03;
pub const BNEP_CONN_NOT_ALLOWED: i32 = 0x04;
pub const BNEP_FILTER_UNSUPPORTED_REQ: i32 = 0x01;
pub const BNEP_FILTER_INVALID_RANGE: i32 = 0x02;
pub const BNEP_FILTER_INVALID_MCADDR: i32 = 0x02;
pub const BNEP_FILTER_LIMIT_REACHED: i32 = 0x03;
pub const BNEP_FILTER_DENIED_SECURITY: i32 = 0x04;
// L2CAP settings
pub const BNEP_MTU: i32 = 1691;
pub const BNEP_FLUSH_TO: i32 = 0xffff;
pub const BNEP_CONNECT_TO: i32 = 15;
pub const BNEP_FILTER_TO: i32 = 15;
pub const BNEP_PSM: i32 = 0x0f;
// BNEP headers
pub const BNEP_TYPE_MASK: i32 = 0x7f;
pub const BNEP_EXT_HEADER: i32 = 0x80;
STRUCT! {#[repr(packed)] struct bnep_setup_conn_req {
    type_: u8,
    ctrl: u8,
    uuid_size: u8,
    service: [u8; 0],
}}
STRUCT! {#[repr(packed)] struct bnep_set_filter_req {
    type_: u8,
    ctrl: u8,
    len: u16,
    list: [u8; 0],
}}
STRUCT! {#[repr(packed)] struct bnep_ctrl_cmd_not_understood_cmd {
    type_: u8,
    ctrl: u8,
    unkn_ctrl: u8,
}}
STRUCT! {#[repr(packed)] struct bnep_control_rsp {
    type_: u8,
    ctrl: u8,
    resp: u16,
}}
STRUCT! {#[repr(packed)] struct bnep_ext_hdr {
    type_: u8,
    len: u8,
    data: [u8; 0],
}}
// BNEP ioctl defines
pub const BNEPCONNADD: c_int = request_code_write!('B', 200, std::mem::size_of::<c_int>());
pub const BNEPCONNDEL: c_int = request_code_write!('B', 201, std::mem::size_of::<c_int>());
pub const BNEPGETCONNLIST: c_int = request_code_read!('B', 210, std::mem::size_of::<c_int>());
pub const BNEPGETCONNINFO: c_int = request_code_read!('B', 211, std::mem::size_of::<c_int>());
pub const BNEPGETSUPPFEAT: c_int = request_code_read!('B', 212, std::mem::size_of::<c_int>());
pub const BNEP_SETUP_RESPONSE: i32 = 0;
STRUCT! {struct bnep_connadd_req {
    sock: c_char,           // Connected socket
    flags: u32,
    role: u16,
    device: [c_char; 16],   // Name of the Ethernet device
}}
STRUCT! {struct bnep_conndel_req {
    flags: u32,
    dst: [u8; ETH_ALEN],
}}
STRUCT! {struct bnep_conninfo {
    flags: u32,
    role: u16,
    state: u16,
    dst: [u8; ETH_ALEN],
    device: [c_char; 16],
}}
STRUCT! {struct bnep_connlist_req {
    cnum: u32,
    ci: *mut bnep_conninfo,
}}
