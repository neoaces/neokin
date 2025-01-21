use crate::{constants, links::JointState};
use crate::robot::RobotConfiguration;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::ops::Index;

#[allow(dead_code)]
/// Configuration for the application
///
/// Includes both the nannou/egui contexts and the robot configuration.
///
/// * `window`: Window id, used with egui
/// * `config`: Robot configuration
/// * `egui`: egui context
/// * `last_mouse`: last position of mouse
pub struct AppConfig {
    window: WindowId,
    config: RobotConfiguration,
    egui: Egui,
    last_mouse: Vec2,
}

/// Handle events related to the window and update the model if necessary
/// * `app`: Nannou app context
/// * `model`: Application configuration
/// * `event`: Window events
pub fn event(app: &App, model: &mut AppConfig, event: WindowEvent) {
    match event {
        MousePressed(_) => {
            model.last_mouse = vec2(app.mouse.x, app.mouse.y);
        }

        _ => {}
    }
}

/// Draw the state of your `Model` into the given `Frame` here.
///
/// * `app`: Nannou app context
/// * `model`: Application configuration
/// * `frame`: Window frame 
pub fn view(app: &App, model: &AppConfig, frame: Frame) {
    let draw = app.draw();
    let config = &model.config;
    let mut back = config.origin;
    
    frame.clear(WHITE);

    // Draw robot workspace
    for i in 0..config.joints.len() {
        let link = config.links.index(i); 
        let joint = config.joints.index(i); 
        let mut state: JointState = JointState { x: Vec2::new(0.0, 0.0) };
        
        // Processes for the next position.
        joint.process(&mut state, link);
        state.x.x += back.x;
        state.x.y += back.y;
    
        // Draw the link
        draw.line()
            .weight(16.0)
            .color(constants::ROBOT_COLOURS[i])
            .caps_round()
            .points(back, state.x.clone());

        back = state.x;
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

/// Initialize app configuration
///
/// * `app`: Nannou app context 
pub fn init(app: &App) -> AppConfig {
    let _window = app
        .new_window()
        .size(1900, 1028)
        .title("neokin")
        .view(view)
        .event(event)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let id = app.window(_window).unwrap();
    let egui = Egui::from_window(&id);

    AppConfig {
        window: _window,
        config: RobotConfiguration::default(),
        egui,
        last_mouse: Vec2::default(),
    }
}

/// Updates the state of the engine
///
/// * `_app`: Nannou app context 
/// * `model`: Application configuration
/// * `update`: Update events
pub fn update(_app: &App, model: &mut AppConfig, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("JOINT ANGLES\n");
    });
    }


// Used by egui to handle window movement and interaction
fn raw_window_event(_app: &App, model: &mut AppConfig, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}


// TODO: Allow public access to the internal config (through API)
/// Runs the application
pub fn run() {
    nannou::app(init)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
