// An attribute to hide warnings for unused code.
#![allow(dead_code)]
use std::ops::Add;

#[derive(Debug)]
pub struct Value {
    data: f32,
    grad: f32,
    backward: FnMut() - > (),
    children: Vec<Value>,
    op: String
}

impl Value {
    fn new(data: f32, children: Option<Vec<Value>>,  op: Option<String>) -> Self {
        Self {
            data: data, 
            grad: 0.0,
            backward: ||,
            children.unwrap_or(vec![]),
            op.unwrap_or("".to_string())
        }
        
    }
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
    // let v1 : Value = Value {data: 1.0, grad: 0.0, op: "".to_string()};
    //println!("{:#?}", v1);
    let v1 = Value::new(5 None, None);
}

