use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};


#[derive(Debug)]
pub struct KeyConfig {
    /// Change the focus/active plugin according to the direction of the 
    /// arrow key pressed
    pub focus_left:         KeyEvent,
    pub focus_right:        KeyEvent,
    pub focus_up:           KeyEvent,
    pub focus_down:         KeyEvent,
    /// Create a new plugin window. This takes into account the current focused
    /// plugin and creates the new plugin to the right of it
    /// Future releases will give you the power to create plugins in 
    /// all directions
    pub new_plugin:         KeyEvent,
    /// Remove the current focused plugin
    /// TODO: Add an are you sure dialog box
    pub remove_plugin:      KeyEvent,
    /// Move a plugin in a certain direction
    /// keep the idea used by Rectangle
    /// Move plugin up
    /// Move plugin down
    /// Move plugin left
    /// Move plugin right
    pub move_left:          KeyEvent,

    /// Switch between tabs
    /// Move to the tab on the right
    pub tab_right:          KeyEvent,
    /// Move to the tab on the left
    pub tab_left:           KeyEvent,

    /// Quit application>
    pub quit:               KeyEvent,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            focus_left: KeyEvent {
                code:       KeyCode::Left,
                modifiers:  KeyModifiers::empty()
            },
            focus_right: KeyEvent {
                code:       KeyCode::Right,
                modifiers:  KeyModifiers::empty()
            },
            focus_up: KeyEvent {
                code:       KeyCode::Up,
                modifiers:  KeyModifiers::empty()
            },
            focus_down: KeyEvent {
                code:       KeyCode::Down,
                modifiers:  KeyModifiers::empty()
            },
            new_plugin: KeyEvent {
                code:       KeyCode::Char('n'),
                modifiers:  KeyModifiers::CONTROL,
            },
            remove_plugin: KeyEvent {
                code:       KeyCode::Char('r'),
                modifiers:  KeyModifiers::CONTROL,
            },
            // TODO: add for move_*(right, up, down)
            move_left: KeyEvent {
                code:       KeyCode::Left,
                modifiers:  KeyModifiers::CONTROL,
            },
            tab_right: KeyEvent {
                code:       KeyCode::Right,
                modifiers:  KeyModifiers::SHIFT,
            },
            tab_left: KeyEvent {
                code:       KeyCode::Left,
                modifiers:  KeyModifiers::SHIFT,
            },
            quit: KeyEvent {
                code:       KeyCode::Char('q'),
                modifiers:  KeyModifiers::CONTROL,
            }
        }
    }
}

impl KeyConfig {
    pub fn init() -> Self {
        Self::default()
    }
}
