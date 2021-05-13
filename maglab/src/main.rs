use std::fs;
use std::io;
use std::thread;
use std::sync::mpsc;
use std::time::{Instant, Duration};

use crossterm::event;
use crossterm::event::{KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use tui::Terminal;
use tui::text::{Span, Spans};
use tui::backend::CrosstermBackend;
use tui::style::{Style, Modifier, Color};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::widgets::{Paragraph, Borders, Tabs, Block, BorderType, ListState};
use tui::widgets::{Table, Row, Cell, List, ListItem};

use serde::{Serialize, Deserialize};

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

use thiserror::Error;

mod gui;

const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct File {
    sha256: String,
    file_type: String,
    tag: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum MagLabEvent<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    FileManager,
    Files,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::FileManager => 0,
            MenuItem::Files => 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //gui::screen::draw_screen().expect("Failed to draw main screen");

    // Enter in raw mode
    enable_raw_mode().expect("Can run in raw mode");

    // Create a multiproducer, single consumer channel to communicate
    // between the input handler and the rendering loop.
    let (tx, rx) = mpsc::channel();

    // Duration of a tick. If withing this interval there is no input from the
    // user, than send a tick to the backend
    let tick_rate = Duration::from_millis(200);

    // Create a new thread that handles the input from the user
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                // Check if we have a new key event
                if let event::Event::Key(key) = event::read()
                        .expect("Can read events") {
                    tx.send(MagLabEvent::Input(key)).expect("Can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(MagLabEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["FileManager", "Files", "Add", "Delete", "Quit"];
    let mut active_menu_item = MenuItem::FileManager;

    let mut file_list_state = ListState::default();
    file_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        // Menu
                        Constraint::Length(3),
                        // Content
                        Constraint::Min(2),
                        // Footer
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            // The Footer content
            let copyright = Paragraph::new("MagLab-CLI 2021 - all rights")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            // Menu content
            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            match active_menu_item {
                MenuItem::FileManager => {
                    rect.render_widget(render_file_manager(), chunks[1]);
                },
                MenuItem::Files => {
                    let files_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            // List View
                            Constraint::Percentage(20),
                            // Detailed View
                            Constraint::Percentage(80)
                        ].as_ref())
                        .split(chunks[1]);
                    let (left, right) = render_files(&file_list_state);
                    rect.render_stateful_widget(left, files_chunks[0],
                        &mut file_list_state);
                    rect.render_widget(right, files_chunks[1]);
                },
            }
            rect.render_widget(copyright, chunks[2]);
            rect.render_widget(tabs, chunks[0]);
        });

        match rx.recv()? {
            MagLabEvent::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('m') => active_menu_item = MenuItem::FileManager,
                KeyCode::Char('f') => active_menu_item = MenuItem::Files,
                KeyCode::Char('a') => {
                    add_random_file_to_db().expect("Can add new random file");
                },
                KeyCode::Char('d') => {
                    remove_file_at_index(&mut file_list_state)
                        .expect("Can remove pet");
                },
                KeyCode::Down => {
                    if let Some(selected) = file_list_state.selected() {
                        let amount_files = read_db()
                            .expect("Can fetch file list").len();
                        if selected >= amount_files - 1 {
                            file_list_state.select(Some(0));
                        } else {
                            file_list_state.select(Some(selected + 1));
                        }
                    }
                },
                KeyCode::Up => {
                    if let Some(selected) = file_list_state.selected() {
                        let amount_files = read_db()
                            .expect("Can fecth file list").len();
                        if selected > 0 {
                            file_list_state.select(Some(selected - 1));
                        } else {
                            file_list_state.select(Some(amount_files - 1));
                        }
                    }
                },
                _ => {}
            },
            MagLabEvent::Tick => {}
        }
    };

    Ok(())
}

fn render_file_manager<'a>() -> Paragraph<'a> {
    let file_manager = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "maglab-CLI",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'm' to access the File Manager, \
            'f' to access the Files and 'q' to quit")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(
                Spans::from(vec![Span::styled(
                    "maglab-CLI",
                    Style::default().fg(Color::LightRed),
                )]))
            .border_type(BorderType::Plain),
    );
    file_manager
}

fn read_db() -> Result<Vec<File>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<File> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn add_random_file_to_db() -> Result<Vec<File>, Error> {
    let mut rng = thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<File> = serde_json::from_str(&db_content)?;
    let verdict = match rng.gen_range(0..=1) {
        0 => "dirty",
        _ => "clean",
    };

    let rnd_file_type = match rng.gen_range(0..=2) {
        0 => "pe",
        1 => "elf",
        _ => "macho",
    };

    let random_file = File {
        sha256: rng.sample_iter(Alphanumeric).take(10).map(char::from).collect(),
        file_type: rnd_file_type.to_string(),
        tag: verdict.to_string(),
    };

    // Add the new file to our list
    parsed.push(random_file);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

fn remove_file_at_index(file_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = file_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<File> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        file_list_state.select(Some(selected - 1));
    }
    Ok(())
}

fn render_files<'a>(file_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let files = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Files")
        .border_type(BorderType::Plain);

    let file_list = read_db().expect("Can fetch file list");
    let items: Vec<_> = file_list
        .iter()
        .map(|file| {
            ListItem::new(Spans::from(vec![
                Span::styled(file.sha256.clone(), Style::default())]))
        })
        .collect();

    let selected_file = file_list.get(
        file_list_state.selected().expect("There is always a selected file"))
        .expect("Exists")
        .clone();

    let list = List::new(items).block(files).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let file_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_file.sha256)),
        Cell::from(Span::raw(selected_file.file_type)),
        Cell::from(Span::raw(selected_file.tag)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Sha256",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "FileType",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Tag",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, file_detail)
}
