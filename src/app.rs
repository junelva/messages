// messages app.rs
//

use crossterm::style::{Color, SetForegroundColor};
use crossterm::{cursor, event, style, terminal, QueueableCommand};
use rand::{thread_rng, Rng};
use std::io::{self, Error, Write};
use std::time::SystemTime;
use std::{thread, time::Duration};

const MSG: &[&str] = &[
    "you are beautiful",
    "you are loved",
    "you are lovable",
    "you matter",
    "you are worthy",
    "you will be ok",
    "it'll work out",
    "keep holding on",
    "hang in there",
    "you've got this",
    "slow down as needed",
    "keep going",
    "you are strong",
    "thank you",
    "you are good",
    "keep it up",
    "it's ok",
    "don't give up",
    "you're a good friend",
    "make yourself a priority",
    "self-care is self-love",
    "success is personal",
    "drink water",
    "get plenty of sleep",
    "eat something!",
    "<3",
    "you are enough",
    "you can be kind",
    "take care of yourself",
    "<3 <3 <3",
    "trust yourself",
    "trust your intuition",
    "you can make a difference",
    "break things down",
    "forgive yourself",
    "you can solve problems",
    "you can create",
    "you can heal",
    "you can love",
    "you are safe",
];

const COLORS: &[&Color] = &[
    &Color::Magenta,
    &Color::DarkMagenta,
    &Color::Cyan,
    &Color::DarkCyan,
    &Color::Grey,
    &Color::Yellow,
    &Color::DarkYellow,
    &Color::Green,
    &Color::DarkGreen,
    &Color::Blue,
];

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct BoxTheme<'a> {
    ul: &'a str,
    ll: &'a str,
    ur: &'a str,
    lr: &'a str,
    sv: &'a str,
    sh: &'a str,
    ee: &'a str,
}

#[allow(dead_code)]
const BOX_ROUND: &BoxTheme = &BoxTheme {
    ul: "╭",
    ll: "╰",
    ur: "╮",
    lr: "╯",
    sv: "│",
    sh: "┈",
    ee: " ",
};

#[allow(dead_code)]
const BOX_REGULAR: &BoxTheme = &BoxTheme {
    ul: "┌",
    ll: "└",
    ur: "┐",
    lr: "┘",
    sv: "│",
    sh: "─",
    ee: " ",
};

#[allow(dead_code)]
const BOX_BOLD: &BoxTheme = &BoxTheme {
    ul: "┏",
    ll: "┗",
    ur: "┓",
    lr: "┛",
    sv: "┃",
    sh: "━",
    ee: " ",
};

#[allow(dead_code)]
const BOX_DOUBLE: &BoxTheme = &BoxTheme {
    ul: "╔",
    ll: "╚",
    ur: "╗",
    lr: "╝",
    sv: "║",
    sh: "═",
    ee: " ",
};

