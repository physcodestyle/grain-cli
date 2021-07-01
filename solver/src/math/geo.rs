use super::vector::{Vec2d, Vec3d};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Point<T> {
    origin: T,
}

impl<T> Point<T> {
    pub fn new(origin: T) -> Self {
        Self { origin: origin }
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.origin.to_string())
    }
}

pub trait Plain {
    fn is_inside(&self, coords: &Point<Vec2d>, accuracy: &f64) -> bool;
    fn square(&self) -> f64;
}

pub trait Space {
    fn is_inside(&self, coords: &Point<Vec3d>, accuracy: &f64) -> bool;
    fn normal(&self) -> Vec3d;
    fn square(&self) -> f64;
}

pub struct Triangle<T> {
    points: (Point<T>, Point<T>, Point<T>),
}

impl<T> fmt::Display for Triangle<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}",
            self.points.0.to_string(),
            self.points.1.to_string(),
            self.points.2.to_string()
        )
    }
}

impl<T> Triangle<T> {
    pub fn new(points: (Point<T>, Point<T>, Point<T>)) -> Self {
        Self { points: points }
    }
}

impl Plain for Triangle<Vec2d> {
    fn is_inside(&self, coords: &Point<Vec2d>, accuracy: &f64) -> bool {
        let s = self.square();
        let s1 = Triangle::new((*coords, self.points.0, self.points.1));
        let s2 = Triangle::new((*coords, self.points.1, self.points.2));
        let s3 = Triangle::new((*coords, self.points.2, self.points.0));
        (s - s1.square() - s2.square() - s3.square()).abs() < *accuracy
    }

    fn square(&self) -> f64 {
        let a = self.points.0.origin.sub(&self.points.1.origin);
        let b = self.points.1.origin.sub(&self.points.2.origin);
        let c = self.points.2.origin.sub(&self.points.0.origin);
        let p: f64 = (a.length() + b.length() + c.length()) / 2.0f64;
        (p * (p - a.length()) * (p - b.length()) * (p - c.length())).sqrt()
    }
}

impl Space for Triangle<Vec3d> {
    fn is_inside(&self, coords: &Point<Vec3d>, accuracy: &f64) -> bool {
        let s = self.square();
        let s1 = Triangle::new((*coords, self.points.0, self.points.1));
        let s2 = Triangle::new((*coords, self.points.1, self.points.2));
        let s3 = Triangle::new((*coords, self.points.2, self.points.0));
        (s - s1.square() - s2.square() - s3.square()).abs() < *accuracy
    }

    fn normal(&self) -> Vec3d {
        let a = Vec3d::new(
            self.points.1.origin.x - self.points.0.origin.x,
            self.points.1.origin.y - self.points.0.origin.y,
            self.points.1.origin.z - self.points.0.origin.z,
        );
        let b = Vec3d::new(
            self.points.2.origin.x - self.points.0.origin.x,
            self.points.2.origin.y - self.points.0.origin.y,
            self.points.2.origin.z - self.points.0.origin.z,
        );
        let n = a.vep(&b);
        n.mul(&(1.0 / n.length()))
    }

    fn square(&self) -> f64 {
        let a = self.points.0.origin.sub(&self.points.1.origin);
        let b = self.points.1.origin.sub(&self.points.2.origin);
        let c = self.points.2.origin.sub(&self.points.0.origin);
        let p: f64 = (a.length() + b.length() + c.length()) / 2.0f64;
        (p * (p - a.length()) * (p - b.length()) * (p - c.length())).sqrt()
    }
}
