use libc::{c_char, c_int, c_void};
pub const SDP_UNIX_PATH: &str = "/var/run/sdp";
pub const SDP_RESPONSE_TIMEOUT: i32 = 20;
pub const SDP_REQ_BUFFER_SIZE: i32 = 2048;
pub const SDP_RSP_BUFFER_SIZE: i32 = 65535;
pub const SDP_PDU_CHUNK_SIZE: i32 = 1024;
/*
 * All definitions are based on Bluetooth Assigned Numbers
 * of the Bluetooth Specification
 */
pub const SDP_PSM: i32 = 0x0001;
/*
 * Protocol UUIDs
 */
pub const SDP_UUID: i32 = 0x0001;
pub const UDP_UUID: i32 = 0x0002;
pub const RFCOMM_UUID: i32 = 0x0003;
pub const TCP_UUID: i32 = 0x0004;
pub const TCS_BIN_UUID: i32 = 0x0005;
pub const TCS_AT_UUID: i32 = 0x0006;
pub const ATT_UUID: i32 = 0x0007;
pub const OBEX_UUID: i32 = 0x0008;
pub const IP_UUID: i32 = 0x0009;
pub const FTP_UUID: i32 = 0x000a;
pub const HTTP_UUID: i32 = 0x000c;
pub const WSP_UUID: i32 = 0x000e;
pub const BNEP_UUID: i32 = 0x000f;
pub const UPNP_UUID: i32 = 0x0010;
pub const HIDP_UUID: i32 = 0x0011;
pub const HCRP_CTRL_UUID: i32 = 0x0012;
pub const HCRP_DATA_UUID: i32 = 0x0014;
pub const HCRP_NOTE_UUID: i32 = 0x0016;
pub const AVCTP_UUID: i32 = 0x0017;
pub const AVDTP_UUID: i32 = 0x0019;
pub const CMTP_UUID: i32 = 0x001b;
pub const UDI_UUID: i32 = 0x001d;
pub const MCAP_CTRL_UUID: i32 = 0x001e;
pub const MCAP_DATA_UUID: i32 = 0x001f;
pub const L2CAP_UUID: i32 = 0x0100;
/*
 * Service class identifiers of standard services and service groups
 */
