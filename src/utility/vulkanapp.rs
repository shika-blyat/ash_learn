pub struct VulkanApp{

}
impl VulkanApp{
	pub fn new() -> Self {
		VulkanApp{}
	}
    pub fn run(&self) {
        self.init_vulkan()
    }
    pub fn init_vulkan(&self){
        self.create_instance();
    }
}