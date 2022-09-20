use std::ops::Add;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod engine {
pub struct Value {
    data: f32,
    grad: f32,
    op: String
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            data: self.data + other.data,
            grad: self.grad,
            op: self.op
        }
    }
}

}
