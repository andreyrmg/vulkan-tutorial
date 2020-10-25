mod bindings;
mod glfw;

struct HelloTriangleApplication<'glfw> {
    window: glfw::Window<'glfw>,
}

impl<'g> HelloTriangleApplication<'g> {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    fn new(glfw: &'g glfw::GLFW) -> Self {
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
    let app = HelloTriangleApplication::new(&glfw);
    app.run(&glfw);
}
