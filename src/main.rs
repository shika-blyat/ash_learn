use ash_test::utility::vulkanapp::VulkanApp;
use log::info;
use pretty_env_logger;
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    init_logger();
    info!("Starting vulkan app !");
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Window creation failed");

    let app = VulkanApp::new(&window);
    app.run_loop(window, event_loop);
}

fn init_logger() {
    pretty_env_logger::init_custom_env("ASH_LOG");
}
