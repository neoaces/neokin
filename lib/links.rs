use std::f32::consts::PI;
use std::fmt::Debug;
use nannou::prelude::*;

#[derive(Clone, Copy, Debug)]
/// Represents the position of the next joint to be solved for.
///
/// Only used as an intermediate step when using JointType::process.
///
/// * `x`: Position
pub struct JointState {
    pub x: Vec2
}

/// All joints must define JointType.
pub trait JointType: Debug {
    fn get_name(&self) -> String;
    fn move_joint(&mut self, value: f32);
    fn process(&self, state: &mut JointState, link: &Link); // Link PROCEEDING joint
}

#[derive(Clone, Copy, Debug)]
/// Holds additional info for a Revolute joint
///
/// * `theta`: Angle of the joint CCW from +x 
/// * `omega`: Angular velocity at the joint
pub struct Revolute {
    pub theta: f32,
    pub omega: f32
}

impl JointType for Revolute {
    fn get_name(&self) -> String {
        "Revolute Joint".to_owned()
    }

    fn move_joint(&mut self, value: f32) {
        self.theta = f32::clamp(self.theta + value, 0.0, 2.0 * PI)
    }

    fn process(&self, state: &mut JointState, link: &Link) {
        // TODO: Consider including the theta increment in process.
        state.x.x += link.len * f32::cos(self.theta);
        state.x.y += link.len * f32::sin(self.theta);
    }
}

impl Default for Revolute {
    fn default() -> Self {
        Revolute {
            theta: 0.0,
            omega: 0.0,
        }
    }
}


// TODO : Update the documentation example
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

#[derive(Clone, Copy, Debug)]
/// Represents a joint in the robot space.
///
/// * `mode`: Type of joint (revolute, prismatic, etc.)
pub struct Joint<T> where T: JointType {
    pub mode: T,
}

impl<T> Default for Joint<T> where T: Default + JointType {
    fn default() -> Self {
        Joint {
            mode: T::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
/// Represents a link in the robot space.
///
/// Used mainly in rendering the link to the screen.
///
/// * `len`: 
pub struct Link {
    pub len: f32,
    // TODO: Add the centre of mass and inertia once prototype is finished.
}

impl Default for Link {
    fn default() -> Self {
        Link {
            len: 200.0
        }
    }
}
