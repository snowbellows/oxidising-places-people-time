#[macro_use]
extern crate lazy_static;

use nannou::{
    image,
    noise::{Perlin, Seedable},
    prelude::*,
    rand::SeedableRng,
    text::font,
    text::font::default_notosans,
};
use oxidising_places_people_time::{
    grid::ImageGrid, rust_patches::RustPatch, skyline, webcam::WebcamFaceCapture,
};
use rand_chacha::ChaCha8Rng;

const RNG_SEED: u32 = 3452392;
const FREQUENCY: f32 = 2.0;
const AMPLITUDE: f32 = 0.003;

const START_NUM_RUST_PATCHES: usize = 10;
const NEXT_NUM_RUST_PATCHES: usize = 10;
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
    skyline_image: image::DynamicImage,
    rust_patches: Vec<RustPatch>,
    perlin: Perlin,
    rng: ChaCha8Rng,
    cam: WebcamFaceCapture,
    font: text::Font,
    next_patch_index: usize,
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

    let rng = ChaCha8Rng::seed_from_u64(RNG_SEED as u64);

    let image_grid = ImageGrid::new(&app.window_rect(), CELL_SIZE, &skyline_image);
    let rust_patches: Vec<RustPatch> =
        Vec::with_capacity(START_NUM_RUST_PATCHES + NEXT_NUM_RUST_PATCHES);

    let perlin = Perlin::new().set_seed(RNG_SEED);

    let cam = WebcamFaceCapture::new(app, 0);

    let assets = app.assets_path().unwrap();
    let font_path = assets.join("opensans.ttf");
    let font = font::from_file(font_path).unwrap();

    Model {
        window_id,
        fullscreen: false,
        image_grid,
        skyline_image,
        rust_patches,
        perlin,
        rng,
        cam,
        font,
        next_patch_index: 0,
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
    if app.elapsed_frames() == 0 {
        // Have to call this here because camera isn't available until now
        reset_model(model, &app, window_rect)
    } else if (app.time % 60.0) <= 0.1 {
        // Reset every minute
        reset_model(model, &app, window_rect)
    } else {
        // Update patch size
        for patch in &mut model.rust_patches {
            patch.update(model.perlin, FREQUENCY, AMPLITUDE, app.time);
        }

        // add rust patch to grid
        model.image_grid.add_cell_colours(|c| {
            model.rust_patches.iter().fold(None, |acc, patch| {
                patch
                    .overlap_colour(&c.position)
                    .filter(|colour| colour != &c.colour)
                    .or(acc)
            })
        })
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);

    // for patch in &model.rust_patches {
    //     patch.draw(&draw, COLOURS.as_slice())
    // }

    model.image_grid.draw(&draw, GRID_SCALE_FACTOR);
    let frame_rate_text = format!("FPS: {}\nTime: {}", app.fps(), app.time);
    draw.text(&frame_rate_text)
        .w(app.window_rect().w())
        .font_size(40)
        .font(model.font.clone())
        .left_justify()
        .color(GREENYELLOW)
        .font(default_notosans())
        .y(10.0);

    draw.to_frame(app, &frame).unwrap();
}

fn reset_model(model: &mut Model, app: &App, window_rect: Rect) {
    model.image_grid = ImageGrid::new(&window_rect, CELL_SIZE, &model.skyline_image);

    if let Some(face) = model.cam.read_image() {
        model.image_grid.add_image_centre(
            &face,
            vec2(window_rect.h() * 0.66 * 0.88, window_rect.h() * 0.66),
        )
    }

    model.rust_patches.clear();
    for _ in 0..START_NUM_RUST_PATCHES {
        let patch = RustPatch::new_rand(
            &mut model.rng,
            &window_rect,
            2.0,
            PATCH_SIZE / 2.0,
            app.time,
            *RED_BROWN,
        );
        model.rust_patches.push(patch);
        for i in 0..NEXT_NUM_RUST_PATCHES {
            let ref_patch = &model.rust_patches
                [map_range(i, 0, NEXT_NUM_RUST_PATCHES, 0, START_NUM_RUST_PATCHES)];
            let patch = RustPatch::new_from_ref(ref_patch, *ORANGE, app.time);
            model.rust_patches.push(patch);
        }
    }
}
