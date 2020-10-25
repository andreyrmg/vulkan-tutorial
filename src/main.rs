mod bindings;
mod glfw;
mod vk;

struct HelloTriangleApplication<'glfw> {
    window: glfw::Window<'glfw>,
}

impl<'g, 'v> HelloTriangleApplication<'g> {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    fn new(glfw: &'g glfw::GLFW, _instance: &'v vk::Instance) -> Self {
        let window = glfw.new_window(
            Self::WIDTH,
            Self::HEIGHT,
            "Vulkan",
            &[
                glfw::WindowHint::ClientApi(glfw::ClientApi::NoApi),
                glfw::WindowHint::Resizable(false),
            ],
        );
        HelloTriangleApplication { window }
    }

    fn run(self, glfw: &'g glfw::GLFW) {
        while !self.window.should_close() {
            glfw.poll_events();
        }
    }
}

fn main() {
    let glfw = glfw::new();
    let instance = vk::Instance::new("Hello Triangle", "No Engine");
    let app = HelloTriangleApplication::new(&glfw, &instance);
    app.run(&glfw);
}