pub const SDP_SERVER_SVCLASS_ID: i32 = 0x1000;
pub const BROWSE_GRP_DESC_SVCLASS_ID: i32 = 0x1001;
pub const PUBLIC_BROWSE_GROUP: i32 = 0x1002;
pub const SERIAL_PORT_SVCLASS_ID: i32 = 0x1101;
pub const LAN_ACCESS_SVCLASS_ID: i32 = 0x1102;
pub const DIALUP_NET_SVCLASS_ID: i32 = 0x1103;
pub const IRMC_SYNC_SVCLASS_ID: i32 = 0x1104;
pub const OBEX_OBJPUSH_SVCLASS_ID: i32 = 0x1105;
pub const OBEX_FILETRANS_SVCLASS_ID: i32 = 0x1106;
pub const IRMC_SYNC_CMD_SVCLASS_ID: i32 = 0x1107;
pub const HEADSET_SVCLASS_ID: i32 = 0x1108;
pub const CORDLESS_TELEPHONY_SVCLASS_ID: i32 = 0x1109;
pub const AUDIO_SOURCE_SVCLASS_ID: i32 = 0x110a;
pub const AUDIO_SINK_SVCLASS_ID: i32 = 0x110b;
pub const AV_REMOTE_TARGET_SVCLASS_ID: i32 = 0x110c;
pub const ADVANCED_AUDIO_SVCLASS_ID: i32 = 0x110d;
pub const AV_REMOTE_SVCLASS_ID: i32 = 0x110e;
pub const AV_REMOTE_CONTROLLER_SVCLASS_ID: i32 = 0x110f;
pub const INTERCOM_SVCLASS_ID: i32 = 0x1110;
pub const FAX_SVCLASS_ID: i32 = 0x1111;
pub const HEADSET_AGW_SVCLASS_ID: i32 = 0x1112;
pub const WAP_SVCLASS_ID: i32 = 0x1113;
pub const WAP_CLIENT_SVCLASS_ID: i32 = 0x1114;
pub const PANU_SVCLASS_ID: i32 = 0x1115;
pub const NAP_SVCLASS_ID: i32 = 0x1116;
pub const GN_SVCLASS_ID: i32 = 0x1117;
pub const DIRECT_PRINTING_SVCLASS_ID: i32 = 0x1118;
pub const REFERENCE_PRINTING_SVCLASS_ID: i32 = 0x1119;
pub const IMAGING_SVCLASS_ID: i32 = 0x111a;
pub const IMAGING_RESPONDER_SVCLASS_ID: i32 = 0x111b;
pub const IMAGING_ARCHIVE_SVCLASS_ID: i32 = 0x111c;
pub const IMAGING_REFOBJS_SVCLASS_ID: i32 = 0x111d;
pub const HANDSFREE_SVCLASS_ID: i32 = 0x111e;
pub const HANDSFREE_AGW_SVCLASS_ID: i32 = 0x111f;
pub const DIRECT_PRT_REFOBJS_SVCLASS_ID: i32 = 0x1120;
pub const REFLECTED_UI_SVCLASS_ID: i32 = 0x1121;
pub const BASIC_PRINTING_SVCLASS_ID: i32 = 0x1122;
pub const PRINTING_STATUS_SVCLASS_ID: i32 = 0x1123;
pub const HID_SVCLASS_ID: i32 = 0x1124;
pub const HCR_SVCLASS_ID: i32 = 0x1125;
pub const HCR_PRINT_SVCLASS_ID: i32 = 0x1126;
pub const HCR_SCAN_SVCLASS_ID: i32 = 0x1127;
pub const CIP_SVCLASS_ID: i32 = 0x1128;
pub const VIDEO_CONF_GW_SVCLASS_ID: i32 = 0x1129;
pub const UDI_MT_SVCLASS_ID: i32 = 0x112a;
pub const UDI_TA_SVCLASS_ID: i32 = 0x112b;
pub const AV_SVCLASS_ID: i32 = 0x112c;
pub const SAP_SVCLASS_ID: i32 = 0x112d;
pub const PBAP_PCE_SVCLASS_ID: i32 = 0x112e;
pub const PBAP_PSE_SVCLASS_ID: i32 = 0x112f;
pub const PBAP_SVCLASS_ID: i32 = 0x1130;
pub const MAP_MSE_SVCLASS_ID: i32 = 0x1132;
pub const MAP_MCE_SVCLASS_ID: i32 = 0x1133;
pub const MAP_SVCLASS_ID: i32 = 0x1134;
pub const GNSS_SVCLASS_ID: i32 = 0x1135;
pub const GNSS_SERVER_SVCLASS_ID: i32 = 0x1136;
pub const MPS_SC_SVCLASS_ID: i32 = 0x113A;
pub const MPS_SVCLASS_ID: i32 = 0x113B;
pub const PNP_INFO_SVCLASS_ID: i32 = 0x1200;
pub const GENERIC_NETWORKING_SVCLASS_ID: i32 = 0x1201;
pub const GENERIC_FILETRANS_SVCLASS_ID: i32 = 0x1202;
pub const GENERIC_AUDIO_SVCLASS_ID: i32 = 0x1203;
pub const GENERIC_TELEPHONY_SVCLASS_ID: i32 = 0x1204;
pub const UPNP_SVCLASS_ID: i32 = 0x1205;
pub const UPNP_IP_SVCLASS_ID: i32 = 0x1206;
pub const UPNP_PAN_SVCLASS_ID: i32 = 0x1300;
pub const UPNP_LAP_SVCLASS_ID: i32 = 0x1301;
pub const UPNP_L2CAP_SVCLASS_ID: i32 = 0x1302;
pub const VIDEO_SOURCE_SVCLASS_ID: i32 = 0x1303;
pub const VIDEO_SINK_SVCLASS_ID: i32 = 0x1304;
pub const VIDEO_DISTRIBUTION_SVCLASS_ID: i32 = 0x1305;
pub const HDP_SVCLASS_ID: i32 = 0x1400;
pub const HDP_SOURCE_SVCLASS_ID: i32 = 0x1401;
pub const HDP_SINK_SVCLASS_ID: i32 = 0x1402;
pub const GENERIC_ACCESS_SVCLASS_ID: i32 = 0x1800;
pub const GENERIC_ATTRIB_SVCLASS_ID: i32 = 0x1801;
pub const APPLE_AGENT_SVCLASS_ID: i32 = 0x2112;
/*
 * Standard profile identifiers: descriptor, note these
 * may be identical to some of the service classes defined above
 */
