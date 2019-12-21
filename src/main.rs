use ash_test::utility::{constants::*, vulkanapp::VulkanApp};
use log::info;
use pretty_env_logger;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    init_logger();
    info!("Starting vulkan app !");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize {
            width: WIDTH,
            height: HEIGHT,
        })
        .build(&event_loop)
        .expect("Window creation failed");

    let app = VulkanApp::new(&window);
    app.run_loop(window, event_loop);
}

fn init_logger() {
    pretty_env_logger::init_custom_env("ASH_LOG");
}
