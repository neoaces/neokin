use std::rc::Rc;

use crate::links::Link;
use nannou::prelude::*;

pub struct AppConfig {
    window: WindowId,
    links: Vec<Link>,
    angle: f32,
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
}

pub fn init(app: &App) -> AppConfig {
    let _window = app
        .new_window()
        .size(1900, 1028)
        .title("neokin")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    let start: Rc<Link> = Rc::new(Link {x: vec2(0.0, 0.0)});
    let end: Rc<Link> = Rc::new(Link {x: vec2(100.0, 0.0)});
    AppConfig { window: _window, links: vec![Link {x: vec2(0.0, 0.0)}, Link {x: vec2(100.0, 0.0)}], angle: 0.0}
}

pub fn update(app: &App, model: &mut AppConfig, update: Update) {
    model.angle += 0.1;
    let mut end = model.links.get_mut(1).expect("Does not have a 2nd element");
    
    end.x.x = f32::cos(model.angle) * 100.0;
    end.x.y = f32::sin(model.angle) * 100.0;
    
    println!("Current cs values: {}, {:#?}", model.angle, model.links[1].x);
}