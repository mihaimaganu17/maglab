use tui::{
    style::{Style, Color},
    widgets::{Block, Tabs, Borders},
    text::{Span, Spans},
    layout::{Layout, Constraint},
    terminal::{Frame},
    backend::{Backend},
};

use crate::tabs::TabsState;


/// Struct to hold an application for each tab
pub struct App<'a> {
    pub title: &'a str
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App {
        App { title }
    }
}

/// This is the main application that runs the Malware lab
pub struct MagLabApp<'a> {
    /// Title of our application
    pub title: &'a str,
    /// Status flag if the application should quit
    pub should_quit: bool,
    /// A vector of all tabs in out application
    pub tabs: TabsState<'a>,
}

impl<'a> MagLabApp<'a> {
    pub fn new(title: &'a str, tabs: TabsState<'a>) -> MagLabApp<'a> {
        MagLabApp {
            title,
            should_quit: false,
            tabs,
        }
    }

    /// Called when pressing left arrow-key
    pub fn on_left(&mut self) {
        self.tabs.previous()
    }

    /// Called when pressing right arrow-key
    pub fn on_right(&mut self) {
        self.tabs.next()
    }

    /// Called when pressing any other key
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            },
            _ => {}
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        // Overall app layout
        let layout_constraints = vec![
            // Tabs block, at least 3 lines
            Constraint::Length(3),
            // Rest of the screen
            Constraint::Min(0)
        ];

        let chunks = Layout::default()
            .constraints(layout_constraints.as_ref())
            .split(f.size());

        let titles = self
            .tabs
            .apps
            .iter()
            .map(|t| Spans::from(
                    Span::styled(t.title, Style::default().fg(Color::White))))
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(self.title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.tabs.index);
        f.render_widget(tabs, chunks[0]);
    }
}
