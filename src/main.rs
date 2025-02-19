use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.example.sony_playstation_vita_emulator"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Sony PlayStation Vita Emulator");
        window.set_default_size(800, 600);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Welcome to the Sony PlayStation Vita Emulator"));
        let start_button = Button::with_label("Start Emulator");

        start_button.connect_clicked(|_| {
            Command::new("sony-playstation-vita-emulator.exe").spawn().expect("Failed to start emulator");
        });

        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&start_button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}