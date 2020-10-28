use crate::bindings;

use std::ffi::{CStr, CString};
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

        let (glfw_extensions, glfw_extension_count) = unsafe {
            let mut glfw_extension_count = MaybeUninit::uninit();
            (
                bindings::glfwGetRequiredInstanceExtensions(glfw_extension_count.as_mut_ptr()),
                glfw_extension_count.assume_init(),
            )
        };

        check_all_extensions_included_in_supported_list(glfw_extensions, glfw_extension_count);

        let mut create_info: bindings::VkInstanceCreateInfo = unsafe { zeroed() };
        create_info.sType = bindings::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        create_info.pApplicationInfo = &app_info;
        create_info.enabledExtensionCount = glfw_extension_count;
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

fn check_all_extensions_included_in_supported_list(
    glfw_extensions: *const *const i8,
    glfw_extension_count: u32,
) {
    let extensions: Vec<bindings::VkExtensionProperties> = unsafe {
        let mut count = MaybeUninit::uninit();
        bindings::vkEnumerateInstanceExtensionProperties(
            ptr::null(),
            count.as_mut_ptr(),
            ptr::null_mut(),
        );
        let mut buffer = Vec::with_capacity(count.assume_init() as usize);
        bindings::vkEnumerateInstanceExtensionProperties(
            ptr::null(),
            count.as_mut_ptr(),
            buffer.as_mut_ptr(),
        );
        buffer.set_len(count.assume_init() as usize);
        buffer
    };
    println!("available extensions:");
    for extension in extensions.iter() {
        println!(
            "\t{}",
            unsafe { CStr::from_ptr(extension.extensionName.as_ptr()) }.to_string_lossy()
        );
    }
    let glfw_extensions =
        unsafe { &*ptr::slice_from_raw_parts(glfw_extensions, glfw_extension_count as usize) }
            .into_iter()
            .map(|&name| unsafe { CStr::from_ptr(name) });
    for glfw_extension in glfw_extensions {
        assert!(
            extensions
                .iter()
                .find(|extension| unsafe {
                    CStr::from_ptr(extension.extensionName.as_ptr()) == glfw_extension
                })
                .is_some(),
            "required extension {} not supported",
            glfw_extension.to_string_lossy()
        )
    }
}
