mod coords;

pub(crate) struct AABB {
    pub(crate) _min: coords::Vec2f32,
    pub(crate) _max: coords::Vec2f32,
}

impl AABB {
    fn create(first: coords::Vec2f32, second: coords::Vec2f32) -> AABB {
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

    fn create_fixtures_from(points: Vec<coords::Vec2f32>) -> (AABB, AABB) {
        if points.len() < 5 {
            panic!("Arguments are invalid.");
        }
        let mut min: Vec2f32 = Vec2f32{_x: f32::MAX, _y: f32::MAX};
        let mut pred_min: Vec2f32 = Vec2f32{_x: f32::MIN + 1, _y: f32::MIN + 1};
        let mut max: Vec2f32 = Vec2f32{_x: f32::MIN, _y: f32::MIN};
        let mut pred_max: Vec2f32 = Vec2f32{_x: f32::MAX - 1, _y: f32::MAX - 1};
        for (x, y) in points {
            if min.x >= x {
                pred_min.x = min.x;
                min.x = x;
            }
            if max.x <= x {
                pred_max.x = max.x;
                max.x = x;
            }
            if min.y >= y {
                pred_min.y = min.y;
                min.y = y;
            }
            if max.y <= y {
                pred_max.y = max.y;
                max.y = y;
            }
        }
        return (AABB {_min: min, _max: max}, AABB {_min: pred_min, _max: pred_max});

    }
}