use std::ops::Mul;

#[derive(Clone)]
pub struct Rgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
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

impl<T: Mul<T, Output = T>> Mul<Rgb<T>> for Rgb<T> {
    type Output = Rgb<T>;

    fn mul(self, rhs: Rgb<T>) -> Rgb<T> {
        Rgb::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
            )
    }
}
