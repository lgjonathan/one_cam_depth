use opencv::{self, prelude::*};

fn run() -> opencv::Result<()> {
    let window = "video capture";
    opencv::highgui::named_window(window, 1)?;
    let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY)?; // 0 is the default camera
    while cam.is_opened()?{
        let mut frame = opencv::core::Mat::default();
        cam.read(&mut frame)?;
        opencv::highgui::imshow(window, &mut frame)?;
        let key = opencv::highgui::wait_key(1)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    opencv::highgui::destroy_window(window)?;
    Ok(())
}

fn main() {
    run().unwrap()
}
