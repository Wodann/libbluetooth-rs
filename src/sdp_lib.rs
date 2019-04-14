use crate::bluetooth::bdaddr_t;
use crate::hci::inquiry_info;
use crate::sdp::{
    sdp_buf_t, sdp_data_t, sdp_list_t, sdp_record_t, uuid_t, SDP_ATTR_BROWSE_GRP_LIST,
    SDP_ATTR_CLNT_EXEC_URL, SDP_ATTR_DOC_URL, SDP_ATTR_ICON_URL, SDP_ATTR_PROVNAME_PRIMARY,
    SDP_ATTR_RECORD_STATE, SDP_ATTR_SERVICE_AVAILABILITY, SDP_ATTR_SVCDESC_PRIMARY,
    SDP_ATTR_SVCINFO_TTL, SDP_ATTR_SVCLASS_ID_LIST, SDP_ATTR_SVCNAME_PRIMARY, SDP_UINT32,
    SDP_UINT8,
};
use libc::{c_char, c_int, c_void, size_t};
use std::ptr;
/*
 * a session with an SDP server
 */
STRUCT! {struct sdp_session_t {
    sock: c_int,
    state: c_int,
    local: c_int,
    flags: c_int,
    tid: u16,    // Current transaction ID
    priv_: *mut c_void,
}}
ENUM! {enum sdp_attrreq_type_t {
    /*
     *  Attributes are specified as individual elements
     */
    SDP_ATTR_REQ_INDIVIDUAL = 1,
    /*
     *  Attributes are specified as a range
     */
    SDP_ATTR_REQ_RANGE,
}}
/*
 * Values of the flags parameter to sdp_record_register
 */
pub const SDP_RECORD_PERSIST: i32 = 0x01;
pub const SDP_DEVICE_RECORD: i32 = 0x02;
/*
 * Values of the flags parameter to sdp_connect
 */
pub const SDP_RETRY_IF_BUSY: i32 = 0x01;
pub const SDP_WAIT_ON_CLOSE: i32 = 0x02;
pub const SDP_NON_BLOCKING: i32 = 0x04;
pub const SDP_LARGE_MTU: i32 = 0x08;
pub const MAX_LEN_UUID_STR: i32 = 37;
pub const MAX_LEN_PROTOCOL_UUID_STR: i32 = 8;
pub const MAX_LEN_SERVICECLASS_UUID_STR: i32 = 28;
pub const MAX_LEN_PROFILEDESCRIPTOR_UUID_STR: i32 = 28;
pub type sdp_list_func_t = extern "C" fn(*mut c_void, *mut c_void);
pub type sdp_free_func_t = extern "C" fn(*mut c_void);
pub type sdp_comp_func_t = extern "C" fn(*const c_void, *const c_void) -> c_int;
/*
 * When the pdu_id(type) is a sdp error response, check the status value
 * to figure out the error reason. For status values 0x0001-0x0006 check
 * Bluetooth SPEC. If the status is 0xffff, call sdp_get_error function
 * to get the real reason:
 *   - wrong transaction ID(EPROTO)
 *   - wrong PDU id or(EPROTO)
 *   - I/O error
 */
pub type sdp_callback_t =
    extern "C" fn(type_: u8, status: u16, rsp: *mut u8, size: size_t, udata: *mut c_void);