pub const SDP_SERVER_PROFILE_ID: i32 = SDP_SERVER_SVCLASS_ID;
pub const BROWSE_GRP_DESC_PROFILE_ID: i32 = BROWSE_GRP_DESC_SVCLASS_ID;
pub const SERIAL_PORT_PROFILE_ID: i32 = SERIAL_PORT_SVCLASS_ID;
pub const LAN_ACCESS_PROFILE_ID: i32 = LAN_ACCESS_SVCLASS_ID;
pub const DIALUP_NET_PROFILE_ID: i32 = DIALUP_NET_SVCLASS_ID;
pub const IRMC_SYNC_PROFILE_ID: i32 = IRMC_SYNC_SVCLASS_ID;
pub const OBEX_OBJPUSH_PROFILE_ID: i32 = OBEX_OBJPUSH_SVCLASS_ID;
pub const OBEX_FILETRANS_PROFILE_ID: i32 = OBEX_FILETRANS_SVCLASS_ID;
pub const IRMC_SYNC_CMD_PROFILE_ID: i32 = IRMC_SYNC_CMD_SVCLASS_ID;
pub const HEADSET_PROFILE_ID: i32 = HEADSET_SVCLASS_ID;
pub const CORDLESS_TELEPHONY_PROFILE_ID: i32 = CORDLESS_TELEPHONY_SVCLASS_ID;
pub const AUDIO_SOURCE_PROFILE_ID: i32 = AUDIO_SOURCE_SVCLASS_ID;
pub const AUDIO_SINK_PROFILE_ID: i32 = AUDIO_SINK_SVCLASS_ID;
pub const AV_REMOTE_TARGET_PROFILE_ID: i32 = AV_REMOTE_TARGET_SVCLASS_ID;
pub const ADVANCED_AUDIO_PROFILE_ID: i32 = ADVANCED_AUDIO_SVCLASS_ID;
pub const AV_REMOTE_PROFILE_ID: i32 = AV_REMOTE_SVCLASS_ID;
pub const INTERCOM_PROFILE_ID: i32 = INTERCOM_SVCLASS_ID;
pub const FAX_PROFILE_ID: i32 = FAX_SVCLASS_ID;
pub const HEADSET_AGW_PROFILE_ID: i32 = HEADSET_AGW_SVCLASS_ID;
pub const WAP_PROFILE_ID: i32 = WAP_SVCLASS_ID;
pub const WAP_CLIENT_PROFILE_ID: i32 = WAP_CLIENT_SVCLASS_ID;
pub const PANU_PROFILE_ID: i32 = PANU_SVCLASS_ID;
pub const NAP_PROFILE_ID: i32 = NAP_SVCLASS_ID;
pub const GN_PROFILE_ID: i32 = GN_SVCLASS_ID;
pub const DIRECT_PRINTING_PROFILE_ID: i32 = DIRECT_PRINTING_SVCLASS_ID;
pub const REFERENCE_PRINTING_PROFILE_ID: i32 = REFERENCE_PRINTING_SVCLASS_ID;
pub const IMAGING_PROFILE_ID: i32 = IMAGING_SVCLASS_ID;
pub const IMAGING_RESPONDER_PROFILE_ID: i32 = IMAGING_RESPONDER_SVCLASS_ID;
pub const IMAGING_ARCHIVE_PROFILE_ID: i32 = IMAGING_ARCHIVE_SVCLASS_ID;
pub const IMAGING_REFOBJS_PROFILE_ID: i32 = IMAGING_REFOBJS_SVCLASS_ID;
pub const HANDSFREE_PROFILE_ID: i32 = HANDSFREE_SVCLASS_ID;
pub const HANDSFREE_AGW_PROFILE_ID: i32 = HANDSFREE_AGW_SVCLASS_ID;
pub const DIRECT_PRT_REFOBJS_PROFILE_ID: i32 = DIRECT_PRT_REFOBJS_SVCLASS_ID;
pub const REFLECTED_UI_PROFILE_ID: i32 = REFLECTED_UI_SVCLASS_ID;
pub const BASIC_PRINTING_PROFILE_ID: i32 = BASIC_PRINTING_SVCLASS_ID;
pub const PRINTING_STATUS_PROFILE_ID: i32 = PRINTING_STATUS_SVCLASS_ID;
pub const HID_PROFILE_ID: i32 = HID_SVCLASS_ID;
pub const HCR_PROFILE_ID: i32 = HCR_SCAN_SVCLASS_ID;
pub const HCR_PRINT_PROFILE_ID: i32 = HCR_PRINT_SVCLASS_ID;
pub const HCR_SCAN_PROFILE_ID: i32 = HCR_SCAN_SVCLASS_ID;
pub const CIP_PROFILE_ID: i32 = CIP_SVCLASS_ID;
pub const VIDEO_CONF_GW_PROFILE_ID: i32 = VIDEO_CONF_GW_SVCLASS_ID;
pub const UDI_MT_PROFILE_ID: i32 = UDI_MT_SVCLASS_ID;
pub const UDI_TA_PROFILE_ID: i32 = UDI_TA_SVCLASS_ID;
pub const AV_PROFILE_ID: i32 = AV_SVCLASS_ID;
pub const SAP_PROFILE_ID: i32 = SAP_SVCLASS_ID;
pub const PBAP_PCE_PROFILE_ID: i32 = PBAP_PCE_SVCLASS_ID;
pub const PBAP_PSE_PROFILE_ID: i32 = PBAP_PSE_SVCLASS_ID;
pub const PBAP_PROFILE_ID: i32 = PBAP_SVCLASS_ID;
pub const MAP_PROFILE_ID: i32 = MAP_SVCLASS_ID;
pub const PNP_INFO_PROFILE_ID: i32 = PNP_INFO_SVCLASS_ID;
pub const GENERIC_NETWORKING_PROFILE_ID: i32 = GENERIC_NETWORKING_SVCLASS_ID;
pub const GENERIC_FILETRANS_PROFILE_ID: i32 = GENERIC_FILETRANS_SVCLASS_ID;
pub const GENERIC_AUDIO_PROFILE_ID: i32 = GENERIC_AUDIO_SVCLASS_ID;
pub const GENERIC_TELEPHONY_PROFILE_ID: i32 = GENERIC_TELEPHONY_SVCLASS_ID;
pub const UPNP_PROFILE_ID: i32 = UPNP_SVCLASS_ID;
pub const UPNP_IP_PROFILE_ID: i32 = UPNP_IP_SVCLASS_ID;
pub const UPNP_PAN_PROFILE_ID: i32 = UPNP_PAN_SVCLASS_ID;
pub const UPNP_LAP_PROFILE_ID: i32 = UPNP_LAP_SVCLASS_ID;
pub const UPNP_L2CAP_PROFILE_ID: i32 = UPNP_L2CAP_SVCLASS_ID;
pub const VIDEO_SOURCE_PROFILE_ID: i32 = VIDEO_SOURCE_SVCLASS_ID;
pub const VIDEO_SINK_PROFILE_ID: i32 = VIDEO_SINK_SVCLASS_ID;
pub const VIDEO_DISTRIBUTION_PROFILE_ID: i32 = VIDEO_DISTRIBUTION_SVCLASS_ID;
pub const HDP_PROFILE_ID: i32 = HDP_SVCLASS_ID;
pub const HDP_SOURCE_PROFILE_ID: i32 = HDP_SOURCE_SVCLASS_ID;
pub const HDP_SINK_PROFILE_ID: i32 = HDP_SINK_SVCLASS_ID;
pub const GENERIC_ACCESS_PROFILE_ID: i32 = GENERIC_ACCESS_SVCLASS_ID;
pub const GENERIC_ATTRIB_PROFILE_ID: i32 = GENERIC_ATTRIB_SVCLASS_ID;
pub const APPLE_AGENT_PROFILE_ID: i32 = APPLE_AGENT_SVCLASS_ID;
pub const MPS_PROFILE_ID: i32 = MPS_SC_SVCLASS_ID;
/*
 * Compatibility macros for the old MDP acronym
 */
