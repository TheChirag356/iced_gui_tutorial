// modules
#[allow(unused_imports)]
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Theme;
use iced::{Background, Length, Padding, Shadow, Vector};
use iced::widget::{button, container, text, Button, Column, Container, TextInput, Row};
use iced::{Alignment, Border, Element, Sandbox, Settings};

// Entry point
pub fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

struct RustUI {
    // main variables used in making the instance
    theme: Theme,
    page: Page, // to keep track of pages
    login_field: LoginField,
}

// separate struct for login field
struct LoginField {
    email: String,
    password: String,
}

// enum for Page => Each var inside Page will create a new view/page
#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Login,
    Register,
}

// define message => similar to callbacks
#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,                      // used to toggle Light / Dark theme
    LoginSubmit,                      // to trigger to print email + password to console
    Router(String),                   // change the page depending on route
    LoginFieldChange(String, String), // updates the input fields for email and password
}

// now we implement a Sandbox for RustUI
impl Sandbox for RustUI {
    type Message = Message;

    // app constructor
    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            page: Page::Login,
            login_field: LoginField {
                email: String::new(),
                password: String::new(),
            },
        }
    }

    // defines app title
    fn title(&self) -> String {
        String::from("Rust UI - Iced")
    }

    // define the app theme
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // define the update method
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            Message::LoginFieldChange(email, password) => {
                self.login_field.email = email;
                self.login_field.password = password;
            }
            Message::LoginSubmit => {}
            Message::Router(route) => {
                if route == "Login" {
                    self.page = Page::Login;
                } else if route == "Register" {
                    let _ = self.page == Page::Register;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.page {
            Page::Login => log_in_page(&self.login_field),
            Page::Register => register_page(),
        };

        let wrapper = Column::new()
            .spacing(50)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(content)
            .push(
                match self.page {
                    Page::Login => page_footer(
                        button("Page Two")
                            .on_press(Message::Router("Register".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                    ),
                    Page::Register => page_footer(
                        button("Main Page - Login")
                            .on_press(Message::Router("Login".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                    ),
                }
            );

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(20))
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
            .into()
    }
}

// page footer
fn page_footer(btn: Button<Message>) -> Container<Message> {
    let footer = Row::new().push(
        button("Toggle Theme")
            .on_press(Message::ToggleTheme)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)),
            ),
    )
        .push(btn)
        .align_items(Alignment::Center)
        .spacing(10);

    container(footer).center_x().center_y()
}


// login page
fn log_in_page(login_field: &LoginField) -> Container<Message> {
    let column = Column::new()
        .push(text("Graphical User Interface - Iced!"))
        .push(
            input_field("Email Address... ", &login_field.email)
                .on_input(
                    |email| {
                        Message::LoginFieldChange(email, login_field.password.clone())
                    }
                )
        )
        .push(
            input_field("Password... ", &login_field.password)
                .on_input(
                    |password| {
                        Message::LoginFieldChange(login_field.email.clone(), password)
                    }
                )
        )
        .push(submit_btn("Login", Message::LoginSubmit))
        .padding(Padding::from([50, 20]))
        .align_items(Alignment::Center)
        .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
}


// register page
fn register_page() -> Container<'static, Message> {
    let column = Column::new().push(text("Page Two").size(64));
    container(column)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_y()
        .center_x()
}


// input field
fn input_field(_placeholder: &str, _value: &str) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

// submit button
fn submit_btn(name: &str, event: Message) -> Button<Message> {
    Button::new(
        text(name)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(21),
    )
        .on_press(event)
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        // custom style
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

// button styling
enum ButtonStyle {
    Standard,
    ThemeButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    // define active trait => default
    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.059, 0.463, 0.702),
                Self::ThemeButton => iced::Color::default(),
            })),
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 0.4),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::BLACK,
                        Self::ThemeButton => iced::Color::BLACK,
                    }
                }
            },
            ..Default::default()
        }
    }
}

// define container styling
struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    // active trait
    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: None,
            text_color: Default::default(),
            border: Border::with_radius(5),
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
            ..Default::default()
        }
    }
}
