use std::os::raw::{c_char, c_int, c_long, c_uint, c_ushort, c_void};

use crate::bluetooth::bdaddr_t;
use crate::hci::{
    hci_dev_info, hci_filter, inquiry_info, HCI_FLT_EVENT_BITS, HCI_FLT_TYPE_BITS, HCI_VENDOR_PKT,
};

STRUCT! {struct hci_request {
    ogf: c_ushort,
    ocf: c_ushort,
    event: c_int,
    cparam: *mut c_void,
    clen: c_int,
    rparam: * mut c_void,
    rlen: c_int,
}}

STRUCT! {struct hci_version {
    manufacturer: c_ushort,
    hci_ver: c_char,
    hci_rev: c_ushort,
    lmp_ver: c_char,
    lmp_subver: c_ushort,
}}

#[link(name = "bluetooth")]
extern "C" {
    pub fn hci_open_dev(dev_id: c_int) -> c_int;
    pub fn hci_close_dev(dd: c_int) -> c_int;
    pub fn hci_send_cmd(
        dd: c_int,
        ogf: c_ushort,
        ocf: c_ushort,
        plen: c_char,
        param: *mut c_void,
    ) -> c_int;
    pub fn hci_send_req(dd: c_int, req: *mut hci_request, timeout: c_int) -> c_int;
    pub fn hci_create_connection(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        ptype: c_ushort,
        clkoffset: c_ushort,
        rswitch: c_char,
        handle: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_disconnect(dd: c_int, handle: c_ushort, reason: c_char, to: c_int) -> c_int;
    pub fn hci_inquiry(
        dev_id: c_int,
        len: c_int,
        num_rsp: c_int,
        lap: *const c_char,
        ii: *mut *mut inquiry_info,
        flags: c_long,
    ) -> c_int;
    pub fn hci_devinfo(dev_id: c_int, di: *mut hci_dev_info) -> c_int;
    pub fn hci_devba(dev_id: c_int, bdaddr: *mut bdaddr_t) -> c_int;
    pub fn hci_devid(str_: *const c_char) -> c_int;
    pub fn hci_read_local_name(dd: c_int, len: c_int, name: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_local_name(dd: c_int, name: *const c_char, to: c_int) -> c_int;
    pub fn hci_read_remote_name(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        len: c_int,
        name: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_remote_name_with_clock_offset(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        pscan_rep_mode: c_char,
        clkoffset: c_ushort,
        len: c_int,
        name: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_remote_name_cancel(dd: c_int, bdaddr: *const bdaddr_t, to: c_int) -> c_int;
    pub fn hci_read_remote_version(
        dd: c_int,
        handle: c_ushort,
        ver: *mut hci_version,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_remote_features(
        dd: c_int,
        handle: c_ushort,
        features: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_remote_ext_features(
        dd: c_int,
        handle: c_ushort,
        page: c_char,
        max_page: *mut c_char,
        features: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_clock_offset(
        dd: c_int,
        handle: c_ushort,
        clkoffset: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_local_version(dd: c_int, ver: *mut hci_version, to: c_int) -> c_int;
    pub fn hci_read_local_commands(dd: c_int, commands: *mut c_char, to: c_int) -> c_int;
    pub fn hci_read_local_features(dd: c_int, features: *mut c_char, to: c_int) -> c_int;
    pub fn hci_read_local_ext_features(
        dd: c_int,
        page: c_char,
        max_page: *mut c_char,
        features: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_bd_addr(dd: c_int, bdaddr: *mut bdaddr_t, to: c_int) -> c_int;
    pub fn hci_read_class_of_dev(dd: c_int, cls: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_class_of_dev(dd: c_int, cls: c_uint, to: c_int) -> c_int;
    pub fn hci_read_voice_setting(dd: c_int, vs: *mut c_ushort, to: c_int) -> c_int;
    pub fn hci_write_voice_setting(dd: c_int, vs: c_ushort, to: c_int) -> c_int;
    pub fn hci_read_current_iac_lap(
        dd: c_int,
        num_iac: *mut c_char,
        lap: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_write_current_iac_lap(
        dd: c_int,
        num_iac: c_char,
        lap: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_stored_link_key(
        dd: c_int,
        bdaddr: *mut bdaddr_t,
        all: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_write_stored_link_key(
        dd: c_int,
        bdaddr: *mut bdaddr_t,
        key: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_delete_stored_link_key(
        dd: c_int,
        bdaddr: *mut bdaddr_t,
        all: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_authenticate_link(dd: c_int, handle: c_ushort, to: c_int) -> c_int;
    pub fn hci_encrypt_link(dd: c_int, handle: c_ushort, encrypt: c_char, to: c_int) -> c_int;
    pub fn hci_change_link_key(dd: c_int, handle: c_ushort, to: c_int) -> c_int;
    pub fn hci_switch_role(dd: c_int, bdaddr: *mut bdaddr_t, role: c_char, to: c_int) -> c_int;
    pub fn hci_park_mode(
        dd: c_int,
        handle: c_ushort,
        max_interval: c_ushort,
        min_interval: c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_exit_park_mode(dd: c_int, handle: c_ushort, to: c_int) -> c_int;
    pub fn hci_read_inquiry_scan_type(dd: c_int, type_: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_inquiry_scan_type(dd: c_int, type_: c_char, to: c_int) -> c_int;
    pub fn hci_read_inquiry_mode(dd: c_int, mode: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_inquiry_mode(dd: c_int, mode: c_char, to: c_int) -> c_int;
    pub fn hci_read_afh_mode(dd: c_int, mode: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_afh_mode(dd: c_int, mode: c_char, to: c_int) -> c_int;
    pub fn hci_read_ext_inquiry_response(
        dd: c_int,
        fec: *mut c_char,
        data: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_write_ext_inquiry_response(
        dd: c_int,
        fec: c_char,
        data: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_simple_pairing_mode(dd: c_int, mode: *mut c_char, to: c_int) -> c_int;
    pub fn hci_write_simple_pairing_mode(dd: c_int, mode: c_char, to: c_int) -> c_int;
    pub fn hci_read_local_oob_data(
        dd: c_int,
        hash: *mut c_char,
        randomizer: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_inq_response_tx_power_level(dd: c_int, level: *mut c_char, to: c_int) -> c_int;
    pub fn hci_read_inquiry_transmit_power_level(dd: c_int, level: *mut c_char, to: c_int)
        -> c_int;
    pub fn hci_write_inquiry_transmit_power_level(dd: c_int, level: c_char, to: c_int) -> c_int;
    pub fn hci_read_transmit_power_level(
        dd: c_int,
        handle: c_ushort,
        type_: c_char,
        level: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_link_policy(
        dd: c_int,
        handle: c_ushort,
        policy: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_write_link_policy(dd: c_int, handle: c_ushort, policy: c_ushort, to: c_int)
        -> c_int;
    pub fn hci_read_link_supervision_timeout(
        dd: c_int,
        handle: c_ushort,
        timeout: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_write_link_supervision_timeout(
        dd: c_int,
        handle: c_ushort,
        timeout: c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_set_afh_classification(dd: c_int, map: *mut c_char, to: c_int) -> c_int;
    pub fn hci_read_link_quality(
        dd: c_int,
        handle: c_ushort,
        link_quality: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_rssi(dd: c_int, handle: c_ushort, rssi: *mut c_char, to: c_int) -> c_int;
    pub fn hci_read_afh_map(
        dd: c_int,
        handle: c_ushort,
        mode: *mut c_char,
        map: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_read_clock(
        dd: c_int,
        handle: c_ushort,
        which: c_char,
        clock: *mut c_uint,
        accuracy: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_set_scan_enable(
        dev_id: c_int,
        enable: c_char,
        filter_dup: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_set_scan_parameters(
        dev_id: c_int,
        type_: c_char,
        interval: c_ushort,
        window: c_ushort,
        own_type: c_char,
        filter: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_set_advertise_enable(dev_id: c_int, enable: c_char, to: c_int) -> c_int;
    pub fn hci_le_create_conn(
        dd: c_int,
        interval: c_ushort,
        window: c_ushort,
        initiator_filter: c_char,
        peer_bdaddr_type: c_char,
        peer_bdaddr: bdaddr_t,
        own_bdaddr_type: c_char,
        min_interval: c_ushort,
        max_interval: c_ushort,
        latency: c_ushort,
        supervision_timeout: c_ushort,
        min_ce_length: c_ushort,
        max_ce_length: c_ushort,
        handle: *mut c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_conn_update(
        dd: c_int,
        handle: c_ushort,
        min_interval: c_ushort,
        max_interval: c_ushort,
        latency: c_ushort,
        supervision_timeout: c_ushort,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_add_white_list(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        type_: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_rm_white_list(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        type_: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_read_white_list_size(dd: c_int, size: *mut c_char, to: c_int) -> c_int;
    pub fn hci_le_clear_white_list(dd: c_int, to: c_int) -> c_int;
    pub fn hci_le_add_resolving_list(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        type_: c_char,
        peer_irk: *mut c_char,
        local_irk: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_rm_resolving_list(
        dd: c_int,
        bdaddr: *const bdaddr_t,
        type_: c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_le_clear_resolving_list(dd: c_int, to: c_int) -> c_int;
    pub fn hci_le_read_resolving_list_size(dd: c_int, size: *mut c_char, to: c_int) -> c_int;
    pub fn hci_le_set_address_resolution_enable(dev_id: c_int, enable: c_char, to: c_int) -> c_int;
    pub fn hci_le_read_remote_features(
        dd: c_int,
        handle: c_ushort,
        features: *mut c_char,
        to: c_int,
    ) -> c_int;
    pub fn hci_for_each_dev(
        flag: c_int,
        func: extern "C" fn(dd: c_int, dev_id: c_int, arg: c_long) -> c_int,
        arg: c_long,
    ) -> c_int;
    pub fn hci_get_route(bdaddr: *mut bdaddr_t) -> c_int;
    pub fn hci_bustostr(bus: c_int) -> *mut c_char;
    pub fn hci_typetostr(type_: c_int) -> *mut c_char;
    pub fn hci_dtypetostr(type_: c_int) -> *mut c_char;
    pub fn hci_dflagstostr(flags: c_uint) -> *mut c_char;
    pub fn hci_ptypetostr(ptype: c_uint) -> c_int;
    pub fn hci_strtoptype(str_: *mut c_char, val: *mut c_uint) -> c_int;
    pub fn hci_scoptypetostr(ptype: c_uint) -> *mut c_char;
    pub fn hci_strtoscoptype(str_: *mut c_char, val: *mut c_uint) -> c_int;
    pub fn hci_lptostr(ptype: c_uint) -> *mut c_char;
    pub fn hci_strtolp(str_: *mut c_char, val: *mut c_uint) -> c_int;
    pub fn hci_lmtostr(ptype: c_uint) -> *mut c_char;
    pub fn hci_strtolm(str_: *mut c_char, val: *mut c_uint) -> c_int;
    pub fn hci_cmdtostr(cmd: c_uint) -> *mut c_char;
    pub fn hci_commandstostr(commands: *mut c_char, pref: *mut c_char, width: c_int)
        -> *mut c_char;
    pub fn hci_vertostr(ver: c_uint) -> *mut c_char;
    pub fn hci_strtover(str_: *mut c_char, ver: *mut c_uint) -> c_int;
    pub fn lmp_vertostr(ver: c_uint) -> *mut c_char;
    pub fn lmp_strtover(str_: *mut c_char, ver: *mut c_uint) -> c_int;
    pub fn pal_vertostr(ver: c_uint) -> *mut c_char;
    pub fn pal_strtover(str_: *mut c_char, ver: *mut c_uint) -> c_int;
    pub fn lmp_featurestostr(features: *mut c_char, pref: *mut c_char, width: c_int)
        -> *mut c_char;
}
#[inline]
pub fn hci_set_bit(nr: c_int, addr: *mut c_uint) {
    let bitset = unsafe { addr.offset((nr >> 5) as isize).as_mut() }.unwrap();
    *bitset |= 1 << (nr & 31);
}
#[inline]
pub fn hci_clear_bit(nr: c_int, addr: *mut c_uint) {
    let bitset = unsafe { addr.offset((nr >> 5) as isize).as_mut() }.unwrap();
    *bitset &= !(1 << (nr & 31));
}
#[inline]
pub fn hci_test_bit(nr: c_int, addr: *mut c_uint) -> c_uint {
    let bitset = unsafe { addr.offset((nr >> 5) as isize).as_ref() }.unwrap();
    *bitset & (1 << (nr & 31))
}
// HCI filter tools
#[inline]
#[cfg(feature = "impl-default")]
pub fn hci_filter_clear(f: &mut hci_filter) {
    *f = Default::default();
}
#[inline]
pub fn hci_filter_set_ptype(t: c_int, f: &mut hci_filter) {
    let nr = if t == HCI_VENDOR_PKT {
        0
    } else {
        t & HCI_FLT_TYPE_BITS
    };
    hci_set_bit(nr, &mut f.type_mask as *mut c_uint);
}
#[inline]
pub fn hci_filter_clear_ptype(t: c_int, f: &mut hci_filter) {
    let nr = if t == HCI_VENDOR_PKT {
        0
    } else {
        t & HCI_FLT_TYPE_BITS
    };
    hci_clear_bit(nr, &mut f.type_mask as *mut c_uint);
}
#[inline]
pub fn hci_filter_test_ptype(t: c_int, f: &mut hci_filter) -> c_uint {
    let nr = if t == HCI_VENDOR_PKT {
        0
    } else {
        t & HCI_FLT_TYPE_BITS
    };
    hci_test_bit(nr, &mut f.type_mask as *mut c_uint)
}
#[inline]
pub fn hci_filter_all_ptypes(f: &mut hci_filter) {
    f.type_mask = c_uint::max_value();
}
#[inline]
pub fn hci_filter_set_event(e: c_int, f: &mut hci_filter) {
    hci_set_bit(e & HCI_FLT_EVENT_BITS, &mut f.event_mask[0] as *mut c_uint);
}
#[inline]
pub fn hci_filter_clear_event(e: c_int, f: &mut hci_filter) {
    hci_clear_bit(e & HCI_FLT_EVENT_BITS, &mut f.event_mask[0] as *mut c_uint);
}
#[inline]
pub fn hci_filter_test_event(e: c_int, f: &mut hci_filter) -> c_uint {
    hci_test_bit(e & HCI_FLT_EVENT_BITS, &mut f.event_mask[0] as *mut c_uint)
}
#[inline]
pub fn hci_filter_all_events(f: &mut hci_filter) {
    for mask in &mut f.event_mask {
        *mask = c_uint::max_value();
    }
}
#[inline]
pub fn hci_filter_set_opcode(opcode: c_ushort, f: &mut hci_filter) {
    f.opcode = opcode;
}
#[inline]
pub fn hci_filter_clear_opcode(f: &mut hci_filter) {
    f.opcode = 0;
}
#[inline]
pub fn hci_filter_test_opcode(opcode: c_ushort, f: &mut hci_filter) -> bool {
    f.opcode == opcode
}
