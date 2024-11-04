mod AABB;

struct DrawingShape{
    _points: Vec<Vec2f32>,
    _in_fixture: AABB,
    _out_fixture: AABB,
    _foreground_color: Color,
    _background_color: Color,
}

impl DrawingShape{
    fn new(points: Vec<Vec2f32>, fg: Color, bg: Color) -> DrawingShape{
        let fixs: (AABB, AABB) = AABB::create_fixtures_from(points);
        let fin = fixs[0];
        let fout = fixs[1];
        DrawingShape{_points: points, _in_fixture: fin, _out_fixture: fout, _foreground_color: fg, background_color: bg};
    }
}