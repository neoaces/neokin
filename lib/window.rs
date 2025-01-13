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
pub fn event(_app: &App, _model: &mut AppConfig, _event: WindowEvent) {
    // println!("{:?}", event);
}

// Draw the state of your `Model` into the given `Frame` here.
pub fn view(app: &App, model: &AppConfig, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);

    let mut link_iter = model.links.iter().peekable();
    let mut last_x: Vec2 = Vec2::default();

    while let Some((start, end)) = link_iter.next().zip(link_iter.peek()) {
        match start {
            // 1. At beginning of robot.
            Link::RootRevolute {
                x: sx,
                theta,
                omega: _,
            } => {
                match end {
                    Link::Revolute {
                        x: ex,
                        theta: _,
                        omega: _,
                        length: _,
                    } => {
                        draw.line()
                            .weight(8.0)
                            .color(BLACK)
                            .caps_round()
                            .points(*sx, *ex);

                        last_x = *sx;
                    }

                    // Should not have more than one root (handles Root_ arms of enum)
                    _ => unimplemented!(),
                }
            }

            // 2. Render a non-starting link
            Link::Revolute {
                x: sx,
                theta,
                omega: _,
                length: _,
            } => {
                match end {
                    Link::Revolute {
                        x: ex,
                        theta: _,
                        omega: _,
                        length: _,
                    } => {
                        draw.line()
                            .weight(8.0)
                            .color(BLACK)
                            .caps_round()
                            .points(*sx + last_x, *ex);

                        last_x = *sx;
                    }

                    // Should not have more than one root (handles Root_ arms of enum)
                    _ => panic!("Robot has more than one root."),
                }
            }
        }
    }

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
            Link::RootRevolute {
                x: vec2(0.0, 0.0),
                omega: 0.0,
                theta: 0.0,
            },
            Link::Revolute {
                x: vec2(100.0, 0.0),
                omega: 0.0,
                theta: 0.0,
                length: 100.0,
            },
            Link::Revolute {
                x: vec2(0.0, 50.0),
                omega: 0.0,
                theta: 0.0,
                length: 50.0,
            },
        ],
        angle: 0.0,
        egui: egui,
    }
}

pub fn update(_app: &App, model: &mut AppConfig, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        for (i, link) in model.links.iter_mut().enumerate().skip(1) {
            ui.label("JOINT ANGLES");
            ui.label(format!(
                "Angle of joint #{} [\u{0424}]: {:.3}",
                i, model.angle
            ));

            match link {
                Link::Revolute {
                    x: _,
                    theta,
                    omega: _,
                    length: _,
                } => {
                    ui.add(egui::Slider::new(theta, 0.0..=(2.0 * PI)));
                }

                _ => todo!(),
            }
        }
    });

    let mut sum_theta: f32 = 0.0;
    let mut last_x: Vec2 = Vec2::default();

    for link in model.links.iter_mut() {
        match link {
            Link::Revolute {
                x,
                theta,
                omega: _,
                length,
            } => {
                x.x = f32::cos(*theta + sum_theta) * *length + last_x.x;
                x.y = f32::sin(*theta + sum_theta) * *length + last_x.y;

                sum_theta += *theta;
                last_x = *x;
            }

            Link::RootRevolute {
                x,
                theta: _,
                omega: _,
            } => last_x = *x,
        }
    }
}

fn raw_window_event(_app: &App, model: &mut AppConfig, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

pub fn run() {
    nannou::app(init)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
