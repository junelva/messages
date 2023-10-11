// messages main.rs
//

use clap::Parser;

#[derive(Parser)]
#[command(name = "messages")]
#[command(version = "1.0")]
#[command(about = "positive messages in your terminal", long_about = None)]
struct Cli {
    #[arg(
        help = "milliseconds between messages",
        // short = 'w',
        // long = "wait",
        default_value_t = 2000
    )]
    wait: u64,
    #[arg(
        help = "clear after n messages",
        // short = 'c',
        // long = "clear",
        default_value_t = 4
    )]
    clear_after: u64,
}

use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use std::{io, time::Duration};

mod app;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let popup_length = if cli.wait > 1 { cli.wait } else { 1 };
    let clear_after = cli.clear_after;

    let mut app = app::App::new()?;

    let mut first = true;
    let mut last_clear_time = 0;
    let mut last_popup_time = 0;
    'main: loop {
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::FocusGained => continue,
                Event::FocusLost => continue,
                Event::Key(event) => {
                    if cfg!(debug_assertions) {
                        app.debug_print(format!("{:?}", event).as_str())?;
                    }
                    app.render()?;

                    let code = event.code;
                    let mods = event.modifiers;
                    let _kind = event.kind;
                    let _state = event.state;

                    if code == KeyCode::Esc
                        || (code == KeyCode::Char('c') && mods == KeyModifiers::CONTROL)
                        || code == KeyCode::Char('q')
                    {
                        break 'main;
                    }
                }
                Event::Mouse(event) => {
                    if cfg!(debug_assertions) {
                        app.debug_print(format!("{:?}", event).as_str())?;
                    }

                    app.render()?;
                }
                Event::Paste(_data) => continue,
                Event::Resize(_width, _height) => continue,
            }
        }

        if first || app.get_time() > last_popup_time + popup_length {
            first = false;
            if clear_after > 0 && app.get_time() > last_clear_time + clear_after * popup_length {
                app.clear()?;
                last_clear_time = app.get_time();
            }

            let pos = app.rand_inside();
            app.create_popup(pos)?;
            last_popup_time = app.get_time();
        }

        app.update()?;
        app.render()?;
        app.sleep(100);
    }

    app.exit()?;
    Ok(())
}