pub const MDP_SVCLASS_ID: i32 = HDP_SVCLASS_ID;
pub const MDP_SOURCE_SVCLASS_ID: i32 = HDP_SOURCE_SVCLASS_ID;
pub const MDP_SINK_SVCLASS_ID: i32 = HDP_SINK_SVCLASS_ID;
pub const MDP_PROFILE_ID: i32 = HDP_PROFILE_ID;
pub const MDP_SOURCE_PROFILE_ID: i32 = HDP_SOURCE_PROFILE_ID;
pub const MDP_SINK_PROFILE_ID: i32 = HDP_SINK_PROFILE_ID;
/*
 * Attribute identifier codes
 */
pub const SDP_SERVER_RECORD_HANDLE: i32 = 0x0000;
/*
 * Possible values for attribute-id are listed below.
 * See SDP Spec, section "Service Attribute Definitions" for more details.
 */
pub const SDP_ATTR_RECORD_HANDLE: u16 = 0x0000;
pub const SDP_ATTR_SVCLASS_ID_LIST: u16 = 0x0001;
pub const SDP_ATTR_RECORD_STATE: u16 = 0x0002;
pub const SDP_ATTR_SERVICE_ID: u16 = 0x0003;
pub const SDP_ATTR_PROTO_DESC_LIST: u16 = 0x0004;
pub const SDP_ATTR_BROWSE_GRP_LIST: u16 = 0x0005;
pub const SDP_ATTR_LANG_BASE_ATTR_ID_LIST: u16 = 0x0006;
pub const SDP_ATTR_SVCINFO_TTL: u16 = 0x0007;
pub const SDP_ATTR_SERVICE_AVAILABILITY: u16 = 0x0008;
pub const SDP_ATTR_PFILE_DESC_LIST: u16 = 0x0009;
pub const SDP_ATTR_DOC_URL: u16 = 0x000a;
pub const SDP_ATTR_CLNT_EXEC_URL: u16 = 0x000b;
pub const SDP_ATTR_ICON_URL: u16 = 0x000c;
pub const SDP_ATTR_ADD_PROTO_DESC_LIST: u16 = 0x000d;
pub const SDP_ATTR_GROUP_ID: u16 = 0x0200;
pub const SDP_ATTR_IP_SUBNET: u16 = 0x0200;
pub const SDP_ATTR_VERSION_NUM_LIST: u16 = 0x0200;
pub const SDP_ATTR_SUPPORTED_FEATURES_LIST: u16 = 0x0200;
pub const SDP_ATTR_GOEP_L2CAP_PSM: u16 = 0x0200;
pub const SDP_ATTR_SVCDB_STATE: u16 = 0x0201;
pub const SDP_ATTR_MPSD_SCENARIOS: u16 = 0x0200;
pub const SDP_ATTR_MPMD_SCENARIOS: u16 = 0x0201;
pub const SDP_ATTR_MPS_DEPENDENCIES: u16 = 0x0202;
pub const SDP_ATTR_SERVICE_VERSION: u16 = 0x0300;
pub const SDP_ATTR_EXTERNAL_NETWORK: u16 = 0x0301;
pub const SDP_ATTR_SUPPORTED_DATA_STORES_LIST: u16 = 0x0301;
pub const SDP_ATTR_DATA_EXCHANGE_SPEC: u16 = 0x0301;
pub const SDP_ATTR_NETWORK: u16 = 0x0301;
pub const SDP_ATTR_FAX_CLASS1_SUPPORT: u16 = 0x0302;
pub const SDP_ATTR_REMOTE_AUDIO_VOLUME_CONTROL: u16 = 0x0302;
pub const SDP_ATTR_MCAP_SUPPORTED_PROCEDURES: u16 = 0x0302;
pub const SDP_ATTR_FAX_CLASS20_SUPPORT: u16 = 0x0303;
pub const SDP_ATTR_SUPPORTED_FORMATS_LIST: u16 = 0x0303;
pub const SDP_ATTR_FAX_CLASS2_SUPPORT: u16 = 0x0304;
pub const SDP_ATTR_AUDIO_FEEDBACK_SUPPORT: u16 = 0x0305;
pub const SDP_ATTR_NETWORK_ADDRESS: u16 = 0x0306;
pub const SDP_ATTR_WAP_GATEWAY: u16 = 0x0307;
pub const SDP_ATTR_HOMEPAGE_URL: u16 = 0x0308;
pub const SDP_ATTR_WAP_STACK_TYPE: u16 = 0x0309;
pub const SDP_ATTR_SECURITY_DESC: u16 = 0x030a;
pub const SDP_ATTR_NET_ACCESS_TYPE: u16 = 0x030b;
pub const SDP_ATTR_MAX_NET_ACCESSRATE: u16 = 0x030c;
pub const SDP_ATTR_IP4_SUBNET: u16 = 0x030d;
pub const SDP_ATTR_IP6_SUBNET: u16 = 0x030e;
pub const SDP_ATTR_SUPPORTED_CAPABILITIES: u16 = 0x0310;
pub const SDP_ATTR_SUPPORTED_FEATURES: u16 = 0x0311;
pub const SDP_ATTR_SUPPORTED_FUNCTIONS: u16 = 0x0312;
pub const SDP_ATTR_TOTAL_IMAGING_DATA_CAPACITY: u16 = 0x0313;
pub const SDP_ATTR_SUPPORTED_REPOSITORIES: u16 = 0x0314;
pub const SDP_ATTR_MAS_INSTANCE_ID: u16 = 0x0315;
pub const SDP_ATTR_SUPPORTED_MESSAGE_TYPES: u16 = 0x0316;
pub const SDP_ATTR_PBAP_SUPPORTED_FEATURES: u16 = 0x0317;
pub const SDP_ATTR_MAP_SUPPORTED_FEATURES: u16 = 0x0317;
pub const SDP_ATTR_SPECIFICATION_ID: u16 = 0x0200;
pub const SDP_ATTR_VENDOR_ID: u16 = 0x0201;
pub const SDP_ATTR_PRODUCT_ID: u16 = 0x0202;
pub const SDP_ATTR_VERSION: u16 = 0x0203;
pub const SDP_ATTR_PRIMARY_RECORD: u16 = 0x0204;
pub const SDP_ATTR_VENDOR_ID_SOURCE: u16 = 0x0205;
pub const SDP_ATTR_HID_DEVICE_RELEASE_NUMBER: u16 = 0x0200;
pub const SDP_ATTR_HID_PARSER_VERSION: u16 = 0x0201;
pub const SDP_ATTR_HID_DEVICE_SUBCLASS: u16 = 0x0202;
pub const SDP_ATTR_HID_COUNTRY_CODE: u16 = 0x0203;
pub const SDP_ATTR_HID_VIRTUAL_CABLE: u16 = 0x0204;
pub const SDP_ATTR_HID_RECONNECT_INITIATE: u16 = 0x0205;
pub const SDP_ATTR_HID_DESCRIPTOR_LIST: u16 = 0x0206;
pub const SDP_ATTR_HID_LANG_ID_BASE_LIST: u16 = 0x0207;
pub const SDP_ATTR_HID_SDP_DISABLE: u16 = 0x0208;
pub const SDP_ATTR_HID_BATTERY_POWER: u16 = 0x0209;
pub const SDP_ATTR_HID_REMOTE_WAKEUP: u16 = 0x020a;
pub const SDP_ATTR_HID_PROFILE_VERSION: u16 = 0x020b;
pub const SDP_ATTR_HID_SUPERVISION_TIMEOUT: u16 = 0x020c;
pub const SDP_ATTR_HID_NORMALLY_CONNECTABLE: u16 = 0x020d;
pub const SDP_ATTR_HID_BOOT_DEVICE: u16 = 0x020e;
/*
 * These identifiers are based on the SDP spec stating that
 * "base attribute id of the primary (universal) language must be 0x0100"
 *
 * Other languages should have their offset: own, e.g.:
 * pub const XXXLangBase: i32 = yyyy;
 * pub const AttrServiceName_XXX: i32 = 0x0000+XXXLangBase;
 */
