use ratatui::{DefaultTerminal, Frame};

use crate::grid::Grid;
mod grid;
mod operations;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let grid = Grid { cols: 4, rows: 3, cells: vec![] };
    frame.render_widget(grid, frame.area());
}