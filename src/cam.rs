use opencv::{self as cv, prelude::*};

pub fn run() -> cv::Result<()> {
    let window = "video capture";
    cv::highgui::named_window(window, 1)?;
    let mut detector = cv::features2d::ORB::default()?;
    let mut cam = cv::videoio::VideoCapture::new(0, cv::videoio::CAP_ANY)?; // 0 is the default camera
    while cam.is_opened()? {
        let mut frame = Mat::default();
        if !cam.read(&mut frame)? {
            return Err(cv::Error::new(404, "empty frame"));
        }
        let mut keypoints = cv::core::Vector::<cv::core::KeyPoint>::default();
        let mask = Mat::default();
        let mut descriptors = Mat::default();
        detector.detect_and_compute(&frame, &mask, &mut keypoints, &mut descriptors, false)?;
        let mut out_frame = frame.clone();
        cv::features2d::draw_keypoints(
            &frame,
            &keypoints,
            &mut out_frame,
            cv::core::Vec4d::from((0.0, 255.0, 0.0, 255.0)),
            cv::features2d::DrawMatchesFlags::DEFAULT,
        )?;
        cv::highgui::imshow(window, &out_frame)?;
        let key = cv::highgui::wait_key(1)?;
        let not_pressed = 255;
        if key > 0 && key != not_pressed {
            break;
        }
    }
    cv::highgui::destroy_window(window)?;
    Ok(())
}