#[link(name = "bluetooth")]
extern "C" {
    /*
     * SDP lists
     */
    pub fn sdp_list_append(list: *mut sdp_list_t, d: *mut c_void) -> *mut sdp_list_t;
    pub fn sdp_list_remove(list: *mut sdp_list_t, d: *mut c_void) -> *mut sdp_list_t;
    pub fn sdp_list_insert_sorted(
        list: *mut sdp_list_t,
        data: *mut c_void,
        f: sdp_comp_func_t,
    ) -> *mut sdp_list_t;
    pub fn sdp_list_free(list: *mut sdp_list_t, f: sdp_free_func_t);
    /*
     * create an L2CAP connection to a Bluetooth device
     *
     * INPUT:
     *
     *  src: *mut bdaddr_t:
     *    Address of the local device to use to make the connection
     *    (or BDADDR_ANY)
     *
     *  dst: *mut bdaddr_t:
     *    Address of the SDP server device
     */
    pub fn sdp_connect(
        src: *const bdaddr_t,
        dst: *const bdaddr_t,
        flags: u32,
    ) -> *mut sdp_session_t;
    pub fn sdp_close(session: *mut sdp_session_t) -> c_int;
    pub fn sdp_get_socket(session: *const sdp_session_t) -> c_int;
    /*
     * SDP transaction: functions for asynchronous search.
     */
    pub fn sdp_create(sk: c_int, flags: u32) -> *mut sdp_session_t;
    pub fn sdp_get_error(session: *mut sdp_session_t) -> c_int;
    pub fn sdp_process(session: *mut sdp_session_t) -> c_int;
    pub fn sdp_set_notify(
        session: *mut sdp_session_t,
        func: *mut sdp_callback_t,
        udata: *mut c_void,
    ) -> c_int;
    pub fn sdp_service_search_async(
        session: *mut sdp_session_t,
        search: *const sdp_list_t,
        max_rec_num: u16,
    ) -> c_int;
    pub fn sdp_service_attr_async(
        session: *mut sdp_session_t,
        handle: u32,
        reqtype: sdp_attrreq_type_t,
        attrid_list: *const sdp_list_t,
    ) -> c_int;
    pub fn sdp_service_search_attr_async(
        session: *mut sdp_session_t,
        search: *const sdp_list_t,
        reqtype: sdp_attrreq_type_t,
        attrid_list: *const sdp_list_t,
    ) -> c_int;
    pub fn sdp_gen_tid(session: *mut sdp_session_t) -> u16;
    /*
     * find all devices in the piconet
     */
    pub fn sdp_general_inquiry(
        ii: *mut inquiry_info,
        dev_num: c_int,
        duration: c_int,
        found: *mut u8,
    ) -> c_int;
    /*
     * flexible extraction of basic attributes - Jean II
     */
    pub fn sdp_get_int_attr(rec: *const sdp_record_t, attr: u16, value: *mut c_int) -> c_int;
    pub fn sdp_get_string_attr(
        rec: *const sdp_record_t,
        attr: u16,
        value: *mut c_char,
        valuelen: c_int,
    ) -> c_int;
    /*
     * Basic sdp data functions
     */
    pub fn sdp_data_alloc(dtd: u8, value: *const c_void) -> *mut sdp_data_t;
    pub fn sdp_data_alloc_with_length(
        dtd: u8,
        value: *const c_void,
        length: u32,
    ) -> *mut sdp_data_t;
    pub fn sdp_data_free(data: *mut sdp_data_t);
    pub fn sdp_data_get(rec: *const sdp_record_t, attr_id: u16) -> *mut sdp_data_t;
    pub fn sdp_seq_alloc(
        dtds: *mut *mut c_void,
        values: *mut *mut c_void,
        len: c_int,
    ) -> *mut sdp_data_t;
    pub fn sdp_seq_alloc_with_length(
        dtds: *mut *mut c_void,
        values: *mut *mut c_void,
        length: *mut c_int,
        len: c_int,
    ) -> *mut sdp_data_t;
    pub fn sdp_seq_append(seq: *mut sdp_data_t, data: *mut sdp_data_t) -> *mut sdp_data_t;
    pub fn sdp_attr_add(rec: *mut sdp_record_t, attr: u16, data: *mut sdp_data_t) -> c_int;
    pub fn sdp_attr_remove(rec: *mut sdp_record_t, attr: u16);
    pub fn sdp_attr_replace(rec: *mut sdp_record_t, attr: u16, data: *mut sdp_data_t);
    pub fn sdp_set_uuidseq_attr(rec: *mut sdp_record_t, attr: u16, seq: *mut sdp_list_t) -> c_int;
    pub fn sdp_get_uuidseq_attr(
        rec: *const sdp_record_t,
        attr: u16,
        seqp: *mut *mut sdp_list_t,
    ) -> c_int;
    /*
     * NOTE that none of the functions below will update the SDP server,
     * unless the {register, update}sdp_record_t() function is invoked.
     * All functions which return an integer value, return 0 on success
     * or -1 on failure.
     */
    /*
     * Create an attribute and add it to the service record's attribute list.
     * This consists of the data type descriptor of the attribute,
     * the value of the attribute and the attribute identifier.
     */
    pub fn sdp_attr_add_new(rec: *mut sdp_record_t, attr: u16, dtd: u8, p: *const c_void) -> c_int;
    /*
     * Set the information attributes of the service record.
     * The set of attributes comprises service name, description
     * and provider name
     */
    pub fn sdp_set_info_attr(
        rec: *mut sdp_record_t,
        name: *const c_char,
        prov: *const c_char,
        desc: *const c_char,
    );
    /*
     * Set the access protocols of the record to those specified in proto
     */
    pub fn sdp_set_access_protos(rec: *mut sdp_record_t, proto: *const sdp_list_t) -> c_int;
    /*
     * Set the additional access protocols of the record to those specified in proto
     */
    pub fn sdp_set_add_access_protos(rec: *mut sdp_record_t, proto: *const sdp_list_t) -> c_int;
    /*
     * Get protocol port (i.e. PSM for L2CAP, Channel for RFCOMM)
     */
    pub fn sdp_get_proto_port(list: *const sdp_list_t, proto: c_int) -> c_int;
    /*
     * Get protocol descriptor.
     */
    pub fn sdp_get_proto_desc(list: *mut sdp_list_t, proto: c_int) -> *mut sdp_data_t;
    /*
     * Set the LanguageBase attributes to the values specified in list
     * (a linked list of sdp_lang_attr_t objects, one for each language in
     * which user-visible attributes are present).
     */
    pub fn sdp_set_lang_attr(rec: *mut sdp_record_t, list: *const sdp_list_t) -> c_int;
    /*
     * Set the ServiceID attribute of a service.
     */
    pub fn sdp_set_service_id(rec: *mut sdp_record_t, uuid: uuid_t);
    /*
     * Set the GroupID attribute of a service
     */
    pub fn sdp_set_group_id(rec: *mut sdp_record_t, grouuuid: uuid_t);
    /*
     * Set the ServiceAvailability attribute of a service.
     *
     * Note that this represents the relative availability
     * of the service: 0x00 means completely unavailable;
     * 0xFF means maximum availability.
     */
    /*
     * Set the profile descriptor list attribute of a record.
     *
     * Each element in the list is an object of type
     * sdp_profile_desc_t which is a definition of the
     * Bluetooth profile that this service conforms to.
     */
    pub fn sdp_set_profile_descs(rec: *mut sdp_record_t, desc: *const sdp_list_t) -> c_int;
    /*
     * Set URL attributes of a record.
     *
     * ClientExecutableURL: a URL to a client's platform specific (WinCE,
     * PalmOS) executable code that can be used to access this service.
     *
     * DocumentationURL: a URL pointing to service documentation
     *
     * IconURL: a URL to an icon that can be used to represent this service.
     *
     * Note: pass NULL for any URLs that you don't want to set or remove
     */
    pub fn sdp_set_url_attr(
        rec: *mut sdp_record_t,
        clientExecURL: *const c_char,
        docURL: *const c_char,
        iconURL: *const c_char,
    );
    /*
     * a service search request.
     *
     *  INPUT :
     *
     *    *mut sdp_list_t search
     *      list containing elements of the search
     *      pattern. Each entry in the list is a UUID
     *      of the service to be searched
     *
     *    u16 max_rec_num
     *       An integer specifying the maximum number of
     *       entries that the client can handle in the response.
     *
     *  OUTPUT :
     *
     *    c_int return value
     *      0
     *        The request completed successfully. This does not
     *        mean the requested services were found
     *      -1
     *        The request completed unsuccessfully
     *
     *    *mut sdp_list_t rsp_list
     *      This variable is set on a successful return if there are
     *      non-zero service handles. It is a singly linked list of
     *      service record handles (u16)
     */
    pub fn sdp_service_search_req(
        session: *mut sdp_session_t,
        search: *const sdp_list_t,
        max_rec_num: u16,
        rsp_list: *mut *mut sdp_list_t,
    ) -> c_int;
    /*
     *  a service attribute request.
     *
     *  INPUT :
     *
     *    u32 handle
     *      The handle of the service for which the attribute(s) are
     *      requested
     *
     *    sdp_attrreq_type_t reqtype
     *      Attribute identifiers are 16 bit unsigned integers specified
     *      in one of 2 ways described below :
     *      SDP_ATTR_REQ_INDIVIDUAL - 16bit individual identifiers
     *         They are the actual attribute identifiers in ascending order
     *
     *      SDP_ATTR_REQ_RANGE - 32bit identifier range
     *         The high-order 16bits is the start of range
     *         the low-order 16bits are the end of range
     *         0x0000 to 0xFFFF gets all attributes
     *
     *    *mut sdp_list_t attrid_list
     *      Singly linked list containing attribute identifiers desired.
     *      Every element is either a u16(attrSpec = SDP_ATTR_REQ_INDIVIDUAL)
     *      or a u32(attrSpec=SDP_ATTR_REQ_RANGE)
     *
     *  OUTPUT :
     *    c_int return value
     *      0
     *        The request completed successfully. This does not
     *        mean the requested services were found
     *      -1
     *        The request completed unsuccessfully due to a timeout
     */
    pub fn sdp_service_attr_req(
        session: *mut sdp_session_t,
        handle: u32,
        reqtype: sdp_attrreq_type_t,
        attrid_list: *const sdp_list_t,
    ) -> *mut sdp_record_t;
    /*
     *  This is a service search request combined with the service
     *  attribute request. First a service class match is done and
     *  for matching service, requested attributes are extracted
     *
     *  INPUT :
     *
     *    *mut sdp_list_t search
     *      Singly linked list containing elements of the search
     *      pattern. Each entry in the list is a UUID(DataTypeSDP_UUID16)
     *      of the service to be searched
     *
     *    AttributeSpecification attrSpec
     *      Attribute identifiers are 16 bit unsigned integers specified
     *      in one of 2 ways described below :
     *      SDP_ATTR_REQ_INDIVIDUAL - 16bit individual identifiers
     *         They are the actual attribute identifiers in ascending order
     *
     *      SDP_ATTR_REQ_RANGE - 32bit identifier range
     *         The high-order 16bits is the start of range
     *         the low-order 16bits are the end of range
     *         0x0000 to 0xFFFF gets all attributes
     *
     *    *mut sdp_list_t attrid_list
     *      Singly linked list containing attribute identifiers desired.
     *      Every element is either a u16(attrSpec = SDP_ATTR_REQ_INDIVIDUAL)
     *      or a u32(attrSpec=SDP_ATTR_REQ_RANGE)
     *
     *  OUTPUT :
     *    c_int return value
     *      0
     *        The request completed successfully. This does not
     *        mean the requested services were found
     *      -1
     *        The request completed unsuccessfully due to a timeout
     *
     *    *mut sdp_list_t rsp_list
     *      This variable is set on a successful return to point to
     *      service(s) found. Each element of this list is of type
     *      *mut sdp_record_t .
     */
    pub fn sdp_service_search_attr_req(
        session: *mut sdp_session_t,
        search: *const sdp_list_t,
        reqtype: sdp_attrreq_type_t,
        attrid_list: *const sdp_list_t,
        rsp_list: *mut *mut sdp_list_t,
    ) -> c_int;
    /*
     * Allocate/free a service record and its attributes
     */
    pub fn sdp_record_alloc() -> *mut sdp_record_t;
    pub fn sdp_record_free(rec: *mut sdp_record_t);
    /*
     * Register a service record.
     *
     * Note: It is the responsbility of the Service Provider to create the
     * record first and set its attributes using setXXX() methods.
     *
     * The service provider must then call sdp_record_register() to make
     * the service record visible to SDP clients.  This function returns 0
     * on success or -1 on failure (and sets errno).
     */
    pub fn sdp_device_record_register_binary(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        data: *mut u8,
        size: u32,
        flags: u8,
        handle: *mut u32,
    ) -> c_int;
    pub fn sdp_device_record_register(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        rec: *mut sdp_record_t,
        flags: u8,
    ) -> c_int;
    pub fn sdp_record_register(
        session: *mut sdp_session_t,
        rec: *mut sdp_record_t,
        flags: u8,
    ) -> c_int;
    /*
     * Unregister a service record.
     */
    pub fn sdp_device_record_unregister_binary(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        handle: u32,
    ) -> c_int;
    pub fn sdp_device_record_unregister(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        rec: *mut sdp_record_t,
    ) -> c_int;
    pub fn sdp_record_unregister(session: *mut sdp_session_t, rec: *mut sdp_record_t) -> c_int;
    /*
     * Update an existing service record.  (Calling this function
     * before a previous call to sdp_record_register() will result
     * in an error.)
     */
    pub fn sdp_device_record_update_binary(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        handle: u32,
        data: *mut u8,
        size: u32,
    ) -> c_int;
    pub fn sdp_device_record_update(
        session: *mut sdp_session_t,
        device: *mut bdaddr_t,
        rec: *const sdp_record_t,
    ) -> c_int;
    pub fn sdp_record_update(sess: *mut sdp_session_t, rec: *const sdp_record_t) -> c_int;
    pub fn sdp_record_print(rec: *const sdp_record_t);
    /*
     * UUID functions
     */
    pub fn sdp_uuid16_create(uuid: *mut uuid_t, data: u16) -> *mut uuid_t;
    pub fn sdp_uuid32_create(uuid: *mut uuid_t, data: u32) -> *mut uuid_t;
    pub fn sdp_uuid128_create(uuid: *mut uuid_t, data: *const c_void) -> *mut uuid_t;
    pub fn sdp_uuid16_cmp(p1: *const c_void, p2: *const c_void) -> c_int;
    pub fn sdp_uuid128_cmp(p1: *const c_void, p2: *const c_void) -> c_int;
    pub fn sdp_uuid_cmp(p1: *const c_void, p2: *const c_void) -> c_int;
    pub fn sdp_uuid_to_uuid128(uuid: *const uuid_t) -> *mut uuid_t;
    pub fn sdp_uuid16_to_uuid128(uuid128: *mut uuid_t, uuid16: *const uuid_t);
    pub fn sdp_uuid32_to_uuid128(uuid128: *mut uuid_t, uuid32: *const uuid_t);
    pub fn sdp_uuid128_to_uuid(uuid: *mut uuid_t) -> c_int;
    pub fn sdp_uuid_to_proto(uuid: *mut uuid_t) -> c_int;
    pub fn sdp_uuid_extract(
        buffer: *const u8,
        bufsize: c_int,
        uuid: *mut uuid_t,
        scanned: *mut c_int,
    ) -> c_int;
    pub fn sdp_uuid_print(uuid: *const uuid_t);
    pub fn sdp_uuid2strn(uuid: *const uuid_t, str: *mut c_char, n: size_t) -> c_int;
    pub fn sdp_proto_uuid2strn(uuid: *const uuid_t, str: *mut c_char, n: size_t) -> c_int;
    pub fn sdp_svclass_uuid2strn(uuid: *const uuid_t, str: *mut c_char, n: size_t) -> c_int;
    pub fn sdp_profile_uuid2strn(uuid: *const uuid_t, str: *mut c_char, n: size_t) -> c_int;
    /*
     * In all the sdp_get_XXX(handle, xxx: *mut XXX) functions below,
     * the *mut XXX  is set to point to the value, should it exist
     * and 0 is returned. If the value does not exist, -1 is
     * returned and errno set to ENODATA.
     *
     * In all the methods below, the memory management rules are
     * simple. Don't free anything! The pointer returned, in the
     * case of constructed types, is a pointer to the contents
     * of the sdp_record_t.
     */
    /*
     * Get the access protocols from the service record
     */
    pub fn sdp_get_access_protos(rec: *const sdp_record_t, protos: *mut *mut sdp_list_t) -> c_int;
    /*
     * Get the additional access protocols from the service record
     */
    pub fn sdp_get_add_access_protos(
        rec: *const sdp_record_t,
        protos: *mut *mut sdp_list_t,
    ) -> c_int;
    /*
     * Extract language attribute meta-data of the service record.
     * For each language in the service record, LangSeq has a struct of type
     * sdp_lang_attr_t.
     */
    pub fn sdp_get_lang_attr(rec: *const sdp_record_t, langSeq: *mut *mut sdp_list_t) -> c_int;
    /*
     * Extract the Bluetooth profile descriptor sequence from a record.
     * Each element in the list is of type sdp_profile_desc_t
     * which contains the UUID of the profile and its version number
     * (encoded as major and minor in the high-order 8bits
     * and low-order 8bits respectively of the u16)
     */
    pub fn sdp_get_profile_descs(rec: *const sdp_record_t, profDesc: *mut *mut sdp_list_t)
        -> c_int;
    /*
     * Extract SDP server version numbers
     *
     * Note: that this is an attribute of the SDP server only and
     * contains a list of u16 each of which represent the
     * major and minor SDP version numbers supported by this server
     */
    pub fn sdp_get_server_ver(rec: *const sdp_record_t, pVnumList: *mut *mut sdp_list_t) -> c_int;
    pub fn sdp_get_service_id(rec: *const sdp_record_t, uuid: *mut uuid_t) -> c_int;
    pub fn sdp_get_group_id(rec: *const sdp_record_t, uuid: *mut uuid_t) -> c_int;
    pub fn sdp_get_record_state(rec: *const sdp_record_t, svcRecState: *mut u32) -> c_int;
    pub fn sdp_get_service_avail(rec: *const sdp_record_t, svcAvail: *mut u8) -> c_int;
    pub fn sdp_get_service_ttl(rec: *const sdp_record_t, svcTTLInfo: *mut u32) -> c_int;
    pub fn sdp_get_database_state(rec: *const sdp_record_t, svcDBState: *mut u32) -> c_int;
    /*
     * Set the supported features
     * sf should be a list of list with each feature data
     * Returns 0 on success -1 on fail
     */
    pub fn sdp_set_supp_feat(rec: *mut sdp_record_t, sf: *const sdp_list_t) -> c_int;
    /*
     * Get the supported features
     * seqp is set to a list of list with each feature data
     * Returns 0 on success, if an error occurred -1 is returned and errno is set
     */
    pub fn sdp_get_supp_feat(rec: *const sdp_record_t, seqp: *mut *mut sdp_list_t) -> c_int;
    pub fn sdp_extract_pdu(
        pdata: *const u8,
        bufsize: c_int,
        scanned: *mut c_int,
    ) -> *mut sdp_record_t;
    pub fn sdp_copy_record(rec: *mut sdp_record_t) -> *mut sdp_record_t;
    pub fn sdp_data_print(data: *mut sdp_data_t);
    pub fn sdp_print_service_attr(alist: *mut sdp_list_t);
    pub fn sdp_attrid_comp_func(key1: *const c_void, key2: *const c_void) -> c_int;
    pub fn sdp_set_seq_len(ptr: *mut u8, length: u32);
    pub fn sdp_set_attrid(pdu: *mut sdp_buf_t, id: u16);
    pub fn sdp_append_to_pdu(dst: *mut sdp_buf_t, d: *mut sdp_data_t);
    pub fn sdp_append_to_buf(dst: *mut sdp_buf_t, data: *mut u8, len: u32);
    pub fn sdp_gen_pdu(pdu: *mut sdp_buf_t, data: *mut sdp_data_t) -> c_int;
    pub fn sdp_gen_record_pdu(rec: *const sdp_record_t, pdu: *mut sdp_buf_t) -> c_int;
    pub fn sdp_extract_seqtype(
        buf: *const u8,
        bufsize: c_int,
        dtdp: *mut u8,
        size: *mut c_int,
    ) -> c_int;
    pub fn sdp_extract_attr(
        pdata: *const u8,
        bufsize: c_int,
        extractedLength: *mut c_int,
        rec: *mut sdp_record_t,
    ) -> *mut sdp_data_t;
    pub fn sdp_pattern_add_uuid(rec: *mut sdp_record_t, uuid: *mut uuid_t);
    pub fn sdp_pattern_add_uuidseq(rec: *mut sdp_record_t, seq: *mut sdp_list_t);
    pub fn sdp_send_req_w4_rsp(
        session: *mut sdp_session_t,
        req: *mut u8,
        rsp: *mut u8,
        reqsize: u32,
        rspsize: *mut u32,
    ) -> c_int;
    pub fn sdp_add_lang_attr(rec: *mut sdp_record_t);
}
#[inline]
pub fn sdp_list_len(list: *const sdp_list_t) -> c_int {
    let mut list = list;
    let mut n: c_int = 0;
    while list != ptr::null_mut() {
        n += 1;
        list = unsafe { *list }.next;
    }
    return n;
}
#[inline]
pub fn sdp_list_find(list: *mut sdp_list_t, u: *mut c_void, f: sdp_comp_func_t) -> *mut sdp_list_t {
    let mut list = list;
    while list != ptr::null_mut() {
        if f(unsafe { *list }.data, u) == 0 {
            return list;
        }
        list = unsafe { *list }.next;
    }
    return ptr::null_mut();
}
#[inline]
pub fn sdp_list_foreach(list: *mut sdp_list_t, f: sdp_list_func_t, u: *mut c_void) {
    let mut list = list;
    while list != ptr::null_mut() {
        f(unsafe { *list }.data, u);
        list = unsafe { *list }.next;
    }
}
/*
 * Set the ServiceClassID attribute to the sequence specified by seq.
 * Note that the identifiers need to be in sorted order from the most
 * specific to the most generic service class that this service
 * conforms to.
 */
