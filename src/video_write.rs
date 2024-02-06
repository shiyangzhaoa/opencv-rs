use std::env;
use opencv::{core::{Mat, Point_, Size}, highgui, hub_prelude::MatTraitConst, imgproc, videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst, VideoWriterTrait}};

fn main() -> opencv::Result<()> {
    let source = env::args().nth(1).expect("Missing argument");

    println!("source = {}", source);

    let _ = highgui::named_window("Example", highgui::WINDOW_AUTOSIZE);
    let _ = highgui::named_window("Log_Polar", highgui::WINDOW_AUTOSIZE);

    let mut cap = videoio::VideoCapture::default()?;

    let _ = cap.open_def(0);

    if cap.is_opened()? == false {
        println!("Couldn't open capture.");

        return Ok(());
    }

    let mut frame = Mat::default();
    let mut logpolar_frame = Mat::default();
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let mut writer = videoio::VideoWriter::default()?;
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)?;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)?;

    let _ = writer.open(
        &source,
        videoio::CAP_PROP_FOURCC,
        fps,
        Size {
            width: width as i32,
            height: height as i32
        },
        true
    );

    loop {
        let _ = cap.read(&mut frame);

        if frame.empty() {
            break;
        }

        let _ = highgui::imshow("Example", &mut frame);
        let x = frame.cols() as f32;
        let y = frame.rows()as f32;

        let _ = imgproc::warp_polar(
            &mut frame,
            &mut logpolar_frame,
            Size {
                width: width as i32,
                height: height as i32
            },
            Point_::<f32>::new(x, y),
            40.0,
            imgproc::WARP_FILL_OUTLIERS,
        );

        let _ = highgui::imshow("Log_Polar", &mut logpolar_frame);

        let _ = writer.write(&mut logpolar_frame);

        let c = highgui::wait_key(33)?;

        if c == 'r' as i32 {
            println!("release");
            break;
        }
    }

    println!("release");

    let _ = writer.release();

    Ok(())
}