use ash_test::utility::vulkanapp::VulkanApp;
use pretty_env_logger;
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    init_logger();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Window creation failed");

    let app = VulkanApp::new(&window);
    app.run_loop(window, event_loop);
}

fn init_logger() {
    #[cfg(debug_assertion)]
    pretty_env_logger::builder()
        .filter_level(&mut self, LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertion))]
    pretty_env_logger::init();
}
