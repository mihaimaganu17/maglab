use std::io;

use termion::raw::IntoRawMode;

use tui::Terminal;
//use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

pub fn draw_screen() -> Result<(), io::Error> {
    /* let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Clear the terminal
    terminal.clear()?;

    // Draw the main screen
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("MagLab")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })
    */
    Ok(())
}
