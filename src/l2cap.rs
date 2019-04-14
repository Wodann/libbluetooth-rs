use libc::{c_ushort, sa_family_t};
use crate::bluetooth::bdaddr_t;
// L2CAP defaults
pub const L2CAP_DEFAULT_MTU: i32 = 672;
pub const L2CAP_DEFAULT_FLUSH_TO: i32 = 0xFFFF;
// L2CAP socket address
STRUCT! {struct sockaddr_l2 {
	l2_family: sa_family_t,
	l2_psm: c_ushort,
	l2_bdaddr: bdaddr_t,
	l2_cid: c_ushort,
	l2_bdaddr_type: u8,
}}
// L2CAP socket options
pub const L2CAP_OPTIONS: i32 = 0x01;
STRUCT! {struct l2cap_options {
	omtu: u16,
	imtu: u16,
	flush_to: u16,
	mode: u8,
	fcs: u8,
	max_tx: u8,
	txwin_size: u16,
}}
pub const L2CAP_CONNINFO: i32 = 0x02;
STRUCT! {struct l2cap_conninfo {
	hci_handle: u16,
	dev_class: [u8; 3],
}}
pub const L2CAP_LM: i32 = 0x03;
pub const L2CAP_LM_MASTER: i32 = 0x0001;
pub const L2CAP_LM_AUTH: i32 = 0x0002;
pub const L2CAP_LM_ENCRYPT: i32 = 0x0004;
pub const L2CAP_LM_TRUSTED: i32 = 0x0008;
pub const L2CAP_LM_RELIABLE: i32 = 0x0010;
pub const L2CAP_LM_SECURE: i32 = 0x0020;
// L2CAP command codes
pub const L2CAP_COMMAND_REJ: i32 = 0x01;
pub const L2CAP_CONN_REQ: i32 = 0x02;
pub const L2CAP_CONN_RSP: i32 = 0x03;
pub const L2CAP_CONF_REQ: i32 = 0x04;
pub const L2CAP_CONF_RSP: i32 = 0x05;
pub const L2CAP_DISCONN_REQ: i32 = 0x06;
pub const L2CAP_DISCONN_RSP: i32 = 0x07;
pub const L2CAP_ECHO_REQ: i32 = 0x08;
pub const L2CAP_ECHO_RSP: i32 = 0x09;
pub const L2CAP_INFO_REQ: i32 = 0x0a;
pub const L2CAP_INFO_RSP: i32 = 0x0b;
pub const L2CAP_CREATE_REQ: i32 = 0x0c;
pub const L2CAP_CREATE_RSP: i32 = 0x0d;
pub const L2CAP_MOVE_REQ: i32 = 0x0e;
pub const L2CAP_MOVE_RSP: i32 = 0x0f;
pub const L2CAP_MOVE_CFM: i32 = 0x10;
pub const L2CAP_MOVE_CFM_RSP: i32 = 0x11;
// L2CAP extended feature mask
pub const L2CAP_FEAT_FLOWCTL: i32 = 0x00000001;
pub const L2CAP_FEAT_RETRANS: i32 = 0x00000002;
pub const L2CAP_FEAT_BIDIR_QOS: i32 = 0x00000004;
pub const L2CAP_FEAT_ERTM: i32 = 0x00000008;
pub const L2CAP_FEAT_STREAMING: i32 = 0x00000010;
pub const L2CAP_FEAT_FCS: i32 = 0x00000020;
pub const L2CAP_FEAT_EXT_FLOW: i32 = 0x00000040;
pub const L2CAP_FEAT_FIXED_CHAN: i32 = 0x00000080;
pub const L2CAP_FEAT_EXT_WINDOW: i32 = 0x00000100;
pub const L2CAP_FEAT_UCD: i32 = 0x00000200;
// L2CAP fixed channels
pub const L2CAP_FC_L2CAP: i32 = 0x02;
pub const L2CAP_FC_CONNLESS: i32 = 0x04;
pub const L2CAP_FC_A2MP: i32 = 0x08;
/* L2CAP STRUCT! {structures */
STRUCT! {#[repr(packed)] struct l2cap_hdr {
	len: u16,
	cid: u16,
}}
pub const L2CAP_HDR_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_cmd_hdr {
	code: u8,
	ident: u8,
	len: u16,
}}
pub const L2CAP_CMD_HDR_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_cmd_rej {
	reason: u16,
}}
pub const L2CAP_CMD_REJ_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct l2cap_conn_req {
	psm: u16,
	scid: u16,
}}
pub const L2CAP_CONN_REQ_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_conn_rsp {
	dcid: u16,
	scid: u16,
	result: u16,
	status: u16,
}}
pub const L2CAP_CONN_RSP_SIZE: usize = 8;
// connect result
pub const L2CAP_CR_SUCCESS: i32 = 0x0000;
pub const L2CAP_CR_PEND: i32 = 0x0001;
pub const L2CAP_CR_BAD_PSM: i32 = 0x0002;
pub const L2CAP_CR_SEC_BLOCK: i32 = 0x0003;
pub const L2CAP_CR_NO_MEM: i32 = 0x0004;
// connect status
pub const L2CAP_CS_NO_INFO: i32 = 0x0000;
pub const L2CAP_CS_AUTHEN_PEND: i32 = 0x0001;
pub const L2CAP_CS_AUTHOR_PEND: i32 = 0x0002;
STRUCT! {#[repr(packed)] struct l2cap_conf_req {
	dcid: u16,
	flags: u16,
	data: [u8; 0],
}}
pub const L2CAP_CONF_REQ_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_conf_rsp {
	scid: u16,
	flags: u16,
	result: u16,
	data: [u8; 0],
}}
pub const L2CAP_CONF_RSP_SIZE: usize = 6;
pub const L2CAP_CONF_SUCCESS: i32 = 0x0000;
pub const L2CAP_CONF_UNACCEPT: i32 = 0x0001;
pub const L2CAP_CONF_REJECT: i32 = 0x0002;
pub const L2CAP_CONF_UNKNOWN: i32 = 0x0003;
pub const L2CAP_CONF_PENDING: i32 = 0x0004;
pub const L2CAP_CONF_EFS_REJECT: i32 = 0x0005;
STRUCT! {#[repr(packed)] struct l2cap_conf_opt {
	type_: u8,
	len: u8,
	val: [u8; 0],
}}
pub const L2CAP_CONF_OPT_SIZE: usize = 2;
pub const L2CAP_CONF_MTU: i32 = 0x01;
pub const L2CAP_CONF_FLUSH_TO: i32 = 0x02;
pub const L2CAP_CONF_QOS: i32 = 0x03;
pub const L2CAP_CONF_RFC: i32 = 0x04;
pub const L2CAP_CONF_FCS: i32 = 0x05;
pub const L2CAP_CONF_EFS: i32 = 0x06;
pub const L2CAP_CONF_EWS: i32 = 0x07;
pub const L2CAP_CONF_MAX_SIZE: usize = 22;
pub const L2CAP_MODE_BASIC: i32 = 0x00;
pub const L2CAP_MODE_RETRANS: i32 = 0x01;
pub const L2CAP_MODE_FLOWCTL: i32 = 0x02;
pub const L2CAP_MODE_ERTM: i32 = 0x03;
pub const L2CAP_MODE_STREAMING: i32 = 0x04;
pub const L2CAP_SERVTYPE_NOTRAFFIC: i32 = 0x00;
pub const L2CAP_SERVTYPE_BESTEFFORT: i32 = 0x01;
pub const L2CAP_SERVTYPE_GUARANTEED: i32 = 0x02;
STRUCT! {#[repr(packed)] struct l2cap_disconn_req {
	dcid: u16,
	scid: u16,
}}
pub const L2CAP_DISCONN_REQ_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_disconn_rsp {
	dcid: u16,
	scid: u16,
}}
pub const L2CAP_DISCONN_RSP_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_info_req {
	type_: u16,
}}
pub const L2CAP_INFO_REQ_SIZE: usize = 2;
STRUCT! {#[repr(packed)] struct l2cap_info_rsp {
	type_: u16,
	result: u16,
	data: [u8; 0],
}}
pub const L2CAP_INFO_RSP_SIZE: usize = 4;
// info type
pub const L2CAP_IT_CL_MTU: i32 = 0x0001;
pub const L2CAP_IT_FEAT_MASK: i32 = 0x0002;
// info result
pub const L2CAP_IR_SUCCESS: i32 = 0x0000;
pub const L2CAP_IR_NOTSUPP: i32 = 0x0001;
STRUCT! {#[repr(packed)] struct l2cap_create_req {
	psm: u16,
	scid: u16,
	id: u8,
}}
pub const L2CAP_CREATE_REQ_SIZE: usize = 5;
STRUCT! {#[repr(packed)] struct l2cap_create_rsp {
	dcid: u16,
	scid: u16,
	result: u16,
	status: u16,
}}
pub const L2CAP_CREATE_RSP_SIZE: usize = 8;
STRUCT! {#[repr(packed)] struct l2cap_move_req {
	icid: u16,
	id: u8,
}}
pub const L2CAP_MOVE_REQ_SIZE: usize = 3;
STRUCT! {#[repr(packed)] struct l2cap_move_rsp {
	icid: u16,
	result: u16,
}}
pub const L2CAP_MOVE_RSP_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_move_cfm {
	icid: u16,
	result: u16,
}}
pub const L2CAP_MOVE_CFM_SIZE: usize = 4;
STRUCT! {#[repr(packed)] struct l2cap_move_cfm_rsp {
	icid: u16,
}}
pub const L2CAP_MOVE_CFM_RSP_SIZE: usize = 2;
