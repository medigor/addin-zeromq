mod addin_info;
mod addin_rep;
mod addin_req;
mod addin_pub;
mod addin_sub;
mod addin_push;
mod addin_pull;
mod client;

use std::{
    ffi::{c_int, c_long, c_void},
    sync::atomic::{AtomicI32, Ordering},
};

use addin1c::{create_component, destroy_component, name, AttachType};

pub static mut PLATFORM_CAPABILITIES: AtomicI32 = AtomicI32::new(-1);

#[allow(non_snake_case)]
#[no_mangle]
/// # Safety
/// This function should be called from 1C.
pub unsafe extern "C" fn GetClassObject(name: *const u16, component: *mut *mut c_void) -> c_long {
    match *name as u8 {
        b'0' => {
            if let Ok(addin) = addin_rep::AddinRep::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'1' => {
            if let Ok(addin) = addin_req::AddinReq::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'2' => {
            if let Ok(addin) = addin_pub::AddinPub::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'3' => {
            if let Ok(addin) = addin_sub::AddinSub::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'4' => {
            if let Ok(addin) = addin_push::AddinPush::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'5' => {
            if let Ok(addin) = addin_pull::AddinPull::new() {
                create_component(component, addin)
            } else {
                0
            }
        }
        b'6' => {
            let addin = addin_info::AddinInfo::new();
            create_component(component, addin)
        }
        _ => 0,
    }
}

#[allow(non_snake_case)]
#[no_mangle]
/// # Safety
/// This function should be called from 1C.
pub unsafe extern "C" fn DestroyObject(component: *mut *mut c_void) -> c_long {
    destroy_component(component)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn GetClassNames() -> *const u16 {
    name!("0|1|2|3|4|5|6").as_ptr()
}

#[allow(non_snake_case)]
#[no_mangle]
/// # Safety
/// This function should be called from 1C.
pub unsafe extern "C" fn SetPlatformCapabilities(capabilities: c_int) -> c_int {
    PLATFORM_CAPABILITIES.store(capabilities, Ordering::Relaxed);
    3
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn GetAttachType() -> AttachType {
    AttachType::Any
}
