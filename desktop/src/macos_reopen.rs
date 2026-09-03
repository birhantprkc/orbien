#![cfg(target_os = "macos")]

use std::sync::Mutex;

use objc2::runtime::{AnyClass, AnyObject, Bool, Imp, Sel};
use objc2::sel;

type ShowFn = Box<dyn Fn() + Send + Sync + 'static>;

static SHOW_MAIN: Mutex<Option<ShowFn>> = Mutex::new(None);

pub fn install(show_main: impl Fn() + Send + Sync + 'static) {
    *SHOW_MAIN.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(show_main));
    patch_winit_delegate();
}

fn schedule_show() {
    let _ = slint::invoke_from_event_loop(|| {
        if let Some(show) = SHOW_MAIN.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
            show();
        }
    });
}

fn patch_winit_delegate() {
    let sel = sel!(applicationShouldHandleReopen:hasVisibleWindows:);
    let Some(cls) = AnyClass::get(c"WinitApplicationDelegate") else {
        tracing::warn!("WinitApplicationDelegate missing; Dock reopen not installed");
        return;
    };
    if cls.instance_method(sel).is_some() {
        return;
    }

    unsafe extern "C-unwind" fn handle_reopen(
        _this: *mut AnyObject,
        _cmd: Sel,
        _app: *mut AnyObject,
        _has_visible_windows: Bool,
    ) -> Bool {
        schedule_show();
        Bool::NO
    }

    let added = unsafe {
        objc2::ffi::class_addMethod(
            (cls as *const AnyClass).cast_mut(),
            sel,
            core::mem::transmute::<
                unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject, Bool) -> Bool,
                Imp,
            >(handle_reopen),
            c"B@:@B".as_ptr(),
        )
    };
    if !added.as_bool() {
        tracing::warn!("failed to add applicationShouldHandleReopen to WinitApplicationDelegate");
    }
}
