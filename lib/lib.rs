//! This crate holds the API for [neokin], a robot arm simulator. Currently,
//! the simulator only support Revolute joints, but an implementation will
//! be included for prismatic joints at the minimum.
//!
//! See robot.rs as a starting point.
//! [neokin]: https://github.com/neoaces/neokin

mod constants;
pub mod links;
pub mod robot;
pub mod window;
