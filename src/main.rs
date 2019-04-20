extern crate cgmath;
mod render;

use cgmath::Vector3;


fn main() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    println!("{}", v.x);
}
