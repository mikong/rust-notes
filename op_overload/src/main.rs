use std::ops::Add;

#[derive(Copy, Clone)]
struct Zero;

impl<T: Add> Add<T> for Zero {
    type Output = T;
    fn add(self, rhs: T) -> Self::Output {
        rhs
    }
}

fn main() {
    let a = Zero + 5;
    println!("Zero + 5 = {}", a);
}
