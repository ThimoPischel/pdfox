#[derive(Debug)]
pub enum Link<T>{
    Value(T),
    Link(String)
}


