// Import necessary traits for arithmetic operations: Add, Sub, Mul, and Div
use std::ops::{Add, Div, Mul, Sub};

// Define a trait `Scalar` that requires types to implement basic arithmetic traits,
// be Sized (known at compile-time), and Clone.
pub trait Scalar: Add + Div + Mul + Sub + std::marker::Sized + Clone {
    // Associated type `Item` that represents the scalar's value type
    type Item;

    // Function to return the zero value for the scalar type
    fn zero() -> Self::Item;

    // Function to return the one value for the scalar type
    fn one() -> Self::Item;
}

// Implement `Scalar` trait for `u32` type
impl Scalar for u32 {
    type Item = u32;

    // Return zero as a `u32`
    fn zero() -> Self::Item {
        0 as u32
    }

    // Return one as a `u32`
    fn one() -> Self::Item {
        1 as u32
    }
}

// Implement `Scalar` trait for `u64` type
impl Scalar for u64 {
    type Item = u64;

    // Return zero as a `u64`
    fn zero() -> Self::Item {
        0 as u64
    }

    // Return one as a `u64`
    fn one() -> Self::Item {
        1 as u64
    }
}

// Implement `Scalar` trait for `i32` type
impl Scalar for i32 {
    type Item = i32;

    // Return zero as an `i32`
    fn zero() -> Self::Item {
        0 as i32
    }

    // Return one as an `i32`
    fn one() -> Self::Item {
        1 as i32
    }
}

// Implement `Scalar` trait for `i64` type
impl Scalar for i64 {
    type Item = i64;

    // Return zero as an `i64`
    fn zero() -> Self::Item {
        0 as i64
    }

    // Return one as an `i64`
    fn one() -> Self::Item {
        1 as i64
    }
}

// Implement `Scalar` trait for `f32` type
impl Scalar for f32 {
    type Item = f32;

    // Return zero as an `f32`
    fn zero() -> Self::Item {
        0.0
    }

    // Return one as an `f32`
    fn one() -> Self::Item {
        1.0
    }
}

// Implement `Scalar` trait for `f64` type
impl Scalar for f64 {
    type Item = f64;

    // Return zero as an `f64`
    fn zero() -> Self::Item {
        0.0
    }

    // Return one as an `f64`
    fn one() -> Self::Item {
        1.0
    }
}

