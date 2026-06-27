use std::fs;
use std::io;
use std::env;
use std::fmt::Write;
use std::error::Error;

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

        let b1 = Block::bordered()
            .title(" Hexdump ");
        let b2 = Block::bordered()
            .title(" Hexview ");

        let mut colors: Vec<Line> = Vec::new();
        for chunk in self.bytes.chunks(16) {
            let spans: Vec<Span> = chunk
                .iter()
                .map(|&x| Span::styled("██", Style::default().fg(Color::Rgb(x, x, x))))
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

        Paragraph::new(lines)
            .centered()
            .block(b1)
            .render(l, buf);
        Paragraph::new(colors)
            .centered()
            .block(b2)
            .render(r, buf);       
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from("Missing filepath argument\n Usage: ./hexust /path/to/file"));
    }

    let bytes = fs::read(&args[1])?;
    let hx = Hexust {bytes: bytes};

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions { viewport: Viewport::Inline(10) },
    )?;
    terminal.draw(|f| f.render_widget(&hx, f.area()))?;

    Ok(())
}
