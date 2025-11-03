mod device;
mod instance;
pub mod context;
pub mod surface;

pub use context::*;
pub use surface::*;

pub trait Handle<T> {
    fn handle(&self) -> T;
}