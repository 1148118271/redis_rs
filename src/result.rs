pub enum State<T> {
    OK(T),
    ERROR(String),
}