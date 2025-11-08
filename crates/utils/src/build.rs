
pub trait Build {
    type Target;

    fn build(&self) -> Self::Target;
}

pub trait Buildable: Sized {
    type Builder<'a>: Default + Build<Target = Self>;

    fn builder<'a>() -> Self::Builder<'a> {
        Self::Builder::default()
    }

    fn build() -> Self {
        Self::builder().build()
    }
}