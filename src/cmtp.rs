use crate::bluetooth::bdaddr_t;
use libc::c_int;
// CMTP defaults
pub const CMTP_MINIMUM_MTU: i32 = 152;
pub const CMTP_DEFAULT_MTU: i32 = 672;
// CMTP ioctl defines
pub const CMTPCONNADD: libc::c_ulong = request_code_write!('C', 200, std::mem::size_of::<c_int>());
pub const CMTPCONNDEL: libc::c_ulong = request_code_write!('C', 201, std::mem::size_of::<c_int>());
pub const CMTPGETCONNLIST: libc::c_ulong = request_code_read!('C', 210, std::mem::size_of::<c_int>());
pub const CMTPGETCONNINFO: libc::c_ulong = request_code_read!('C', 211, std::mem::size_of::<c_int>());
pub const CMTP_LOOPBACK: i32 = 0;
STRUCT! {struct cmtp_connadd_req {
    sock: c_int,    // Connected socket
    flags: u32,
}}
STRUCT! {struct cmtp_conndel_req {
    bdaddr: bdaddr_t,
    flags: u32,
}}
STRUCT! {struct cmtp_conninfo {
    bdaddr: bdaddr_t,
    flags: u32,
    state: u16,
    num: c_int,
}}
STRUCT! {struct cmtp_connlist_req {
    cnum: u32,
    ci: *mut cmtp_conninfo,
}}