pub const SDP_PRIMARY_LANG_BASE: u16 = 0x0100;
pub const SDP_ATTR_SVCNAME_PRIMARY: u16 = 0x0000 + SDP_PRIMARY_LANG_BASE;
pub const SDP_ATTR_SVCDESC_PRIMARY: u16 = 0x0001 + SDP_PRIMARY_LANG_BASE;
pub const SDP_ATTR_PROVNAME_PRIMARY: u16 = 0x0002 + SDP_PRIMARY_LANG_BASE;
/*
 * The Data representation in SDP PDUs (pps 339, 340 of BT SDP Spec)
 * These are the exact data type+size descriptor values
 * that go into the PDU buffer.
 *
 * The datatype (leading 5bits) + size descriptor (last 3 bits)
 * is 8 bits. The size descriptor is critical to extract the
 * right number of bytes for the data value from the PDU.
 *
 * For most basic types, the datatype+size descriptor is
 * straightforward. However for constructed types and strings,
 * the size of the data is in the next "n" bytes following the
 * 8 bits (datatype+size) descriptor. Exactly what the "n" is
 * specified in the 3 bits of the data size descriptor.
 *
 * TextString and URLString can be of size 2^{8, 16, 32} bytes
 * DataSequence and DataSequenceAlternates can be of size 2^{8, 16, 32}
 * The size are computed post-facto in the API and are not known apriori
 */
