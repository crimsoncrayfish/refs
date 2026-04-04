use eframe::egui;

use crate::util::pos2::Pos2;

use super::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub pos: Pos2,
    pub size: Vec2,
}
impl Rect {
    pub fn new(pos: Pos2, size: Vec2) -> Self {
        Self { pos, size }
    }
    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        assert!(w > 0.0);
        assert!(h > 0.0);
        Self {
            pos: Pos2::new(x, y),
            size: Vec2::new(w, h),
        }
    }
    pub fn from_min_max(min: Pos2, max: Pos2) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        Self {
            pos: min,
            size: max - min,
        }
    }
    pub fn from_center_size(center: Pos2, size: Vec2) -> Self {
        Self {
            pos: center - size / 2.0,
            size,
        }
    }
    pub fn from_two_points(p1: Pos2, p2: Pos2) -> Self {
        let min = p1.min(p2);
        let max = p1.max(p2);
        Self::from_min_max(min, max)
    }
    pub fn zero() -> Self {
        Self {
            pos: Pos2::zero(),
            size: Vec2::zero(),
        }
    }
    pub fn x(&self) -> f32 {
        self.pos.x
    }
    pub fn y(&self) -> f32 {
        self.pos.y
    }
    pub fn left(&self) -> f32 {
        self.pos.x
    }
    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x
    }
    pub fn top(&self) -> f32 {
        self.pos.y
    }
    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.y
    }
    pub fn width(&self) -> f32 {
        self.size.x
    }
    pub fn height(&self) -> f32 {
        self.size.y
    }
    pub fn min(&self) -> Pos2 {
        self.pos
    }
    pub fn max(&self) -> Pos2 {
        self.pos + self.size
    }
    pub fn center(&self) -> Pos2 {
        self.pos + self.size * 0.5
    }
    pub fn top_right(&self) -> Pos2 {
        Pos2::new(self.right(), self.top())
    }
    pub fn top_left(&self) -> Pos2 {
        Pos2::new(self.left(), self.top())
    }
    pub fn bottom_right(&self) -> Pos2 {
        Pos2::new(self.right(), self.bottom())
    }
    pub fn bottom_left(&self) -> Pos2 {
        Pos2::new(self.left(), self.bottom())
    }

    pub fn contains(&self, point: Pos2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }
    pub fn intersects(&self, other: Rect) -> bool {
        //x overlap
        self.left() <= other.right()
            && self.right() >= other.left()
            //y overlap
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }
    pub fn intersection(&self, other: Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }
        Some(Self::from_min_max(
            self.min().max(other.min()),
            self.max().min(other.max()),
        ))
    }
    pub fn bounding(&self, other: Self) -> Self {
        Self::from_min_max(self.min().min(other.min()), self.max().max(other.max()))
    }
    pub fn expand(&self, margin: f32) -> Self {
        Self {
            pos: self.pos - Vec2::splat(margin),
            size: self.size + Vec2::splat(margin * 2.0),
        }
    }
    pub fn shrink(&self, margin: f32) -> Self {
        self.expand(-margin)
    }
    pub fn translate(&self, offset: Vec2) -> Self {
        Self {
            pos: self.pos + offset,
            size: self.size,
        }
    }
    pub fn scale_from_center_easy(&self, scale: f32) -> Self {
        debug_assert!(scale > 0.0);
        Self::from_center_size(self.center(), self.size * scale)
    }
    pub fn scale_from_center(&self, scale: f32) -> Self {
        assert!(scale > 0.0);
        let new_size = self.size * scale;

        Self {
            pos: self.pos - ((new_size - self.size) / 2.0),
            size: new_size,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.size.x >= 0.0 && self.size.y >= 0.0
    }
    pub fn area(&self) -> f32 {
        self.size.x * self.size.y
    }
    pub fn aspect_ratio(&self) -> f32 {
        if self.size.y == 0.0 {
            1.0
        } else {
            self.size.x / self.size.y
        }
    }
    /// Clamp a point to be within the rect
    pub fn clamp_point(&self, point: Pos2) -> Pos2 {
        Pos2 {
            x: point.x.clamp(self.left(), self.right()),
            y: point.y.clamp(self.top(), self.bottom()),
        }
    }
    pub fn closest_edge_point(&self, point: Pos2) -> Pos2 {
        let closest_point: Pos2;
        if self.contains(point) {
            let to_left = (point.x - self.left()).abs();
            let to_right = (point.x - self.right()).abs();
            let to_top = (point.y - self.top()).abs();
            let to_bottom = (point.y - self.bottom()).abs();
            let min = to_left.min(to_right).min(to_bottom).min(to_top);
            if min == to_left {
                closest_point = Pos2::new(self.left(), point.y);
            } else if min == to_right {
                closest_point = Pos2::new(self.right(), point.y);
            } else if min == to_top {
                closest_point = Pos2::new(point.x, self.top());
            } else {
                closest_point = Pos2::new(point.x, self.bottom());
            }
        } else {
            closest_point = self.clamp_point(point);
        }
        closest_point
    }
}

