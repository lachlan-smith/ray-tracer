use crate::{point::Point, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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
        1.0
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.x(), other.x())
            && relative_eq!(self.y(), other.y())
            && relative_eq!(self.z(), other.z())
            && relative_eq!(self.w(), other.w())
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x(),
            y: -self.y(),
            z: -self.z(),
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

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
    fn new_vector() {
        let vec = Vector::new(4.3, -4.2, 3.1);

        assert_relative_eq!(vec.x(), 4.3);
        assert_relative_eq!(vec.y(), -4.2);
        assert_relative_eq!(vec.z(), 3.1);
        assert_relative_eq!(vec.w(), 1.0)
    }

    #[test]
    fn add_vector_and_vector() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);

        let result = vector1 + vector2;
        let expected = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn sub_vector_from_vector() {
        let vector1 = Vector::new(3.0, 2.0, 1.0);
        let vector2 = Vector::new(5.0, 6.0, 7.0);

        let result = vector1 - vector2;
        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn sub_point_from_point() {
        let point1 = Point::new(3.0, 2.0, 1.0);
        let point2 = Point::new(5.0, 6.0, 7.0);

        let result = point1 - point2;
        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn negate_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let expected = Vector::new(-1.0, 2.0, -3.0);

        assert_eq!(-vector, expected)
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector * 3.5;
        let expected = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn multiply_vector_by_fraction() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector * 0.5;
        let expected = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn divide_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector / 2.0;
        let expected = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(result, expected)
    }

    #[test]
    fn divide_vector_by_fraction() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector / 0.5;
        let expected = Vector::new(2.0, -4.0, 6.0);

        assert_eq!(result, expected)
    }
}
