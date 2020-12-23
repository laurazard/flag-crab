pub trait UseCase<T> {
    fn invoke(&self) -> &T;
}