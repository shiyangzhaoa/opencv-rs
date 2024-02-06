use std::env;
use opencv::{core::{Mat}, highgui, imgcodecs, imgproc};

fn main() -> opencv::Result<()> {
    let source = env::args().nth(1).expect("Missing argument");

    let _ = highgui::named_window("Example1", highgui::WINDOW_AUTOSIZE);
    let _ = highgui::named_window("Example2", highgui::WINDOW_AUTOSIZE);

    let img = imgcodecs::imread(&source, -1)?;

    let _ = highgui::imshow("Example1", &img);

    let mut new_img = Mat::default();

    let _ = imgproc::pyr_down_def(&img, &mut new_img);

    let _ = highgui::imshow("Example2", &new_img);

    let _ = highgui::wait_key(0)?;

    Ok(())
}