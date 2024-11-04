use std::ops;

pub(crate) struct Vec2f32 {
    pub(crate) _x: f32,
    pub(crate) _y: f32,
}

impl ops::Add<Vec2f32> for Vec2f32 {
    type Output = Vec2f32;
    fn add(self, rhs: Vec2f32) -> Vec2f32 {
        Vec2f32 {
            _x: self._x + rhs._x,
            _y: self._y + rhs._y,
        }
    }
}

impl ops::AddAssign<Vec2f32> for Vec2f32 {
    fn add_assign(&mut self, rhs: Vec2f32) {
        *self {
            _x: self._x + rhs._x,
            _y: self._y + rhs._y,
        }
    }
}

impl ops::Sub<Vec2f32> for Vec2f32 {
    type Output = Vec2f32;
    fn sub(self, rhs: Vec2f32) -> Vec2f32 {
        Vec2f32 {
            _x: self._x - rhs._x,
            _y: self._y - rhs._y,
        }
    }
}