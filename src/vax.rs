use num::Num;
use std::fmt::Display;
use std::fmt::Formatter;

pub trait PrimaryOperationsTrait<V, U> {
    fn plus(&self, other: V) -> Variable<U> ;

    fn minus(&self, other: V) -> Variable<U> ;

    fn multiply_by(&self, other: V) -> Variable<U> ;

    fn divide_in(&self, other: V) -> Variable<U> ;
}

#[derive(Clone)]
pub struct Variable<T> {
    value: T
}

impl<T> Variable<T> {
    pub fn new(value: T) -> Self { Variable {value} }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> PrimaryOperationsTrait<T, T> for T where T:Num + Copy {
    fn plus(&self, other: T) -> Variable<T> {
        Variable::new(*self + other)
    }

    fn minus(&self, other: T) -> Variable<T> {

        Variable::new(*self - other)
    }

    fn multiply_by(&self, other: T) -> Variable<T> {

        Variable::new(*self * other)
    }

    fn divide_in(&self, other: T) -> Variable<T> {

        Variable::new(*self / other)
    }

}

impl<T> PrimaryOperationsTrait<&Variable<T>, T> for T where T:Num + Copy {
    fn plus(&self, other: &Variable<T>) -> Variable<T> {
        other.plus(*self)
    }

    fn minus(&self, other: &Variable<T>) -> Variable<T> {
        other.minus(*self)
    }

    fn multiply_by(&self, other: &Variable<T>) -> Variable<T> {
        other.multiply_by(*self)
    }

    fn divide_in(&self, other: &Variable<T>) -> Variable<T> {
        other.divide_in(*self)
    }
}

impl<T> PrimaryOperationsTrait<Variable<T>, T> for T where T:Num + Copy {
    fn plus(&self, other: Variable<T>) -> Variable<T> {
        other.value().plus(*self)
    }

    fn minus(&self, other: Variable<T>) -> Variable<T> {
        other.value().minus(*self)
    }

    fn multiply_by(&self, other: Variable<T>) -> Variable<T> {
        other.value().multiply_by(*self)
    }

    fn divide_in(&self, other: Variable<T>) -> Variable<T> {
        other.value().divide_in(*self)
    }
}

impl<T> Variable<T> where T:PrimaryOperationsTrait<T, T> {

    pub fn plus(&self, other: T) -> Variable<T> { self.value.plus(other) }
    pub fn minus(&self, other: T) -> Variable<T> { self.value.minus(other) }
    pub fn divide_in(&self, other: T) -> Variable<T> { self.value.divide_in(other) }
    pub fn multiply_by(&self, other: T) -> Variable<T> { self.value.multiply_by(other) }

}

impl<T> Display for Variable<T> where T:Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.value())
    }
}

