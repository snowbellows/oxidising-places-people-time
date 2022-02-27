use nannou::prelude::*;

struct Model {
    window_id: WindowId,
    fullscreen: bool,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size( 1920, 1080)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        window_id,
        fullscreen: false,
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

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(WHITE);
}
