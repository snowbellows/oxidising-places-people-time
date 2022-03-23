use nannou::prelude::*;
use opencv::{core::*, imgcodecs, imgproc, prelude::*, videoio};

pub struct WebcamCapture {
    capture: videoio::VideoCapture,
    cam_frame_mat: Mat,
    image_file_path: tempfile::TempPath,
    image_texture: Option<wgpu::Texture>,
}

impl WebcamCapture {
    pub fn new(device_index: i32) -> Self {
        let capture = videoio::VideoCapture::new(device_index, videoio::CAP_ANY).unwrap();
        let cam_frame_mat = Mat::default();

        let image_file_path = tempfile::Builder::new()
            .suffix(".png")
            .tempfile()
            .unwrap()
            .into_temp_path();

        WebcamCapture {
            capture,
            cam_frame_mat,
            image_file_path,
            image_texture: None,
        }
    }

    pub fn read_image(&mut self, app: &App) {
        self.capture.read(&mut self.cam_frame_mat).unwrap();

        imgcodecs::imwrite(
            self.image_file_path.to_str().unwrap(),
            &self.cam_frame_mat,
            &opencv::core::Vector::new(),
        )
        .expect("Failed to write temp file");

        self.image_texture = Some(
            wgpu::Texture::from_path(app, &self.image_file_path).expect("Unable to read temp file"),
        );
    }

    pub fn get_texture(&self) -> &Option<wgpu::Texture> {
        &self.image_texture
    }
}
