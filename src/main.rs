use std::{
    thread,
    io::stdout,
    error::Error,
    sync::mpsc,
    time::{Duration, Instant},
};

use tui::{
    backend::{CrosstermBackend, Backend},
    terminal::{Terminal, Frame},
    text::{Span, Spans},
    widgets::{Block, Tabs, Borders},
    layout::{Layout, Constraint},
    style::{Color, Modifier, Style},
};

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen},
    event::{self, Event as CtEvent, EnableMouseCapture, DisableMouseCapture,
        KeyCode},
    execute,
};



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
}

/// Structure describing all the tabs in our application
pub struct TabsState<'a> {
    /// A vector containing all the apps. Each tab contains a separate App
    pub apps: Vec<App<'a>>,
    /// Index of the active Tab
    pub index: usize,
}

impl<'a> TabsState<'a> {
    /// Create a new TabsState given the `apps`
    pub fn new(apps: Vec<App<'a>>) -> TabsState<'a> {
        TabsState { apps, index: 0 }
    }

    /// Go to the next tab
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.apps.len();
    }

    /// Go to the previous tab
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.apps.len() - 1;
        }
    }
}

/// Struct to hold an application for each tab
pub struct App<'a> {
    pub title: &'a str
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App {
        App { title }
    }
}

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>>{
    // Put terminal in raw mode
    enable_raw_mode()?;

    // Get a new handle to the standard output
    let mut stdout = stdout();
    // Give app an alternate screen
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Create a new Backend
    let backend = CrosstermBackend::new(stdout);

    // Create a new Terminal
    let mut terminal = Terminal::new(backend)?;

    // Setup a multiproduce-singleconsumer channel
    let (tx, rx) = mpsc::channel();

    // Setup a timeout tick rate
    let tick_rate = Duration::from_millis(1000);

    // Spanw a new thread that will handle the event pipeline
    let events_thread = thread::spawn(move || {
        // Get current time
        let mut last_tick = Instant::now();

        // Event loop
        loop {
            // Poll for tick rate duration, if no events, send a tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // If we get a timeout
            if event::poll(timeout).unwrap() {
                // Check if we have an event
                if let CtEvent::Key(key) = event::read().unwrap() {
                    // Send the event to the consumer
                    tx.send(Event::Input(key)).unwrap();
                }

                // If we get a timeout, send a tick event and reset the tick
                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut tabs = TabsState::new(vec![
        App::new("FileManager"),
        App::new("MachO"),
        App::new("PE"),
    ]);

    // Create a new MagLab app
    let mut mag_lab_app = MagLabApp::new("MagLab", tabs);

    // Clear terminal output so we have a clean canvas
    terminal.clear()?;

    loop {
        // Draw the canvas
        terminal.draw(|f| draw_app(f, &mut mag_lab_app))?;
        // Handle user input
        match rx.recv()? {
            Event::Input(event) => match event.code {
                // Handles quitting the app
                KeyCode::Char('q') => {
                    // Get terminal back into normal mode
                    disable_raw_mode()?;
                    // Leave our crossterm screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture,
                    )?;
                    terminal.show_cursor()?;
                    mag_lab_app.should_quit = true;
                    break;
                },
                KeyCode::Char(c) => mag_lab_app.on_key(c),
                _ => {}
            },

            Event::Tick => {},
        };

        // Currently do nothing on tick

        // Check if we should exit the app
        if mag_lab_app.should_quit {
            break;
        }

    }


    // Join the event thread at the end
    // let res = events_thread.join();

    Ok(())
}

pub fn draw_app<B: Backend>(f: &mut Frame<B>, mag_lab_app: &mut MagLabApp) {
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

    let titles = mag_lab_app
        .tabs
        .apps
        .iter()
        .map(|t| Spans::from(Span::styled(t.title, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(mag_lab_app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(mag_lab_app.tabs.index);
    f.render_widget(tabs, chunks[0]);
}
