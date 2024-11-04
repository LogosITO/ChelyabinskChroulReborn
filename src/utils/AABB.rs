mod coords;

struct AABB {
    min: coords::Vec2f32,
    max: coords::Vec2f32,
}

impl AABB {
    fn create_AABB(first: coords::Vec2f32, second: coords::Vec2f32) -> AABB {
        if first._x == second._x || first_.y == second._y {
            panic!("Arguments are invalid.");
        }
        else if first._x < second._x && first_.y < second._y {
            return AABB {min: first, max: second, };
        } else {
            return AABB {min: second, max: first};
        }
    }

    fn is_intersecting(&self, other: AABB) -> bool {
        if self.min._x < other.max._x && self.max._x > self.min._x &&
            self.min._y < other.max._y && self.max._y > other.min._y {
            return true;
        }
        return false;
    }
}