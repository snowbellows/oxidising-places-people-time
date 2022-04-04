#[macro_use]
extern crate lazy_static;

use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
    rand::SeedableRng,
};
use oxidising_places_people_time::{
    rust_patches::RustPatch, skyline, utils::draw_texture_fullscreen, webcam::WebcamCapture,
};
use rand_chacha::ChaCha8Rng;

const RNG_SEED: u32 = 3452392;
const FREQUENCY: f32 = 2.0;
const AMPLITUDE: f32 = 0.006;

const START_NUM_RUST_PATCHES: usize = 1;
const MAX_NUM_RUST_PATCHES: usize = 56;
const PATCH_SIZE: f32 = 200.0;

lazy_static! {
    static ref ORANGE: Hsla = hsla(18.0 / 360.0, 0.63, 0.53, 0.5);
    static ref RED: Hsla = hsla(358.0 / 360.0, 0.53, 0.58, 0.5);
    static ref DARK_BROWN: Hsla = hsla(0.0, 0.74, 0.08, 0.8);
    static ref COLOURS: [Hsla; 3] = [*ORANGE, *RED, *DARK_BROWN];
}

struct Model {
    window_id: WindowId,
    fullscreen: bool,
    skyline_texture: wgpu::Texture,
    rust_patches: Vec<RustPatch>,
    perlin: Perlin,
    rng: ChaCha8Rng,
    cam: WebcamCapture,
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

    let skyline_texture = skyline::get_skyline_texture(app);

    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED as u64);

    let window_rect = app.window(window_id).unwrap().rect();

    let mut rust_patches: Vec<RustPatch> = Vec::with_capacity(MAX_NUM_RUST_PATCHES);
    for _ in 0..START_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(&mut rng, window_rect, PATCH_SIZE);
        rust_patches.push(patch);
    }

    let perlin = Perlin::new().set_seed(RNG_SEED);

    let cam = WebcamCapture::new(&app, 2);

    Model {
        window_id,
        fullscreen: false,
        skyline_texture,
        rust_patches,
        perlin,
        rng,
        cam,
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

fn update(app: &App, model: &mut Model, _update: Update) {
    for patch in &mut model.rust_patches {
        patch.update(model.perlin, FREQUENCY, AMPLITUDE);
    }

    if app.elapsed_frames() % 120 == 0 && model.rust_patches.len() < MAX_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(
            &mut model.rng,
            app.window(model.window_id).unwrap().rect(),
            PATCH_SIZE,
        );
        model.rust_patches.push(patch);
    }

    model.rust_patches.len();

    if app.elapsed_frames() % 120 == 0 {
        model.cam.read_image(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);

    draw_texture_fullscreen(app, &draw, &model.skyline_texture);

    if let Some(image_textures) = model.cam.get_texture() {
        if image_textures.len() != 0 {
            let texture = &image_textures[0];
            let r = Rect::from_w_h(640.0, 720.0).middle_of(app.window_rect());
            draw.texture(texture).xy(r.xy()).wh(r.wh());
        }
    }

    for patch in &model.rust_patches {
        patch.draw(&draw, COLOURS.as_slice())
    }

    draw.to_frame(app, &frame).unwrap();
}
