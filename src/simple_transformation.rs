use std::env;
use opencv::{core::{Mat, Size}, highgui, imgcodecs, imgproc};

fn main() -> opencv::Result<()> {
    let source = env::args().nth(1).expect("Missing argument");

    let mut img = imgcodecs::imread(&source, -1)?;
    let _ = highgui::named_window("ExampleIn", highgui::WINDOW_AUTOSIZE);
    let _ = highgui::named_window("ExampleOut", highgui::WINDOW_AUTOSIZE);

    let _ = highgui::imshow("ExampleIn", &img);

    let mut out = Mat::default();
    let mut out_copy = Mat::default();

    let _ =imgproc::gaussian_blur_def(&mut img, &mut out, Size { width: 5, height: 5 }, 3.0);
    let _ =imgproc::gaussian_blur_def(&mut img, &mut out_copy, Size { width: 5, height: 5 }, 3.0);
    let _ =imgproc::gaussian_blur_def(&mut out, &mut out_copy, Size { width: 5, height: 5 }, 3.0);

    let _ = highgui::imshow("ExampleOut", &out);

    let _ = highgui::wait_key(0);

    Ok(())
}