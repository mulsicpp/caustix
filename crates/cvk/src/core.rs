mod device;
mod instance;
pub mod context;

pub use context::*;

pub trait VkHandle<T> {
    fn handle(&self) -> T;
}