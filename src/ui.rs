use gtk::prelude::*;
use gtk::{Window, Button, Box, Label};

pub struct UI {
    window: Window,
}

impl UI {
    pub fn new() -> Self {
        let window = Window::new(gtk::WindowType::Toplevel);
        window.set_title("Sony PlayStation Vita Emulator");
        window.set_default_size(800, 600);
        UI { window }
    }

    pub fn setup(&self) {
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Welcome to the Sony PlayStation Vita Emulator"));
        let start_button = Button::with_label("Start Emulator");

        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&start_button, true, true, 0);
        self.window.add(&vbox);
        self.window.show_all();
    }
}