const BOX_THEMES: &[&BoxTheme] = &[BOX_REGULAR, BOX_BOLD, BOX_DOUBLE, BOX_ROUND];

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl From<Coordinate> for (u16, u16) {
    fn from(base: Coordinate) -> (u16, u16) {
        let Coordinate { x, y } = base;
        (x, y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Region {
    ul: Coordinate,
    wh: Coordinate,
    msg: usize,
    color: Color,
    box_theme: usize,
    needs_draw: bool,
}

#[derive(Debug)]
pub struct App {
    time_start: SystemTime,
    time: u64,
    r: Vec<Region>,
}

impl App {
    pub fn new() -> Result<App, Error> {
        let app = App {
            time_start: SystemTime::now(),
            time: 0,
            r: vec![],
        };

        terminal::enable_raw_mode()?;
        io::stdout()
            .queue(cursor::Hide)?
            .queue(cursor::SavePosition)?
            .queue(terminal::EnterAlternateScreen)?
            .queue(event::EnableMouseCapture)?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .flush()?;
        Ok(app)
    }

    pub fn debug_print(&mut self, text: &str) -> Result<(), Error> {
        io::stdout()
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::Print(text))?;
        Ok(())
    }

    pub fn rand_inside(&mut self) -> (u16, u16) {
        let sz = terminal::size().unwrap();
        (
            thread_rng().gen_range(0..(sz.0)),
            thread_rng().gen_range(0..(sz.1)),
        )
    }

    fn prune_old_regions(&mut self, amount: usize) {
        while self.r.len() > amount - 1 {
            self.r.remove(0);
        }
    }

    pub fn create_popup(&mut self, ul_init: (u16, u16)) -> Result<(), Error> {
        let sz = terminal::size().unwrap();
        let msg_index = thread_rng().gen_range(0..MSG.len());
        let msg_len = MSG[msg_index as usize].len();
        let wh = Coordinate {
            x: (msg_len + 4) as u16,
            y: 5,
        };
        let ul = Coordinate {
            x: if (ul_init.0 + wh.x / 2) > sz.0 {
                sz.0 - wh.x / 2
            } else if (ul_init.0 as i16 - wh.x as i16 / 2) < 0 {
                wh.x / 2
            } else {
                ul_init.0
            },
            y: if (ul_init.1 + wh.y / 2) >= sz.1 {
                sz.1 - 1 - wh.y / 2
            } else if (ul_init.1 as i16 - wh.y as i16 / 2) < 0 {
                wh.y / 2
            } else {
                ul_init.1
            },
        };

        let col_index = thread_rng().gen_range(0..COLORS.len());
        let color = COLORS[col_index];

        let theme_index = thread_rng().gen_range(0..BOX_THEMES.len());

        self.r.push(Region {
            ul: Coordinate { x: ul.x, y: ul.y },
            wh: Coordinate { x: wh.x, y: wh.y },
            msg: msg_index,
            color: *color,
            box_theme: theme_index,
            needs_draw: true,
        });
        self.prune_old_regions(12);

        Ok(())
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }

    fn draw_popup(&self, r: &Region) -> Result<(), Error> {
        io::stdout().queue(SetForegroundColor(r.color))?;

        let theme = BOX_THEMES[r.box_theme];
        let w = r.wh.x - 1;
        let h = r.wh.y - 1;
        for y in 0..r.wh.y {
            for x in 0..r.wh.x {
                let s = if (x, y) == (0, 0) {
                    theme.ul
                } else if (x, y) == (0, h) {
                    theme.ll
                } else if (x, y) == (w, 0) {
                    theme.ur
                } else if (x, y) == (w, h) {
                    theme.lr
                } else if x == 0 || x == w {
                    theme.sv
                } else if y == 0 || y == h {
                    theme.sh
                } else {
                    theme.ee
                };
                io::stdout()
                    .queue(cursor::MoveTo(
                        r.ul.x + x - r.wh.x / 2,
                        r.ul.y + y - r.wh.y / 2,
                    ))?
                    .queue(style::Print(s))?;
            }
        }

        let tpos = Coordinate {
            x: r.ul.x - ((r.wh.x - 4) / 2),
            y: r.ul.y,
        };

        let s = MSG[r.msg];
        io::stdout()
            .queue(cursor::MoveTo(tpos.x, tpos.y))?
            .queue(style::Print(s))?;

        Ok(())
    }

    fn update_time(&mut self) -> Result<(), Error> {
        self.time = SystemTime::now()
            .duration_since(self.time_start)
            .unwrap()
            .as_millis() as u64;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.update_time()?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), Error> {
        io::stdout().queue(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        for i in 0..self.r.len() {
            if self.r[i].needs_draw {
                self.r[i].needs_draw = false;
                self.draw_popup(&self.r[i])?;
            }
        }
        io::stdout().flush()?;
        Ok(())
    }

    pub fn sleep(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }

    pub fn exit(&self) -> Result<(), Error> {
        terminal::disable_raw_mode()?;
        io::stdout()
            .queue(event::DisableMouseCapture)?
            .queue(terminal::LeaveAlternateScreen)?
            .queue(cursor::RestorePosition)?
            .queue(cursor::Show)?
            .flush()?;
        Ok(())
    }
}
