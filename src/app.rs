use iced::widget::Row;
use iced::widget::Toggler;
use iced::widget::{button, Column, Scrollable, Text, TextInput};
use iced::{Application, Command, Element, Length, Settings, Subscription};
use reqwest::Url;
use scraper::{Html, Selector};
use std::sync::{Arc, Mutex}; // Import at the top of your Rust file

enum ContentPiece {
    Text(String),
    Link(String, String), // Text, Href
}

async fn get_request(input: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(input).await?;
    let body = response.text().await?;

    Ok(body)
}

pub fn run_app(captured_domains: Arc<Mutex<Vec<String>>>) -> iced::Result {
    Home::run(Settings::with_flags(captured_domains.clone()))
}

pub struct Home {
    captured_domains: Arc<Mutex<Vec<String>>>,
    input_state: Arc<Mutex<String>>,
    show_domain_list: bool,
    content_pieces: Vec<ContentPiece>,
    base_domain: String,
}

impl Home {
    pub fn new(captured_domains: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            captured_domains,
            input_state: Arc::new(Mutex::new(String::new())),
            show_domain_list: false, // Initialize the flag to true or false as needed
            content_pieces: Vec::new(),
            base_domain: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    RefreshView,
    InputTextChanged(String), // Rename the message variant
    InputSubmitted,
    ToggleView,
    LinkClicked(String),
}

impl Application for Home {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Arc<Mutex<Vec<String>>>;
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::new(flags.clone()), Command::none())
    }

    fn title(&self) -> String {
        String::from("internet")
    }

    #[tokio::main]
    async fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RefreshView => {
                // Handle the refresh button press here
                // You can add logic to update the captured domains data
                Command::none()
            }
            Message::InputTextChanged(input) => {
                // Update the input_state with the new input
                let mut input_state = self.input_state.lock().unwrap();
                *input_state = input;
                Command::none()
            }
            Message::InputSubmitted => {
                let input = self.input_state.lock().unwrap().to_string(); // Convert to String
                self.base_domain = get_base_domain(&input).unwrap_or_default();

                match get_request(&input).await {
                    Ok(body) => {
                        self.content_pieces = parse_html_content(&body);
                    }
                    Err(error) => {
                        eprintln!("Error: {:?}", error);
                        // Handle the error here
                    }
                }
                Command::none()
            }
            Message::ToggleView => {
                // Toggle the boolean flag when the switch is pressed
                self.show_domain_list = !self.show_domain_list;
                Command::none()
            }
            Message::LinkClicked(mut url) => {
                if url.starts_with("/") {
                    url = format!("{}{}", self.base_domain, url);
                }

                // Perform the same action as in InputSubmitted
                match get_request(&url).await {
                    Ok(body) => {
                        self.content_pieces = parse_html_content(&body);
                        self.base_domain = get_base_domain(&url).unwrap_or_default();
                        // Update the input state with the new URL
                        *self.input_state.lock().unwrap() = url;
                    }
                    Err(error) => {
                        eprintln!("Error fetching URL: {:?}", error);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(self: &Self) -> Element<Self::Message> {
        let captured_domains = self.captured_domains.lock().unwrap();
        let mut content = Column::new().spacing(10);

        let label = if self.show_domain_list {
            "hide captured domain list"
        } else {
            "show captured domain list"
        };

        let toggler = Toggler::new(
            Some(label.to_string()),          // Convert the label to an Option<String>
            self.show_domain_list,            // Pass the boolean flag
            |_new_state| Message::ToggleView, // Closure to handle state change
        );

        let switch_row = Row::new().spacing(10).push(toggler);

        content = content.push(switch_row);

        if self.show_domain_list {
            // Display the domain list sniffer view with a refresh button
            let top_row = Row::new()
                .spacing(10)
                .push(button(Text::new("refresh view")).on_press(Message::RefreshView));

            content = content.push(top_row);

            let mut domain_list = Column::new()
                .spacing(10)
                .align_items(iced::Alignment::Start);

            for domain in captured_domains.iter() {
                domain_list = domain_list.push(Text::new(domain.clone()));
            }

            let scrollable = Scrollable::new(domain_list)
                .width(Length::Fill)
                .height(Length::Fill);

            content = content.push(scrollable);
        } else {
            content = content.push(
                TextInput::new("enter url", &mut *self.input_state.lock().unwrap())
                    .on_input(Message::InputTextChanged)
                    .on_submit(Message::InputSubmitted),
            );

            let mut content_layout = Column::new().spacing(5);
            for piece in &self.content_pieces {
                match piece {
                    ContentPiece::Text(text) => {
                        content_layout = content_layout.push(Text::new(text.clone()));
                    }
                    ContentPiece::Link(text, href) => {
                        let link_text = Text::new(text.clone()).size(16);
                        let link_button =
                            button(link_text).on_press(Message::LinkClicked(href.clone()));
                        content_layout = content_layout.push(link_button);
                    }
                }
            }

            let scrollable_content = Scrollable::new(content_layout)
                .width(Length::Fill)
                .height(Length::Fill);

            content = content.push(scrollable_content);
        }

        Element::from(content)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

fn parse_html_content(html_content: &str) -> Vec<ContentPiece> {
    let document = Html::parse_document(html_content);
    let mut content_pieces = Vec::new();

    // Iterate over all elements
    for element in document.select(&Selector::parse("*").unwrap()) {
        let tag_name = element.value().name();

        if ["script", "style", "head", "meta", "title"].contains(&tag_name) {
            continue;
        }

        if tag_name == "a" {
            // Handle link elements
            let link_text = element.text().collect::<Vec<_>>().join("");
            let href = element.value().attr("href").unwrap_or_default().to_string();
            content_pieces.push(ContentPiece::Link(link_text, href));
        } else if tag_name == "p" {
            // Handle paragraph elements
            let paragraph_text = element.text().collect::<Vec<_>>().join("");
            content_pieces.push(ContentPiece::Text(paragraph_text));
        }
    }

    content_pieces
}

fn get_base_domain(url: &str) -> Option<String> {
    let parsed_url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return None,
    };

    match parsed_url.host_str() {
        Some(host) => Some(format!("{}://{}", parsed_url.scheme(), host)),
        None => None,
    }
}
