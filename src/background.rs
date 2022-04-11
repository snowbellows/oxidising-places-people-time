use nannou::{image, prelude::*};

use crate::utils::random_image_from_folder;

pub fn get_background_image(app: &App) -> image::DynamicImage {
    let backgrounds_path = app.assets_path().unwrap().join("backgrounds");
    let img_path = random_image_from_folder(backgrounds_path).unwrap();

    image::open(img_path).unwrap()
}
