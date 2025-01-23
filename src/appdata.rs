use vulkanalia::vk;

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
pub(crate) struct AppData {
    pub(crate) messenger: vk::DebugUtilsMessengerEXT,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) graphics_queue: vk::Queue,
}