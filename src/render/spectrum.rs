use std::ops::Mul;

#[derive(Clone)]
pub struct Spec<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

impl<T> Spec<T> {
    pub fn new(red: T, green: T, blue: T) -> Spec<T> {
        Spec { red, green, blue }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Spec<T> {
    type Output = Spec<T>;

    fn mul(self, rhs: T) -> Spec<T> {
        Spec::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl<T: Mul<T, Output = T>> Mul<Spec<T>> for Spec<T> {
    type Output = Spec<T>;

    fn mul(self, rhs: Spec<T>) -> Spec<T> {
        Spec::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}
