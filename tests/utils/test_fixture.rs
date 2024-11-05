pub(crate) use src::utils::fixture;
pub(crate) use ggez::mint::Point2;

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) use super::*;

    const PF: Point2<f32> = Point2 {x: 0f32, y: 0f32};
    const FF: fixture::AABB = fixture::AABB {
        _min: Point2 {x: -1f32, y: 0f32},
        _max: Point2 {x: 1f32, y: 1f32}
    };
    const FS: fixture::AABB = fixture::AABB {
        _min: Point2 {x: -5f32, y: -2f32},
        _max: Point2 {x: 3f32, y: 5f32}
    };
    const CF: fixture::Circle = fixture::Circle {
        _center: Point2 {x: 1f32, y: 1f32},
        _radius: 3f32
    };
    const CS: fixture::Circle = fixture::Circle {
        _center: Point2 {x: -3f32, y: -3f32},
        _radius: 2f32
    };

    #[test]
    pub(crate) fn PointvsAABB() {
        assert_eq!(fixture::PointvsAABB(PF, FF), true);
        assert_eq!(fixture::PointvsAABB(PF, FS), true);
    }

    #[test]
    pub(crate) fn PointvsCircle() {
        assert_eq!(fixture::PointvsCircle(PF, CF), true);
        assert_eq!(fixture::PointvsCircle(PF, CS), false);
    }

    #[test]
    pub(crate) fn AABBvsAABB() {
        assert_eq!(fixture::AABBvsAABB(FF, FS), true);
    }

    #[test]
    pub(crate) fn CirclevsCircle() {
        assert_eq!(fixture::CirclevsCircle(CF, CS). true);
    }

    #[test]
    pub(crate) fn AABBvsCircle() {
        assert_eq!(fixture::AABBvsCircle(FF, CF), true);
        assert_eq!(fixture::AABBvsCircle(FS, CF), true);
        assert_eq!(fixture::AABBvsCircle(FF, CS), false);
        assert_eq!(fixture::AABBvsCircle(FS, CS), false);
    }

}