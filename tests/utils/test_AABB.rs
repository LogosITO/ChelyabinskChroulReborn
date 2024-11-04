mod src;

pub use src::utils::coords;

#[cfg(test)]
mod tests {
    use super::*;

    const first: AABB = AABB{ min: Vec2f32{ _x: 0f32, _y: 0f32 },
                        max: Vec2f32{ _x: 2f32, _y: 2f32 }};
    const second: AABB = AABB{ min: Vec2f32{ _x: 1f32, _y: -1f32 },
                        max: Vec2f32{ _x: 6f32, _y: 2f32 }};

    #[test]
    fn create() -> Result<(), String> {
        let new_aabb = AABB::create(Vec2f32{ _x: 0f32, _y: 0f32 }, Vec2f32{ _x: -1f32, _y: -1f32 });
        assert_eq!(new_aabb.min.x, -1f32);
        assert_eq!(new_aabb.min.y, -1f32);
        Ok(());
    }

    #[test]
    fn is_intercecting() -> Result<(), String> {
        assert_eq!(AABB::is_intercecting(first, second), true);
    }
}