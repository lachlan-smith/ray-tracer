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

impl std::ops::Add<Point> for Vector {
    type Output = Vector;

    fn add(self, rhs: Point) -> Self::Output {
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
    fn new_vector() {
        let vec = Vector::new(4.3, -4.2, 3.1);

        assert_relative_eq!(vec.x(), 4.3);
        assert_relative_eq!(vec.y(), -4.2);
        assert_relative_eq!(vec.z(), 3.1);
        assert_relative_eq!(vec.w(), 1.0)
    }

    #[test]
    fn add_vector() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);

        let result = vector1 + vector2;
        let expected = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }

    #[test]
    fn add_point() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        let point = Point::new(-2.0, 3.0, 1.0);

        let result = vector + point;
        let expected = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected)
    }
}
