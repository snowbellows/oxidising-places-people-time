use nannou::prelude::*;

pub fn get_skyline_texture(app: &App) -> wgpu::Texture {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("greyscale-skyline.jpg");

    wgpu::Texture::from_path(app, img_path).unwrap()
}
