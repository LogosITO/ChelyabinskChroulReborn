mod src;

pub use src::utils::coords;

#[cfg(test)]
mod tests {
    use super::*;

    const first: Vec2f32 = Vec2f32 { _x: -1f32, _y: -1f32 };
    const second: Vec2f32 = Vec2f32 { _x: 1f32, _y: 2f32 };

    #[test]
    fn test_vec_ops() -> Result<(), String> {
        assert!((first + second)._x == 0f32 && (first + second)._y == 1f32);
        Ok(());
    }

    #[test]
    fn test_vec_ops_asssign() -> Result<(), String> {
        assert!((first += second)._x == 0f32 && (first += second)._y == 1f32);
        Ok(());
    }

    #[test]
    fn test_vec_sub() -> Result<(), String> {
        assert!((first - second)._x == -2f32 && (first - second)._y == -3f32);
        assert!((second - first)._x == 2f32 && (second - first)._y == 3f32);
        Ok(());
    }
}