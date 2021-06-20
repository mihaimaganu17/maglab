use crate::App;

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

    /// Remove the current focused tab
    /// Returns true if it is the last tab remainning in the app
    pub fn remove_tab(&mut self) -> bool {
        // If this is the last tab remainning in the app, return true
        if self.apps.len() == 1 {
            return true
        }

        // We remove the current tab
        self.apps.remove(self.index);
        // If the tab was on the last index, we go to the previous one,
        // otherwise we keep the index
        if self.index == self.apps.len() - 1 {
            self.previous();
        }

        return false
    }
}

