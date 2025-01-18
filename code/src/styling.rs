use ratatui::crossterm::event::{self, Event};
use ratatui::layout::Margin;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::Frame;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw)?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let block = Block::bordered()
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .border_type(BorderType::Rounded)
        .title(Line::from(" ✨ Title ✨ ".black().on_blue()).centered())
        .title_bottom(Line::from(" ⚡ Bottom ⚡ ".black().on_green().italic()).right_aligned());

    let text = Paragraph::new("Hello, World!".red())
        .wrap(Wrap { trim: true })
        .centered()
        .block(block);

    frame.render_widget(
        text,
        frame.area().inner(Margin {
            horizontal: 15,
            vertical: 5,
        }),
    );
}
