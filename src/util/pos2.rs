use std::{
    fmt,
    ops::{Add, Sub},
};

use eframe::egui;

use super::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}
impl Pos2 {
    ///Creates a new instance of Pos2
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Creates a zero Pos2
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    /// calculate the distance between this and another
    pub fn distance_to(&self, _other: Self) -> Vec2 {
        todo!("This needs to be built")
    }
    /// linear intorpolation between this and another
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
    pub fn min(&self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    pub fn max(&self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl fmt::Display for Pos2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}
impl Sub<Vec2> for Pos2 {
    type Output = Pos2;

    fn sub(self, other: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub for Pos2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add<Vec2> for Pos2 {
    type Output = Pos2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<egui::Pos2> for Pos2 {
    fn from(v: egui::Pos2) -> Self {
        Pos2 { x: v.x, y: v.y }
    }
}
impl From<Pos2> for egui::Pos2 {
    fn from(v: Pos2) -> Self {
        egui::Pos2 { x: v.x, y: v.y }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let v = Pos2::new(4.0, 5.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 5.0);
    }
    #[test]
    fn is_copy() {
        let a = Pos2::new(0.0, 0.0);
        let b = a;
        assert_eq!(a.x, b.x);
    }

    #[test]
    fn minus() {
        let a = Pos2::new(0.0, 0.0);
        let b = Pos2::new(1.0, 2.0);
        assert_eq!(Vec2::new(-1.0, -2.0), a - b)
    }
    #[test]
    fn test_to_egui_pos2() {
        let a = Pos2::new(3.0, 4.0);
        let e_pos2: egui::Pos2 = a.into();
        assert_eq!(a.x, e_pos2.x);
        assert_eq!(a.y, e_pos2.y);
    }
    #[test]
    fn test_from_egui_pos2() {
        let e_pos2 = egui::Pos2::new(2.2, 3.2);
        let a: Pos2 = e_pos2.into();
        assert_eq!(a.x, e_pos2.x);
        assert_eq!(a.y, e_pos2.y);
    }
}
