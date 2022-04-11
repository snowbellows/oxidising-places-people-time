use nannou::{image, prelude::*};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn map_rng_range<X, Y>(value: X, out_min: Y, out_max: Y) -> Y
where
    X: NumCast + Bounded,
    Y: NumCast,
{
    map_range(value, X::min_value(), X::max_value(), out_min, out_max)
}

pub fn draw_texture_fullscreen(app: &App, draw: &Draw, texture: &wgpu::Texture) {
    let window_id = app.window_id();
    let win_rect = app.window(window_id).unwrap().rect();
    let points = win_rect.corners_iter().map(|q| {
        (
            Point2::from(q),
            pt2(q[0] / win_rect.w() + 0.5, 1.0 - (q[1] / win_rect.h() + 0.5)),
        )
    });
    draw.polygon().points_textured(&texture, points);
}

pub fn random_image_from_folder<P>(path: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let paths: Vec<PathBuf> = fs::read_dir(&path)?
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter_map(|entry| {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    return Some(entry.path());
                }
            }

            None
        })
        .collect();

     Ok(path.as_ref().join(&paths[random_range(0, paths.len())]))

}
