use std::env;
use opencv::{core::Mat, highgui, hub_prelude::MatTraitConst, videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst}};

fn main() -> opencv::Result<()> {
    let argc = env::args().len();
    let source = env::args().nth(1);

    let _ = highgui::named_window("Example", highgui::WINDOW_AUTOSIZE);

    let mut cap = videoio::VideoCapture::default()?;

    if argc == 1 {
        let _ = cap.open_def(0);
    } else {
        match source {
            Some(value) => {
                let _ = cap.open_file_def(&value)?;
            }
            None => {
                println!("Missing argument")
            }
        }
    }

    if cap.is_opened()? == false {
        println!("Couldn't open capture.");

        return Ok(());
    }

    let mut frame = Mat::default();

    loop {
        let _ = cap.read(&mut frame);

        if frame.empty() {
            break;
        }

        let _ = highgui::imshow("Example", &mut frame);

        let r = highgui::wait_key(33)?;

        if r >= 0 {
            break;
        }
    }

    Ok(())
}