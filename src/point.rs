use crate::{tuple::Tuple, vector::Vector};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.x(), other.x())
            && relative_eq!(self.y(), other.y())
            && relative_eq!(self.z(), other.z())
            && relative_eq!(self.w(), other.w())
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point() {
        let point = Point::new(4.3, -4.2, 3.1);

        assert_relative_eq!(point.x(), 4.3);
        assert_relative_eq!(point.y(), -4.2);
        assert_relative_eq!(point.z(), 3.1);
        assert_relative_eq!(point.w(), 0.0)
    }

    #[test]
    fn add_point() {
        let point1 = Point::new(-2.0, 3.0, 1.0);
        let point2 = Point::new(3.0, -2.0, 5.0);

        let result = point1 + point2;
        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn add_vector() {
        let point = Point::new(-2.0, 3.0, 1.0);
        let vector = Vector::new(3.0, -2.0, 5.0);

        let result = point + vector;
        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }
}
