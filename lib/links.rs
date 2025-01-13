use nannou::geom::Vec2;

///
/// Represents joints within the robot body.
///
/// Example:
///
/// ```
/// let link = Link::Revolute {x: vec2(1.0, 2.0), theta: 0.0, omega: 0.0, length: 0.0}
/// assert!(link.x.x, 1.0)
/// ```
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
        length: f32,
    },
}
