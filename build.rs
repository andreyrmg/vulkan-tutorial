use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=vulkan");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_var("GLFW_FALSE|GLFW_TRUE|GLFW_CLIENT_API|GLFW_NO_API|GLFW_RESIZABLE")
        .whitelist_function("glfwCreateWindow")
        .whitelist_function("glfwDefaultWindowHints")
        .whitelist_function("glfwDestroyWindow")
        .whitelist_function("glfwGetRequiredInstanceExtensions")
        .whitelist_function("glfwInit")
        .whitelist_function("glfwPollEvents")
        .whitelist_function("glfwTerminate")
        .whitelist_function("glfwWindowHint")
        .whitelist_function("glfwWindowShouldClose")
        .whitelist_function("vk(Create|Destroy)Instance")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
