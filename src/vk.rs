use crate::bindings;

use std::ffi::CString;
use std::mem::{zeroed, MaybeUninit};
use std::ops;
use std::ptr;

macro_rules! vk_make_version {
    ($major:expr, $minor:expr, $patch: expr) => {
        ($major as u32) << 22 | ($minor as u32) << 12 | $patch as u32
    };
}

pub struct Instance {
    inner: bindings::VkInstance,
}

impl ops::Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            bindings::vkDestroyInstance(self.inner, ptr::null());
        }
    }
}

impl Instance {
    pub fn new(application_name: &str, engine_name: &str) -> Self {
        let application_name = CString::new(application_name)
            .expect("application name should not contain an internal 0 byte");
        let engine_name =
            CString::new(engine_name).expect("engine name should not contain an internal 0 byte");

        let mut app_info: bindings::VkApplicationInfo = unsafe { zeroed() };
        app_info.sType = bindings::VK_STRUCTURE_TYPE_APPLICATION_INFO;
        app_info.pApplicationName = application_name.as_ptr();
        app_info.applicationVersion = vk_make_version!(1, 0, 0);
        app_info.pEngineName = engine_name.as_ptr();
        app_info.engineVersion = vk_make_version!(1, 0, 0);
        app_info.apiVersion = vk_make_version!(1, 0, 0);

        let mut glfw_extension_count = MaybeUninit::uninit();
        let glfw_extensions = unsafe {
            bindings::glfwGetRequiredInstanceExtensions(glfw_extension_count.as_mut_ptr())
        };

        let mut create_info: bindings::VkInstanceCreateInfo = unsafe { zeroed() };
        create_info.sType = bindings::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        create_info.pApplicationInfo = &app_info;
        create_info.enabledExtensionCount = unsafe { glfw_extension_count.assume_init() };
        create_info.ppEnabledExtensionNames = glfw_extensions;

        let mut instance = MaybeUninit::uninit();
        assert_eq!(
            unsafe {
                bindings::vkCreateInstance(&create_info, ptr::null_mut(), instance.as_mut_ptr())
            },
            bindings::VK_SUCCESS,
            "failed to create instance"
        );
        Self {
            inner: unsafe { instance.assume_init() },
        }
    }
}
