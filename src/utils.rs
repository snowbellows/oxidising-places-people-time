use nannou::{prelude::*, image};
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

pub fn pixel_to_hsla(pixel: image::Rgba<u8>) -> Hsla {

    let red = pixel[0] as f32 / 255.0;
    let green = pixel[1] as f32 / 255.0;
    let blue = pixel[2] as f32 / 255.0;

    srgba(red, green, blue, 1.0).into()
}