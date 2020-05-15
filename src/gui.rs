/*
GUI module for the beejlander package
*/

use iced::{Element, Sandbox, Settings, Text};

pub fn run() {
    MainWindow::run(Settings::default())
}

struct MainWindow;

impl Sandbox for MainWindow {
    type Message = ();

    fn new() -> MainWindow {
        MainWindow
    }

    fn title(&self) -> String {
        String::from("Beejlander")
    }

    fn update(&mut self, _message: Self::Message) {

    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello World!").into()
    }
}