pub const SDP_DATA_NIL: u8 = 0x00;
pub const SDP_UINT8: u8 = 0x08;
pub const SDP_UINT16: u8 = 0x09;
pub const SDP_UINT32: u8 = 0x0A;
pub const SDP_UINT64: u8 = 0x0B;
pub const SDP_UINT128: u8 = 0x0C;
pub const SDP_INT8: u8 = 0x10;
pub const SDP_INT16: u8 = 0x11;
pub const SDP_INT32: u8 = 0x12;
pub const SDP_INT64: u8 = 0x13;
pub const SDP_INT128: u8 = 0x14;
pub const SDP_UUID_UNSPEC: u8 = 0x18;
pub const SDP_UUID16: u8 = 0x19;
pub const SDP_UUID32: u8 = 0x1A;
pub const SDP_UUID128: u8 = 0x1C;
pub const SDP_TEXT_STR_UNSPEC: u8 = 0x20;
pub const SDP_TEXT_STR8: u8 = 0x25;
pub const SDP_TEXT_STR16: u8 = 0x26;
pub const SDP_TEXT_STR32: u8 = 0x27;
pub const SDP_BOOL: u8 = 0x28;
pub const SDP_SEQ_UNSPEC: u8 = 0x30;
pub const SDP_SEQ8: u8 = 0x35;
pub const SDP_SEQ16: u8 = 0x36;
pub const SDP_SEQ32: u8 = 0x37;
pub const SDP_ALT_UNSPEC: u8 = 0x38;
pub const SDP_ALT8: u8 = 0x3D;
pub const SDP_ALT16: u8 = 0x3E;
pub const SDP_ALT32: u8 = 0x3F;
pub const SDP_URL_STR_UNSPEC: u8 = 0x40;
pub const SDP_URL_STR8: u8 = 0x45;
pub const SDP_URL_STR16: u8 = 0x46;
pub const SDP_URL_STR32: u8 = 0x47;
/*
 * The PDU identifiers of SDP packets between client and server
 */
