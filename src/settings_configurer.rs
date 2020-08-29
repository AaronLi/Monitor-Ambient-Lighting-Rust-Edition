extern crate iced;

use iced::Element;
use iced::HorizontalAlignment;
use iced::pick_list::{self, PickList};
use self::iced::{Container, Sandbox, Length, Row, Align, Text, TextInput, text_input, Column};
use iced::settings::Settings;
use std::ops::Deref;
use crate::baudrate::Baudrate::B500000;
use std::fmt::{Display, Formatter};
use self::iced::window::icon::Icon;
use crate::baudrate::Baudrate;
use std::path::Path;
use image::{ImageBuffer, Rgb, GenericImageView};

pub struct SettingsConfig {
    port_options_state: pick_list::State<String>,
    baudrate_options_state: pick_list::State<Baudrate>,
    refreshrate_state: text_input::State,

    selected_port: String,
    selected_baudrate: Baudrate,
    desired_refreshrate: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    PortSelected(String),
    BaudrateSelected(Baudrate),
    RefreshrateSelected(String),
}

impl Default for SettingsConfig {
    fn default() -> Self {
        SettingsConfig {
            port_options_state: Default::default(),
            baudrate_options_state: Default::default(),
            refreshrate_state: Default::default(),
            selected_port: SettingsConfig::get_serial_port_options().pop().unwrap_or(String::from("")),
            selected_baudrate: Default::default(),
            desired_refreshrate: String::from("15.0"),
        }
    }
}

impl Sandbox for SettingsConfig {
    fn view(&mut self) -> Element<Message> {
        let port_picker = PickList::new(
            &mut self.port_options_state,
            SettingsConfig::get_serial_port_options(),
            Some(self.selected_port.clone()),
            Message::PortSelected,
        ).text_size(20);

        let baudrate_picker = PickList::new(
            &mut self.baudrate_options_state,
            &Baudrate::ALL[..],
            Some(self.selected_baudrate),
            Message::BaudrateSelected,
        ).text_size(20);

        let framerate_selection = TextInput::new(
            &mut self.refreshrate_state,
            "20.0",
            self.desired_refreshrate.as_str(),
            Message::RefreshrateSelected,
        ).width(Length::Units(100));

        let port_baud_row = Row::new()
            .align_items(Align::Start)
            .spacing(10)
            .push(Text::new("Communicate on"))
            .push(port_picker)
            .push(Text::new("at"))
            .push(baudrate_picker)
            .push(Text::new("baud."));
        let framerate_row = Row::new()
            .align_items(Align::Start)
            .spacing(10)
            .push(Text::new("Refresh LEDs at"))
            .push(framerate_selection)
            .push(Text::new("fps."));

        let selection_column = Column::new()
            .push(port_baud_row)
            .push(framerate_row)
            .spacing(20);

        Container::new(selection_column)
            .center_x()
            .center_y()
            .align_x(Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Configure Ambient Lighting Settings")
    }

    fn update(&mut self, message: Self::Message) {
        println!("{:?}", message);
        match message {
            Message::PortSelected(port) => {
                self.selected_port = port;
            }
            Message::BaudrateSelected(baudrate) => {
                self.selected_baudrate = baudrate;
            }
            Message::RefreshrateSelected(refreshrate) => {
                self.desired_refreshrate = refreshrate;
            }
        }
    }
}

impl SettingsConfig {
    fn get_serial_port_options() -> Vec<String> {
        let mut ports_out = Vec::new();
        let available_ports = match serialport::available_ports() {
            Ok(portlist) => portlist,
            Err(_e) => Vec::new()
        };

        for port in available_ports {
            ports_out.push(port.port_name)
        }
        ports_out
    }

    pub fn default_window_settings(path_in_opt: Option<&str>) -> Settings<()> {
        let mut out = Settings::default();
        if path_in_opt.is_some(){
            SettingsConfig::try_set_icon(&mut out, path_in_opt.unwrap());
        }
        out.window.size = (575, 175);
        out.window.resizable = false;
        out.default_text_size = 25;
        out
    }

    fn try_set_icon(settings_in: &mut Settings<()>, icon_path: &str) {
        /*
        Attempts to load an image and set it as the icon
        Does nothing if an error occurs along the line of setting the image
        */
        let path = Path::new(icon_path);
        match image::open(path) {
            Ok(image) => {
                match image.as_rgba8() {
                    Some(data) => {
                        settings_in.window.icon = Icon::from_rgba(data.to_vec(), image.width(), image.height()).ok();
                    },
                    _ => {}
                }
            },
            _ => {}
        };
    }
}