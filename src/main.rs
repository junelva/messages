// main.rs
//

use std::{time::Duration, io};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

mod app;

fn main() -> io::Result<()> {
    let mut app = app::App::new()?;
    
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
                    
                    if code == KeyCode::Esc ||
                            (code == KeyCode::Char('c') && mods == KeyModifiers::CONTROL) ||
                            code == KeyCode::Char('q') {
                        break 'main;
                    }
                },
                Event::Mouse(event) => {
                    app.debug_print(format!("{:?}", event).as_str())?;
                    app.render()?;
                },
                Event::Paste(_data) => continue,
                Event::Resize(_width, _height) => continue,
            }
        }
        
        if app.get_time() > last_popup_time + 2000 {
            let pos = app.rand_inside();
            app.create_popup(pos)?;
            last_popup_time = app.get_time();
        }

        app.update()?;
        app.render()?;
    }

    app.exit()?;
    Ok(())
}

