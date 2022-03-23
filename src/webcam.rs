use nannou::prelude::*;
use opencv::{core::*, imgcodecs, imgproc, objdetect, prelude::*, videoio};

pub struct WebcamCapture {
    capture: videoio::VideoCapture,
    cam_frame_mat: Mat,
    image_textures: Option<Vec<wgpu::Texture>>,
    lbp_face_cascade: objdetect::CascadeClassifier,
}

fn detect_faces(img: &Mat, f_cascade: &mut objdetect::CascadeClassifier) -> Vec<Mat> {
    let mut grey = Mat::default();
    imgproc::cvt_color(img, &mut grey, imgproc::COLOR_BGR2Luv, 0).unwrap();
    let mut objects: Vector<Rect_<i32>> = Vector::new();
    f_cascade
        .detect_multi_scale(
            &grey,
            &mut objects,
            1.1,
            3,
            0,
            Size::new(0, 0),
            Size::new(0, 0),
        )
        .unwrap();

    objects
        .iter()
        .map(|rect| Mat::roi(img, rect).unwrap())
        .collect()
}

fn pixelate(mut img: Mat) -> Mat {
    imgproc::resize(
        &img.clone(),
        &mut img,
        Size::new(16, 18),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .unwrap();

    imgproc::resize(
        &img.clone(),
        &mut img,
        Size::new(640, 720),
        0.0,
        0.0,
        imgproc::INTER_NEAREST,
    )
    .unwrap();

    img
}

impl WebcamCapture {
    pub fn new(app: &App, device_index: i32) -> Self {
        let capture = videoio::VideoCapture::new(device_index, videoio::CAP_ANY).unwrap();
        let cam_frame_mat = Mat::default();

        let assets = app.assets_path().unwrap();
        let cascade_path = assets.join("lbpcascade_frontalface_improved.xml");

        let lbp_face_cascade =
            objdetect::CascadeClassifier::new(cascade_path.to_str().unwrap()).unwrap();

        WebcamCapture {
            capture,
            cam_frame_mat,
            image_textures: None,
            lbp_face_cascade,
        }
    }

    pub fn read_image(&mut self, app: &App) {
        self.capture.read(&mut self.cam_frame_mat).unwrap();

        let pixelated_faces = detect_faces(&self.cam_frame_mat, &mut self.lbp_face_cascade)
            .into_iter()
            .map(|face| pixelate(face))
            .map(|face| {
                let image_file_path = tempfile::Builder::new()
                    .suffix(".png")
                    .tempfile()
                    .unwrap()
                    .into_temp_path();

                imgcodecs::imwrite(image_file_path.to_str().unwrap(), &face, &Vector::new())
                    .expect("Failed to write temp file");

                image_file_path
            })
            .map(|file_path| {
                wgpu::Texture::from_path(app, file_path)
                    .expect("Unable to read temp file")
            })
            .collect();

        self.image_textures = Some(pixelated_faces);
    }

    pub fn get_texture(&self) -> &Option<Vec<wgpu::Texture>> {
        &self.image_textures
    }
}
