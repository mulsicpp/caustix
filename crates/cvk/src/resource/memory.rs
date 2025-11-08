
#[repr(u32)]
#[derive(Copy, Clone, Default, Debug)]
pub enum MemoryUsage {
    #[default]
    Auto,
    PreferDevice,
    PreferHost,
}

impl MemoryUsage {
    pub(crate) fn as_vma(&self) -> vk_mem::MemoryUsage {
        match *self {
            MemoryUsage::Auto => vk_mem::MemoryUsage::Auto,
            MemoryUsage::PreferDevice => vk_mem::MemoryUsage::AutoPreferDevice,
            MemoryUsage::PreferHost => vk_mem::MemoryUsage::AutoPreferHost,
        }
    }
}