#[macro_use]
extern crate lazy_static;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
    rand::SeedableRng, // text::font,
};
use oxidising_places_people_time::{
    background::get_background_image, grid::ImageGrid, rust_patches::RustPatch,
    webcam::WebcamFaceCapture,
};
use rand_chacha::ChaCha8Rng;

const RNG_SEED: u32 = 3452392;
const FREQUENCY: f32 = 2.0;
const AMPLITUDE: f32 = 0.003;

const START_NUM_RUST_PATCHES: usize = 15;
const NEXT_NUM_RUST_PATCHES: usize = 7;
const PATCH_SIZE: f32 = 1000.0;

const CELL_SIZE: u32 = 12;
const GRID_SCALE_FACTOR: f32 = 1.2;

lazy_static! {
    static ref ORANGE: Hsla = hsla(19.0 / 360.0, 0.63, 0.31, 1.0);
    static ref RED_BROWN: Hsla = hsla(9.0 / 360.0, 0.72, 0.13, 1.0);
    static ref DARK_BROWN: Hsla = hsla(9.0 / 360.0, 0.63, 0.05, 1.0);
    static ref COLOURS: [Hsla; 3] = [*DARK_BROWN, *ORANGE, *RED_BROWN];
}

struct Model {
    window_id: WindowId,
    fullscreen: bool,
    image_grid: ImageGrid,
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

    let rng = ChaCha8Rng::seed_from_u64(RNG_SEED as u64);

    let background_image = get_background_image(app);

    let image_grid = ImageGrid::new(&app.window_rect(), CELL_SIZE, &background_image);
    let rust_patches: Vec<RustPatch> =
        Vec::with_capacity(START_NUM_RUST_PATCHES + NEXT_NUM_RUST_PATCHES);

    let perlin = Perlin::new().set_seed(RNG_SEED);

    let cam = WebcamFaceCapture::new(app, 2);

    // let font_path = app.assets_path().unwrap().join("opensans.ttf");
    // let font = font::from_file(font_path).unwrap();

    Model {
        window_id,
        fullscreen: false,
        image_grid,
        rust_patches,
        perlin,
        rng,
        cam,
    }
}

fn window_resized(app: &App, model: &mut Model, _dim: Vec2) {
    reset_model(model, app, app.window_rect());
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
    let window_rect = app.window(model.window_id).unwrap().rect();
    if app.elapsed_frames() == 0 || (app.time % 60.0) <= 0.1 {
        // Call on start when camera available and reset every minute
        reset_model(model, app, window_rect)
    } else {
        // Update patch size
        for patch in &mut model.rust_patches {
            patch.update(model.perlin, FREQUENCY, AMPLITUDE, app.time);
        }

        // Add rust patch to grid
        for cell in model.image_grid.iter_mut_cells() {
            let overlap_colour: Option<Hsla> = model
                .rust_patches
                .iter()
                .fold_while(None, |acc, patch| {
                    if patch.contains(&cell.position) {
                        return Done(Some(*patch.colour()));
                    }
                    Continue(acc)
                })
                .into_inner();
            if let Some(colour) = overlap_colour {
                cell.colour = colour;
                cell.colour.hue += random_range(-10.0, 10.0);
                cell.colour.saturation += random_range(-0.1, 0.1);
                cell.colour.lightness += random_range(-0.1, 0.1);

                cell.finished();
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);

    model.image_grid.draw(&draw, GRID_SCALE_FACTOR);
    // let frame_rate_text = format!("FPS: {}\nTime: {}", app.fps(), app.time);
    // draw.text(&frame_rate_text)
    //     .w(app.window_rect().w())
    //     .font_size(40)
    //     .font(model.font.clone())
    //     .left_justify()
    //     .color(GREENYELLOW)
    //     .y(10.0);

    draw.to_frame(app, &frame).unwrap();
}

fn reset_model(model: &mut Model, app: &App, window_rect: Rect) {
    let background_image = get_background_image(app);

    model.image_grid = ImageGrid::new(&window_rect, CELL_SIZE, &background_image);

    if let Some(face) = model.cam.read_image(app) {
        model.image_grid.add_image_centre(
            &face,
            vec2(
                window_rect.h() * 0.66 * 0.88 * random_range(0.9, 1.1),
                window_rect.h() * 0.66 * random_range(0.9, 1.1),
            ),
        )
    }

    model.rust_patches.clear();
    for _ in 0..START_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(
            &mut model.rng,
            &window_rect,
            5.0,
            PATCH_SIZE / 2.0,
            app.time,
            *RED_BROWN,
        );
        model.rust_patches.push(patch);
    }

    for i in 0..NEXT_NUM_RUST_PATCHES {
        let ref_patch =
            &model.rust_patches[map_range(i, 0, NEXT_NUM_RUST_PATCHES, 0, START_NUM_RUST_PATCHES)];
        let patch = RustPatch::new_from_ref(ref_patch, *ORANGE, app.time);
        model.rust_patches.push(patch);
    }
}
