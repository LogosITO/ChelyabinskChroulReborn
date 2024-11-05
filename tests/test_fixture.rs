pub(crate) use CCR::utils::fixture;
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
    const CF: fixture::CircleFixture = fixture::CircleFixture {
        _center: Point2 {x: 1f32, y: 1f32},
        _radius: 3f32
    };
    const CS: fixture::CircleFixture = fixture::CircleFixture {
        _center: Point2 {x: -3f32, y: -3f32},
        _radius: 2f32
    };

    #[test]
    pub(crate) fn pointvs_aabb() {
        assert_eq!(fixture::pointvs_aabb(PF, FF), true);
        assert_eq!(fixture::pointvs_aabb(PF, FS), true);
    }

    #[test]
    pub(crate) fn pointvs_circle() {
        assert_eq!(fixture::pointvs_circle(PF, CF), true);
        assert_eq!(fixture::pointvs_circle(PF, CS), false);
    }

    #[test]
    pub(crate) fn aabbvs_aabb() {
        assert_eq!(fixture::aabbvs_aabb(FF, FS), false);
    }

    #[test]
    pub(crate) fn circlevs_circle() {
        assert_eq!(fixture::circlevs_circle(CF, CS), true);
    }

    #[test]
    pub(crate) fn aabbvs_circle() {
        assert_eq!(fixture::aabbvs_circle(FF, CF), true);
        assert_eq!(fixture::aabbvs_circle(FS, CF), true);
        assert_eq!(fixture::aabbvs_circle(FF, CS), false);
        assert_eq!(fixture::aabbvs_circle(FS, CS), true);
    }

}