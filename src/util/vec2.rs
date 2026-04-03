use std::{
    fmt::{self, write},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    ///Creates a new instance of Vec2
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    ///Creates a new instance of Vec2
    pub fn splat(r: f32) -> Self {
        Self { x: r, y: r }
    }
    /// Creates a zero Vec2
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    /// Length form origin
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// new copy of the vec2 thats normalized
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
    /// dot product of this and another
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    /// calculate the distance between this and another
    pub fn distance_to(&self, other: Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
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
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, s: f32) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, s: f32) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
    }
}

impl From<egui::Vec2> for Vec2 {
    fn from(v: egui::Vec2) -> Self {
        Vec2 { x: v.x, y: v.y }
    }
}
impl From<Vec2> for egui::Vec2 {
    fn from(v: Vec2) -> Self {
        egui::Vec2 { x: v.x, y: v.y }
    }
}

impl From<egui::Pos2> for Vec2 {
    fn from(p: egui::Pos2) -> Self {
        Vec2 { x: p.x, y: p.y }
    }
}

impl From<Vec2> for egui::Pos2 {
    fn from(v: Vec2) -> Self {
        egui::Pos2::new(v.x, v.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec2::new(4.0, 5.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 5.0);
    }
    #[test]
    fn test_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0)
    }
    #[test]
    fn test_zero_lenght() {
        let v = Vec2::zero();
        assert_eq!(v.length(), 0.0)
    }
    #[test]
    fn test_distance() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.distance_to(b), 5.0);
    }
    #[test]
    fn is_copy() {
        let a = Vec2::new(0.0, 0.0);
        let b = a;
        assert_eq!(a.x, b.x);
    }
    #[test]
    fn test_add() {
        let a = Vec2::new(4.0, 8.0);
        let b = a;
        let added = a + b;
        assert_eq!(added.x, 8.0);
        assert_eq!(added.y, 16.0);
    }
    #[test]
    fn test_mul() {
        let a = Vec2::new(4.0, 8.0);
        let b = a;
        let multiplied = a * b;
        assert_eq!(multiplied.x, 16.0);
        assert_eq!(multiplied.y, 64.0);
    }
    #[test]
    fn test_mul_assign() {
        let a = Vec2::new(4.0, 8.0);
        let multiplied = a * 2.0;
        assert_eq!(multiplied.x, 8.0);
        assert_eq!(multiplied.y, 16.0);
    }
    #[test]
    fn test_neg() {
        let a = Vec2::new(4.0, 8.0);
        let b = -a;
        assert_eq!(b.x, -4.0);
        assert_eq!(b.y, -8.0);
    }
    #[test]
    fn test_div_assign() {
        let a = Vec2::new(4.0, 8.0);
        let multiplied = a / 2.0;
        assert_eq!(multiplied.x, 2.0);
        assert_eq!(multiplied.y, 4.0);
    }
    #[test]
    fn test_to_egui_vec2() {
        let a = Vec2::new(3.0, 4.0);
        let e_vec2: egui::Vec2 = a.into();
        assert_eq!(a.x, e_vec2.x);
        assert_eq!(a.y, e_vec2.y);
    }
    #[test]
    fn test_from_egui_vec2() {
        let e_vec2 = egui::Vec2::new(2.2, 3.2);
        let a: Vec2 = e_vec2.into();
        assert_eq!(a.x, e_vec2.x);
        assert_eq!(a.y, e_vec2.y);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    fn v2_strat() -> impl Strategy<Value = Vec2> {
        (-1000.0f32..1000.0, -1000.0f32..1000.0).prop_map(|(x, y)| Vec2::new(x, y))
    }
    proptest! {
        #[test]
        fn test_length_is_not_negative(a in v2_strat()) {
            assert!(a.length() >= 0.0);
        }
    }
}
