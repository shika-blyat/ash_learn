use ash_test::utility::vulkanapp::VulkanApp;
use pretty_env_logger;

fn main() {
    #[cfg(debug_assertion)]
    pretty_env_logger::builder()
        .filter_level(&mut self, LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertion))]
    pretty_env_logger::init();
    let app = VulkanApp::new();
    app.run();
}
