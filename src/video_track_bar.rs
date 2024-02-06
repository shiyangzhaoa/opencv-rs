use std::env;
use opencv::{core::Mat, highgui, videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst}};

fn main() -> opencv::Result<()> {
    let source = env::args().nth(1).expect("Missing argument");
    let _ = highgui::named_window("Example", -1);
    let mut cap = videoio::VideoCapture::from_file_def(&source)?;

    let mut g_slider_position = 0;
    let mut g_run = 1;
    let mut g_dontset = 0;

    let mut on_trackbar_change: Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>> = None;

    on_trackbar_change = Some(Box::new(move |pos| {
        print!("pos {}", pos);

        // let _ = cap.set(videoio::CAP_PROP_POS_FRAMES, pos as f64);
        if g_dontset == 0 {
            g_run = 1;
        }

        g_dontset = 0;
    }));

    highgui::named_window("Trackbar Example", highgui::WINDOW_NORMAL)?;

    let trackbar_name = "Value";
    let winname = "Trackbar Example";
    let frames = cap.get(videoio::CAP_PROP_FRAME_COUNT)?;

    highgui::create_trackbar(
        trackbar_name,
        winname,
        Some(&mut g_slider_position),
        frames as i32,
        on_trackbar_change,
    )?;

    let mut frame = Mat::default();

    loop {
        if g_run != 0 {
          let _ = cap.read(&mut frame);
          let current_pos = cap.get(videoio::CAP_PROP_POS_FRAMES)?;
          g_dontset = 1;

          let _ = highgui::set_trackbar_pos(trackbar_name, winname, current_pos as i32)?;

          let _ = highgui::imshow("Trackbar Example", &mut frame);
          g_run -= 1;
        }

        let c = highgui::wait_key(10)?;

        if  c == 's' as i32 {
          g_run = 1;

          println!("Single step, run = {}", g_run);
        } else if c == 'r' as i32 {
            g_run = -1;

            println!("Run mode, run = {}", g_run);
        } else if c == 27 {
            break;
        }
    }

    Ok(())
}