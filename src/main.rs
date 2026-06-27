use std::fs;
use std::io;
use std::env;
use std::error::Error;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};
use ratatui::{Terminal, TerminalOptions, Viewport, backend::CrosstermBackend};



pub fn hexdump(bytes: &Vec<u8>) -> String {
    return String::from("Pane 1")
}

pub fn hexview(bytes: &Vec<u8>) -> String {
    return String::from("Pane 2")
}

#[derive(Debug, Default)]
pub struct Hexust {
    hexdump: String,
    hexview: String,
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

        Paragraph::new(self.hexdump.as_str())
            .centered()
            .block(b1)
            .render(l, buf);
        Paragraph::new(self.hexview.as_str())
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
    let hxd = hexdump(&bytes);
    let hxv = hexview(&bytes);
    let mut hx = Hexust {hexdump: hxd, hexview: hxv};

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions { viewport: Viewport::Inline(10) },
    )?;
    terminal.draw(|f| f.render_widget(&hx, f.area()))?;

    Ok(())
}
