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

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x() / rhs,
            y: self.y() / rhs,
            z: self.z() / rhs,
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
    fn add_point_and_vector() {
        let point = Point::new(-2.0, 3.0, 1.0);
        let vector = Vector::new(3.0, -2.0, 5.0);

        let result = point + vector;
        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn add_vector_and_point() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        let point = Point::new(-2.0, 3.0, 1.0);

        let result = vector + point;
        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn sub_vector_from_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);

        let result = point - vector;
        let expected = Point::new(-2.0, -4.0, -6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn multiply_point_by_scalar() {
        let point = Point::new(1.0, -2.0, 3.0);

        let result = point * 3.5;
        let expected = Point::new(3.5, -7.0, 10.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn multiply_point_by_fraction() {
        let point = Point::new(1.0, -2.0, 3.0);

        let result = point * 0.5;
        let expected = Point::new(0.5, -1.0, 1.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn divide_point_by_scalar() {
        let point = Point::new(1.0, -2.0, 3.0);

        let result = point / 2.0;
        let expected = Point::new(0.5, -1.0, 1.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn divide_point_by_fraction() {
        let point = Point::new(1.0, -2.0, 3.0);

        let result = point / 0.5;
        let expected = Point::new(2.0, -4.0, 6.0);

        assert_eq!(result, expected)
    }
}
