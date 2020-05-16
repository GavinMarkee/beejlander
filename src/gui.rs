/*
GUI module for the beejlander package
*/

use iced::{button, text_input, Align, Button, Checkbox, Column, Container, Element, Length, ProgressBar, Row, Sandbox, Settings, Space, Text, TextInput};

pub fn run() {
    MainWindow::run(Settings::default())
}

struct MainWindow {
    silver_value: bool,
    common_text: text_input::State,
    common_value: String,
    common_color: style::Theme,
    uncommon_text: text_input::State,
    uncommon_value: String,
    uncommon_color: style::Theme,
    rare_text: text_input::State,
    rare_value: String,
    rare_color: style::Theme,
    land_text: text_input::State,
    land_value: String,
    land_color: style::Theme,
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
        MainWindow {
            silver_value: true,
            common_text: text_input::State::new(),
            common_value: String::from(""),
            common_color: style::Theme::Normal,
            uncommon_text: text_input::State::new(),
            uncommon_value: String::from(""),
            uncommon_color: style::Theme::Normal,
            rare_text: text_input::State::new(),
            rare_value: String::from(""),
            rare_color: style::Theme::Normal,
            land_text: text_input::State::new(),
            land_value: String::from(""),
            land_color: style::Theme::Normal,
            progress_text: String::from("Not Started"),
            progress_value: 0.0,
            run_button: button::State::new()
        }
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
            Message::RunPressed => {
                let mut price_vec: Vec<&str> = Vec::new();
                if self.common_value == "" {
                    price_vec.push("0.1");
                }
                else {
                    price_vec.push(&self.common_value);
                }
                if self.uncommon_value == "" {
                    price_vec.push("0.1");
                }
                else {
                    price_vec.push(&self.uncommon_value);
                }
                if self.rare_value == "" {
                    price_vec.push("0.25");
                }
                else {
                    price_vec.push(&self.rare_value);
                }
                if self.land_value == "" {
                    price_vec.push("0.2");
                }
                else {
                    price_vec.push(&self.land_value);
                }
                let config = super::Config::parse(
                    self.silver_value,
                    price_vec
                );
                match config.config {
                    Some(c) => {
                        self.common_color = style::Theme::Normal;
                        self.uncommon_color = style::Theme::Normal;
                        self.rare_color = style::Theme::Normal;
                        self.land_color = style::Theme::Normal;
                        self.progress_text = String::from("Running");
                        super::run_fetch(c);
                        self.progress_text = String::from("Finished - Saved cards to 'cards.txt'");
                    },
                    None => {
                        self.progress_text = String::from("Failed");
                        if config.errors[0] {
                            self.common_color = style::Theme::Bad;
                        }
                        else {
                            self.common_color = style::Theme::Good;
                        }
                        if config.errors[1] {
                            self.uncommon_color = style::Theme::Bad;
                        }
                        else {
                            self.uncommon_color = style::Theme::Good;
                        }
                        if config.errors[2] {
                            self.rare_color = style::Theme::Bad;
                        }
                        else {
                            self.rare_color = style::Theme::Good;
                        }
                        if config.errors[3] {
                            self.land_color = style::Theme::Bad;
                        }
                        else {
                            self.land_color = style::Theme::Good;
                        }
                    }
                }
            },
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
            .size(18)
            .style(self.common_color);

        let uncommon_input = TextInput::new(
            &mut self.uncommon_text,
            "Uncommon USD Price Limit (Default of 0.10)",
            &self.uncommon_value,
            Message::UncommonChanged,
        )
            .padding(10)
            .size(18)
            .style(self.uncommon_color);

        let rare_input = TextInput::new(
            &mut self.rare_text,
            "Rare USD Price Limit (Default of 0.25)",
            &self.rare_value,
            Message::RareChanged,
        )
            .padding(10)
            .size(18)
            .style(self.rare_color);

        let land_input = TextInput::new(
            &mut self.land_text,
            "Land USD Price Limit (Default of 0.20)",
            &self.land_value,
            Message::LandChanged,
        )
            .padding(10)
            .size(18)
            .style(self.land_color);

        let progress_output = Text::new(&format!("{}", self.progress_text))
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

mod style {
    use iced::text_input;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Normal,
        Good,
        Bad
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Normal
        }
    }

    impl From<Theme> for Box<dyn text_input::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Normal => Default::default(),
                Theme::Good => good::TextInput.into(),
                Theme::Bad => bad::TextInput.into()
            }
        }
    }
    
    mod good {
        use iced::{text_input, Background, Color};

        const BORDER: Color = Color::from_rgb(
            0.0,
            0.75,
            0.0
        );

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: Background::Color(Color::WHITE),
                    border_radius: 5,
                    border_width: 1,
                    border_color: Color::from_rgb(0.7, 0.7, 0.7),
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.7, 0.7, 0.7)
            }

            fn value_color(&self) -> Color {
                BORDER
            }

            fn selection_color(&self) -> Color {
                Color::from_rgb(0.8, 0.8, 1.0)
            }
        }
    }

    mod bad {
        use iced::{text_input, Background, Color};

        const BORDER: Color = Color::from_rgb(
            0.95,
            0.0,
            0.0
        );

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: Background::Color(Color::WHITE),
                    border_radius: 5,
                    border_width: 1,
                    border_color: Color::from_rgb(0.7, 0.7, 0.7),
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.7, 0.7, 0.7)
            }

            fn value_color(&self) -> Color {
                BORDER
            }

            fn selection_color(&self) -> Color {
                Color::from_rgb(0.8, 0.8, 1.0)
            }
        }
    }
}