use nannou::prelude::*;

pub fn get_skyline_texture(app: &App) -> wgpu::Texture {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("greyscale-skyline.jpg");

    wgpu::Texture::from_path(app, img_path).unwrap()
}

pub fn draw_skyline(app: &App, draw: &Draw, texture: &wgpu::Texture) {
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
