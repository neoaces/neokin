use nannou::prelude::*;
use crate::links::{JointType, Link, Revolute};
use std::fmt::Debug;

#[derive(Debug)]
/// The current configuration of the robot.
///
/// At any point, the robot MUST have the same number of links and joints. 
///
/// * `origin`: 
/// * `joints`: 
/// * `links`: 
pub struct RobotConfiguration {
    pub origin: Vec2,
    pub joints: Vec<Box<dyn JointType>>,
    pub links: Vec<Link>
}

impl Default for RobotConfiguration {
    fn default() -> RobotConfiguration {
        RobotConfiguration { 
            origin: Vec2::new(0.0, 0.0), 
            joints: vec![Box::new(Revolute::default())],
            links: vec![Link::default()] 
        }
    }
}
