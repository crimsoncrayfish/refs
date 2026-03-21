pub mod util;

pub use util::{Rect, Vec2};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_util() {
        let v = util::Vec2::zero();
        assert!(v.length() == 0.0)
    }
}
