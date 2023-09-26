use one_cam_depth::cam::run;
use opencv as cv;

fn main() -> cv::Result<()> {
    run()?;
    Ok(())
}
