/* automatically generated by rust-bindgen 0.68.1 */

extern "C" {
    #[doc = " Returns an opaque identifier of a new TDLib instance.\n The TDLib instance will not send updates until the first request is sent to it.\n \\return Opaque identifier of a new TDLib instance."]
    pub fn td_create_client_id() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Sends request to the TDLib client. May be called from any thread.\n \\param[in] client_id TDLib client identifier.\n \\param[in] request JSON-serialized null-terminated request to TDLib."]
    pub fn td_send(client_id: ::std::os::raw::c_int, request: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Receives incoming updates and request responses. Must not be called simultaneously from two different threads.\n The returned pointer can be used until the next call to td_receive or td_execute, after which it will be deallocated by TDLib.\n \\param[in] timeout The maximum number of seconds allowed for this function to wait for new data.\n \\return JSON-serialized null-terminated incoming update or request response. May be NULL if the timeout expires."]
    pub fn td_receive(timeout: f64) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Synchronously executes a TDLib request.\n A request can be executed synchronously, only if it is documented with \"Can be called synchronously\".\n The returned pointer can be used until the next call to td_receive or td_execute, after which it will be deallocated by TDLib.\n \\param[in] request JSON-serialized null-terminated request to TDLib.\n \\return JSON-serialized null-terminated request response."]
    pub fn td_execute(request: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
#[doc = " A type of callback function that will be called when a message is added to the internal TDLib log.\n\n \\param verbosity_level Log verbosity level with which the message was added from -1 up to 1024.\n                        If 0, then TDLib will crash as soon as the callback returns.\n                        None of the TDLib methods can be called from the callback.\n \\param message Null-terminated UTF-8-encoded string with the message added to the log."]
pub type td_log_message_callback_ptr = ::std::option::Option<
    unsafe extern "C" fn(
        verbosity_level: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    ),
>;
extern "C" {
    #[doc = " Sets the callback that will be called when a message is added to the internal TDLib log.\n None of the TDLib methods can be called from the callback.\n By default the callback is not set.\n\n \\param[in] max_verbosity_level The maximum verbosity level of messages for which the callback will be called.\n \\param[in] callback Callback that will be called when a message is added to the internal TDLib log.\n                     Pass nullptr to remove the callback."]
    pub fn td_set_log_message_callback(
        max_verbosity_level: ::std::os::raw::c_int,
        callback: td_log_message_callback_ptr,
    );
}
extern "C" {
    #[doc = " Creates a new instance of TDLib.\n \\return Pointer to the created instance of TDLib."]
    pub fn td_json_client_create() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Sends request to the TDLib client. May be called from any thread.\n \\param[in] client The client.\n \\param[in] request JSON-serialized null-terminated request to TDLib."]
    pub fn td_json_client_send(
        client: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Receives incoming updates and request responses from the TDLib client. May be called from any thread, but\n must not be called simultaneously from two different threads.\n Returned pointer will be deallocated by TDLib during next call to td_json_client_receive or td_json_client_execute\n in the same thread, so it can't be used after that.\n \\param[in] client The client.\n \\param[in] timeout The maximum number of seconds allowed for this function to wait for new data.\n \\return JSON-serialized null-terminated incoming update or request response. May be NULL if the timeout expires."]
    pub fn td_json_client_receive(
        client: *mut ::std::os::raw::c_void,
        timeout: f64,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Synchronously executes TDLib request. May be called from any thread.\n Only a few requests can be executed synchronously.\n Returned pointer will be deallocated by TDLib during next call to td_json_client_receive or td_json_client_execute\n in the same thread, so it can't be used after that.\n \\param[in] client The client. Currently ignored for all requests, so NULL can be passed.\n \\param[in] request JSON-serialized null-terminated request to TDLib.\n \\return JSON-serialized null-terminated request response."]
    pub fn td_json_client_execute(
        client: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Destroys the TDLib client instance. After this is called the client instance must not be used anymore.\n \\param[in] client The client."]
    pub fn td_json_client_destroy(client: *mut ::std::os::raw::c_void);
}