pub const SDP_ERROR_RSP: i32 = 0x01;
pub const SDP_SVC_SEARCH_REQ: i32 = 0x02;
pub const SDP_SVC_SEARCH_RSP: i32 = 0x03;
pub const SDP_SVC_ATTR_REQ: i32 = 0x04;
pub const SDP_SVC_ATTR_RSP: i32 = 0x05;
pub const SDP_SVC_SEARCH_ATTR_REQ: i32 = 0x06;
pub const SDP_SVC_SEARCH_ATTR_RSP: i32 = 0x07;
/*
 * Some additions to support service registration.
 * These are outside the scope of the Bluetooth specification
 */
pub const SDP_SVC_REGISTER_REQ: i32 = 0x75;
pub const SDP_SVC_REGISTER_RSP: i32 = 0x76;
pub const SDP_SVC_UPDATE_REQ: i32 = 0x77;
pub const SDP_SVC_UPDATE_RSP: i32 = 0x78;
pub const SDP_SVC_REMOVE_REQ: i32 = 0x79;
pub const SDP_SVC_REMOVE_RSP: i32 = 0x80;
/*
 * SDP Error codes
 */
pub const SDP_INVALID_VERSION: i32 = 0x0001;
pub const SDP_INVALID_RECORD_HANDLE: i32 = 0x0002;
pub const SDP_INVALID_SYNTAX: i32 = 0x0003;
pub const SDP_INVALID_PDU_SIZE: i32 = 0x0004;
pub const SDP_INVALID_CSTATE: i32 = 0x0005;
/*
 * SDP PDU
 */
