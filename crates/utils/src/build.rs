
pub trait Build {
    type Target;

    fn build(&self) -> Self::Target;
}

pub trait Buildable: Sized {
    type Builder: Default + Build<Target = Self>;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    fn build() -> Self {
        Self::builder().build()
    }
}