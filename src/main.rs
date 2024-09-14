mod executor;

use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::theme::Theme;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Application, Command, Element, Length, Settings};
use iced::window;
use iced::window::icon;
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
                // splash_image: iced::widget::image::Handle::from_memory(include_bytes!("../resources/splash.png").as_ref()),
            },
            Command::perform(async {}, |_| Message::SplashTimeout),
        )
    }

    fn title(&self) -> String {
        String::from("AI Terminal")
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
                        self.output_log.push_str(&format!("> {}\n{}\n", command, output));

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
                    text("Welcome to AI Terminal!")
                        .size(40)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()

                // If using an image:
                /*
                container(
                    image(&self.splash_image)
                        .width(Length::Units(400))
                        .height(Length::Units(400)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
                */
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
            .on_input(Message::InputChanged)
            .on_submit(Message::SubmitPressed)
            .width(Length::FillPortion(3));

        // Create the submit button
        let submit_button = button("Run")
            .on_press(Message::SubmitPressed)
            .padding(10)
            .width(Length::Shrink);

        // Arrange input field and button horizontally
        let input_row = row![input, submit_button]
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        // Create the output display area
        let output = scrollable(
            text(&self.output_log)
                .size(14)
                .width(Length::Fill),
        )
        .height(Length::Fill);

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
    // Load the icon image from the resources directory
    // let icon_bytes = include_bytes!("../resources/icon.png");

    // // Decode the image using the `image` crate
    // let image = image::load_from_memory(icon_bytes).expect("Failed to load icon image");

    // // Convert the image to RGBA8 format
    // let image = image.to_rgba8();

    // // Get the dimensions of the image
    // let (width, height) = image.dimensions();

    // // Get the raw RGBA pixel data
    // let pixels = image.into_raw();

    // // Create the window icon
    // let icon = icon::from_rgba(pixels, width, height).expect("Failed to create icon");

    // // Run the application with the window icon set
    // GuiShell::run(Settings {
    //     window: window::Settings {
    //         icon: Some(icon),
    //         ..window::Settings::default()
    //     },
    //     ..Settings::default()
    // })

     // If not setting the icon programmatically:
     GuiShell::run(Settings::default())
}
