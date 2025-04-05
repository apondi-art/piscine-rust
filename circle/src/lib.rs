#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= (self.radius + other.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.0 - other.0).powi(2) + ((self.1 - other.1).powi(2)).sqrt())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
