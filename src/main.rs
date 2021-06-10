/*
 * TODO: Refactor the code in appropriate files
 * TODO: Refactor event handling for each app
 * TODO: Find a way to execute terminal commands
 */
use std::{
    thread,
    io::stdout,
    error::Error,
    sync::mpsc,
    time::{Duration, Instant},
};

use tui::{
    terminal::{Terminal},
    backend::{CrosstermBackend},
};

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen},
    event::{self, Event as CtEvent, EnableMouseCapture, DisableMouseCapture,
        KeyCode},
    execute,
};

pub mod app;
pub mod tabs;
use crate::tabs::{TabsState};
use crate::app::{App, MagLabApp, ColumnsState, PluginsState, Plugin};


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
    thread::spawn(move || {
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


    let plugins1 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView]);
    let plugins2 = PluginsState::new(vec![Plugin::HexView]);
    let plugins3 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView, Plugin::Parser]);
    let cols1 = ColumnsState::new(vec![plugins1, plugins2, plugins3]);
    let plugins1 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView]);
    let plugins2 = PluginsState::new(vec![Plugin::HexView]);
    let plugins3 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView, Plugin::Parser]);
    let cols2 = ColumnsState::new(vec![plugins2, plugins1, plugins3]);
    let plugins1 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView]);
    let plugins2 = PluginsState::new(vec![Plugin::HexView]);
    let plugins3 = PluginsState::new(
        vec![Plugin::FileManager, Plugin::HexView, Plugin::Parser]);
    let cols3 = ColumnsState::new(vec![plugins3, plugins2, plugins1]);

    let tabs = TabsState::new(vec![
        app::App::new("FileManager", cols1),
        app::App::new("MachO", cols2),
        app::App::new("PE", cols3),
    ]);

    // Create a new MagLab app
    let mut mag_lab_app = MagLabApp::new("MagLab", tabs);

    // Clear terminal output so we have a clean canvas
    terminal.clear()?;

    loop {
        // Draw the canvas
        terminal.draw(|f| mag_lab_app.draw(f))?;
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
                KeyCode::Left => mag_lab_app.on_left(),
                KeyCode::Right => mag_lab_app.on_right(),
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

    Ok(())
}
