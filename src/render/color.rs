extern crate cgmath;

use std::ops::Mul;

#[derive(Clone)]
pub struct Rgb<T> {
    red: T,
    green: T,
    blue: T,
}

impl<T> Rgb<T> {
    pub fn new(red: T, green: T, blue: T) -> Rgb<T> {
        Rgb { red, green, blue }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Rgb<T> {
    type Output = Rgb<T>;

    fn mul(self, rhs: T) -> Rgb<T> {
        Rgb::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}
