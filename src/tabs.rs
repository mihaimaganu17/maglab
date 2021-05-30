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
}

