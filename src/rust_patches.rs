// ---- ADDED ----
use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
    rand::RngCore,
};

use crate::utils::map_rng_range;

#[derive(Clone)]
pub struct RustPatch {
    position: Point2,
    size: f32,
    max_size: f32,
    // ellipse: nannou::geom::ellipse::Circumference,
    points: Vec<Vec2>,
    noise_z: f64,
}

impl RustPatch {
    pub fn new_rand<T>(rng: &mut T, window_rect: Rect, max_size: f32) -> Self
    where
        T: RngCore,
    {
        let position = pt2(
            map_rng_range(rng.next_u32(), window_rect.left(), window_rect.right()),
            map_rng_range(rng.next_u32(), window_rect.bottom(), window_rect.top()),
        );

        let ellipse: Vec<Vec2> =
            geom::Ellipse::new(geom::Rect::from_wh(vec2(1.0, 1.0)), window_rect.right())
                .circumference()
                .into_iter()
                .map(|[x, y]| vec2(x, y))
                .collect();

        let noise_z = map_rng_range(rng.next_u32(), 0.0, 100.0);

        RustPatch {
            position,
            size: 1.0,
            max_size,
            // ellipse,
            points: ellipse,
            noise_z,
        }
    }

    pub fn update(&mut self, perlin: Perlin, frequency: f32, amplitude: f32) {
        self.noise_z += 0.003;

        let noise_val = 0.5 + perlin.get([self.x() as f64, self.y() as f64, 100.0 as f64]) as f32;

        let amplitude = amplitude + (amplitude * noise_val);

        let (xx, yy) = (self.x(), self.y());

        // Update points
        for point in &mut self.points {
            let (x, y) = (point.x, point.y);

            let noise_point = [
                (x * frequency) as f64,
                (y * frequency) as f64,
                (self.noise_z + (xx + yy) as f64) * frequency as f64,
            ];
            let variance = (perlin.get(noise_point) as f32) * amplitude / 10.0;

            *point += vec2(x * variance, y * variance);
        }

        // Update size
        self.size = if self.size < 1.0 {
            1.0
        } else if self.size <= self.max_size {
            // Grow to full size over two minutes
            self.size() + self.max_size / (60.0 * 60.0 * 2.0)
        } else {
            self.max_size
        }
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn draw(&self, draw: &Draw, colours: &[Hsla]) {
        for (i, colour) in colours.iter().enumerate() {
            draw.x_y(self.x(), self.y())
                .scale(self.size / (i as f32 + 1.0))
                .polygon()
                .color(*colour)
                .points(self.points.clone());
        }
    }
}
// ---------------
