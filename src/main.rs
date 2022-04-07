#[macro_use]
extern crate lazy_static;

use nannou::{
    image,
    noise::{Perlin, Seedable},
    prelude::*,
    rand::SeedableRng,
};
use oxidising_places_people_time::{
    grid::ImageGrid, rust_patches::RustPatch, skyline, webcam::WebcamFaceCapture,
};
use rand_chacha::ChaCha8Rng;

const RNG_SEED: u32 = 3452392;
const FREQUENCY: f32 = 2.0;
const AMPLITUDE: f32 = 0.006;

const START_NUM_RUST_PATCHES: usize = 1;
const MAX_NUM_RUST_PATCHES: usize = 56;
const PATCH_SIZE: f32 = 200.0;

const CELL_SIZE: u32 = 12;
const GRID_SCALE_FACTOR: f32 = 1.2;

lazy_static! {
    static ref ORANGE: Hsla = hsla(18.0 / 360.0, 0.63, 0.53, 0.5);
    static ref RED: Hsla = hsla(358.0 / 360.0, 0.53, 0.58, 0.5);
    static ref DARK_BROWN: Hsla = hsla(0.0, 0.74, 0.08, 0.8);
    static ref COLOURS: [Hsla; 3] = [*ORANGE, *RED, *DARK_BROWN];
}

struct Model {
    window_id: WindowId,
    fullscreen: bool,
    image_grid: ImageGrid,
    skyline_image: image::DynamicImage,
    rust_patches: Vec<RustPatch>,
    perlin: Perlin,
    rng: ChaCha8Rng,
    cam: WebcamFaceCapture,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(1920, 1080)
        .key_pressed(key_pressed)
        .resized(window_resized)
        .build()
        .unwrap();

    let skyline_image = skyline::get_skyline_image(app);

    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED as u64);

    let window_rect = app.window(window_id).unwrap().rect();

    let mut rust_patches: Vec<RustPatch> = Vec::with_capacity(MAX_NUM_RUST_PATCHES);
    for _ in 0..START_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(&mut rng, &window_rect, PATCH_SIZE);
        rust_patches.push(patch);
    }

    let image_grid = ImageGrid::new(&window_rect, CELL_SIZE, &skyline_image);

    let perlin = Perlin::new().set_seed(RNG_SEED);

    let cam = WebcamFaceCapture::new(app, 0);

    Model {
        window_id,
        fullscreen: false,
        image_grid,
        skyline_image,
        rust_patches,
        perlin,
        rng,
        cam,
    }
}

fn window_resized(app: &App, model: &mut Model, _dim: Vec2) {
    let window_rect = app.window(app.window_id()).unwrap().rect();

    model.image_grid = ImageGrid::new(&window_rect, CELL_SIZE, &model.skyline_image);

    if let Some(face) = model.cam.read_image() {
        model.image_grid.add_image_centre(
            &face,
            vec2(window_rect.h() * 0.66 * 0.88, window_rect.h() * 0.66),
        )
    }

    let mut rust_patches: Vec<RustPatch> = Vec::with_capacity(MAX_NUM_RUST_PATCHES);
    for _ in 0..START_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(&mut model.rng, &window_rect, PATCH_SIZE);
        rust_patches.push(patch);
    }

    model.rust_patches = rust_patches;

    
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
    let window_rect = app.window(model.window_id).unwrap().rect();

    if app.elapsed_frames() % 120 == 0 && model.rust_patches.len() < MAX_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(&mut model.rng, &window_rect, PATCH_SIZE);
        model.rust_patches.push(patch);
    }

    // grid.add_texture();

    if app.elapsed_frames() % 120 == 0 {
        if let Some(face) = model.cam.read_image() {
            model.image_grid = ImageGrid::new(&window_rect, CELL_SIZE, &model.skyline_image);

            model.image_grid.add_image_centre(
                &face,
                vec2(window_rect.h() * 0.66 * 0.88, window_rect.h() * 0.66),
            )
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    // let window_rect = app.window(model.window_id).unwrap().rect();
    model.image_grid.draw(&draw, GRID_SCALE_FACTOR);

    for patch in &model.rust_patches {
        patch.draw(&draw, COLOURS.as_slice())
    }

    draw.to_frame(app, &frame).unwrap();
}
