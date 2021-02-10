use cg_practicum::math::Vector;

fn main() {
    let a = Vector::new(1, 2, 3);
    let b = Vector::new(2, 3, 4);

    let c = &a + &b;
    println!("{:?}", c);
}
