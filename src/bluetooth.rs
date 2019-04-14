pub const AF_BLUETOOTH: u16 = 31;
pub const PF_BLUETOOTH: u16 = AF_BLUETOOTH;
pub const BTPROTO_L2CAP: i32 = 0;
pub const BTPROTO_HCI: i32 = 1;
pub const BTPROTO_SCO: i32 = 2;
pub const BTPROTO_RFCOMM: i32 = 3;
pub const BTPROTO_BNEP: i32 = 4;
pub const BTPROTO_CMTP: i32 = 5;
pub const BTPROTO_HIDP: i32 = 6;
pub const BTPROTO_AVDTP: i32 = 7;
pub const SOL_HCI: i32 = 0;
pub const SOL_L2CAP: i32 = 6;
pub const SOL_SCO: i32 = 17;
pub const SOL_RFCOMM: i32 = 18;
pub const SOL_BLUETOOTH: i32 = 274;
pub const BT_SECURITY: i32 = 4;
STRUCT! {struct bt_security {
    level: u8,
    key_size: u8,
}}
pub const BT_SECURITY_SDP: i32 = 0;
pub const BT_SECURITY_LOW: i32 = 1;
pub const BT_SECURITY_MEDIUM: i32 = 2;
pub const BT_SECURITY_HIGH: i32 = 3;
pub const BT_SECURITY_FIPS: i32 = 4;
pub const BT_DEFER_SETUP: i32 = 7;
pub const BT_FLUSHABLE: i32 = 8;
pub const BT_FLUSHABLE_OFF: i32 = 0;
pub const BT_FLUSHABLE_ON: i32 = 1;
pub const BT_POWER: i32 = 9;
pub const BT_POWER_FORCE_ACTIVE_OFF: i32 = 0;
pub const BT_POWER_FORCE_ACTIVE_ON: i32 = 1;
pub const BT_CHANNEL_POLICY: i32 = 10;
// BR/EDR only (default policy)
//  AMP controllers cannot be used.
//  Channel move requests from the remote device are denied.
//  If the L2CAP channel is currently using AMP, move the channel to BR/EDR.
pub const BT_CHANNEL_POLICY_BREDR_ONLY: i32 = 0;
// BR/EDR Preferred
//  Allow use of AMP controllers.
//  If the L2CAP channel is currently on AMP, move it to BR/EDR.
//  Channel move requests from the remote device are allowed.
pub const BT_CHANNEL_POLICY_BREDR_PREFERRED: i32 = 1;
// AMP Preferred
//  Allow use of AMP controllers
//  If the L2CAP channel is currently on BR/EDR and AMP controller
//    resources are available, initiate a channel move to AMP.
//  Channel move requests from the remote device are allowed.
//  If the L2CAP socket has not been connected yet, try to create
//    and configure the channel directly on an AMP controller rather
//    than BR/EDR.
pub const BT_CHANNEL_POLICY_AMP_PREFERRED: i32 = 2;
pub const BT_VOICE: i32 = 11;
STRUCT! {struct bt_voice {
    setting: u16,
}}
pub const BT_SNDMTU: i32 = 12;
pub const BT_RCVMTU: i32 = 13;
pub const BT_VOICE_TRANSPARENT: i32 = 0x0003;
pub const BT_VOICE_CVSD_16BIT: i32 = 0x0060;
ENUM! {enum bt_state {
    BT_CONNECTED = 1,
    BT_OPEN,
    BT_BOUND,
    BT_LISTEN,
    BT_CONNECT,
    BT_CONNECT2,
    BT_CONFIG,
    BT_DISCONN,
    BT_CLOSED,
}}
STRUCT! {#[repr(packed)] struct bdaddr_t {
    b: [u8; 6],
}}
pub const BDADDR_BREDR: i32 = 0x00;
pub const BDADDR_LE_PUBLIC: i32 = 0x01;
pub const BDADDR_LE_RANDOM: i32 = 0x02;
pub const BDADDR_ANY: bdaddr_t = bdaddr_t { b: [0; 6] };
pub const BDADDR_ALL: bdaddr_t = bdaddr_t { b: [0xff; 6] };
pub const BDADDR_LOCAL: bdaddr_t = bdaddr_t {
    b: [0, 0, 0, 0xff, 0xff, 0xff],
};
