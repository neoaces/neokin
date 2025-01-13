use nannou::geom::Vec2;

///
/// Represents joints within the robot body.
///
/// Example:
///
/// ```
/// use nannou::prelude::vec2;
/// use neolib::links::Link;
///
/// let link = Link::Revolute {x: vec2(1.0, 2.0), theta: 0.0, omega: 0.0, length: 0.0};
///
/// if let Link::Revolute { x, theta: _, omega: _, length: _ } = link {
///     assert_eq!(x.x, 1.0)
/// }
/// ```
///
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
