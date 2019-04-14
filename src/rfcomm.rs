use std::os::raw::c_int;

use crate::bluetooth::bdaddr_t;
use libc::sa_family_t;

pub const RFCOMM_DEFAULT_MTU: i32 = 127;
pub const RFCOMM_PSM: i32 = 3;
STRUCT! {struct sockaddr_rc {
    rc_family: sa_family_t,
    rc_bdaddr: bdaddr_t,
    rc_channel: u8,
}}
pub const RFCOMM_CONNINFO: i32 = 0x02;
STRUCT! {struct rfcomm_conninfo {
    hci_handle: u16,
    dev_class: [u8; 3],
}}
pub const RFCOMM_LM: i32 = 0x03;
pub const RFCOMM_LM_MASTER: i32 = 0x0001;
pub const RFCOMM_LM_AUTH: i32 = 0x0002;
pub const RFCOMM_LM_ENCRYPT: i32 = 0x0004;
pub const RFCOMM_LM_TRUSTED: i32 = 0x0008;
pub const RFCOMM_LM_RELIABLE: i32 = 0x0010;
pub const RFCOMM_LM_SECURE: i32 = 0x0020;
pub const RFCOMM_MAX_DEV: i32 = 256;
pub const RFCOMMCREATEDEV: u64 = request_code_write!('R', 200, std::mem::size_of::<c_int>());
pub const RFCOMMRELEASEDEV: u64 = request_code_write!('R', 201, std::mem::size_of::<c_int>());
pub const RFCOMMGETDEVLIST: u64 = request_code_read!('R', 210, std::mem::size_of::<c_int>());
pub const RFCOMMGETDEVINFO: u64 = request_code_read!('R', 211, std::mem::size_of::<c_int>());
STRUCT! {struct rfcomm_dev_req {
    dev_id: i16,
    flags: u32,
    src: bdaddr_t,
    dst: bdaddr_t,
    channel: u8,
}}
pub const RFCOMM_REUSE_DLC: i32 = 0;
pub const RFCOMM_RELEASE_ONHUP: i32 = 1;
pub const RFCOMM_HANGUP_NOW: i32 = 2;
pub const RFCOMM_TTY_ATTACHED: i32 = 3;
STRUCT! {struct rfcomm_dev_info {
    id: i16,
    flags: u32,
    state: u16,
    src: bdaddr_t,
    dst: bdaddr_t,
    channel: u8,
}}
STRUCT! {struct rfcomm_dev_list_req {
    dev_num: u16,
    dev_info: [rfcomm_dev_info; 0],
}}
