use nannou::{image, prelude::*};
use opencv::{core::*, imgcodecs, imgproc, objdetect, prelude::*, videoio};

use crate::utils;

pub struct WebcamFaceCapture {
    capture: videoio::VideoCapture,
    cam_frame_mat: Mat,
    lbp_face_cascade: objdetect::CascadeClassifier,
    image_temp_path: tempfile::TempPath,
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
        .filter_map(|rect| {
            let padded_rect = rect + Size_::<i32>::new(rect.width / 2, rect.height / 2)
                - Point_::<i32>::new(rect.width / (2 * 2), rect.height / (2 * 2));

            Mat::roi(img, padded_rect).ok()
        })
        .collect()
}

pub fn dynamic_image_to_mat(image: image::DynamicImage) -> Mat {
    let frame = image.into_rgb8().into_flat_samples();

    unsafe {
        Mat::new_rows_cols_with_data(
            frame.layout.height.try_into().unwrap(),
            frame.layout.width.try_into().unwrap(),
            CV_8UC3,
            frame.samples[0] as *mut std::ffi::c_void,
            frame.layout.channel_stride,
        )
        .unwrap()
    }
}
// fn pixelate(mut img: Mat) -> Mat {
//     imgproc::resize(
//         &img.clone(),
//         &mut img,
//         Size::new(16, 18),
//         0.0,
//         0.0,
//         imgproc::INTER_LINEAR,
//     )
//     .unwrap();

//     imgproc::resize(
//         &img.clone(),
//         &mut img,
//         Size::new(640, 720),
//         0.0,
//         0.0,
//         imgproc::INTER_NEAREST,
//     )
//     .unwrap();

//     img
// }

impl WebcamFaceCapture {
    pub fn new(app: &App, device_index: i32) -> Self {
        let capture = videoio::VideoCapture::new(device_index, videoio::CAP_ANY).unwrap();
        let cam_frame_mat = Mat::default();

        let assets = app.assets_path().unwrap();
        let cascade_path = assets.join("lbpcascade_frontalface_improved.xml");

        let lbp_face_cascade =
            objdetect::CascadeClassifier::new(cascade_path.to_str().unwrap()).unwrap();

        let image_temp_path = tempfile::Builder::new()
            .suffix(".png")
            .tempfile()
            .unwrap()
            .into_temp_path();

        WebcamFaceCapture {
            capture,
            cam_frame_mat,
            lbp_face_cascade,
            image_temp_path,
        }
    }

    pub fn read_image(&mut self, app: &App) -> Option<image::DynamicImage> {
        self.capture.read(&mut self.cam_frame_mat).unwrap();

        let faces = detect_faces(&self.cam_frame_mat, &mut self.lbp_face_cascade);

        if faces.len() > 0 {
            imgcodecs::imwrite(
                &self.image_temp_path.to_str().unwrap(),
                &faces[random_range(0, faces.len())],
                &Vector::new(),
            )
            .expect("Failed to write temp file");

            return Some(image::open(&self.image_temp_path).unwrap());
        } else {
            let image_path =
                utils::random_image_from_folder(app.assets_path().unwrap().join("faces"))
                    .unwrap();

            let mat = imgcodecs::imread(image_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR).unwrap();
            let faces = detect_faces(&mat, &mut self.lbp_face_cascade);

            if faces.len() > 0 {
                imgcodecs::imwrite(
                    &self.image_temp_path.to_str().unwrap(),
                    &faces[random_range(0, faces.len())],
                    &Vector::new(),
                )
                .expect("Failed to write temp file");

                return Some(image::open(&self.image_temp_path).unwrap());
            }
        }

        None
    }
}