STRUCT! {#[repr(packed)] struct sdp_pdu_hdr_t {
    pdu_id: u8,
    tid: u16,
    plen: u16,
}}
/*
 * Common definitions for attributes in the SDP.
 * Should the type of any of these change, you need only make a change here.
 */
UNION! {union uuid_t_value {
    [u32; 4],
    uuid16 uuid16_mut: u16,
    uuid32 uuid32_mut: u32,
    uuid128 uuid128_mut: u128,
}}
STRUCT! {struct uuid_t {
    type_: u8,
    value: uuid_t_value,
}}
#[inline]
pub fn SDP_IS_UUID(x: i32) -> bool {
    x == SDP_UUID16 as i32 || x == SDP_UUID32 as i32 || x == SDP_UUID128 as i32
}
#[inline]
pub fn SDP_IS_ALT(x: i32) -> bool {
    x == SDP_ALT8 as i32 || x == SDP_ALT16 as i32 || x == SDP_ALT32 as i32
}
#[inline]
pub fn SDP_IS_SEQ(x: i32) -> bool {
    x == SDP_SEQ8 as i32 || x == SDP_SEQ16 as i32 || x == SDP_SEQ32 as i32
}
#[inline]
pub fn SDP_IS_TEXT_STR(x: i32) -> bool {
    x == SDP_TEXT_STR8 as i32 || x == SDP_TEXT_STR16 as i32 || x == SDP_TEXT_STR32 as i32
}
STRUCT! {struct _sdp_list {
    next: *mut sdp_list_t,
    data: *mut c_void,
}}
pub type sdp_list_t = _sdp_list;
/*
 * User-visible strings can be in many languages
 * in addition to the universal language.
 *
 * Language meta-data includes language code in ISO639
 * followed by the encoding format. The third field in this
 * structure is the attribute offset for the language.
 * User-visible strings in the specified language can be
 * obtained at this offset.
 */
STRUCT! {struct sdp_lang_attr_t {
    code_ISO639: u16,
    encoding: u16,
    base_offset: u16,
}}
/*
 * Profile descriptor is the Bluetooth profile metadata. If a
 * service conforms to a well-known profile, then its profile
 * identifier (UUID) is an attribute of the service. In addition,
 * if the profile has a version number it is specified here.
 */
STRUCT! {struct sdp_profile_desc_t {
    uuid: uuid_t,
    version: u16,
}}
STRUCT! {struct sdp_version_t {
    major: u8,
    minor: u8,
}}
STRUCT! {struct sdp_buf_t {
    data: *mut u8,
    data_size: u32,
    buf_size: u32,
}}
STRUCT! {struct sdp_record_t {
    handle: u32,
    pattern: *mut sdp_list_t,   // Search pattern: a sequence of all UUIDs seen in this record
    attrlist: *mut sdp_list_t,  // Main service class for Extended Inquiry Response
    svclass: uuid_t,
}}
UNION! {union sdp_data_struct_val {
    [u64; 3],
    int8 int8_mut: i8,
    int16 int16_mut: i16,
    int32 int32_mut: i32,
    int64 int64_mut: i64,
    int128 int128_mut: u128,
    uint8 uint8_mut: u8,
    uint16 uint16_mut: u16,
    uint32 uint32_mut: u32,
    uint64 uint64_mut: u64,
    uint128 uint128_mut: u128,
    uuid uuid_mut: uuid_t,
    str_ str_mut: *mut c_char,
    dataseq dataseq_mut: *mut sdp_data_t,
}}
STRUCT! {struct sdp_data_struct {
    dtd: u8,
    attrId: u16,
    val: sdp_data_struct_val,
    next: *mut sdp_data_t,
    unitSize: c_int,
}}
pub type sdp_data_t = sdp_data_struct;
