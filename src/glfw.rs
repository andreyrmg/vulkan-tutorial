use crate::bindings;

use std::ffi::CString;
use std::ops;
use std::ptr;

pub struct Instance;

impl Instance {
    pub fn new() -> Self {
        let result = unsafe { bindings::glfwInit() };
        debug_assert_eq!(result, bindings::GLFW_TRUE as i32);
        Self
    }

    pub fn new_window(&self, width: i32, height: i32, title: &str, hints: &[WindowHint]) -> Window {
        unsafe {
            bindings::glfwDefaultWindowHints();
        }
        for hint in hints {
            let (hint, value) = match hint {
                WindowHint::ClientApi(api) => (bindings::GLFW_CLIENT_API as i32, *api as i32),
                WindowHint::Resizable(v) => (
                    bindings::GLFW_RESIZABLE as i32,
                    if *v {
                        bindings::GLFW_TRUE
                    } else {
                        bindings::GLFW_FALSE
                    } as i32,
                ),
            };
            unsafe {
                bindings::glfwWindowHint(hint, value);
            }
        }
        let title =
            CString::new(title).expect("supplied title should not contain an internal 0 byte");
        let ptr = unsafe {
            bindings::glfwCreateWindow(
                width,
                height,
                title.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        debug_assert!(!ptr.is_null());
        Window {
            ptr,
            pd: std::marker::PhantomData,
        }
    }

    pub fn poll_events(&self) {
        unsafe {
            bindings::glfwPollEvents();
        }
    }
}

impl ops::Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            bindings::glfwTerminate();
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ClientApi {
    NoApi = bindings::GLFW_NO_API,
}

pub enum WindowHint {
    ClientApi(ClientApi),
    Resizable(bool),
}

pub struct Window<'glfw> {
    ptr: *mut bindings::GLFWwindow,
    pd: std::marker::PhantomData<&'glfw Instance>,
}

impl Window<'_> {
    pub fn should_close(&self) -> bool {
        let should_close = unsafe { bindings::glfwWindowShouldClose(self.ptr) };
        should_close == bindings::GLFW_TRUE as i32
    }
}

impl ops::Drop for Window<'_> {
    fn drop(&mut self) {
        unsafe {
            bindings::glfwDestroyWindow(self.ptr);
        }
    }
}