#[inline]
pub fn sdp_set_service_classes(rec: *mut sdp_record_t, seq: *mut sdp_list_t) -> c_int {
    unsafe { sdp_set_uuidseq_attr(rec, SDP_ATTR_SVCLASS_ID_LIST, seq) }
}
/*
 * Get the service classes to which the service conforms.
 *
 * When set, the list contains elements of ServiceClassIdentifer(u16)
 * ordered from most specific to most generic
 */
#[inline]
pub fn sdp_get_service_classes(rec: *const sdp_record_t, seqp: *mut *mut sdp_list_t) -> c_int {
    unsafe { sdp_get_uuidseq_attr(rec, SDP_ATTR_SVCLASS_ID_LIST, seqp) }
}
/*
 * Set the BrowseGroupList attribute to the list specified by seq.
 *
 * A service can belong to one or more service groups
 * and the list comprises such group identifiers (UUIDs)
 */
#[inline]
pub fn sdp_set_browse_groups(rec: *mut sdp_record_t, seq: *mut sdp_list_t) -> c_int {
    unsafe { sdp_set_uuidseq_attr(rec, SDP_ATTR_BROWSE_GRP_LIST, seq) }
}
/*
 * Set the ServiceInfoTimeToLive attribute of the service.
 * This is the number of seconds that this record is guaranteed
 * not to change after being obtained by a client.
 */
