use nannou::prelude::*;
use oxidising_places_people_time::skyline::{draw_skyline, get_skyline_texture};

struct Model {
    window_id: WindowId,
    fullscreen: bool,
    skyline_texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(1920, 1080)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let skyline_texture = get_skyline_texture(app);

    Model {
        window_id,
        fullscreen: false,
        skyline_texture,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::F {
        let fullscreen = !model.fullscreen;

        model.fullscreen = fullscreen;

        app.window(model.window_id)
            .unwrap()
            .set_fullscreen(fullscreen);
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);

    draw_skyline(app, &draw, &model.skyline_texture);
    draw.to_frame(app, &frame).unwrap();
}
