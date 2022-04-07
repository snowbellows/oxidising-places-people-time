use nannou::{
    image,
    prelude::*,
};

pub fn get_skyline_image(app: &App) -> image::DynamicImage {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("skyline.jpg");

    image::open(img_path).unwrap()
}
