use std::convert::{TryFrom};

use tui::{
    terminal::{Frame},
    backend::{Backend},
    text::{Span, Spans},
    style::{Style, Color},
    layout::{Layout, Constraint, Rect, Direction},
    widgets::{Block, Tabs, Borders, BorderType},
};

use crate::tabs::TabsState;

pub enum Plugin {
    FileManager,
    HexView,
    Parser,
}

/// Struct to hold an application for each tab
pub struct App<'a,> {
    pub title: &'a str,
    pub grid: ColumnsState,
}

pub struct ColumnsState {
    pub columns: Vec<PluginsState>,
    pub index: usize,
}

impl ColumnsState {
    pub fn new(columns: Vec<PluginsState>) -> ColumnsState {
        ColumnsState { columns, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.columns.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.columns.len() - 1;
        }
    }
}

pub struct PluginsState {
    pub plugins: Vec<Plugin>,
    pub index: usize,
}

impl PluginsState {
    pub fn new(plugins: Vec<Plugin>) -> PluginsState {
        PluginsState { plugins, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.plugins.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.plugins.len() - 1;
        }
    }
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, grid: ColumnsState) -> App {
        App { title, grid }
    }

    pub fn next_column(&mut self) {
        self.grid.next();
    }

    pub fn previous_column(&mut self) {
        self.grid.previous();
    }

    pub fn next_line(&mut self) {
        let col_index = self.grid.index;
        self.grid.columns[col_index].next();
    }

    pub fn previous_line(&mut self) {
        let col_index = self.grid.index;
        self.grid.columns[col_index].previous();
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let ncols = self.grid.columns.len();
        let constraints: Vec<Constraint> = self.grid.columns.iter().map(|_|
            Constraint::Percentage(
                u16::try_from(100usize / ncols).unwrap())
        ).collect();

        let col_chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Horizontal)
            .split(area);

        for (i, col) in self.grid.columns.iter().enumerate() {
            // Get the number of plugins to render
            let nplugins = col.plugins.len();

            // We create an invisible widget, so that we also render columns
            let col_block = Block::default();
            f.render_widget(col_block, col_chunks[i]);

            // Distribute the plugins evenly across the column
            let line_constraints: Vec<Constraint> = col.plugins.iter()
                .map(|_| {
                // TODO: Replace unwrap and be a real programmer
                Constraint::Percentage(
                    u16::try_from(100usize / nplugins).unwrap())
            }).collect();

            // Split the column chunks based on the constraints above
            let line_chunks = Layout::default()
                .constraints(line_constraints)
                .direction(Direction::Vertical)
                .split(col_chunks[i]);

            // Now we render each plugin
            for (j, _line) in col.plugins.iter().enumerate() {
                let mut plugin = Block::default().borders(Borders::ALL)
                    .title(format!("Plugin{}", j));

                // If the plugin matches the selected one, we highlight it
                if i == self.grid.index && j == col.index {
                    plugin = plugin
                        .border_style(Style::default().fg(Color::Red))
                        .border_type(BorderType::Rounded);
                } else {
                    plugin = plugin
                        .border_style(Style::default().fg(Color::White))
                }
                f.render_widget(plugin, line_chunks[j]);
            }
        }
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

    pub fn tab_left(&mut self) {
        self.tabs.previous()
    }

    pub fn tab_right(&mut self) {
        self.tabs.next()
    }

    pub fn focus_left(&mut self) {
        self.tabs.apps[self.tabs.index].previous_column();
    }

    pub fn focus_right(&mut self) {
        self.tabs.apps[self.tabs.index].next_column();
    }

    pub fn focus_up(&mut self) {
        self.tabs.apps[self.tabs.index].previous_line();
    }

    pub fn focus_down(&mut self) {
        self.tabs.apps[self.tabs.index].next_line();
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
        self.tabs.apps[self.tabs.index].draw(f, chunks[1]);
    }
}
