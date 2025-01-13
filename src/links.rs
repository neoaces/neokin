use nannou::geom::Vec2;

pub enum Link {
    RootRevolute {
        x: Vec2,
        theta: f32,
        omega: f32,
    },

    Revolute {
        x: Vec2,
        theta: f32,
        omega: f32,
        length: f32
    }
}