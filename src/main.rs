use std::ops;
use std::ptr;

mod bindings;

struct HelloTriangleApplication {
    window: *mut bindings::GLFWwindow,
}

impl HelloTriangleApplication {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    fn new() -> Self {
        let window = Self::new_window();
        HelloTriangleApplication { window }
    }

    fn new_window() -> *mut bindings::GLFWwindow {
        unsafe {
            assert!(bindings::glfwInit() == bindings::GLFW_TRUE as i32);
            bindings::glfwWindowHint(
                bindings::GLFW_CLIENT_API as i32,
                bindings::GLFW_NO_API as i32,
            );
            bindings::glfwWindowHint(bindings::GLFW_RESIZABLE as i32, bindings::GLFW_FALSE as i32);
            bindings::glfwCreateWindow(
                Self::WIDTH,
                Self::HEIGHT,
                b"Vulkan\0".as_ptr() as *const i8,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    }

    fn run(self) {
        while unsafe { bindings::glfwWindowShouldClose(self.window) } != bindings::GLFW_TRUE as i32
        {
            unsafe {
                bindings::glfwPollEvents();
            }
        }
    }
}

impl ops::Drop for HelloTriangleApplication {
    fn drop(&mut self) {
        unsafe {
            bindings::glfwDestroyWindow(self.window);
            bindings::glfwTerminate();
        }
    }
}

fn main() {
    let app = HelloTriangleApplication::new();
    app.run();
}
