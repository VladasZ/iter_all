#![feature(const_trait_impl)]

pub trait IterAll: Sized {
    fn iter_all(action: impl FnMut(Self));
}

#[const_trait]
pub trait ConstDefault {
    fn const_default() -> Self;
}
