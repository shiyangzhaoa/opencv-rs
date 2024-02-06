use std::env;
use opencv::{core::{Mat}, highgui, imgcodecs, imgproc};

fn main() -> opencv::Result<()> {
    let source = env::args().nth(1).expect("Missing argument");

    let _ = highgui::named_window("Example1", highgui::WINDOW_AUTOSIZE);
    let _ = highgui::named_window("Example2", highgui::WINDOW_AUTOSIZE);

    let img = imgcodecs::imread(&source, -1)?;

    let _ = highgui::imshow("Example1", &img);

    let mut img_gry = Mat::default();

    let _ = imgproc::cvt_color_def(&img, &mut img_gry, imgproc::COLOR_BGR2GRAY);

    let _ = highgui::imshow("Example1", &img_gry);

    let mut img_cny = Mat::default();

    let _ =  imgproc::canny(&img_gry, &mut img_cny, 10.0, 100.0, 3, true);

    let _ = highgui::imshow("Example2", &img_cny);

    let _ = highgui::wait_key(0)?;

    Ok(())
}