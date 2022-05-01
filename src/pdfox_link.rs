use crate::pdfox_object::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Link<T>{
    Value(T),
    Link(String)
}

