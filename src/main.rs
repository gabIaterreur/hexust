use std::env;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use ratatui::{Terminal, TerminalOptions, Viewport, backend::CrosstermBackend};

#[derive(Debug, Default)]
pub struct Hexust {
    bytes: Vec<u8>,
}

pub fn to_dumbass_colors(gray: u8) -> (u8, u8, u8) {
    let hue = (gray as f32);
    let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);

    if hue < 85.0 {
        r = 0.0;
        g = hue * 3.0;
        b = 255.0 - hue * 3.0;
    } else if hue < 170.0 {
        let hue = hue - 85.0;
        r = hue * 3.0;
        g = 255.0 - hue * 3.0;
        b = 0.0;
    } else {
        let hue = hue - 170.0;
        r = 255.0 - hue * 3.0;
        g = 0.0;
        b = hue * 3.0;
    }

    return (r as u8, g as u8, b as u8);
}

impl Hexust {
    pub fn run(&self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &Hexust {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [l, r] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(area);

        let b1 = Block::bordered().title(" Hexdump ");
        let b2 = Block::bordered().title(" Hexview ");

        let mut colors: Vec<Line> = Vec::new();
        for chunk in self.bytes.chunks(32) {
            let spans: Vec<Span> = chunk
                .iter()
                .map(|&x| {
                    let (r, g, b) = to_dumbass_colors(x);
                    Span::styled("██", Style::default().fg(Color::Rgb(r, g, b)))
                })
                .collect();
            colors.push(Line::from(spans))
        }

        let mut lines: Vec<Line> = Vec::new();
        for (index, chunk) in self.bytes.chunks(16).enumerate() {
            let mut out = String::new();
            let offset = (index * 16) as u64;

            let _ = write!(out, "{:08x} ", offset);
            for b in chunk {
                let _ = write!(out, "{:02x} ", b);
            }
            for b in chunk {
                let c = if b.is_ascii_graphic() || *b == b' ' {
                    *b as char
                } else {
                    '.'
                };
                let _ = write!(out, "{}", c);
            }
            out.push('\n');
            lines.push(Line::from(out));
        }

        Paragraph::new(lines).centered().block(b1).render(l, buf);
        Paragraph::new(colors).centered().block(b2).render(r, buf);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from(
            "Missing filepath argument\n Usage: ./hexust /path/to/file",
        ));
    }

    let bytes = fs::read(&args[1])?;
    let max_len = bytes.chunks(16).count();
    let hx = Hexust { bytes: bytes };

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(max_len as u16),
        },
    )?;
    terminal.draw(|f| f.render_widget(&hx, f.area()))?;

    Ok(())
}