impl From<egui::Rect> for Rect {
    fn from(v: egui::Rect) -> Self {
        Self {
            pos: v.min.into(),
            size: Vec2::new(v.width(), v.height()),
        }
    }
}

impl From<Rect> for egui::Rect {
    fn from(v: Rect) -> Self {
        Self::from_min_size(v.pos.into(), v.size.into())
    }
}

#[cfg(test)]
mod test {
    use crate::util::pos2::Pos2;

    use super::{Rect, Vec2};

    #[test]
    fn test_new() {
        let r = Rect::new(Pos2::new(1.0, 2.0), Vec2::new(4.0, 5.0));
        assert_eq!(r.pos.x, 1.0);
        assert_eq!(r.pos.y, 2.0);
        assert_eq!(r.size.x, 4.0);
        assert_eq!(r.size.y, 5.0);
    }
    #[test]
    fn test_from_xywh() {
        let r = Rect::from_xywh(1.0, 2.0, 4.0, 5.0);
        assert_eq!(r.pos.x, 1.0);
        assert_eq!(r.pos.y, 2.0);
        assert_eq!(r.size.x, 4.0);
        assert_eq!(r.size.y, 5.0);
    }
    #[test]
    fn test_from_min_max() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(5.0, 7.0));
        assert_eq!(r.pos.x, 1.0);
        assert_eq!(r.pos.y, 2.0);
        assert_eq!(r.size.x, 4.0);
        assert_eq!(r.size.y, 5.0);
    }
    #[test]
    fn test_from_center_size() {
        let r = Rect::from_center_size(
            Pos2::new(1.0, 2.0).lerp(Pos2::new(5.0, 7.0), 0.5),
            Vec2::new(4.0, 5.0),
        );
        assert_eq!(r.pos.x, 1.0);
        assert_eq!(r.pos.y, 2.0);
        assert_eq!(r.size.x, 4.0);
        assert_eq!(r.size.y, 5.0);
    }
    #[test]
    fn test_from_two_points() {
        let r = Rect::from_two_points(Pos2::new(4.0, 5.0), Pos2::new(1.0, 2.0));
        assert_eq!(r.left(), 1.0);
        assert_eq!(r.top(), 2.0);
        assert_eq!(r.right(), 4.0);
        assert_eq!(r.bottom(), 5.0);
    }
    #[test]
    fn test_zero() {
        let r = Rect::zero();
        assert_eq!(r.pos.x, 0.0);
        assert_eq!(r.pos.y, 0.0);
        assert_eq!(r.size.x, 0.0);
        assert_eq!(r.size.y, 0.0);
    }
    #[test]
    fn test_xy_left_right_top_bottom() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(1.0, r.x());
        assert_eq!(2.0, r.y());
        assert_eq!(1.0, r.left());
        assert_eq!(2.0, r.top());
        assert_eq!(4.0, r.right());
        assert_eq!(5.0, r.bottom());
    }
    #[test]
    fn test_vec2s() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(Pos2::new(1.0, 2.0), r.min());
        assert_eq!(Pos2::new(4.0, 5.0), r.max());
        assert_eq!(Pos2::new(2.5, 3.5), r.center());
        assert_eq!(Pos2::new(4.0, 2.0), r.top_right());
        assert_eq!(Pos2::new(1.0, 2.0), r.top_left());
        assert_eq!(Pos2::new(1.0, 5.0), r.bottom_left());
        assert_eq!(Pos2::new(4.0, 5.0), r.bottom_right());
    }
    #[test]
    fn test_contains() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert!(r.contains(Pos2::new(3.0, 3.0)));
        assert!(!r.contains(Pos2::new(9.0, 0.0)));
        assert!(!r.contains(Pos2::new(9.0, 9.0)));
        assert!(!r.contains(Pos2::new(0.0, 0.0)));
    }
    #[test]
    fn test_intersects() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        let r_inside = Rect::from_min_max(Pos2::new(2.0, 3.0), Pos2::new(5.0, 6.0));
        let r_not = Rect::from_min_max(Pos2::new(10.0, 20.0), Pos2::new(40.0, 50.0));
        let r_touch = Rect::from_min_max(Pos2::new(4.0, 5.0), Pos2::new(40.0, 50.0));
        assert!(r.intersects(r_inside));
        assert!(r.intersects(r_touch));
        assert!(!r.intersects(r_not));
    }
    #[test]
    fn test_intersection() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        let r_inside = Rect::from_min_max(Pos2::new(2.0, 3.0), Pos2::new(5.0, 6.0));
        let r_not = Rect::from_min_max(Pos2::new(10.0, 20.0), Pos2::new(40.0, 50.0));
        let r_touch = Rect::from_min_max(Pos2::new(4.0, 5.0), Pos2::new(40.0, 50.0));
        assert_eq!(
            Some(Rect::from_min_max(Pos2::new(2.0, 3.0), Pos2::new(4.0, 5.0))),
            r.intersection(r_inside)
        );
        assert_eq!(
            Some(Rect::from_center_size(Pos2::new(4.0, 5.0), Vec2::zero())),
            r.intersection(r_touch)
        );
        assert_eq!(None, r.intersection(r_not));
    }
    #[test]
    fn test_bounding() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        let r2 = Rect::from_min_max(Pos2::new(10.0, 20.0), Pos2::new(40.0, 50.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(40.0, 50.0)),
            r.bounding(r2)
        );
        let r3 = Rect::from_min_max(Pos2::new(2.0, 3.0), Pos2::new(6.0, 7.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(6.0, 7.0)),
            r.bounding(r3)
        );
    }
    #[test]
    fn test_expand() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(0.0, 1.0), Pos2::new(5.0, 6.0)),
            r.expand(1.0)
        );
    }
    #[test]
    fn test_shrink() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(2.0, 3.0), Pos2::new(3.0, 4.0)),
            r.shrink(1.0)
        );
    }
    #[test]
    fn test_translate() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(2.0, 4.0), Pos2::new(5.0, 7.0)),
            r.translate(Vec2::new(1.0, 2.0))
        );
    }
    #[test]
    fn test_scale_from_center() {
        let r0 = Rect::from_center_size(Pos2::new(1.0, 1.0), Vec2::new(4.0, 5.0));
        assert_eq!(
            Rect::from_center_size(Pos2::new(1.0, 1.0), Vec2::new(8.0, 10.0)),
            r0.scale_from_center(2.0)
        );

        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(
            Rect::from_min_max(Pos2::new(-0.5, 0.5), Pos2::new(5.5, 6.5)),
            r.scale_from_center(2.0)
        );
        assert_eq!(
            Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0)),
            r.scale_from_center(1.0)
        );
        assert_eq!(
            Rect::from_min_max(Pos2::new(1.75, 2.75), Pos2::new(3.25, 4.25)),
            r.scale_from_center(0.5)
        );
    }
    #[test]
    #[should_panic(expected = "assertion failed: scale > 0.0")]
    fn test_scale_from_center_negative_panics() {
        let rect = Rect::from_center_size(Pos2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        rect.scale_from_center(-1.0);
    }
    #[test]
    fn test_is_valid() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert!(r.is_valid());
        let r2 = Rect::new(Pos2::new(1.0, 2.0), Vec2::new(-4.0, -5.0));
        assert!(!r2.is_valid())
    }
    #[test]
    fn test_area() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(9.0, r.area());
    }
    #[test]
    fn test_aspect_ratio() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(1.0, r.aspect_ratio());
        let r = Rect::from_min_max(Pos2::new(1.0, 4.0), Pos2::new(4.0, 5.0));
        assert_eq!(3.0, r.aspect_ratio());
        let r = Rect::from_min_max(Pos2::new(4.0, 2.0), Pos2::new(5.0, 4.0));
        assert_eq!(0.5, r.aspect_ratio());
    }
    #[test]
    fn test_clamp_point() {
        //0 1 2 3 4 5 6
        //1   .
        //2 x x x x
        //3.x     x  .
        //4 x .   x
        //5 x x x x .
        //6   .
        //
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(Pos2::new(2.0, 3.0), r.clamp_point(Pos2::new(2.0, 3.0)));
        assert_eq!(Pos2::new(1.0, 2.0), r.clamp_point(Pos2::new(0.0, 0.0)));
        assert_eq!(Pos2::new(1.0, 2.0), r.clamp_point(Pos2::new(0.0, 1.0)));
        assert_eq!(Pos2::new(1.0, 2.0), r.clamp_point(Pos2::new(1.0, 1.0)));
        assert_eq!(Pos2::new(2.5, 2.0), r.clamp_point(Pos2::new(2.5, 1.0)),);
        assert_eq!(Pos2::new(4.0, 2.0), r.clamp_point(Pos2::new(5.0, 1.0)),);
        assert_eq!(Pos2::new(1.0, 3.5), r.clamp_point(Pos2::new(0.0, 3.5)),);
        assert_eq!(Pos2::new(4.0, 3.5), r.clamp_point(Pos2::new(5.0, 3.5)),);
        assert_eq!(Pos2::new(1.0, 5.0), r.clamp_point(Pos2::new(0.0, 6.0)),);
        assert_eq!(Pos2::new(2.5, 5.0), r.clamp_point(Pos2::new(2.5, 6.0)),);
        assert_eq!(Pos2::new(4.0, 5.0), r.clamp_point(Pos2::new(5.0, 6.0)),);
    }
    #[test]
    fn test_closest_edge_point() {
        let r = Rect::from_min_max(Pos2::new(1.0, 2.0), Pos2::new(4.0, 5.0));
        assert_eq!(
            Pos2::new(1.0, 3.0),
            r.closest_edge_point(Pos2::new(2.0, 3.0))
        );
        assert_eq!(
            Pos2::new(1.0, 2.0),
            r.closest_edge_point(Pos2::new(0.0, 0.0))
        );
        assert_eq!(
            Pos2::new(1.0, 2.0),
            r.closest_edge_point(Pos2::new(0.0, 1.0))
        );
        assert_eq!(
            Pos2::new(1.0, 2.0),
            r.closest_edge_point(Pos2::new(1.0, 1.0))
        );
        assert_eq!(
            Pos2::new(2.5, 2.0),
            r.closest_edge_point(Pos2::new(2.5, 1.0)),
        );
        assert_eq!(
            Pos2::new(4.0, 2.0),
            r.closest_edge_point(Pos2::new(5.0, 1.0)),
        );
        assert_eq!(
            Pos2::new(1.0, 3.5),
            r.closest_edge_point(Pos2::new(0.0, 3.5)),
        );
        assert_eq!(
            Pos2::new(4.0, 3.5),
            r.closest_edge_point(Pos2::new(5.0, 3.5)),
        );
        assert_eq!(
            Pos2::new(1.0, 5.0),
            r.closest_edge_point(Pos2::new(0.0, 6.0)),
        );
        assert_eq!(
            Pos2::new(2.5, 5.0),
            r.closest_edge_point(Pos2::new(2.5, 6.0)),
        );
        assert_eq!(
            Pos2::new(4.0, 5.0),
            r.closest_edge_point(Pos2::new(5.0, 6.0)),
        );
    }
}