#[inline]
pub fn sdp_set_service_ttl(rec: *mut sdp_record_t, ttl: u32) -> c_int {
    unsafe {
        sdp_attr_add_new(
            rec,
            SDP_ATTR_SVCINFO_TTL,
            SDP_UINT32,
            &ttl as *const _ as *const _,
        )
    }
}
/*
 * Set the ServiceRecordState attribute of a service. This is
 * guaranteed to change if there is any kind of modification to
 * the record.
 */
#[inline]
pub fn sdp_set_record_state(rec: *mut sdp_record_t, state: u32) -> c_int {
    unsafe {
        sdp_attr_add_new(
            rec,
            SDP_ATTR_RECORD_STATE,
            SDP_UINT32,
            &state as *const _ as *const _,
        )
    }
}
#[inline]
pub fn sdp_set_service_avail(rec: *mut sdp_record_t, avail: u8) -> c_int {
    unsafe {
        sdp_attr_add_new(
            rec,
            SDP_ATTR_SERVICE_AVAILABILITY,
            SDP_UINT8,
            &avail as *const _ as *const _,
        )
    }
}
/*
 * Extract the list of browse groups to which the service belongs.
 * When set, seqp contains elements of GroupID (u16)
 */
#[inline]
pub fn sdp_get_browse_groups(rec: *const sdp_record_t, seqp: *mut *mut sdp_list_t) -> c_int {
    unsafe { sdp_get_uuidseq_attr(rec, SDP_ATTR_BROWSE_GRP_LIST, seqp) }
}
#[inline]
pub fn sdp_get_service_name(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_SVCNAME_PRIMARY, str, len) }
}
#[inline]
pub fn sdp_get_service_desc(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_SVCDESC_PRIMARY, str, len) }
}
#[inline]
pub fn sdp_get_provider_name(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_PROVNAME_PRIMARY, str, len) }
}
#[inline]
pub fn sdp_get_doc_url(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_DOC_URL, str, len) }
}
#[inline]
pub fn sdp_get_clnt_exec_url(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_CLNT_EXEC_URL, str, len) }
}
#[inline]
pub fn sdp_get_icon_url(rec: *const sdp_record_t, str: *mut c_char, len: c_int) -> c_int {
    unsafe { sdp_get_string_attr(rec, SDP_ATTR_ICON_URL, str, len) }
}
