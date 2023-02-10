use std::{io::stdout, sync::mpsc::channel, thread, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rscam::Camera;
use sixel::encoder::{Encoder, QuickFrameBuilder};
use sixel_sys::PixelFormat;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "webcam to sixel")]
struct Opt {
    #[structopt(long, short, default_value = "/dev/video0")]
    device: String,
    #[structopt(long, short, default_value = "640")]
    width: u32,
    #[structopt(long, short, default_value = "480")]
    height: u32,
    #[structopt(long, short, default_value = "30")]
    fps: u32,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide, EnableMouseCapture)?;

    let (tx, rx) = channel();

    thread::spawn(move || loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => (),
                    (KeyCode::Esc, KeyModifiers::NONE) => (),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => (),
                    _ => break,
                }
                tx.send(()).unwrap();
            }
            Err(_) => tx.send(()).unwrap(),
            _ => (),
        }
    });

    let mut camera = Camera::new(&opt.device)?;

    camera.start(&rscam::Config {
        interval: (1, opt.fps),
        resolution: (opt.width, opt.height),
        format: b"RGB3",
        ..Default::default()
    })?;

    let encoder = Encoder::new().unwrap();
    loop {
        if rx.recv_timeout(Duration::from_millis(1)).is_ok() {
            break;
        }

        execute!(stdout(), MoveTo(0, 0))?;

        let frame = camera.capture()?;
        let qframe = QuickFrameBuilder::new()
            .width(usize::try_from(opt.width)?)
            .height(usize::try_from(opt.height)?)
            .format(PixelFormat::RGB888)
            .pixels((frame[..]).to_vec());

        encoder.encode_bytes(qframe).unwrap();
    }

    execute!(stdout(), LeaveAlternateScreen, Show, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
