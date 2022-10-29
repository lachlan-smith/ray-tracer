#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Colour { r, g, b }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.r, other.r)
            && relative_eq!(self.g, other.g)
            && relative_eq!(self.b, other.b)
    }
}

impl std::ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl std::ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_colour() {
        let colour = Colour::new(-0.5, 0.4, 1.7);

        assert_relative_eq!(colour.r, -0.5);
        assert_relative_eq!(colour.g, 0.4);
        assert_relative_eq!(colour.b, 1.7);
    }

    #[test]
    fn colours_equality() {
        let colour1 = Colour::new(-0.5, 0.4, 1.7);
        let colour2 = Colour::new(-0.5, 0.4, 1.7);

        assert_eq!(colour1, colour2)
    }

    #[test]
    fn add_colours() {
        let colour1 = Colour::new(0.9, 0.6, 0.75);
        let colour2 = Colour::new(0.7, 0.1, 0.25);

        let result = colour1 + colour2;
        let expected = Colour::new(1.6, 0.7, 1.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn sub_colours() {
        let colour1 = Colour::new(0.9, 0.6, 0.75);
        let colour2 = Colour::new(0.7, 0.1, 0.25);

        let result = colour1 - colour2;
        let expected = Colour::new(0.2, 0.5, 0.5);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_colours() {
        let colour1 = Colour::new(1.0, 0.2, 0.4);
        let colour2 = Colour::new(0.9, 1.0, 0.1);

        let result = colour1 * colour2;
        let expected = Colour::new(0.9, 0.2, 0.04);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_colour_by_scalar() {
        let colour = Colour::new(0.2, 0.3, 0.4);

        let result = colour * 2.0;
        let expected = Colour::new(0.4, 0.6, 0.8);

        assert_eq!(result, expected);
    }
}
