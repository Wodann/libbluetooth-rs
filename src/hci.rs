use std::{mem, os::raw::c_ushort};

use crate::bluetooth::bdaddr_t;
use libc::sa_family_t;

pub const HCI_MAX_DEV: i32 = 16;
pub const HCI_MAX_ACL_SIZE: usize = 1492 + 4;
pub const HCI_MAX_SCO_SIZE: usize = 255;
pub const HCI_MAX_EVENT_SIZE: usize = 260;
pub const HCI_MAX_FRAME_SIZE: usize = HCI_MAX_ACL_SIZE + 4;
// HCI dev events
pub const HCI_DEV_REG: i32 = 1;
pub const HCI_DEV_UNREG: i32 = 2;
pub const HCI_DEV_UP: i32 = 3;
pub const HCI_DEV_DOWN: i32 = 4;
pub const HCI_DEV_SUSPEND: i32 = 5;
pub const HCI_DEV_RESUME: i32 = 6;
// HCI bus types
pub const HCI_VIRTUAL: i32 = 0;
pub const HCI_USB: i32 = 1;
pub const HCI_PCCARD: i32 = 2;
pub const HCI_UART: i32 = 3;
pub const HCI_RS232: i32 = 4;
pub const HCI_PCI: i32 = 5;
pub const HCI_SDIO: i32 = 6;
pub const HCI_SPI: i32 = 7;
pub const HCI_I2C: i32 = 8;
pub const HCI_SMD: i32 = 9;
// HCI controller types
pub const HCI_PRIMARY: i32 = 0x00;
pub const HCI_AMP: i32 = 0x01;
pub const HCI_BREDR: i32 = HCI_PRIMARY;
// HCI device flags
ENUM! {enum HCIDeviceFlags {
    HCI_UP,
    HCI_INIT,
    HCI_RUNNING,
    HCI_PSCAN,
    HCI_ISCAN,
    HCI_AUTH,
    HCI_ENCRYPT,
    HCI_INQUIRY,
    HCI_RAW,
}}
// LE address type
ENUM! {enum LEAddressType {
    LE_PUBLIC_ADDRESS = 0x00,
    LE_RANDOM_ADDRESS = 0x01,
}}
// HCI ioctl defines
pub const HCIDEVUP: i32 = request_code_write!('H', 201, mem::size_of::<i32>());
pub const HCIDEVDOWN: i32 = request_code_write!('H', 202, mem::size_of::<i32>());
pub const HCIDEVRESET: i32 = request_code_write!('H', 203, mem::size_of::<i32>());
pub const HCIDEVRESTAT: i32 = request_code_write!('H', 204, mem::size_of::<i32>());
pub const HCIGETDEVLIST: i32 = request_code_read!('H', 210, mem::size_of::<i32>());
pub const HCIGETDEVINFO: i32 = request_code_read!('H', 211, mem::size_of::<i32>());
pub const HCIGETCONNLIST: i32 = request_code_read!('H', 212, mem::size_of::<i32>());
pub const HCIGETCONNINFO: i32 = request_code_read!('H', 213, mem::size_of::<i32>());
pub const HCIGETAUTHINFO: i32 = request_code_read!('H', 215, mem::size_of::<i32>());
pub const HCISETRAW: i32 = request_code_write!('H', 220, mem::size_of::<i32>());
pub const HCISETSCAN: i32 = request_code_write!('H', 221, mem::size_of::<i32>());
pub const HCISETAUTH: i32 = request_code_write!('H', 222, mem::size_of::<i32>());
pub const HCISETENCRYPT: i32 = request_code_write!('H', 223, mem::size_of::<i32>());
pub const HCISETPTYPE: i32 = request_code_write!('H', 224, mem::size_of::<i32>());
pub const HCISETLINKPOL: i32 = request_code_write!('H', 225, mem::size_of::<i32>());
pub const HCISETLINKMODE: i32 = request_code_write!('H', 226, mem::size_of::<i32>());
pub const HCISETACLMTU: i32 = request_code_write!('H', 227, mem::size_of::<i32>());
pub const HCISETSCOMTU: i32 = request_code_write!('H', 228, mem::size_of::<i32>());
pub const HCIBLOCKADDR: i32 = request_code_write!('H', 230, mem::size_of::<i32>());
pub const HCIUNBLOCKADDR: i32 = request_code_write!('H', 231, mem::size_of::<i32>());
pub const HCIINQUIRY: i32 = request_code_read!('H', 240, mem::size_of::<i32>());
// HCI Packet types
pub const HCI_COMMAND_PKT: i32 = 0x01;
pub const HCI_ACLDATA_PKT: i32 = 0x02;
pub const HCI_SCODATA_PKT: i32 = 0x03;
pub const HCI_EVENT_PKT: i32 = 0x04;
pub const HCI_VENDOR_PKT: i32 = 0xff;
// HCI Packet types
pub const HCI_2DH1: i32 = 0x0002;
pub const HCI_3DH1: i32 = 0x0004;
pub const HCI_DM1: i32 = 0x0008;
pub const HCI_DH1: i32 = 0x0010;
pub const HCI_2DH3: i32 = 0x0100;
pub const HCI_3DH3: i32 = 0x0200;
pub const HCI_DM3: i32 = 0x0400;
pub const HCI_DH3: i32 = 0x0800;
pub const HCI_2DH5: i32 = 0x1000;
pub const HCI_3DH5: i32 = 0x2000;
pub const HCI_DM5: i32 = 0x4000;
pub const HCI_DH5: i32 = 0x8000;
pub const HCI_HV1: i32 = 0x0020;
pub const HCI_HV2: i32 = 0x0040;
pub const HCI_HV3: i32 = 0x0080;
pub const HCI_EV3: i32 = 0x0008;
pub const HCI_EV4: i32 = 0x0010;
pub const HCI_EV5: i32 = 0x0020;
pub const HCI_2EV3: i32 = 0x0040;
pub const HCI_3EV3: i32 = 0x0080;
pub const HCI_2EV5: i32 = 0x0100;
pub const HCI_3EV5: i32 = 0x0200;
pub const SCO_PTYPE_MASK: i32 = (HCI_HV1 | HCI_HV2 | HCI_HV3);
pub const ACL_PTYPE_MASK: i32 = (HCI_DM1 | HCI_DH1 | HCI_DM3 | HCI_DH3 | HCI_DM5 | HCI_DH5);
// HCI Error codes
pub const HCI_UNKNOWN_COMMAND: i32 = 0x01;
pub const HCI_NO_CONNECTION: i32 = 0x02;
pub const HCI_HARDWARE_FAILURE: i32 = 0x03;
pub const HCI_PAGE_TIMEOUT: i32 = 0x04;
pub const HCI_AUTHENTICATION_FAILURE: i32 = 0x05;
pub const HCI_PIN_OR_KEY_MISSING: i32 = 0x06;
pub const HCI_MEMORY_FULL: i32 = 0x07;
pub const HCI_CONNECTION_TIMEOUT: i32 = 0x08;
pub const HCI_MAX_NUMBER_OF_CONNECTIONS: i32 = 0x09;
pub const HCI_MAX_NUMBER_OF_SCO_CONNECTIONS: i32 = 0x0a;
pub const HCI_ACL_CONNECTION_EXISTS: i32 = 0x0b;
pub const HCI_COMMAND_DISALLOWED: i32 = 0x0c;
pub const HCI_REJECTED_LIMITED_RESOURCES: i32 = 0x0d;
pub const HCI_REJECTED_SECURITY: i32 = 0x0e;
pub const HCI_REJECTED_PERSONAL: i32 = 0x0f;
pub const HCI_HOST_TIMEOUT: i32 = 0x10;
pub const HCI_UNSUPPORTED_FEATURE: i32 = 0x11;
pub const HCI_INVALID_PARAMETERS: i32 = 0x12;
pub const HCI_OE_USER_ENDED_CONNECTION: i32 = 0x13;
pub const HCI_OE_LOW_RESOURCES: i32 = 0x14;
pub const HCI_OE_POWER_OFF: i32 = 0x15;
pub const HCI_CONNECTION_TERMINATED: i32 = 0x16;
pub const HCI_REPEATED_ATTEMPTS: i32 = 0x17;
pub const HCI_PAIRING_NOT_ALLOWED: i32 = 0x18;
pub const HCI_UNKNOWN_LMP_PDU: i32 = 0x19;
pub const HCI_UNSUPPORTED_REMOTE_FEATURE: i32 = 0x1a;
pub const HCI_SCO_OFFSET_REJECTED: i32 = 0x1b;
pub const HCI_SCO_INTERVAL_REJECTED: i32 = 0x1c;
pub const HCI_AIR_MODE_REJECTED: i32 = 0x1d;
pub const HCI_INVALID_LMP_PARAMETERS: i32 = 0x1e;
pub const HCI_UNSPECIFIED_ERROR: i32 = 0x1f;
pub const HCI_UNSUPPORTED_LMP_PARAMETER_VALUE: i32 = 0x20;
pub const HCI_ROLE_CHANGE_NOT_ALLOWED: i32 = 0x21;
pub const HCI_LMP_RESPONSE_TIMEOUT: i32 = 0x22;
pub const HCI_LMP_ERROR_TRANSACTION_COLLISION: i32 = 0x23;
pub const HCI_LMP_PDU_NOT_ALLOWED: i32 = 0x24;
pub const HCI_ENCRYPTION_MODE_NOT_ACCEPTED: i32 = 0x25;
pub const HCI_UNIT_LINK_KEY_USED: i32 = 0x26;
pub const HCI_QOS_NOT_SUPPORTED: i32 = 0x27;
pub const HCI_INSTANT_PASSED: i32 = 0x28;
pub const HCI_PAIRING_NOT_SUPPORTED: i32 = 0x29;
pub const HCI_TRANSACTION_COLLISION: i32 = 0x2a;
pub const HCI_QOS_UNACCEPTABLE_PARAMETER: i32 = 0x2c;
pub const HCI_QOS_REJECTED: i32 = 0x2d;
pub const HCI_CLASSIFICATION_NOT_SUPPORTED: i32 = 0x2e;
pub const HCI_INSUFFICIENT_SECURITY: i32 = 0x2f;
pub const HCI_PARAMETER_OUT_OF_RANGE: i32 = 0x30;
pub const HCI_ROLE_SWITCH_PENDING: i32 = 0x32;
pub const HCI_SLOT_VIOLATION: i32 = 0x34;
pub const HCI_ROLE_SWITCH_FAILED: i32 = 0x35;
pub const HCI_EIR_TOO_LARGE: i32 = 0x36;
pub const HCI_SIMPLE_PAIRING_NOT_SUPPORTED: i32 = 0x37;
pub const HCI_HOST_BUSY_PAIRING: i32 = 0x38;
// ACL flags
pub const ACL_START_NO_FLUSH: i32 = 0x00;
pub const ACL_CONT: i32 = 0x01;
pub const ACL_START: i32 = 0x02;
pub const ACL_ACTIVE_BCAST: i32 = 0x04;
pub const ACL_PICO_BCAST: i32 = 0x08;
// Baseband links
pub const SCO_LINK: i32 = 0x00;
pub const ACL_LINK: i32 = 0x01;
pub const ESCO_LINK: i32 = 0x02;
// LMP features
pub const LMP_3SLOT: i32 = 0x01;
pub const LMP_5SLOT: i32 = 0x02;
pub const LMP_ENCRYPT: i32 = 0x04;
pub const LMP_SOFFSET: i32 = 0x08;
pub const LMP_TACCURACY: i32 = 0x10;
pub const LMP_RSWITCH: i32 = 0x20;
pub const LMP_HOLD: i32 = 0x40;
pub const LMP_SNIFF: i32 = 0x80;
pub const LMP_PARK: i32 = 0x01;
pub const LMP_RSSI: i32 = 0x02;
pub const LMP_QUALITY: i32 = 0x04;
pub const LMP_SCO: i32 = 0x08;
pub const LMP_HV2: i32 = 0x10;
pub const LMP_HV3: i32 = 0x20;
pub const LMP_ULAW: i32 = 0x40;
pub const LMP_ALAW: i32 = 0x80;
pub const LMP_CVSD: i32 = 0x01;
pub const LMP_PSCHEME: i32 = 0x02;
pub const LMP_PCONTROL: i32 = 0x04;
pub const LMP_TRSP_SCO: i32 = 0x08;
pub const LMP_BCAST_ENC: i32 = 0x80;
pub const LMP_EDR_ACL_2M: i32 = 0x02;
pub const LMP_EDR_ACL_3M: i32 = 0x04;
pub const LMP_ENH_ISCAN: i32 = 0x08;
pub const LMP_ILACE_ISCAN: i32 = 0x10;
pub const LMP_ILACE_PSCAN: i32 = 0x20;
pub const LMP_RSSI_INQ: i32 = 0x40;
pub const LMP_ESCO: i32 = 0x80;
pub const LMP_EV4: i32 = 0x01;
pub const LMP_EV5: i32 = 0x02;
pub const LMP_AFH_CAP_SLV: i32 = 0x08;
pub const LMP_AFH_CLS_SLV: i32 = 0x10;
pub const LMP_NO_BREDR: i32 = 0x20;
pub const LMP_LE: i32 = 0x40;
pub const LMP_EDR_3SLOT: i32 = 0x80;
pub const LMP_EDR_5SLOT: i32 = 0x01;
pub const LMP_SNIFF_SUBR: i32 = 0x02;
pub const LMP_PAUSE_ENC: i32 = 0x04;
pub const LMP_AFH_CAP_MST: i32 = 0x08;
pub const LMP_AFH_CLS_MST: i32 = 0x10;
pub const LMP_EDR_ESCO_2M: i32 = 0x20;
pub const LMP_EDR_ESCO_3M: i32 = 0x40;
pub const LMP_EDR_3S_ESCO: i32 = 0x80;
pub const LMP_EXT_INQ: i32 = 0x01;
pub const LMP_LE_BREDR: i32 = 0x02;
pub const LMP_SIMPLE_PAIR: i32 = 0x08;
pub const LMP_ENCAPS_PDU: i32 = 0x10;
pub const LMP_ERR_DAT_REP: i32 = 0x20;
pub const LMP_NFLUSH_PKTS: i32 = 0x40;
pub const LMP_LSTO: i32 = 0x01;
pub const LMP_INQ_TX_PWR: i32 = 0x02;
pub const LMP_EPC: i32 = 0x04;
pub const LMP_EXT_FEAT: i32 = 0x80;
// Extended LMP features
pub const LMP_HOST_SSP: i32 = 0x01;
pub const LMP_HOST_LE: i32 = 0x02;
pub const LMP_HOST_LE_BREDR: i32 = 0x04;
// Link policies
pub const HCI_LP_RSWITCH: i32 = 0x0001;
pub const HCI_LP_HOLD: i32 = 0x0002;
pub const HCI_LP_SNIFF: i32 = 0x0004;
pub const HCI_LP_PARK: i32 = 0x0008;
// Link mode
pub const HCI_LM_ACCEPT: i32 = 0x8000;
pub const HCI_LM_MASTER: i32 = 0x0001;
pub const HCI_LM_AUTH: i32 = 0x0002;
pub const HCI_LM_ENCRYPT: i32 = 0x0004;
pub const HCI_LM_TRUSTED: i32 = 0x0008;
pub const HCI_LM_RELIABLE: i32 = 0x0010;
pub const HCI_LM_SECURE: i32 = 0x0020;
// Link Key types
pub const HCI_LK_COMBINATION: i32 = 0x00;
pub const HCI_LK_LOCAL_UNIT: i32 = 0x01;
pub const HCI_LK_REMOTE_UNIT: i32 = 0x02;
pub const HCI_LK_DEBUG_COMBINATION: i32 = 0x03;
pub const HCI_LK_UNAUTH_COMBINATION: i32 = 0x04;
pub const HCI_LK_AUTH_COMBINATION: i32 = 0x05;
pub const HCI_LK_CHANGED_COMBINATION: i32 = 0x06;
pub const HCI_LK_INVALID: i32 = 0xFF;
// -----  HCI Commands -----
// Link Control
pub const OGF_LINK_CTL: i32 = 0x01;
pub const OCF_INQUIRY: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct inquiry_cp {
    lap: [u8;3],
    length: u8,    // 1.28s units
    num_rsp: u8,
}}
pub const INQUIRY_CP_SIZE: usize = 5;
STRUCT! {#[repr(packed)] struct status_bdaddr_rp {
    status: u8,
    bdaddr: bdaddr_t,
}}
pub const STATUS_BDADDR_RP_SIZE: usize = 7;
pub const OCF_INQUIRY_CANCEL: i32 = 0x0002;
pub const OCF_PERIODIC_INQUIRY: i32 = 0x0003;
STRUCT! {#[repr(packed)] struct periodic_inquiry_cp {
    max_period: u16,    // 1.28s units
    min_period: u16,    // 1.28s units
    lap: [u8;3],
    length: u8,         // 1.28s units
    num_rsp: u8,
}}
pub const PERIODIC_INQUIRY_CP_SIZE: usize = 9;
pub const OCF_EXIT_PERIODIC_INQUIRY: i32 = 0x0004;
pub const OCF_CREATE_CONN: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct create_conn_cp {
    bdaddr: bdaddr_t,
    pkt_type_: u16,
    pscan_rep_mode: u8,
    pscan_mode: u8,
    clock_offset: u16,
    role_switch: u8,
}}
pub const CREATE_CONN_CP_SIZE: usize = 13;
pub const OCF_DISCONNECT: i32 = 0x0006;
STRUCT! {#[repr(packed)] struct disconnect_cp {
    handle: u16,
    reason: u8,
}}
pub const DISCONNECT_CP_SIZE: usize = 3;
pub const OCF_ADD_SCO: i32 = 0x0007;
STRUCT! {#[repr(packed)] struct add_sco_cp {
    handle: u16,
    pkt_type_: u16,
}}
pub const ADD_SCO_CP_SIZE: usize = 4;
pub const OCF_CREATE_CONN_CANCEL: i32 = 0x0008;
STRUCT! {#[repr(packed)] struct create_conn_cancel_cp {
    bdaddr: bdaddr_t,
}}
pub const CREATE_CONN_CANCEL_CP_SIZE: usize = 6;
pub const OCF_ACCEPT_CONN_REQ: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct accept_conn_req_cp {
    bdaddr: bdaddr_t,
    role: u8,
}}
pub const ACCEPT_CONN_REQ_CP_SIZE: usize = 7;
pub const OCF_REJECT_CONN_REQ: i32 = 0x000A;
STRUCT! {#[repr(packed)] struct reject_conn_req_cp {
    bdaddr: bdaddr_t,
    reason: u8,
}}
pub const REJECT_CONN_REQ_CP_SIZE: usize = 7;
pub const OCF_LINK_KEY_REPLY: i32 = 0x000B;
STRUCT! {#[repr(packed)] struct link_key_reply_cp {
    bdaddr: bdaddr_t,
    link_key: [u8; 16],
}}
pub const LINK_KEY_REPLY_CP_SIZE: usize = 22;
pub const OCF_LINK_KEY_NEG_REPLY: i32 = 0x000C;
pub const OCF_PIN_CODE_REPLY: i32 = 0x000D;
STRUCT! {#[repr(packed)] struct pin_code_reply_cp {
    bdaddr: bdaddr_t,
    pin_len: u8,
    pin_code: [u8; 16],
}}
pub const PIN_CODE_REPLY_CP_SIZE: usize = 23;
pub const OCF_PIN_CODE_NEG_REPLY: i32 = 0x000E;
pub const OCF_SET_CONN_PTYPE: i32 = 0x000F;
STRUCT! {#[repr(packed)] struct set_conn_ptype_cp {
    handle: u16,
    pkt_type_: u16,
}}
pub const SET_CONN_PTYPE_CP_SIZE: usize = 4;
pub const OCF_AUTH_REQUESTED: i32 = 0x0011;
STRUCT! {#[repr(packed)] struct auth_requested_cp {
    handle: u16,
}}
pub const AUTH_REQUESTED_CP_SIZE: usize = 2;
pub const OCF_SET_CONN_ENCRYPT: i32 = 0x0013;
STRUCT! {#[repr(packed)] struct set_conn_encrypt_cp {
    handle: u16,
    encrypt: u8,
}}
pub const SET_CONN_ENCRYPT_CP_SIZE: usize = 3;
pub const OCF_CHANGE_CONN_LINK_KEY: i32 = 0x0015;
STRUCT! {#[repr(packed)] struct change_conn_link_key_cp {
    handle: u16,
}}
pub const CHANGE_CONN_LINK_KEY_CP_SIZE: usize = 2;
pub const OCF_MASTER_LINK_KEY: i32 = 0x0017;
STRUCT! {#[repr(packed)] struct master_link_key_cp {
    key_flag: u8,
}}
pub const MASTER_LINK_KEY_CP_SIZE: usize = 1;
pub const OCF_REMOTE_NAME_REQ: i32 = 0x0019;
STRUCT! {#[repr(packed)] struct remote_name_req_cp {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
    pscan_mode: u8,
    clock_offset: u16,
}}
pub const REMOTE_NAME_REQ_CP_SIZE: usize = 10;
pub const OCF_REMOTE_NAME_REQ_CANCEL: i32 = 0x001A;
STRUCT! {#[repr(packed)] struct remote_name_req_cancel_cp {
    bdaddr: bdaddr_t,
}}
pub const REMOTE_NAME_REQ_CANCEL_CP_SIZE: usize = 6;
pub const OCF_READ_REMOTE_FEATURES: i32 = 0x001B;
STRUCT! {#[repr(packed)] struct read_remote_features_cp {
    handle: u16,
}}
pub const READ_REMOTE_FEATURES_CP_SIZE: usize = 2;
pub const OCF_READ_REMOTE_EXT_FEATURES: i32 = 0x001C;
STRUCT! {#[repr(packed)] struct read_remote_ext_features_cp {
    handle: u16,
    page_num: u8,
}}
pub const READ_REMOTE_EXT_FEATURES_CP_SIZE: usize = 3;
pub const OCF_READ_REMOTE_VERSION: i32 = 0x001D;
STRUCT! {#[repr(packed)] struct read_remote_version_cp {
    handle: u16,
}}
pub const READ_REMOTE_VERSION_CP_SIZE: usize = 2;
pub const OCF_READ_CLOCK_OFFSET: i32 = 0x001F;
STRUCT! {#[repr(packed)] struct read_clock_offset_cp {
    handle: u16,
}}
pub const READ_CLOCK_OFFSET_CP_SIZE: usize = 2;
pub const OCF_READ_LMP_HANDLE: i32 = 0x0020;
pub const OCF_SETUP_SYNC_CONN: i32 = 0x0028;
STRUCT! {#[repr(packed)] struct setup_sync_conn_cp {
    handle: u16,
    tx_bandwith: u32,
    rx_bandwith: u32,
    max_latency: u16,
    voice_setting: u16,
    retrans_effort: u8,
    pkt_type_: u16,
}}
pub const SETUP_SYNC_CONN_CP_SIZE: usize = 17;
pub const OCF_ACCEPT_SYNC_CONN_REQ: i32 = 0x0029;
STRUCT! {#[repr(packed)] struct accept_sync_conn_req_cp {
    bdaddr: bdaddr_t,
    tx_bandwith: u32,
    rx_bandwith: u32,
    max_latency: u16,
    voice_setting: u16,
    retrans_effort: u8,
    pkt_type_: u16,
}}
pub const ACCEPT_SYNC_CONN_REQ_CP_SIZE: usize = 21;
pub const OCF_REJECT_SYNC_CONN_REQ: i32 = 0x002A;
STRUCT! {#[repr(packed)] struct reject_sync_conn_req_cp {
    bdaddr: bdaddr_t,
    reason: u8,
}}
pub const REJECT_SYNC_CONN_REQ_CP_SIZE: usize = 7;
pub const OCF_IO_CAPABILITY_REPLY: i32 = 0x002B;
STRUCT! {#[repr(packed)] struct io_capability_reply_cp {
    bdaddr: bdaddr_t,
    capability: u8,
    oob_data: u8,
    authentication: u8,
}}
pub const IO_CAPABILITY_REPLY_CP_SIZE: usize = 9;
pub const OCF_USER_CONFIRM_REPLY: i32 = 0x002C;
STRUCT! {#[repr(packed)] struct user_confirm_reply_cp {
    bdaddr: bdaddr_t,
}}
pub const USER_CONFIRM_REPLY_CP_SIZE: usize = 6;
pub const OCF_USER_CONFIRM_NEG_REPLY: i32 = 0x002D;
pub const OCF_USER_PASSKEY_REPLY: i32 = 0x002E;
STRUCT! {#[repr(packed)] struct user_passkey_reply_cp {
    bdaddr: bdaddr_t,
    passkey: u32,
}}
pub const USER_PASSKEY_REPLY_CP_SIZE: usize = 10;
pub const OCF_USER_PASSKEY_NEG_REPLY: i32 = 0x002F;
pub const OCF_REMOTE_OOB_DATA_REPLY: i32 = 0x0030;
STRUCT! {#[repr(packed)] struct remote_oob_data_reply_cp {
    bdaddr: bdaddr_t,
    hash: [u8; 16],
    randomizer: [u8; 16],
}}
pub const REMOTE_OOB_DATA_REPLY_CP_SIZE: usize = 38;
pub const OCF_REMOTE_OOB_DATA_NEG_REPLY: i32 = 0x0033;
pub const OCF_IO_CAPABILITY_NEG_REPLY: i32 = 0x0034;
STRUCT! {#[repr(packed)] struct io_capability_neg_reply_cp {
    bdaddr: bdaddr_t,
    reason: u8,
}}
pub const IO_CAPABILITY_NEG_REPLY_CP_SIZE: usize = 7;
pub const OCF_CREATE_PHYSICAL_LINK: i32 = 0x0035;
STRUCT! {#[repr(packed)] struct create_physical_link_cp {
    handle: u8,
    key_length: u8,
    key_type_: u8,
    key: [u8; 32],
}}
pub const CREATE_PHYSICAL_LINK_CP_SIZE: usize = 35;
pub const OCF_ACCEPT_PHYSICAL_LINK: i32 = 0x0036;
STRUCT! {#[repr(packed)] struct accept_physical_link_cp {
    handle: u8,
    key_length: u8,
    key_type_: u8,
    key: [u8; 32],
}}
pub const ACCEPT_PHYSICAL_LINK_CP_SIZE: usize = 35;
pub const OCF_DISCONNECT_PHYSICAL_LINK: i32 = 0x0037;
STRUCT! {#[repr(packed)] struct disconnect_physical_link_cp {
    handle: u8,
    reason: u8,
}}
pub const DISCONNECT_PHYSICAL_LINK_CP_SIZE: usize = 2;
pub const OCF_CREATE_LOGICAL_LINK: i32 = 0x0038;
STRUCT! {#[repr(packed)] struct create_logical_link_cp {
    handle: u8,
    tx_flow: [u8; 16],
    rx_flow: [u8; 16],
}}
pub const CREATE_LOGICAL_LINK_CP_SIZE: usize = 33;
pub const OCF_ACCEPT_LOGICAL_LINK: i32 = 0x0039;
pub const OCF_DISCONNECT_LOGICAL_LINK: i32 = 0x003A;
STRUCT! {#[repr(packed)] struct disconnect_logical_link_cp {
    handle: u16,
}}
pub const DISCONNECT_LOGICAL_LINK_CP_SIZE: usize = 2;
pub const OCF_LOGICAL_LINK_CANCEL: i32 = 0x003B;
STRUCT! {#[repr(packed)] struct cancel_logical_link_cp {
    handle: u8,
    tx_flow_id: u8,
}}
pub const LOGICAL_LINK_CANCEL_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct cancel_logical_link_rp {
    status: u8,
    handle: u8,
    tx_flow_id: u8,
}}
pub const LOGICAL_LINK_CANCEL_RP_SIZE: usize = 3;
pub const OCF_FLOW_SPEC_MODIFY: i32 = 0x003C;
// Link Policy
pub const OGF_LINK_POLICY: i32 = 0x02;
pub const OCF_HOLD_MODE: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct hold_mode_cp {
    handle: u16,
    max_interval: u16,
    min_interval: u16,
}}
pub const HOLD_MODE_CP_SIZE: usize = 6;
pub const OCF_SNIFF_MODE: i32 = 0x0003;
STRUCT! {#[repr(packed)] struct sniff_mode_cp {
    handle: u16,
    max_interval: u16,
    min_interval: u16,
    attempt: u16,
    timeout: u16,
}}
pub const SNIFF_MODE_CP_SIZE: usize = 10;
pub const OCF_EXIT_SNIFF_MODE: i32 = 0x0004;
STRUCT! {#[repr(packed)] struct exit_sniff_mode_cp {
    handle: u16,
}}
pub const EXIT_SNIFF_MODE_CP_SIZE: usize = 2;
pub const OCF_PARK_MODE: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct park_mode_cp {
    handle: u16,
    max_interval: u16,
    min_interval: u16,
}}
pub const PARK_MODE_CP_SIZE: usize = 6;
pub const OCF_EXIT_PARK_MODE: i32 = 0x0006;
STRUCT! {#[repr(packed)] struct exit_park_mode_cp {
    handle: u16,
}}
pub const EXIT_PARK_MODE_CP_SIZE: usize = 2;
pub const OCF_QOS_SETUP: i32 = 0x0007;
STRUCT! {#[repr(packed)] struct hci_qos {
    service_type_: u8,       // 1 = best effort
    token_rate: u32,        // Bytes per second
    peak_bandwidth: u32,    // Bytes per second
    latency: u32,           // Microseconds
    delay_variation: u32,   // Microseconds
}}
pub const HCI_QOS_CP_SIZE: usize = 17;
STRUCT! {#[repr(packed)] struct qos_setup_cp {
    handle: u16,
    flags: u8,  // Reserved
    qos: hci_qos,
}}
pub const QOS_SETUP_CP_SIZE: usize = 3 + HCI_QOS_CP_SIZE;
pub const OCF_ROLE_DISCOVERY: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct role_discovery_cp {
    handle: u16,
}}
pub const ROLE_DISCOVERY_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct role_discovery_rp {
    status: u8,
    handle: u16,
    role: u8,
}}
pub const ROLE_DISCOVERY_RP_SIZE: usize = 4;
pub const OCF_SWITCH_ROLE: i32 = 0x000B;
STRUCT! {#[repr(packed)] struct switch_role_cp {
    bdaddr: bdaddr_t,
    role: u8,
}}
pub const SWITCH_ROLE_CP_SIZE: usize = 7;
pub const OCF_READ_LINK_POLICY: i32 = 0x000C;
STRUCT! {#[repr(packed)] struct read_link_policy_cp {
    handle: u16,
}}
pub const READ_LINK_POLICY_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct read_link_policy_rp {
    status: u8,
    handle: u16,
    policy: u16,
}}
pub const READ_LINK_POLICY_RP_SIZE: usize = 5;
pub const OCF_WRITE_LINK_POLICY: i32 = 0x000D;
STRUCT! {#[repr(packed)] struct write_link_policy_cp {
    handle: u16,
    policy: u16,
}}
pub const WRITE_LINK_POLICY_CP_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct write_link_policy_rp {
    status: u8,
    handle: u16,
}}
pub const WRITE_LINK_POLICY_RP_SIZE: usize = 3;
pub const OCF_READ_DEFAULT_LINK_POLICY: i32 = 0x000E;
pub const OCF_WRITE_DEFAULT_LINK_POLICY: i32 = 0x000F;
pub const OCF_FLOW_SPECIFICATION: i32 = 0x0010;
pub const OCF_SNIFF_SUBRATING: i32 = 0x0011;
STRUCT! {#[repr(packed)] struct sniff_subrating_cp {
    handle: u16,
    max_latency: u16,
    min_remote_timeout: u16,
    min_local_timeout: u16,
}}
pub const SNIFF_SUBRATING_CP_SIZE: usize = 8;
// Host Controller and Baseband
pub const OGF_HOST_CTL: i32 = 0x03;
pub const OCF_SET_EVENT_MASK: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct set_event_mask_cp {
    mask: [u8; 8],
}}
pub const SET_EVENT_MASK_CP_SIZE: usize = 8;
pub const OCF_RESET: i32 = 0x0003;
pub const OCF_SET_EVENT_FLT: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct set_event_flt_cp {
    flt_type_: u8,
    cond_type_: u8,
    condition: [u8; 0],
}}
pub const SET_EVENT_FLT_CP_SIZE: usize = 2;
// Filter types
pub const FLT_CLEAR_ALL: i32 = 0x00;
pub const FLT_INQ_RESULT: i32 = 0x01;
pub const FLT_CONN_SETUP: i32 = 0x02;
// INQ_RESULT Condition types
pub const INQ_RESULT_RETURN_ALL: i32 = 0x00;
pub const INQ_RESULT_RETURN_CLASS: i32 = 0x01;
pub const INQ_RESULT_RETURN_BDADDR: i32 = 0x02;
// CONN_SETUP Condition types
pub const CONN_SETUP_ALLOW_ALL: i32 = 0x00;
pub const CONN_SETUP_ALLOW_CLASS: i32 = 0x01;
pub const CONN_SETUP_ALLOW_BDADDR: i32 = 0x02;
// CONN_SETUP Conditions
pub const CONN_SETUP_AUTO_OFF: i32 = 0x01;
pub const CONN_SETUP_AUTO_ON: i32 = 0x02;
pub const OCF_FLUSH: i32 = 0x0008;
pub const OCF_READ_PIN_TYPE: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct read_pin_type_rp {
    status: u8,
    pin_type_: u8,
}}
pub const READ_PIN_TYPE_RP_SIZE: usize = 2;
pub const OCF_WRITE_PIN_TYPE: i32 = 0x000A;
STRUCT! {#[repr(packed)] struct write_pin_type_cp {
    pin_type_: u8,
}}
pub const WRITE_PIN_TYPE_CP_SIZE: usize = 1;
pub const OCF_CREATE_NEW_UNIT_KEY: i32 = 0x000B;
pub const OCF_READ_STORED_LINK_KEY: i32 = 0x000D;
STRUCT! {#[repr(packed)] struct read_stored_link_key_cp {
    bdaddr: bdaddr_t,
    read_all: u8,
}}
pub const READ_STORED_LINK_KEY_CP_SIZE: usize = 7;
STRUCT! {#[repr(packed)] struct read_stored_link_key_rp {
    status: u8,
    max_keys: u16,
    num_keys: u16,
}}
pub const READ_STORED_LINK_KEY_RP_SIZE: usize = 5;
pub const OCF_WRITE_STORED_LINK_KEY: i32 = 0x0011;
STRUCT! {#[repr(packed)] struct write_stored_link_key_cp {
    num_keys: u8,
    // variable length part
}}
pub const WRITE_STORED_LINK_KEY_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_stored_link_key_rp {
    status: u8,
    num_keys: u8,
}}
pub const READ_WRITE_LINK_KEY_RP_SIZE: usize = 2;
pub const OCF_DELETE_STORED_LINK_KEY: i32 = 0x0012;
STRUCT! {#[repr(packed)] struct delete_stored_link_key_cp {
    bdaddr: bdaddr_t,
    delete_all: u8,
}}
pub const DELETE_STORED_LINK_KEY_CP_SIZE: usize = 7;
STRUCT! {#[repr(packed)] struct delete_stored_link_key_rp {
    status: u8,
    num_keys: u16,
}}
pub const DELETE_STORED_LINK_KEY_RP_SIZE: usize = 3;
pub const HCI_MAX_NAME_LENGTH: usize = 248;
pub const OCF_CHANGE_LOCAL_NAME: i32 = 0x0013;
STRUCT! {#[repr(packed)] struct change_local_name_cp {
    name: [u8; HCI_MAX_NAME_LENGTH],
}}
pub const CHANGE_LOCAL_NAME_CP_SIZE: usize = 248;
pub const OCF_READ_LOCAL_NAME: i32 = 0x0014;
STRUCT! {#[repr(packed)] struct read_local_name_rp {
    status: u8,
    name: [u8; HCI_MAX_NAME_LENGTH],
}}
pub const READ_LOCAL_NAME_RP_SIZE: usize = 249;
pub const OCF_READ_CONN_ACCEPT_TIMEOUT: i32 = 0x0015;
STRUCT! {#[repr(packed)] struct read_conn_accept_timeout_rp {
    status: u8,
    timeout: u16,
}}
pub const READ_CONN_ACCEPT_TIMEOUT_RP_SIZE: usize = 3;
pub const OCF_WRITE_CONN_ACCEPT_TIMEOUT: i32 = 0x0016;
STRUCT! {#[repr(packed)] struct write_conn_accept_timeout_cp {
    timeout: u16,
}}
pub const WRITE_CONN_ACCEPT_TIMEOUT_CP_SIZE: usize = 2;
pub const OCF_READ_PAGE_TIMEOUT: i32 = 0x0017;
STRUCT! {#[repr(packed)] struct read_page_timeout_rp {
    status: u8,
    timeout: u16,
}}
pub const READ_PAGE_TIMEOUT_RP_SIZE: usize = 3;
pub const OCF_WRITE_PAGE_TIMEOUT: i32 = 0x0018;
STRUCT! {#[repr(packed)] struct write_page_timeout_cp {
    timeout: u16,
}}
pub const WRITE_PAGE_TIMEOUT_CP_SIZE: usize = 2;
pub const OCF_READ_SCAN_ENABLE: i32 = 0x0019;
STRUCT! {#[repr(packed)] struct read_scan_enable_rp {
    status: u8,
    enable: u8,
}}
pub const READ_SCAN_ENABLE_RP_SIZE: usize = 2;
pub const OCF_WRITE_SCAN_ENABLE: i32 = 0x001A;
pub const SCAN_DISABLED: i32 = 0x00;
pub const SCAN_INQUIRY: i32 = 0x01;
pub const SCAN_PAGE: i32 = 0x02;
pub const OCF_READ_PAGE_ACTIVITY: i32 = 0x001B;
STRUCT! {#[repr(packed)] struct read_page_activity_rp {
    status: u8,
    interval: u16,
    window: u16,
}}
pub const READ_PAGE_ACTIVITY_RP_SIZE: usize = 5;
pub const OCF_WRITE_PAGE_ACTIVITY: i32 = 0x001C;
STRUCT! {#[repr(packed)] struct write_page_activity_cp {
    interval: u16,
    window: u16,
}}
pub const WRITE_PAGE_ACTIVITY_CP_SIZE: usize = 4;
pub const OCF_READ_INQ_ACTIVITY: i32 = 0x001D;
STRUCT! {#[repr(packed)] struct read_inq_activity_rp {
    status: u8,
    interval: u16,
    window: u16,
}}
pub const READ_INQ_ACTIVITY_RP_SIZE: usize = 5;
pub const OCF_WRITE_INQ_ACTIVITY: i32 = 0x001E;
STRUCT! {#[repr(packed)] struct write_inq_activity_cp {
    interval: u16,
    window: u16,
}}
pub const WRITE_INQ_ACTIVITY_CP_SIZE: usize = 4;
pub const OCF_READ_AUTH_ENABLE: i32 = 0x001F;
pub const OCF_WRITE_AUTH_ENABLE: i32 = 0x0020;
pub const AUTH_DISABLED: i32 = 0x00;
pub const AUTH_ENABLED: i32 = 0x01;
pub const OCF_READ_ENCRYPT_MODE: i32 = 0x0021;
pub const OCF_WRITE_ENCRYPT_MODE: i32 = 0x0022;
pub const ENCRYPT_DISABLED: i32 = 0x00;
pub const ENCRYPT_P2P: i32 = 0x01;
pub const ENCRYPT_BOTH: i32 = 0x02;
pub const OCF_READ_CLASS_OF_DEV: i32 = 0x0023;
STRUCT! {#[repr(packed)] struct read_class_of_dev_rp {
    status: u8,
    dev_class: [u8; 3],
}}
pub const READ_CLASS_OF_DEV_RP_SIZE: usize = 4;
pub const OCF_WRITE_CLASS_OF_DEV: i32 = 0x0024;
STRUCT! {#[repr(packed)] struct write_class_of_dev_cp {
    dev_class: [u8; 3],
}}
pub const WRITE_CLASS_OF_DEV_CP_SIZE: usize = 3;
pub const OCF_READ_VOICE_SETTING: i32 = 0x0025;
STRUCT! {#[repr(packed)] struct read_voice_setting_rp {
    status: u8,
    voice_setting: u16,
}}
pub const READ_VOICE_SETTING_RP_SIZE: usize = 3;
pub const OCF_WRITE_VOICE_SETTING: i32 = 0x0026;
STRUCT! {#[repr(packed)] struct write_voice_setting_cp {
    voice_setting: u16,
}}
pub const WRITE_VOICE_SETTING_CP_SIZE: usize = 2;
pub const OCF_READ_AUTOMATIC_FLUSH_TIMEOUT: i32 = 0x0027;
pub const OCF_WRITE_AUTOMATIC_FLUSH_TIMEOUT: i32 = 0x0028;
pub const OCF_READ_NUM_BROADCAST_RETRANS: i32 = 0x0029;
pub const OCF_WRITE_NUM_BROADCAST_RETRANS: i32 = 0x002A;
pub const OCF_READ_HOLD_MODE_ACTIVITY: i32 = 0x002B;
pub const OCF_WRITE_HOLD_MODE_ACTIVITY: i32 = 0x002C;
pub const OCF_READ_TRANSMIT_POWER_LEVEL: i32 = 0x002D;
STRUCT! {#[repr(packed)] struct read_transmit_power_level_cp {
    handle: u16,
    type_: u8,
}}
pub const READ_TRANSMIT_POWER_LEVEL_CP_SIZE: usize = 3;
STRUCT! {#[repr(packed)] struct read_transmit_power_level_rp {
    status: u8,
    handle: u16,
    level: i8,
}}
pub const READ_TRANSMIT_POWER_LEVEL_RP_SIZE: usize = 4;
pub const OCF_READ_SYNC_FLOW_ENABLE: i32 = 0x002E;
pub const OCF_WRITE_SYNC_FLOW_ENABLE: i32 = 0x002F;
pub const OCF_SET_CONTROLLER_TO_HOST_FC: i32 = 0x0031;
pub const OCF_HOST_BUFFER_SIZE: usize = 0x0033;
STRUCT! {#[repr(packed)] struct host_buffer_size_cp {
    acl_mtu: u16,
    sco_mtu: u8,
    acl_max_pkt: u16,
    sco_max_pkt: u16,
}}
pub const HOST_BUFFER_SIZE_CP_SIZE: usize = 7;
pub const OCF_HOST_NUM_COMP_PKTS: i32 = 0x0035;
STRUCT! {#[repr(packed)] struct host_num_comp_pkts_cp {
    num_hndl: u8,
    // variable length part
}}
pub const HOST_NUM_COMP_PKTS_CP_SIZE: usize = 1;
pub const OCF_READ_LINK_SUPERVISION_TIMEOUT: i32 = 0x0036;
STRUCT! {#[repr(packed)] struct read_link_supervision_timeout_rp {
    status: u8,
    handle: u16,
    timeout: u16,
}}
pub const READ_LINK_SUPERVISION_TIMEOUT_RP_SIZE: usize = 5;
pub const OCF_WRITE_LINK_SUPERVISION_TIMEOUT: i32 = 0x0037;
STRUCT! {#[repr(packed)] struct write_link_supervision_timeout_cp {
    handle: u16,
    timeout: u16,
}}
pub const WRITE_LINK_SUPERVISION_TIMEOUT_CP_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct write_link_supervision_timeout_rp {
    status: u8,
    handle: u16,
}}
pub const WRITE_LINK_SUPERVISION_TIMEOUT_RP_SIZE: usize = 3;
pub const OCF_READ_NUM_SUPPORTED_IAC: i32 = 0x0038;
pub const MAX_IAC_LAP: usize = 0x40;
pub const OCF_READ_CURRENT_IAC_LAP: i32 = 0x0039;
STRUCT! {#[repr(packed)] struct read_current_iac_lap_rp {
    status: u8,
    num_current_iac: u8,
    lap: [[u8; MAX_IAC_LAP]; 3],
}}
pub const READ_CURRENT_IAC_LAP_RP_SIZE: usize = 2 + 3 * MAX_IAC_LAP;
pub const OCF_WRITE_CURRENT_IAC_LAP: i32 = 0x003A;
STRUCT! {#[repr(packed)] struct write_current_iac_lap_cp {
    num_current_iac: u8,
    lap: [[u8; MAX_IAC_LAP]; 3],
}}
pub const WRITE_CURRENT_IAC_LAP_CP_SIZE: usize = 1 + 3 * MAX_IAC_LAP;
pub const OCF_READ_PAGE_SCAN_PERIOD_MODE: i32 = 0x003B;
pub const OCF_WRITE_PAGE_SCAN_PERIOD_MODE: i32 = 0x003C;
pub const OCF_READ_PAGE_SCAN_MODE: i32 = 0x003D;
pub const OCF_WRITE_PAGE_SCAN_MODE: i32 = 0x003E;
pub const OCF_SET_AFH_CLASSIFICATION: i32 = 0x003F;
STRUCT! {#[repr(packed)] struct set_afh_classification_cp {
    map: [u8; 10],
}}
pub const SET_AFH_CLASSIFICATION_CP_SIZE: usize = 10;
STRUCT! {#[repr(packed)] struct set_afh_classification_rp {
    status: u8,
}}
pub const SET_AFH_CLASSIFICATION_RP_SIZE: usize = 1;
pub const OCF_READ_INQUIRY_SCAN_TYPE: i32 = 0x0042;
STRUCT! {#[repr(packed)] struct read_inquiry_scan_type_rp {
    status: u8,
    type_: u8,
}}
pub const READ_INQUIRY_SCAN_TYPE_RP_SIZE: usize = 2;
pub const OCF_WRITE_INQUIRY_SCAN_TYPE: i32 = 0x0043;
STRUCT! {#[repr(packed)] struct write_inquiry_scan_type_cp {
    type_: u8,
}}
pub const WRITE_INQUIRY_SCAN_TYPE_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_inquiry_scan_type_rp {
    status: u8,
}}
pub const WRITE_INQUIRY_SCAN_TYPE_RP_SIZE: usize = 1;
pub const OCF_READ_INQUIRY_MODE: i32 = 0x0044;
STRUCT! {#[repr(packed)] struct read_inquiry_mode_rp {
    status: u8,
    mode: u8,
}}
pub const READ_INQUIRY_MODE_RP_SIZE: usize = 2;
pub const OCF_WRITE_INQUIRY_MODE: i32 = 0x0045;
STRUCT! {#[repr(packed)] struct write_inquiry_mode_cp {
    mode: u8,
}}
pub const WRITE_INQUIRY_MODE_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_inquiry_mode_rp {
    status: u8,
}}
pub const WRITE_INQUIRY_MODE_RP_SIZE: usize = 1;
pub const OCF_READ_PAGE_SCAN_TYPE: i32 = 0x0046;
pub const OCF_WRITE_PAGE_SCAN_TYPE: i32 = 0x0047;
pub const PAGE_SCAN_TYPE_STANDARD: i32 = 0x00;
pub const PAGE_SCAN_TYPE_INTERLACED: i32 = 0x01;
pub const OCF_READ_AFH_MODE: i32 = 0x0048;
STRUCT! {#[repr(packed)] struct read_afh_mode_rp {
    status: u8,
    mode: u8,
}}
pub const READ_AFH_MODE_RP_SIZE: usize = 2;
pub const OCF_WRITE_AFH_MODE: i32 = 0x0049;
STRUCT! {#[repr(packed)] struct write_afh_mode_cp {
    mode: u8,
}}
pub const WRITE_AFH_MODE_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_afh_mode_rp {
    status: u8,
}}
pub const WRITE_AFH_MODE_RP_SIZE: usize = 1;
pub const HCI_MAX_EIR_LENGTH: usize = 240;
pub const OCF_READ_EXT_INQUIRY_RESPONSE: i32 = 0x0051;
STRUCT! {#[repr(packed)] struct read_ext_inquiry_response_rp {
    status: u8,
    fec: u8,
    data: [u8; HCI_MAX_EIR_LENGTH],
}}
pub const READ_EXT_INQUIRY_RESPONSE_RP_SIZE: usize = 242;
pub const OCF_WRITE_EXT_INQUIRY_RESPONSE: i32 = 0x0052;
STRUCT! {#[repr(packed)] struct write_ext_inquiry_response_cp {
    fec: u8,
    data: [u8; HCI_MAX_EIR_LENGTH],
}}
pub const WRITE_EXT_INQUIRY_RESPONSE_CP_SIZE: usize = 241;
STRUCT! {#[repr(packed)] struct write_ext_inquiry_response_rp {
    status: u8,
}}
pub const WRITE_EXT_INQUIRY_RESPONSE_RP_SIZE: usize = 1;
pub const OCF_REFRESH_ENCRYPTION_KEY: i32 = 0x0053;
STRUCT! {#[repr(packed)] struct refresh_encryption_key_cp {
    handle: u16,
}}
pub const REFRESH_ENCRYPTION_KEY_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct refresh_encryption_key_rp {
    status: u8,
}}
pub const REFRESH_ENCRYPTION_KEY_RP_SIZE: usize = 1;
pub const OCF_READ_SIMPLE_PAIRING_MODE: i32 = 0x0055;
STRUCT! {#[repr(packed)] struct read_simple_pairing_mode_rp {
    status: u8,
    mode: u8,
}}
pub const READ_SIMPLE_PAIRING_MODE_RP_SIZE: usize = 2;
pub const OCF_WRITE_SIMPLE_PAIRING_MODE: i32 = 0x0056;
STRUCT! {#[repr(packed)] struct write_simple_pairing_mode_cp {
    mode: u8,
}}
pub const WRITE_SIMPLE_PAIRING_MODE_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_simple_pairing_mode_rp {
    status: u8,
}}
pub const WRITE_SIMPLE_PAIRING_MODE_RP_SIZE: usize = 1;
pub const OCF_READ_LOCAL_OOB_DATA: i32 = 0x0057;
STRUCT! {#[repr(packed)] struct read_local_oob_data_rp {
    status: u8,
    hash: [u8; 16],
    randomizer: [u8; 16],
}}
pub const READ_LOCAL_OOB_DATA_RP_SIZE: usize = 33;
pub const OCF_READ_INQ_RESPONSE_TX_POWER_LEVEL: i32 = 0x0058;
STRUCT! {#[repr(packed)] struct read_inq_response_tx_power_level_rp {
    status: u8,
    level: i8,
}}
pub const READ_INQ_RESPONSE_TX_POWER_LEVEL_RP_SIZE: usize = 2;
pub const OCF_READ_INQUIRY_TRANSMIT_POWER_LEVEL: i32 = 0x0058;
STRUCT! {#[repr(packed)] struct read_inquiry_transmit_power_level_rp {
    status: u8,
    level: i8,
}}
pub const READ_INQUIRY_TRANSMIT_POWER_LEVEL_RP_SIZE: usize = 2;
pub const OCF_WRITE_INQUIRY_TRANSMIT_POWER_LEVEL: i32 = 0x0059;
STRUCT! {#[repr(packed)] struct write_inquiry_transmit_power_level_cp {
    level: i8,
}}
pub const WRITE_INQUIRY_TRANSMIT_POWER_LEVEL_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_inquiry_transmit_power_level_rp {
    status: u8,
}}
pub const WRITE_INQUIRY_TRANSMIT_POWER_LEVEL_RP_SIZE: usize = 1;
pub const OCF_READ_DEFAULT_ERROR_DATA_REPORTING: i32 = 0x005A;
STRUCT! {#[repr(packed)] struct read_default_error_data_reporting_rp {
    status: u8,
    reporting: u8,
}}
pub const READ_DEFAULT_ERROR_DATA_REPORTING_RP_SIZE: usize = 2;
pub const OCF_WRITE_DEFAULT_ERROR_DATA_REPORTING: i32 = 0x005B;
STRUCT! {#[repr(packed)] struct write_default_error_data_reporting_cp {
    reporting: u8,
}}
pub const WRITE_DEFAULT_ERROR_DATA_REPORTING_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_default_error_data_reporting_rp {
    status: u8,
}}
pub const WRITE_DEFAULT_ERROR_DATA_REPORTING_RP_SIZE: usize = 1;
pub const OCF_ENHANCED_FLUSH: i32 = 0x005F;
STRUCT! {#[repr(packed)] struct enhanced_flush_cp {
    handle: u16,
    type_: u8,
}}
pub const ENHANCED_FLUSH_CP_SIZE: usize = 3;
pub const OCF_SEND_KEYPRESS_NOTIFY: i32 = 0x0060;
STRUCT! {#[repr(packed)] struct send_keypress_notify_cp {
    bdaddr: bdaddr_t,
    type_: u8,
}}
pub const SEND_KEYPRESS_NOTIFY_CP_SIZE: usize = 7;
STRUCT! {#[repr(packed)] struct send_keypress_notify_rp {
    status: u8,
}}
pub const SEND_KEYPRESS_NOTIFY_RP_SIZE: usize = 1;
pub const OCF_READ_LOGICAL_LINK_ACCEPT_TIMEOUT: i32 = 0x0061;
STRUCT! {#[repr(packed)] struct read_log_link_accept_timeout_rp {
    status: u8,
    timeout: u16,
}}
pub const READ_LOGICAL_LINK_ACCEPT_TIMEOUT_RP_SIZE: usize = 3;
pub const OCF_WRITE_LOGICAL_LINK_ACCEPT_TIMEOUT: i32 = 0x0062;
STRUCT! {#[repr(packed)] struct write_log_link_accept_timeout_cp {
    timeout: u16,
}}
pub const WRITE_LOGICAL_LINK_ACCEPT_TIMEOUT_CP_SIZE: usize = 2;
pub const OCF_SET_EVENT_MASK_PAGE_2: i32 = 0x0063;
pub const OCF_READ_LOCATION_DATA: i32 = 0x0064;
pub const OCF_WRITE_LOCATION_DATA: i32 = 0x0065;
pub const OCF_READ_FLOW_CONTROL_MODE: i32 = 0x0066;
pub const OCF_WRITE_FLOW_CONTROL_MODE: i32 = 0x0067;
pub const OCF_READ_ENHANCED_TRANSMIT_POWER_LEVEL: i32 = 0x0068;
STRUCT! {#[repr(packed)] struct read_enhanced_transmit_power_level_rp {
    status: u8,
    handle: u16,
    level_gfsk: i8,
    level_dqpsk: i8,
    level_8dpsk: i8,
}}
pub const READ_ENHANCED_TRANSMIT_POWER_LEVEL_RP_SIZE: usize = 6;
pub const OCF_READ_BEST_EFFORT_FLUSH_TIMEOUT: i32 = 0x0069;
STRUCT! {#[repr(packed)] struct read_best_effort_flush_timeout_rp {
    status: u8,
    timeout: u32,
}}
pub const READ_BEST_EFFORT_FLUSH_TIMEOUT_RP_SIZE: usize = 5;
pub const OCF_WRITE_BEST_EFFORT_FLUSH_TIMEOUT: i32 = 0x006A;
STRUCT! {#[repr(packed)] struct write_best_effort_flush_timeout_cp {
    handle: u16,
    timeout: u32,
}}
pub const WRITE_BEST_EFFORT_FLUSH_TIMEOUT_CP_SIZE: usize = 6;
STRUCT! {#[repr(packed)] struct write_best_effort_flush_timeout_rp {
    status: u8,
}}
pub const WRITE_BEST_EFFORT_FLUSH_TIMEOUT_RP_SIZE: usize = 1;
pub const OCF_READ_LE_HOST_SUPPORTED: i32 = 0x006C;
STRUCT! {#[repr(packed)] struct read_le_host_supported_rp {
    status: u8,
    le: u8,
    simul: u8,
}}
pub const READ_LE_HOST_SUPPORTED_RP_SIZE: usize = 3;
pub const OCF_WRITE_LE_HOST_SUPPORTED: i32 = 0x006D;
STRUCT! {#[repr(packed)] struct write_le_host_supported_cp {
    le: u8,
    simul: u8,
}}
pub const WRITE_LE_HOST_SUPPORTED_CP_SIZE: usize = 2;
// Informational Parameters
pub const OGF_INFO_PARAM: i32 = 0x04;
pub const OCF_READ_LOCAL_VERSION: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct read_local_version_rp {
    status: u8,
    hci_ver: u8,
    hci_rev: u16,
    lmp_ver: u8,
    manufacturer: u16,
    lmp_subver: u16,
}}
pub const READ_LOCAL_VERSION_RP_SIZE: usize = 9;
pub const OCF_READ_LOCAL_COMMANDS: i32 = 0x0002;
STRUCT! {#[repr(packed)] struct read_local_commands_rp {
    status: u8,
    commands: [u8; 64],
}}
pub const READ_LOCAL_COMMANDS_RP_SIZE: usize = 65;
pub const OCF_READ_LOCAL_FEATURES: i32 = 0x0003;
STRUCT! {#[repr(packed)] struct read_local_features_rp {
    status: u8,
    features: [u8; 8],
}}
pub const READ_LOCAL_FEATURES_RP_SIZE: usize = 9;
pub const OCF_READ_LOCAL_EXT_FEATURES: i32 = 0x0004;
STRUCT! {#[repr(packed)] struct read_local_ext_features_cp {
    page_num: u8,
}}
pub const READ_LOCAL_EXT_FEATURES_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct read_local_ext_features_rp {
    status: u8,
    page_num: u8,
    max_page_num: u8,
    features: [u8; 8],
}}
pub const READ_LOCAL_EXT_FEATURES_RP_SIZE: usize = 11;
pub const OCF_READ_BUFFER_SIZE: usize = 0x0005;
STRUCT! {#[repr(packed)] struct read_buffer_size_rp {
    status: u8,
    acl_mtu: u16,
    sco_mtu: u8,
    acl_max_pkt: u16,
    sco_max_pkt: u16,
}}
pub const READ_BUFFER_SIZE_RP_SIZE: usize = 8;
pub const OCF_READ_COUNTRY_CODE: i32 = 0x0007;
pub const OCF_READ_BD_ADDR: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct read_bd_addr_rp {
    status: u8,
    bdaddr: bdaddr_t,
}}
pub const READ_BD_ADDR_RP_SIZE: usize = 7;
pub const OCF_READ_DATA_BLOCK_SIZE: usize = 0x000A;
STRUCT! {#[repr(packed)] struct read_data_block_size_rp {
    status: u8,
    max_acl_len: u16,
    data_block_len: u16,
    num_blocks: u16,
}}
// Status params
pub const OGF_STATUS_PARAM: i32 = 0x05;
pub const OCF_READ_FAILED_CONTACT_COUNTER: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct read_failed_contact_counter_rp {
    status: u8,
    handle: u16,
    counter: u8,
}}
pub const READ_FAILED_CONTACT_COUNTER_RP_SIZE: usize = 4;
pub const OCF_RESET_FAILED_CONTACT_COUNTER: i32 = 0x0002;
STRUCT! {#[repr(packed)] struct reset_failed_contact_counter_rp {
    status: u8,
    handle: u16,
}}
pub const RESET_FAILED_CONTACT_COUNTER_RP_SIZE: usize = 3;
pub const OCF_READ_LINK_QUALITY: i32 = 0x0003;
STRUCT! {#[repr(packed)] struct read_link_quality_rp {
    status: u8,
    handle: u16,
    link_quality: u8,
}}
pub const READ_LINK_QUALITY_RP_SIZE: usize = 4;
pub const OCF_READ_RSSI: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct read_rssi_rp {
    status: u8,
    handle: u16,
    rssi: i8,
}}
pub const READ_RSSI_RP_SIZE: usize = 4;
pub const OCF_READ_AFH_MAP: i32 = 0x0006;
STRUCT! {#[repr(packed)] struct read_afh_map_rp {
    status: u8,
    handle: u16,
    mode: u8,
    map: [u8; 10],
}}
pub const READ_AFH_MAP_RP_SIZE: usize = 14;
pub const OCF_READ_CLOCK: i32 = 0x0007;
STRUCT! {#[repr(packed)] struct read_clock_cp {
    handle: u16,
    which_clock: u8,
}}
pub const READ_CLOCK_CP_SIZE: usize = 3;
STRUCT! {#[repr(packed)] struct read_clock_rp {
    status: u8,
    handle: u16,
    clock: u32,
    accuracy: u16,
}}
pub const READ_CLOCK_RP_SIZE: usize = 9;
pub const OCF_READ_LOCAL_AMP_INFO: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct read_local_amp_info_rp {
    status: u8,
    amp_status: u8,
    total_bandwidth: u32,
    max_guaranteed_bandwidth: u32,
    min_latency: u32,
    max_pdu_size: u32,
    controller_type_: u8,
    pal_caps: u16,
    max_amp_assoc_length: u16,
    max_flush_timeout: u32,
    best_effort_flush_timeout: u32,
}}
pub const READ_LOCAL_AMP_INFO_RP_SIZE: usize = 31;
pub const OCF_READ_LOCAL_AMP_ASSOC: i32 = 0x000A;
STRUCT! {#[repr(packed)] struct read_local_amp_assoc_cp {
    handle: u8,
    length_so_far: u16,
    assoc_length: u16,
}}
pub const READ_LOCAL_AMP_ASSOC_CP_SIZE: usize = 5;
STRUCT! {#[repr(packed)] struct read_local_amp_assoc_rp {
    status: u8,
    handle: u8,
    length: u16,
    fragment: [u8; HCI_MAX_NAME_LENGTH],
}}
pub const READ_LOCAL_AMP_ASSOC_RP_SIZE: usize = 252;
pub const OCF_WRITE_REMOTE_AMP_ASSOC: i32 = 0x000B;
STRUCT! {#[repr(packed)] struct write_remote_amp_assoc_cp {
    handle: u8,
    length_so_far: u16,
    remaining_length: u16,
    fragment: [u8; HCI_MAX_NAME_LENGTH],
}}
pub const WRITE_REMOTE_AMP_ASSOC_CP_SIZE: usize = 253;
STRUCT! {#[repr(packed)] struct write_remote_amp_assoc_rp {
    status: u8,
    handle: u8,
}}
pub const WRITE_REMOTE_AMP_ASSOC_RP_SIZE: usize = 2;
// Testing commands
pub const OGF_TESTING_CMD: i32 = 0x3e;
pub const OCF_READ_LOOPBACK_MODE: i32 = 0x0001;
pub const OCF_WRITE_LOOPBACK_MODE: i32 = 0x0002;
pub const OCF_ENABLE_DEVICE_UNDER_TEST_MODE: i32 = 0x0003;
pub const OCF_WRITE_SIMPLE_PAIRING_DEBUG_MODE: i32 = 0x0004;
STRUCT! {#[repr(packed)] struct write_simple_pairing_debug_mode_cp {
    mode: u8,
}}
pub const WRITE_SIMPLE_PAIRING_DEBUG_MODE_CP_SIZE: usize = 1;
STRUCT! {#[repr(packed)] struct write_simple_pairing_debug_mode_rp {
    status: u8,
}}
pub const WRITE_SIMPLE_PAIRING_DEBUG_MODE_RP_SIZE: usize = 1;
// LE commands
pub const OGF_LE_CTL: i32 = 0x08;
pub const OCF_LE_SET_EVENT_MASK: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct le_set_event_mask_cp {
    mask: [u8; 8],
}}
pub const LE_SET_EVENT_MASK_CP_SIZE: usize = 8;
pub const OCF_LE_READ_BUFFER_SIZE: usize = 0x0002;
STRUCT! {#[repr(packed)] struct le_read_buffer_size_rp {
    status: u8,
    pkt_len: u16,
    max_pkt: u8,
}}
pub const LE_READ_BUFFER_SIZE_RP_SIZE: usize = 4;
pub const OCF_LE_READ_LOCAL_SUPPORTED_FEATURES: i32 = 0x0003;
STRUCT! {#[repr(packed)] struct le_read_local_supported_features_rp {
    status: u8,
    features: [u8; 8],
}}
pub const LE_READ_LOCAL_SUPPORTED_FEATURES_RP_SIZE: usize = 9;
pub const OCF_LE_SET_RANDOM_ADDRESS: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct le_set_random_address_cp {
    bdaddr: bdaddr_t,
}}
pub const LE_SET_RANDOM_ADDRESS_CP_SIZE: usize = 6;
pub const OCF_LE_SET_ADVERTISING_PARAMETERS: i32 = 0x0006;
STRUCT! {#[repr(packed)] struct le_set_advertising_parameters_cp {
    min_interval: u16,
    max_interval: u16,
    advtype_: u8,
    own_bdaddr_type_: u8,
    direct_bdaddr_type_: u8,
    direct_bdaddr: bdaddr_t,
    chan_map: u8,
    filter: u8,
}}
pub const LE_SET_ADVERTISING_PARAMETERS_CP_SIZE: usize = 15;
pub const OCF_LE_READ_ADVERTISING_CHANNEL_TX_POWER: i32 = 0x0007;
STRUCT! {#[repr(packed)] struct le_read_advertising_channel_tx_power_rp {
    status: u8,
    level: i8,
}}
pub const LE_READ_ADVERTISING_CHANNEL_TX_POWER_RP_SIZE: usize = 2;
pub const OCF_LE_SET_ADVERTISING_DATA: i32 = 0x0008;
STRUCT! {#[repr(packed)] struct le_set_advertising_data_cp {
    length: u8,
    data: [u8; 31],
}}
pub const LE_SET_ADVERTISING_DATA_CP_SIZE: usize = 32;
pub const OCF_LE_SET_SCAN_RESPONSE_DATA: i32 = 0x0009;
STRUCT! {#[repr(packed)] struct le_set_scan_response_data_cp {
    length: u8,
    data: [u8; 31],
}}
pub const LE_SET_SCAN_RESPONSE_DATA_CP_SIZE: usize = 32;
pub const OCF_LE_SET_ADVERTISE_ENABLE: i32 = 0x000A;
STRUCT! {#[repr(packed)] struct le_set_advertise_enable_cp {
    enable: u8,
}}
pub const LE_SET_ADVERTISE_ENABLE_CP_SIZE: usize = 1;
pub const OCF_LE_SET_SCAN_PARAMETERS: i32 = 0x000B;
STRUCT! {#[repr(packed)] struct le_set_scan_parameters_cp {
    type_: u8,
    interval: u16,
    window: u16,
    own_bdaddr_type_: u8,
    filter: u8,
}}
pub const LE_SET_SCAN_PARAMETERS_CP_SIZE: usize = 7;
pub const OCF_LE_SET_SCAN_ENABLE: i32 = 0x000C;
STRUCT! {#[repr(packed)] struct le_set_scan_enable_cp {
    enable: u8,
    filter_dup: u8,
}}
pub const LE_SET_SCAN_ENABLE_CP_SIZE: usize = 2;
pub const OCF_LE_CREATE_CONN: i32 = 0x000D;
STRUCT! {#[repr(packed)] struct le_create_connection_cp {
    interval: u16,
    window: u16,
    initiator_filter: u8,
    peer_bdaddr_type_: u8,
    peer_bdaddr: bdaddr_t,
    own_bdaddr_type_: u8,
    min_interval: u16,
    max_interval: u16,
    latency: u16,
    supervision_timeout: u16,
    min_ce_length: u16,
    max_ce_length: u16,
}}
pub const LE_CREATE_CONN_CP_SIZE: usize = 25;
pub const OCF_LE_CREATE_CONN_CANCEL: i32 = 0x000E;
pub const OCF_LE_READ_WHITE_LIST_SIZE: usize = 0x000F;
STRUCT! {#[repr(packed)] struct le_read_white_list_size_rp {
    status: u8,
    size: u8,
}}
pub const LE_READ_WHITE_LIST_SIZE_RP_SIZE: usize = 2;
pub const OCF_LE_CLEAR_WHITE_LIST: i32 = 0x0010;
pub const OCF_LE_ADD_DEVICE_TO_WHITE_LIST: i32 = 0x0011;
STRUCT! {#[repr(packed)] struct le_add_device_to_white_list_cp {
    bdaddr_type_: u8,
    bdaddr: bdaddr_t,
}}
pub const LE_ADD_DEVICE_TO_WHITE_LIST_CP_SIZE: usize = 7;
pub const OCF_LE_REMOVE_DEVICE_FROM_WHITE_LIST: i32 = 0x0012;
STRUCT! {#[repr(packed)] struct le_remove_device_from_white_list_cp {
    bdaddr_type_: u8,
    bdaddr: bdaddr_t,
}}
pub const LE_REMOVE_DEVICE_FROM_WHITE_LIST_CP_SIZE: usize = 7;
pub const OCF_LE_CONN_UPDATE: i32 = 0x0013;
STRUCT! {#[repr(packed)] struct le_connection_update_cp {
    handle: u16,
    min_interval: u16,
    max_interval: u16,
    latency: u16,
    supervision_timeout: u16,
    min_ce_length: u16,
    max_ce_length: u16,
}}
pub const LE_CONN_UPDATE_CP_SIZE: usize = 14;
pub const OCF_LE_SET_HOST_CHANNEL_CLASSIFICATION: i32 = 0x0014;
STRUCT! {#[repr(packed)] struct le_set_host_channel_classification_cp {
    map: [u8; 5],
}}
pub const LE_SET_HOST_CHANNEL_CLASSIFICATION_CP_SIZE: usize = 5;
pub const OCF_LE_READ_CHANNEL_MAP: i32 = 0x0015;
STRUCT! {#[repr(packed)] struct le_read_channel_map_cp {
    handle: u16,
}}
pub const LE_READ_CHANNEL_MAP_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct le_read_channel_map_rp {
    status: u8,
    handle: u16,
    map: [u8; 5],
}}
pub const LE_READ_CHANNEL_MAP_RP_SIZE: usize = 8;
pub const OCF_LE_READ_REMOTE_USED_FEATURES: i32 = 0x0016;
STRUCT! {#[repr(packed)] struct le_read_remote_used_features_cp {
    handle: u16,
}}
pub const LE_READ_REMOTE_USED_FEATURES_CP_SIZE: usize = 2;
pub const OCF_LE_ENCRYPT: i32 = 0x0017;
STRUCT! {#[repr(packed)] struct le_encrypt_cp {
    key: [u8; 16],
    plaintext: [u8; 16],
}}
pub const LE_ENCRYPT_CP_SIZE: usize = 32;
STRUCT! {#[repr(packed)] struct le_encrypt_rp {
    status: u8,
    data: [u8; 16],
}}
pub const LE_ENCRYPT_RP_SIZE: usize = 17;
pub const OCF_LE_RAND: i32 = 0x0018;
STRUCT! {#[repr(packed)] struct le_rand_rp {
    status: u8,
    random: u64,
}}
pub const LE_RAND_RP_SIZE: usize = 9;
pub const OCF_LE_START_ENCRYPTION: i32 = 0x0019;
STRUCT! {#[repr(packed)] struct le_start_encryption_cp {
    handle: u16,
    random: u64,
    diversifier: u16,
    key: [u8; 16],
}}
pub const LE_START_ENCRYPTION_CP_SIZE: usize = 28;
pub const OCF_LE_LTK_REPLY: i32 = 0x001A;
STRUCT! {#[repr(packed)] struct le_ltk_reply_cp {
    handle: u16,
    key: [u8; 16],
}}
pub const LE_LTK_REPLY_CP_SIZE: usize = 18;
STRUCT! {#[repr(packed)] struct le_ltk_reply_rp {
    status: u8,
    handle: u16,
}}
pub const LE_LTK_REPLY_RP_SIZE: usize = 3;
pub const OCF_LE_LTK_NEG_REPLY: i32 = 0x001B;
STRUCT! {#[repr(packed)] struct le_ltk_neg_reply_cp {
    handle: u16,
}}
pub const LE_LTK_NEG_REPLY_CP_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct le_ltk_neg_reply_rp {
    status: u8,
    handle: u16,
}}
pub const LE_LTK_NEG_REPLY_RP_SIZE: usize = 3;
pub const OCF_LE_READ_SUPPORTED_STATES: i32 = 0x001C;
STRUCT! {#[repr(packed)] struct le_read_supported_states_rp {
    status: u8,
    states: u64,
}}
pub const LE_READ_SUPPORTED_STATES_RP_SIZE: usize = 9;
pub const OCF_LE_RECEIVER_TEST: i32 = 0x001D;
STRUCT! {#[repr(packed)] struct le_receiver_test_cp {
    frequency: u8,
}}
pub const LE_RECEIVER_TEST_CP_SIZE: usize = 1;
pub const OCF_LE_TRANSMITTER_TEST: i32 = 0x001E;
STRUCT! {#[repr(packed)] struct le_transmitter_test_cp {
    frequency: u8,
    length: u8,
    payload: u8,
}}
pub const LE_TRANSMITTER_TEST_CP_SIZE: usize = 3;
pub const OCF_LE_TEST_END: i32 = 0x001F;
STRUCT! {#[repr(packed)] struct le_test_end_rp {
    status: u8,
    num_pkts: u16,
}}
pub const LE_TEST_END_RP_SIZE: usize = 3;
pub const OCF_LE_ADD_DEVICE_TO_RESOLV_LIST: i32 = 0x0027;
STRUCT! {#[repr(packed)] struct le_add_device_to_resolv_list_cp {
    bdaddr_type_: u8,
    bdaddr: bdaddr_t,
    peer_irk: [u8; 16],
    local_irk: [u8; 16],
}}
pub const LE_ADD_DEVICE_TO_RESOLV_LIST_CP_SIZE: usize = 39;
pub const OCF_LE_REMOVE_DEVICE_FROM_RESOLV_LIST: i32 = 0x0028;
STRUCT! {#[repr(packed)] struct le_remove_device_from_resolv_list_cp {
    bdaddr_type_: u8,
    bdaddr: bdaddr_t,
}}
pub const LE_REMOVE_DEVICE_FROM_RESOLV_LIST_CP_SIZE: usize = 7;
pub const OCF_LE_CLEAR_RESOLV_LIST: i32 = 0x0029;
pub const OCF_LE_READ_RESOLV_LIST_SIZE: usize = 0x002A;
STRUCT! {#[repr(packed)] struct le_read_resolv_list_size_rp {
    status: u8,
    size: u8,
}}
pub const LE_READ_RESOLV_LIST_SIZE_RP_SIZE: usize = 2;
pub const OCF_LE_SET_ADDRESS_RESOLUTION_ENABLE: i32 = 0x002D;
STRUCT! {#[repr(packed)] struct le_set_address_resolution_enable_cp {
    enable: u8,
}}
pub const LE_SET_ADDRESS_RESOLUTION_ENABLE_CP_SIZE: usize = 1;
// Vendor specific commands
pub const OGF_VENDOR_CMD: i32 = 0x3f;
// ---- HCI Events ----
pub const EVT_INQUIRY_COMPLETE: i32 = 0x01;
pub const EVT_INQUIRY_RESULT: i32 = 0x02;
STRUCT! {#[repr(packed)] struct inquiry_info {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
    pscan_period_mode: u8,
    pscan_mode: u8,
    dev_class: [u8; 3],
    clock_offset: u16,
}}
pub const INQUIRY_INFO_SIZE: usize = 14;
pub const EVT_CONN_COMPLETE: i32 = 0x03;
STRUCT! {#[repr(packed)] struct evt_conn_complete {
    status: u8,
    handle: u16,
    bdaddr: bdaddr_t,
    link_type_: u8,
    encr_mode: u8,
}}
pub const EVT_CONN_COMPLETE_SIZE: usize = 11;
pub const EVT_CONN_REQUEST: i32 = 0x04;
STRUCT! {#[repr(packed)] struct evt_conn_request {
    bdaddr: bdaddr_t,
    dev_class: [u8; 3],
    link_type_: u8,
}}
pub const EVT_CONN_REQUEST_SIZE: usize = 10;
pub const EVT_DISCONN_COMPLETE: i32 = 0x05;
STRUCT! {#[repr(packed)] struct evt_disconn_complete {
    status: u8,
    handle: u16,
    reason: u8,
}}
pub const EVT_DISCONN_COMPLETE_SIZE: usize = 4;
pub const EVT_AUTH_COMPLETE: i32 = 0x06;
STRUCT! {#[repr(packed)] struct evt_auth_complete {
    status: u8,
    handle: u16,
}}
pub const EVT_AUTH_COMPLETE_SIZE: usize = 3;
pub const EVT_REMOTE_NAME_REQ_COMPLETE: i32 = 0x07;
STRUCT! {#[repr(packed)] struct evt_remote_name_req_complete {
    status: u8,
    bdaddr: bdaddr_t,
    name: [u8; HCI_MAX_NAME_LENGTH],
}}
pub const EVT_REMOTE_NAME_REQ_COMPLETE_SIZE: usize = 255;
pub const EVT_ENCRYPT_CHANGE: i32 = 0x08;
STRUCT! {#[repr(packed)] struct evt_encrypt_change {
    status: u8,
    handle: u16,
    encrypt: u8,
}}
pub const EVT_ENCRYPT_CHANGE_SIZE: usize = 4;
pub const EVT_CHANGE_CONN_LINK_KEY_COMPLETE: i32 = 0x09;
STRUCT! {#[repr(packed)] struct evt_change_conn_link_key_complete {
    status: u8,
    handle: u16,
}}
pub const EVT_CHANGE_CONN_LINK_KEY_COMPLETE_SIZE: usize = 3;
pub const EVT_MASTER_LINK_KEY_COMPLETE: i32 = 0x0A;
STRUCT! {#[repr(packed)] struct evt_master_link_key_complete {
    status: u8,
    handle: u16,
    key_flag: u8,
}}
pub const EVT_MASTER_LINK_KEY_COMPLETE_SIZE: usize = 4;
pub const EVT_READ_REMOTE_FEATURES_COMPLETE: i32 = 0x0B;
STRUCT! {#[repr(packed)] struct evt_read_remote_features_complete {
    status: u8,
    handle: u16,
    features: [u8; 8],
}}
pub const EVT_READ_REMOTE_FEATURES_COMPLETE_SIZE: usize = 11;
pub const EVT_READ_REMOTE_VERSION_COMPLETE: i32 = 0x0C;
STRUCT! {#[repr(packed)] struct evt_read_remote_version_complete {
    status: u8,
    handle: u16,
    lmp_ver: u8,
    manufacturer: u16,
    lmp_subver: u16,
}}
pub const EVT_READ_REMOTE_VERSION_COMPLETE_SIZE: usize = 8;
pub const EVT_QOS_SETUP_COMPLETE: i32 = 0x0D;
STRUCT! {#[repr(packed)] struct evt_qos_setup_complete {
    status: u8,
    handle: u16,
    flags: u8,      // Reserved
    qos: hci_qos,
}}
pub const EVT_QOS_SETUP_COMPLETE_SIZE: usize = 4 + HCI_QOS_CP_SIZE;
pub const EVT_CMD_COMPLETE: i32 = 0x0E;
STRUCT! {#[repr(packed)] struct evt_cmd_complete {
    ncmd: u8,
    opcode: u16,
}}
pub const EVT_CMD_COMPLETE_SIZE: usize = 3;
pub const EVT_CMD_STATUS: i32 = 0x0F;
STRUCT! {#[repr(packed)] struct evt_cmd_status {
    status: u8,
    ncmd: u8,
    opcode: u16,
}}
pub const EVT_CMD_STATUS_SIZE: usize = 4;
pub const EVT_HARDWARE_ERROR: i32 = 0x10;
STRUCT! {#[repr(packed)] struct evt_hardware_error {
    code: u8,
}}
pub const EVT_HARDWARE_ERROR_SIZE: usize = 1;
pub const EVT_FLUSH_OCCURRED: i32 = 0x11;
STRUCT! {#[repr(packed)] struct evt_flush_occured {
    handle: u16,
}}
pub const EVT_FLUSH_OCCURRED_SIZE: usize = 2;
pub const EVT_ROLE_CHANGE: i32 = 0x12;
STRUCT! {#[repr(packed)] struct evt_role_change {
    status: u8,
    bdaddr: bdaddr_t,
    role: u8,
}}
pub const EVT_ROLE_CHANGE_SIZE: usize = 8;
pub const EVT_NUM_COMP_PKTS: i32 = 0x13;
STRUCT! {#[repr(packed)] struct evt_num_comp_pkts {
    num_hndl: u8,
    // variable length part
}}
pub const EVT_NUM_COMP_PKTS_SIZE: usize = 1;
pub const EVT_MODE_CHANGE: i32 = 0x14;
STRUCT! {#[repr(packed)] struct evt_mode_change {
    status: u8,
    handle: u16,
    mode: u8,
    interval: u16,
}}
pub const EVT_MODE_CHANGE_SIZE: usize = 6;
pub const EVT_RETURN_LINK_KEYS: i32 = 0x15;
STRUCT! {#[repr(packed)] struct evt_return_link_keys {
    num_keys: u8,
    // variable length part
}}
pub const EVT_RETURN_LINK_KEYS_SIZE: usize = 1;
pub const EVT_PIN_CODE_REQ: i32 = 0x16;
STRUCT! {#[repr(packed)] struct evt_pin_code_req {
    bdaddr: bdaddr_t,
}}
pub const EVT_PIN_CODE_REQ_SIZE: usize = 6;
pub const EVT_LINK_KEY_REQ: i32 = 0x17;
STRUCT! {#[repr(packed)] struct evt_link_key_req {
    bdaddr: bdaddr_t,
}}
pub const EVT_LINK_KEY_REQ_SIZE: usize = 6;
pub const EVT_LINK_KEY_NOTIFY: i32 = 0x18;
STRUCT! {#[repr(packed)] struct evt_link_key_notify {
    bdaddr: bdaddr_t,
    link_key: [u8; 16],
    key_type_: u8,
}}
pub const EVT_LINK_KEY_NOTIFY_SIZE: usize = 23;
pub const EVT_LOOPBACK_COMMAND: i32 = 0x19;
pub const EVT_DATA_BUFFER_OVERFLOW: i32 = 0x1A;
STRUCT! {#[repr(packed)] struct evt_data_buffer_overflow {
    link_type_: u8,
}}
pub const EVT_DATA_BUFFER_OVERFLOW_SIZE: usize = 1;
pub const EVT_MAX_SLOTS_CHANGE: i32 = 0x1B;
STRUCT! {#[repr(packed)] struct evt_max_slots_change {
    handle: u16,
    max_slots: u8,
}}
pub const EVT_MAX_SLOTS_CHANGE_SIZE: usize = 3;
pub const EVT_READ_CLOCK_OFFSET_COMPLETE: i32 = 0x1C;
STRUCT! {#[repr(packed)] struct evt_read_clock_offset_complete {
    status: u8,
    handle: u16,
    clock_offset: u16,
}}
pub const EVT_READ_CLOCK_OFFSET_COMPLETE_SIZE: usize = 5;
pub const EVT_CONN_PTYPE_CHANGED: i32 = 0x1D;
STRUCT! {#[repr(packed)] struct evt_conn_ptype_changed {
    status: u8,
    handle: u16,
    ptype_: u16,
}}
pub const EVT_CONN_PTYPE_CHANGED_SIZE: usize = 5;
pub const EVT_QOS_VIOLATION: i32 = 0x1E;
STRUCT! {#[repr(packed)] struct evt_qos_violation {
    handle: u16,
}}
pub const EVT_QOS_VIOLATION_SIZE: usize = 2;
pub const EVT_PSCAN_REP_MODE_CHANGE: i32 = 0x20;
STRUCT! {#[repr(packed)] struct evt_pscan_rep_mode_change {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
}}
pub const EVT_PSCAN_REP_MODE_CHANGE_SIZE: usize = 7;
pub const EVT_FLOW_SPEC_COMPLETE: i32 = 0x21;
STRUCT! {#[repr(packed)] struct evt_flow_spec_complete {
    status: u8,
    handle: u16,
    flags: u8,
    direction: u8,
    qos: hci_qos,
}}
pub const EVT_FLOW_SPEC_COMPLETE_SIZE: usize = 5 + HCI_QOS_CP_SIZE;
pub const EVT_INQUIRY_RESULT_WITH_RSSI: i32 = 0x22;
STRUCT! {#[repr(packed)] struct inquiry_info_with_rssi {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
    pscan_period_mode: u8,
    dev_class: [u8; 3],
    clock_offset: u16,
    rssi: i8,
}}
pub const INQUIRY_INFO_WITH_RSSI_SIZE: usize = 14;
STRUCT! {#[repr(packed)] struct inquiry_info_with_rssi_and_pscan_mode {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
    pscan_period_mode: u8,
    pscan_mode: u8,
    dev_class: [u8; 3],
    clock_offset: u16,
    rssi: i8,
}}
pub const INQUIRY_INFO_WITH_RSSI_AND_PSCAN_MODE_SIZE: usize = 15;
pub const EVT_READ_REMOTE_EXT_FEATURES_COMPLETE: i32 = 0x23;
STRUCT! {#[repr(packed)] struct evt_read_remote_ext_features_complete {
    status: u8,
    handle: u16,
    page_num: u8,
    max_page_num: u8,
    features: [u8; 8],
}}
pub const EVT_READ_REMOTE_EXT_FEATURES_COMPLETE_SIZE: usize = 13;
pub const EVT_SYNC_CONN_COMPLETE: i32 = 0x2C;
STRUCT! {#[repr(packed)] struct evt_sync_conn_complete {
    status: u8,
    handle: u16,
    bdaddr: bdaddr_t,
    link_type_: u8,
    trans_interval: u8,
    retrans_window: u8,
    rx_pkt_len: u16,
    tx_pkt_len: u16,
    air_mode: u8,
}}
pub const EVT_SYNC_CONN_COMPLETE_SIZE: usize = 17;
pub const EVT_SYNC_CONN_CHANGED: i32 = 0x2D;
STRUCT! {#[repr(packed)] struct evt_sync_conn_changed {
    status: u8,
    handle: u16,
    trans_interval: u8,
    retrans_window: u8,
    rx_pkt_len: u16,
    tx_pkt_len: u16,
}}
pub const EVT_SYNC_CONN_CHANGED_SIZE: usize = 9;
pub const EVT_SNIFF_SUBRATING: i32 = 0x2E;
STRUCT! {#[repr(packed)] struct evt_sniff_subrating {
    status: u8,
    handle: u16,
    max_tx_latency: u16,
    max_rx_latency: u16,
    min_remote_timeout: u16,
    min_local_timeout: u16,
}}
pub const EVT_SNIFF_SUBRATING_SIZE: usize = 11;
pub const EVT_EXTENDED_INQUIRY_RESULT: i32 = 0x2F;
STRUCT! {#[repr(packed)] struct extended_inquiry_info {
    bdaddr: bdaddr_t,
    pscan_rep_mode: u8,
    pscan_period_mode: u8,
    dev_class: [u8; 3],
    clock_offset: u16,
    rssi: i8,
    data: [u8; HCI_MAX_EIR_LENGTH],
}}
pub const EXTENDED_INQUIRY_INFO_SIZE: usize = 254;
pub const EVT_ENCRYPTION_KEY_REFRESH_COMPLETE: i32 = 0x30;
STRUCT! {#[repr(packed)] struct evt_encryption_key_refresh_complete {
    status: u8,
    handle: u16,
}}
pub const EVT_ENCRYPTION_KEY_REFRESH_COMPLETE_SIZE: usize = 3;
pub const EVT_IO_CAPABILITY_REQUEST: i32 = 0x31;
STRUCT! {#[repr(packed)] struct evt_io_capability_request {
    bdaddr: bdaddr_t,
}}
pub const EVT_IO_CAPABILITY_REQUEST_SIZE: usize = 6;
pub const EVT_IO_CAPABILITY_RESPONSE: i32 = 0x32;
STRUCT! {#[repr(packed)] struct evt_io_capability_response {
    bdaddr: bdaddr_t,
    capability: u8,
    oob_data: u8,
    authentication: u8,
}}
pub const EVT_IO_CAPABILITY_RESPONSE_SIZE: usize = 9;
pub const EVT_USER_CONFIRM_REQUEST: i32 = 0x33;
STRUCT! {#[repr(packed)] struct evt_user_confirm_request {
    bdaddr: bdaddr_t,
    passkey: u32,
}}
pub const EVT_USER_CONFIRM_REQUEST_SIZE: usize = 10;
pub const EVT_USER_PASSKEY_REQUEST: i32 = 0x34;
STRUCT! {#[repr(packed)] struct evt_user_passkey_request {
    bdaddr: bdaddr_t,
}}
pub const EVT_USER_PASSKEY_REQUEST_SIZE: usize = 6;
pub const EVT_REMOTE_OOB_DATA_REQUEST: i32 = 0x35;
STRUCT! {#[repr(packed)] struct evt_remote_oob_data_request {
    bdaddr: bdaddr_t,
}}
pub const EVT_REMOTE_OOB_DATA_REQUEST_SIZE: usize = 6;
pub const EVT_SIMPLE_PAIRING_COMPLETE: i32 = 0x36;
STRUCT! {#[repr(packed)] struct evt_simple_pairing_complete {
    status: u8,
    bdaddr: bdaddr_t,
}}
pub const EVT_SIMPLE_PAIRING_COMPLETE_SIZE: usize = 7;
pub const EVT_LINK_SUPERVISION_TIMEOUT_CHANGED: i32 = 0x38;
STRUCT! {#[repr(packed)] struct evt_link_supervision_timeout_changed {
    handle: u16,
    timeout: u16,
}}
pub const EVT_LINK_SUPERVISION_TIMEOUT_CHANGED_SIZE: usize = 4;
pub const EVT_ENHANCED_FLUSH_COMPLETE: i32 = 0x39;
STRUCT! {#[repr(packed)] struct evt_enhanced_flush_complete {
    handle: u16,
}}
pub const EVT_ENHANCED_FLUSH_COMPLETE_SIZE: usize = 2;
pub const EVT_USER_PASSKEY_NOTIFY: i32 = 0x3B;
STRUCT! {#[repr(packed)] struct evt_user_passkey_notify {
    bdaddr: bdaddr_t,
    passkey: u32,
}}
pub const EVT_USER_PASSKEY_NOTIFY_SIZE: usize = 10;
pub const EVT_KEYPRESS_NOTIFY: i32 = 0x3C;
STRUCT! {#[repr(packed)] struct evt_keypress_notify {
    bdaddr: bdaddr_t,
    type_: u8,
}}
pub const EVT_KEYPRESS_NOTIFY_SIZE: usize = 7;
pub const EVT_REMOTE_HOST_FEATURES_NOTIFY: i32 = 0x3D;
STRUCT! {#[repr(packed)] struct evt_remote_host_features_notify {
    bdaddr: bdaddr_t,
    features: [u8; 8],
}}
pub const EVT_REMOTE_HOST_FEATURES_NOTIFY_SIZE: usize = 14;
pub const EVT_LE_META_EVENT: i32 = 0x3E;
STRUCT! {#[repr(packed)] struct evt_le_meta_event {
    subevent: u8,
    data: [u8; 0],
}}
pub const EVT_LE_META_EVENT_SIZE: usize = 1;
pub const EVT_LE_CONN_COMPLETE: i32 = 0x01;
STRUCT! {#[repr(packed)] struct evt_le_connection_complete {
    status: u8,
    handle: u16,
    role: u8,
    peer_bdaddr_type_: u8,
    peer_bdaddr: bdaddr_t,
    interval: u16,
    latency: u16,
    supervision_timeout: u16,
    master_clock_accuracy: u8,
}}
pub const EVT_LE_CONN_COMPLETE_SIZE: usize = 18;
pub const EVT_LE_ADVERTISING_REPORT: i32 = 0x02;
STRUCT! {#[repr(packed)] struct le_advertising_info {
    evt_type_: u8,
    bdaddr_type_: u8,
    bdaddr: bdaddr_t,
    length: u8,
    data: [u8; 0],
}}
pub const LE_ADVERTISING_INFO_SIZE: usize = 9;
pub const EVT_LE_CONN_UPDATE_COMPLETE: i32 = 0x03;
STRUCT! {#[repr(packed)] struct evt_le_connection_update_complete {
    status: u8,
    handle: u16,
    interval: u16,
    latency: u16,
    supervision_timeout: u16,
}}
pub const EVT_LE_CONN_UPDATE_COMPLETE_SIZE: usize = 9;
pub const EVT_LE_READ_REMOTE_USED_FEATURES_COMPLETE: i32 = 0x04;
STRUCT! {#[repr(packed)] struct evt_le_read_remote_used_features_complete {
    status: u8,
    handle: u16,
    features: [u8; 8],
}}
pub const EVT_LE_READ_REMOTE_USED_FEATURES_COMPLETE_SIZE: usize = 11;
pub const EVT_LE_LTK_REQUEST: i32 = 0x05;
STRUCT! {#[repr(packed)] struct evt_le_long_term_key_request {
    handle: u16,
    random: u64,
    diversifier: u16,
}}
pub const EVT_LE_LTK_REQUEST_SIZE: usize = 12;
pub const EVT_PHYSICAL_LINK_COMPLETE: i32 = 0x40;
STRUCT! {#[repr(packed)] struct evt_physical_link_complete {
    status: u8,
    handle: u8,
}}
pub const EVT_PHYSICAL_LINK_COMPLETE_SIZE: usize = 2;
pub const EVT_CHANNEL_SELECTED: i32 = 0x41;
pub const EVT_DISCONNECT_PHYSICAL_LINK_COMPLETE: i32 = 0x42;
STRUCT! {#[repr(packed)] struct evt_disconn_physical_link_complete {
    status: u8,
    handle: u8,
    reason: u8,
}}
pub const EVT_DISCONNECT_PHYSICAL_LINK_COMPLETE_SIZE: usize = 3;
pub const EVT_PHYSICAL_LINK_LOSS_EARLY_WARNING: i32 = 0x43;
STRUCT! {#[repr(packed)] struct evt_physical_link_loss_warning {
    handle: u8,
    reason: u8,
}}
pub const EVT_PHYSICAL_LINK_LOSS_WARNING_SIZE: usize = 2;
pub const EVT_PHYSICAL_LINK_RECOVERY: i32 = 0x44;
STRUCT! {#[repr(packed)] struct evt_physical_link_recovery {
    handle: u8,
}}
pub const EVT_PHYSICAL_LINK_RECOVERY_SIZE: usize = 1;
pub const EVT_LOGICAL_LINK_COMPLETE: i32 = 0x45;
STRUCT! {#[repr(packed)] struct evt_logical_link_complete {
    status: u8,
    log_handle: u16,
    handle: u8,
    tx_flow_id: u8,
}}
pub const EVT_LOGICAL_LINK_COMPLETE_SIZE: usize = 5;
pub const EVT_DISCONNECT_LOGICAL_LINK_COMPLETE: i32 = 0x46;
pub const EVT_FLOW_SPEC_MODIFY_COMPLETE: i32 = 0x47;
STRUCT! {#[repr(packed)] struct evt_flow_spec_modify_complete {
    status: u8,
    handle: u16,
}}
pub const EVT_FLOW_SPEC_MODIFY_COMPLETE_SIZE: usize = 3;
pub const EVT_NUMBER_COMPLETED_BLOCKS: i32 = 0x48;
STRUCT! {#[repr(packed)] struct cmplt_handle {
    handle: u16,
    num_cmplt_pkts: u16,
    num_cmplt_blks: u16,
}}
STRUCT! {#[repr(packed)] struct evt_num_completed_blocks {
    total_num_blocks: u16,
    num_handles: u8,
    handles: [cmplt_handle; 0],
}}
pub const EVT_AMP_STATUS_CHANGE: i32 = 0x4D;
STRUCT! {#[repr(packed)] struct evt_amp_status_change {
    status: u8,
    amp_status: u8,
}}
pub const EVT_AMP_STATUS_CHANGE_SIZE: usize = 2;
pub const EVT_TESTING: i32 = 0xFE;
pub const EVT_VENDOR: i32 = 0xFF;
// Internal events generated by BlueZ stack
pub const EVT_STACK_INTERNAL: i32 = 0xFD;
STRUCT! {#[repr(packed)] struct evt_stack_internal {
    type_: u16,
    data: [u8; 0],
}}
pub const EVT_STACK_INTERNAL_SIZE: usize = 2;
pub const EVT_SI_DEVICE: i32 = 0x01;
STRUCT! {#[repr(packed)] struct evt_si_device {
    event: u16,
    dev_id: u16,
}}
pub const EVT_SI_DEVICE_SIZE: usize = 4;
// --------  HCI Packet structures  --------
pub const HCI_TYPE_LEN: i32 = 1;
STRUCT! {#[repr(packed)] struct hci_command_hdr {
    opcode: u16,   // OCF & OGF
    plen: u8,
}}
pub const HCI_COMMAND_HDR_SIZE: usize = 3;
STRUCT! {#[repr(packed)] struct hci_event_hdr {
    evt: u8,
    plen: u8,
}}
pub const HCI_EVENT_HDR_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct hci_acl_hdr {
    handle: u16,   // Handle & Flags(PB, BC)
    dlen: u16,
}}
pub const HCI_ACL_HDR_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct hci_sco_hdr {
    handle: u16,
    dlen: u8,
}}
pub const HCI_SCO_HDR_SIZE: usize = 3;
STRUCT! {#[repr(packed)] struct hci_msg_hdr {
    device: u16,
    type_: u16,
    plen: u16,
}}
pub const HCI_MSG_HDR_SIZE: usize = 6;
// Command opcode pack/unpack
#[inline]
pub fn cmd_opcode_pack(ogf: u16, ocf: u16) -> u16 {
    ((ocf & 0x03ff) | (ogf << 10)) as u16
}
#[inline]
pub fn cmd_opcode_ogf(op: u16) -> u16 {
    op >> 10
}
#[inline]
pub fn cmd_opcode_ocf(op: u16) -> u16 {
    op & 0x03ff
}
// ACL handle and flags pack/unpack
#[inline]
pub fn acl_handle_pack(h: u16, f: u16) -> u16 {
    ((h & 0x0fff) | (f << 12)) as u16
}
#[inline]
pub fn acl_handle(h: u16) -> u16 {
    h & 0x0fff
}
#[inline]
pub fn acl_flags(h: u16) -> u16 {
    h >> 12
}
// HCI Socket options
pub const HCI_DATA_DIR: i32 = 1;
pub const HCI_FILTER: i32 = 2;
pub const HCI_TIME_STAMP: i32 = 3;
// HCI CMSG flags
pub const HCI_CMSG_DIR: i32 = 0x0001;
pub const HCI_CMSG_TSTAMP: i32 = 0x0002;
STRUCT! {struct sockaddr_hci {
    hci_family: sa_family_t,
    hci_dev: c_ushort,
    hci_channel: c_ushort,
}}
pub const HCI_DEV_NONE: i32 = 0xffff;
pub const HCI_CHANNEL_RAW: i32 = 0;
pub const HCI_CHANNEL_USER: i32 = 1;
pub const HCI_CHANNEL_MONITOR: i32 = 2;
pub const HCI_CHANNEL_CONTROL: i32 = 3;
pub const HCI_CHANNEL_LOGGING: i32 = 4;
STRUCT! {struct hci_filter {
    type_mask: u32,
    event_mask: [u32; 2],
    opcode: u16,
}}
pub const HCI_FLT_TYPE_BITS: i32 = 31;
pub const HCI_FLT_EVENT_BITS: i32 = 63;
pub const HCI_FLT_OGF_BITS: i32 = 63;
pub const HCI_FLT_OCF_BITS: i32 = 127;
// Ioctl requests structures
STRUCT! {struct hci_dev_stats {
    err_rx: u32,
    err_tx: u32,
    cmd_tx: u32,
    evt_rx: u32,
    acl_tx: u32,
    acl_rx: u32,
    sco_tx: u32,
    sco_rx: u32,
    byte_rx: u32,
    byte_tx: u32,
}}
STRUCT! {struct hci_dev_info {
    dev_id: u16,
    name: [i8; 8],
    bdaddr: bdaddr_t,
    flags: u32,
    type_: u8,
    features: [u8; 8],
    pkt_type_: u32,
    link_policy: u32,
    link_mode: u32,
    acl_mtu: u16,
    acl_pkts: u16,
    sco_mtu: u16,
    sco_pkts: u16,
    stat: hci_dev_stats,
}}
STRUCT! {struct hci_conn_info {
    handle: u16,
    bdaddr: bdaddr_t,
    type_: u8,
    out: u8,
    state: u16,
    link_mode: u32,
}}
STRUCT! {struct hci_dev_req {
    dev_id: u16,
    dev_opt: u32,
}}
STRUCT! {struct hci_dev_list_req {
    dev_num: u16,
    dev_req: [hci_dev_req; 0],  // hci_dev_req structures
}}
STRUCT! {struct hci_conn_list_req {
    dev_id: u16,
    conn_num: u16,
    conn_info: [hci_conn_info; 0],
}}
STRUCT! {struct hci_conn_info_req {
    bdaddr: bdaddr_t,
    type_: u8,
    conn_info: [hci_conn_info; 0],
}}
STRUCT! {struct hci_auth_info_req {
    bdaddr: bdaddr_t,
    type_: u8,
}}
STRUCT! {struct hci_inquiry_req {
    dev_id: u16,
    flags: u16,
    lap: [u8; 3],
    length: u8,
    num_rsp: u8,
}}
pub const IREQ_CACHE_FLUSH: i64 = 0x0001;
