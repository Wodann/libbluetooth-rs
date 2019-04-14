use crate::bluetooth::bdaddr_t;
use libc::sa_family_t;
// SCO defaults
pub const SCO_DEFAULT_MTU: i32 = 500;
pub const SCO_DEFAULT_FLUSH_TO: i32 = 0xFFFF;
#[inline]
pub fn SCO_CONN_TIMEOUT(HZ: i32) -> i32 {
    HZ * 40
}
#[inline]
pub fn SCO_DISCONN_TIMEOUT(HZ: i32) -> i32 {
    HZ * 2
}
#[inline]
pub fn SCO_CONN_IDLE_TIMEOUT(HZ: i32) -> i32 {
    HZ * 60
}
// SCO socket address
STRUCT! {struct sockaddr_sco {
    sco_family: sa_family_t,
    sco_bdaddr: bdaddr_t,
}}
// set/get sockopt defines
pub const SCO_OPTIONS: i32 = 0x01;
STRUCT! {struct sco_options {
    mtu: u16,
}}
pub const SCO_CONNINFO: i32 = 0x02;
STRUCT! {struct sco_conninfo {
    hci_handle: u16,
    dev_class: [u8; 3],
}}
