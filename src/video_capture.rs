use std::env;
use anyhow::{Ok, Result};
use opencv::{core::Mat, highgui, videoio::{self, VideoCaptureTrait as _}, hub_prelude::MatTraitConst};

fn main() -> Result<()> {
    let source = env::args().nth(1).expect("Missing argument");
    let _ = highgui::named_window("Example", -1);
    let mut cap = videoio::VideoCapture::from_file_def(&source)?;

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
    };
    
    Ok(())
}