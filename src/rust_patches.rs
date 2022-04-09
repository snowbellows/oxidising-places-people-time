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
    opacity: f32,
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
            opacity: 1.0,
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

        let noise_val = 0.5 + perlin.get([self.x() as f64, self.y() as f64, 100.0]) as f32;

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
            let variance = (perlin.get(noise_point) as f32) * amplitude;

            *point += vec2(x * variance, y * variance);
        }

        // Update size
        self.size = if self.size < 1.0 {
            1.0
        } else if self.size <= self.max_size {
            // Grow to full size over 1 min
            self.max_size * (time - self.creation_time) / 60.0
        } else {
            // Start fading out
            self.opacity -= 0.002;
            self.size
        };

        // After fading out reset
        if self.opacity <= 0.0 {
            self.opacity = 1.0;
            self.size = 1.0;
        };
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

    pub fn overlap_colour(&self, position: &Vec2) -> Option<Hsla> {
        let plotted_points = self.points.iter().map(|p|
                // Map points to correct size, smallest first
                *p * self.size
                // Position it correctly
                + self.position);
        if let Some(_) = geom::Polygon::new(plotted_points).contains(position) {
            return Some(self.colour.clone());
        }

        None
    }

    pub fn draw(&self, draw: &Draw, colours: &[Hsla]) {
        for (i, colour) in colours.iter().enumerate() {
            let faded_colour = hsla(
                colour.hue.to_positive_degrees() / 360.0,
                colour.saturation,
                colour.lightness,
                colour.alpha * self.opacity,
            );
            draw.x_y(self.x(), self.y())
                .scale(self.size / dbg!(colours.len() - i) as f32)
                .polygon()
                .color(faded_colour)
                .points(self.points.clone());
        }
    }
}
