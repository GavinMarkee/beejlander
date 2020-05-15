/*
GUI module for the beejlander package
*/

use iced::{button, text_input, Align, Button, Checkbox, Column, Container, Element, Length, ProgressBar, Row, Sandbox, Settings, Space, Text, TextInput};

pub fn run() {
    MainWindow::run(Settings::default())
}

#[derive(Default)]
struct MainWindow {
    silver_value: bool,
    common_text: text_input::State,
    common_value: String,
    uncommon_text: text_input::State,
    uncommon_value: String,
    rare_text: text_input::State,
    rare_value: String,
    land_text: text_input::State,
    land_value: String,
    progress_text: String,
    progress_value: f32,
    run_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    SilverChecked(bool),
    CommonChanged(String),
    UncommonChanged(String),
    RareChanged(String),
    LandChanged(String),
    RunPressed,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> MainWindow {
        let mut window = MainWindow::default();
        window.silver_value = true;
        window
    }

    fn title(&self) -> String {
        String::from("Welcome to Beejlander!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SilverChecked(value) => self.silver_value = value,
            Message::CommonChanged(value) => self.common_value = value,
            Message::UncommonChanged(value) => self.uncommon_value = value,
            Message::RareChanged(value) => self.rare_value = value,
            Message::LandChanged(value) => self.land_value = value,
            Message::RunPressed => (),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let silver_input = Checkbox::new(
            self.silver_value,
            "Include Silver Bordered Cards",
            Message::SilverChecked,
        )
            .width(Length::Fill);

        let common_input = TextInput::new(
            &mut self.common_text,
            "Common USD Price Limit (Default of 0.10)",
            &self.common_value,
            Message::CommonChanged,
        )
            .padding(10)
            .size(18);

        let uncommon_input = TextInput::new(
            &mut self.uncommon_text,
            "Uncommon USD Price Limit (Default of 0.10)",
            &self.uncommon_value,
            Message::UncommonChanged,
        )
            .padding(10)
            .size(18);

        let rare_input = TextInput::new(
            &mut self.rare_text,
            "Rare USD Price Limit (Default of 0.25)",
            &self.rare_value,
            Message::RareChanged,
        )
            .padding(10)
            .size(18);

        let land_input = TextInput::new(
            &mut self.land_text,
            "Land USD Price Limit (Default of 0.20)",
            &self.land_value,
            Message::LandChanged,
        )
            .padding(10)
            .size(18);

        let progress_output = Text::new("Not Started")
            .size(18);

        let progress_bar = ProgressBar::new(
            0.0..=100.0,
            self.progress_value,
        );

        let run_input = Button::new(
            &mut self.run_button,
            Text::new("Start"),
        )
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::RunPressed);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(400)
            .push(silver_input)
            .push(common_input)
            .push(uncommon_input)
            .push(rare_input)
            .push(land_input)
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Column::new()
                            .spacing(10)
                            .width(Length::FillPortion(3))
                            .push(progress_output)
                            .push(progress_bar),
                    )
                    .push(
                        Column::new()
                            .spacing(10)
                            .align_items(Align::End)
                            .width(Length::Fill)
                            .push(Space::with_height(Length::Units(8)))
                            .push(run_input),
                    ),
            );
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}