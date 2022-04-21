use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
    rand::RngCore,
};

use crate::utils::map_rng_range;

#[derive(Clone)]
pub struct RustPatch {
    colour: Hsla,
    position: Point2,
    size: f32,
    max_size: f32,
    points: Vec<Vec2>,
    noise_z: f64,
    creation_time: f32,
}

impl RustPatch {
    fn new(
        colour: Hsla,
        position: Vec2,
        start_size: f32,
        max_size: f32,
        noise_z: f64,
        creation_time: f32,
    ) -> Self {
        let ellipse: Vec<Vec2> = geom::Ellipse::new(geom::Rect::from_wh(vec2(1.0, 1.0)), 20.0)
            .circumference()
            .into_iter()
            .map(|[x, y]| vec2(x, y))
            .collect();

        RustPatch {
            colour,
            position,
            size: start_size,
            max_size,
            points: ellipse,
            noise_z,
            creation_time,
        }
    }

    pub fn new_rand<T>(
        rng: &mut T,
        window_rect: &Rect,
        start_size: f32,
        max_size: f32,
        creation_time: f32,
        colour: Hsla,
    ) -> Self
    where
        T: RngCore,
    {
        let position = pt2(
            map_rng_range(rng.next_u32(), window_rect.left(), window_rect.right()),
            map_rng_range(rng.next_u32(), window_rect.bottom(), window_rect.top()),
        );

        let noise_z = map_rng_range(rng.next_u32(), 0.0, 100.0);

        Self::new(
            colour,
            position,
            start_size,
            max_size,
            noise_z,
            creation_time,
        )
    }

    pub fn new_from_ref(ref_patch: &RustPatch, colour: Hsla, creation_time: f32) -> Self {
        RustPatch::new(
            colour,
            ref_patch.position,
            1.0,
            ref_patch.max_size,
            ref_patch.noise_z + 10.0,
            creation_time,
        )
    }

    pub fn update(&mut self, perlin: Perlin, frequency: f32, amplitude: f32, time: f32) {
        self.noise_z += 0.003;

        let noise_val = 0.5 + perlin.get([self.position.x as f64, self.position.y as f64, 100.0]) as f32;

        let amplitude = amplitude + (amplitude * noise_val);

        // Update points
        for point in &mut self.points {
            let (x, y) = (point.x, point.y);

            let noise_point = [
                (x * frequency) as f64,
                (y * frequency) as f64,
                (self.noise_z + (self.position.x + self.position.y) as f64) * frequency as f64,
            ];
            let variance = (perlin.get(noise_point) as f32) * amplitude;

            *point += vec2(x * variance, y * variance);
        }

        // Update size
        if self.size < 1.0 {
            self.size = 1.0
        } else if self.size <= self.max_size {
            // Grow to full size over 1 min
            self.size = self.max_size * (time - self.creation_time) / 60.0
        }
    }

    pub fn colour(&self) -> &Hsla {
        &self.colour
    }

    pub fn contains(&self, position: &Vec2) -> bool {
        let plotted_points = self.points.iter().map(|p|
            // Map points to correct size, smallest first
            *p * self.size
            // Position it correctly
            + self.position);
        if geom::Polygon::new(plotted_points).contains(position).is_some() {
            return true;
        }

        false
    }
}
