// An attribute to hide warnings for unused code.
#![allow(dead_code)]
use std::ops::Add;

#[derive(Debug)]
pub struct Value {
    data: f32,
    grad: f32,
    children: &[Value],
    op: String
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            data: self.data + other.data,
            grad: 0.0, 
            op: "+".to_string(),
        }
    }
}

fn main() {
    let v1 : Value = Value {data: 1.0, grad: 0.0, op: "".to_string()};
    println!("{:#?}", v1);
}

