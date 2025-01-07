use crate::links::Link;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

#[allow(dead_code)]
pub struct AppConfig {
    window: WindowId,
    links: Vec<Link>,
    angle: f32,
    egui: Egui,
}

// Handle events related to the window and update the model if necessary
fn event(_app: &App, _model: &mut AppConfig, event: WindowEvent) {
    // println!("{:?}", event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &AppConfig, frame: Frame) {
    let draw = app.draw();

    frame.clear(WHITE);
    draw.line()
        .weight(8.0)
        .color(BLACK)
        .caps_round()
        .points(model.links[0].x, model.links[1].x);

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}

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
        links: vec![
            Link { x: vec2(0.0, 0.0) },
            Link {
                x: vec2(100.0, 0.0),
            },
        ],
        angle: 0.0,
        egui: egui,
    }
}

pub fn update(app: &App, model: &mut AppConfig, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label(format!("Angle: {:.3}", model.angle));
    });

    model.angle += 0.01;
    let end = model.links.get_mut(1).expect("Does not have a 2nd element");

    end.x.x = f32::cos(model.angle) * 100.0;
    end.x.y = f32::sin(model.angle) * 100.0;

    println!(
        "Current cs values: {}, {:#?}",
        model.angle, model.links[1].x
    );
}

fn raw_window_event(_app: &App, model: &mut AppConfig, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}