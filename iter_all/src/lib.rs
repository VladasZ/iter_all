pub trait IterAll: Sized {
    fn iter_all(action: impl FnMut(Self));
}
