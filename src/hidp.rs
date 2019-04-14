use crate::bluetooth::bdaddr_t;
use libc::c_int;
// HIDP defaults
pub const HIDP_MINIMUM_MTU: i32 = 48;
pub const HIDP_DEFAULT_MTU: i32 = 48;
// HIDP ioctl defines
pub const HIDPCONNADD: u64 = request_code_write!('H', 200, std::mem::size_of::<c_int>());
pub const HIDPCONNDEL: u64 = request_code_write!('H', 201, std::mem::size_of::<c_int>());
pub const HIDPGETCONNLIST: u64 = request_code_read!('H', 210, std::mem::size_of::<c_int>());
pub const HIDPGETCONNINFO: u64 = request_code_read!('H', 211, std::mem::size_of::<c_int>());
pub const HIDP_VIRTUAL_CABLE_UNPLUG: i32 = 0;
pub const HIDP_BOOT_PROTOCOL_MODE: i32 = 1;
pub const HIDP_BLUETOOTH_VENDOR_ID: i32 = 9;
STRUCT! {struct hidp_connadd_req {
    ctrl_sock: c_int,   // Connected control socket
    intr_sock: c_int,   // Connected interrupt socket
    parser: u16,        // Parser version
    rd_size: u16,       // Report descriptor size
    rd_data: *mut u8,   // Report descriptor data
    country: u8,
    subclass: u8,
    vendor: u16,
    product: u16,
    version: u16,
    flags: u32,
    idle_to: u32,
    name: [char; 128],  // Device name
}}
STRUCT! {struct hidp_conndel_req {
    bdaddr: bdaddr_t,
    flags: u32,
}}
STRUCT! {struct hidp_conninfo {
    bdaddr: bdaddr_t,
    flags: u32,
    state: u16,
    vendor: u16,
    product: u16,
    version: u16,
    name: [char; 128],
}}
STRUCT! {struct hidp_connlist_req {
    cnum: u32,
    ci: *mut hidp_conninfo,
}}
