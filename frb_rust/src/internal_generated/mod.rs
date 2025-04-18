// THIS FILE IS AUTO-GENERATED BY flutter_rust_bridge_codegen internal-generate, PLEASE DO NOT EDIT

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_io_extern_func {
    () => {
        #[unsafe(no_mangle)]
        pub extern "C" fn frb_pde_ffi_dispatcher_primary(
            func_id: i32,
            port_: i64,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            pde_ffi_dispatcher_primary_impl(func_id, port_, ptr_, rust_vec_len_, data_len_)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn frb_pde_ffi_dispatcher_sync(
            func_id: i32,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) -> $crate::for_generated::WireSyncRust2DartSse {
            pde_ffi_dispatcher_sync_impl(func_id, ptr_, rust_vec_len_, data_len_)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn frb_dart_fn_deliver_output(
            call_id: i32,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            let message = unsafe {
                $crate::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_web_extern_func {
    () => {
        #[wasm_bindgen]
        pub fn frb_pde_ffi_dispatcher_primary(
            func_id: i32,
            port_: $crate::for_generated::MessagePort,
            ptr_: $crate::for_generated::PlatformGeneralizedUint8ListPtr,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            pde_ffi_dispatcher_primary_impl(func_id, port_, ptr_, rust_vec_len_, data_len_)
        }

        #[wasm_bindgen]
        pub fn frb_pde_ffi_dispatcher_sync(
            func_id: i32,
            ptr_: $crate::for_generated::PlatformGeneralizedUint8ListPtr,
            rust_vec_len_: i32,
            data_len_: i32,
        ) -> $crate::for_generated::WireSyncRust2DartSse {
            pde_ffi_dispatcher_sync_impl(func_id, ptr_, rust_vec_len_, data_len_)
        }

        #[wasm_bindgen]
        pub fn frb_dart_fn_deliver_output(
            call_id: i32,
            ptr_: $crate::for_generated::PlatformGeneralizedUint8ListPtr,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            let message = unsafe {
                $crate::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
        }
    };
}
