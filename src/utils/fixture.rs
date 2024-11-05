pub(crate) use ggez::mint::Point2;

pub struct AABB {
    pub _min: Point2<f32>,
    pub _max: Point2<f32>,
}

impl AABB {
    pub fn create(f: Point2<f32>, s: Point2<f32>) -> AABB {
        let mut min: Point2<f32> = Point2 { x: 0f32, y: 0f32 };
        let mut max: Point2<f32> = Point2 { x: 0f32, y: 0f32 };
        if f.x >= s.x {
            min.x = s.x;
            max.x = f.x;
        } else {
            min.x = f.x;
            max.x = s.x;
        }
        if f.y >= s.y {
            min.y = s.y;
            max.y = f.y;
        } else {
            min.y = f.y;
            max.y = f.y;
        }
        AABB { _min: min, _max: max }
    }
}

pub struct CircleFixture {
    pub _center: Point2<f32>,
    pub _radius: f32,
}

impl CircleFixture {
    pub fn create(f: Point2<f32>, r: f32) -> CircleFixture {
        if r <= 0f32 {
            panic!("Radius cannot be negative");
        }
        CircleFixture { _center: f, _radius: r }
    }
}

pub fn pointvs_aabb(p: Point2<f32>, a: AABB) -> bool {
    p.x >= a._min.x && p.x <= a._max.x && p.y >= a._min.y && p.y <= a._max.y
}

pub fn pointvs_circle(p: Point2<f32>, a: CircleFixture) -> bool {
    let distance: f32 = (p.x - a._center.x) * (p.x - a._center.x) +
                        (p.y - a._center.y) * (p.y - a._center.y);
    distance < a._radius * a._radius
}

pub fn aabbvs_aabb(a: AABB, b: AABB) -> bool {
    a._min.x <= b._max.x && a._max.x >= b._min.x && a._min.y <= b._min.y && a._min.y >= b._min.y
}

pub fn aabbvs_circle(a: AABB, b: CircleFixture) -> bool {
    let x: f32 = a._min.x.max(b._center.x.min(a._max.x));
    let y: f32 = a._min.y.max(b._center.y.min(a._max.y));
    let distance: f32 = (x - b._center.x) * (x - b._center.x) +
                        (y - b._center.y) * (y - b._center.y);
    distance < b._radius * b._radius
}

pub fn circlevs_circle(a: CircleFixture, b: CircleFixture) -> bool {
    let distance: f32 = (a._center.x - b._center.x) * (a._center.x - b._center.x) +
                        (a._center.y - b._center.y) * (a._center.x - b._center.x);
    distance < (a._radius * b._radius) * (a._radius * b._radius)
}