mod executor;

use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::theme::Theme;
use iced::widget::{column, container, row, scrollable, text, text_input};
use iced::{Application, Command, Element, Length, Settings, Font};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Splash,
    Main,
}

struct GuiShell {
    state: AppState,
    input_value: String,
    output_log: String,
    command_history: Vec<String>,
    splash_duration: Duration,
    splash_start_time: Option<Instant>,
    // splash_image: iced::widget::image::Handle,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SubmitPressed,
    SplashTimeout,
}

impl Application for GuiShell {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GuiShell {
                state: AppState::Splash,
                input_value: String::new(),
                output_log: String::new(),
                command_history: Vec::new(),
                splash_duration: Duration::from_secs(3),
                splash_start_time: Some(Instant::now()),
            },
            Command::perform(async {}, |_| Message::SplashTimeout),
        )
    }

    fn title(&self) -> String {
        String::from("AI Shell")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SplashTimeout => {
                if let Some(start_time) = self.splash_start_time {
                    if start_time.elapsed() >= self.splash_duration {
                        self.state = AppState::Main;
                    } else {
                        // Re-queue the SplashTimeout message
                        return Command::perform(async {}, |_| Message::SplashTimeout);
                    }
                }
            }
            Message::InputChanged(value) => {
                if self.state == AppState::Main {
                    self.input_value = value;
                }
            }
            Message::SubmitPressed => {
                if self.state == AppState::Main {
                    let command = self.input_value.trim().to_string();
                    if !command.is_empty() {
                        // Add command to history
                        self.command_history.push(command.clone());

                        // Execute the command
                        let output = executor::execute_command(&command);

                        // Update the output log
                        self.output_log
                            .push_str(&format!("> {}\n{}\n", command, output));

                        // Clear the input field
                        self.input_value.clear();
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Splash => {
                // Display the splash screen
                container(
                    text("Welcome to AI Shell!")
                        .size(40)
                        .font(Font::MONOSPACE)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
            AppState::Main => self.main_view(),
        }
    }
}

impl GuiShell {
    fn main_view(&self) -> Element<Message> {
        // Create the input field
        let input = text_input("Enter command...", &self.input_value)
            .padding(10)
            .size(16)
            .font(Font::MONOSPACE)
            .on_input(Message::InputChanged)
            .on_submit(Message::SubmitPressed)
            .width(Length::FillPortion(3));

        // Arrange input field and button horizontally
        let input_row = row![input]
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        // Create the output display area
        let output =
            scrollable(text(&self.output_log).size(14).width(Length::Fill).font(Font::MONOSPACE)).height(Length::Fill);

        // Arrange everything vertically
        let content = column![output, input_row]
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    GuiShell::run(Settings::default())
}
