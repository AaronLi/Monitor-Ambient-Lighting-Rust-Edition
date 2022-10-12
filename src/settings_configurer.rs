extern crate iced;

use self::iced::{Container, Sandbox, Length, Row, Alignment, Text, TextInput, text_input, Column, Element, pick_list, PickList, Button, button};
use iced::settings::Settings;
use self::iced::window::icon::Icon;
use std::path::Path;
use image::{ImageError};
use std::process::exit;
use iced::alignment::Horizontal;
use crate::{baudrate, program_config};

#[derive(Clone)]
pub struct SettingsConfigurer {
    port_options_state: pick_list::State<String>,
    baudrate_options_state: pick_list::State<baudrate::Baudrate>,
    refreshrate_state: text_input::State,
    save_path_state: text_input::State,
    save_button_state: button::State,
    reset_button_state: button::State,
    ok_button_state: button::State,
    cancel_button_state: button::State,

    current_values_index: usize,
    previous_states: Vec<FieldValues>,
}

#[derive(Clone, PartialEq)]
struct FieldValues{
    selected_port: String,
    selected_baudrate: baudrate::Baudrate,
    desired_refreshrate: String,
    save_file_path: String,
    config_state: ConfigState,
}

#[derive(Debug, Clone)]
pub enum Message {
    PortSelected(String),
    BaudrateSelected(baudrate::Baudrate),
    RefreshrateSelected(String),
    FilePathChanged(String),
    SaveFile,
    ResetSettings,
    SaveAndExit,

}

#[derive(Clone, PartialEq)]
enum ConfigState{
    NoChanges,
    UnsavedChanges,
}

impl Default for SettingsConfigurer {
    fn default() -> Self {
        let default_info = program_config::ProgramConfiguration::load_from_file("assets/program_configuration.json");
        SettingsConfigurer {
            port_options_state: Default::default(),
            baudrate_options_state: Default::default(),
            refreshrate_state: Default::default(),
            save_path_state: Default::default(),
            save_button_state: Default::default(),
            reset_button_state: Default::default(),
            ok_button_state: Default::default(),
            cancel_button_state: Default::default(),
            current_values_index: 0,
            previous_states: vec![
                FieldValues {
                    selected_port: default_info.serial_port,
                    selected_baudrate: default_info.baudrate,
                    desired_refreshrate: format!("{:.2}", default_info.refresh_rate),
                    save_file_path: String::from("assets/program_configuration.json"),
                    config_state: ConfigState::NoChanges
                }
            ]
        }
    }
}

impl Sandbox for SettingsConfigurer {
    fn view(&mut self) -> Element<Message> {
        let render_values = self.get_values();

        let port_picker = PickList::new(
            &mut self.port_options_state,
            SettingsConfigurer::get_serial_port_options(),
            Some(render_values.selected_port),
            Message::PortSelected,
        )
            .text_size(20);

        let baudrate_picker = PickList::new(
            &mut self.baudrate_options_state,
            &baudrate::Baudrate::ALL[..],
            Some(render_values.selected_baudrate),
            Message::BaudrateSelected,
        )
            .text_size(20);

        let framerate_selection = TextInput::new(
            &mut self.refreshrate_state,
            "20.0",
            render_values.desired_refreshrate.as_str(),
            Message::RefreshrateSelected,
        )
            .width(Length::Units(100));

        let save_file_line = TextInput::new(
            &mut self.save_path_state,
            "assets/program_configuration.json",
            render_values.save_file_path.as_str(),
            Message::FilePathChanged
        )
            .width(Length::Units(400));

        let save_button = Button::new(
            &mut self.save_button_state,
            Text::new("Save")
        ).on_press(Message::SaveFile);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
        ).on_press(Message::ResetSettings);

        let ok_button = Button::new(
            &mut self.ok_button_state,
            Text::new("OK")
        ).on_press(Message::SaveAndExit);



        let port_baud_row = Row::new()
            .align_items(Alignment::Start)
            .spacing(10)
            .push(Text::new("Communicate on"))
            .push(port_picker)
            .push(Text::new("at"))
            .push(baudrate_picker)
            .push(Text::new("baud."));
        let framerate_row = Row::new()
            .align_items(Alignment::Start)
            .spacing(10)
            .push(Text::new("Refresh LEDs at"))
            .push(framerate_selection)
            .push(Text::new("hz."));

        let save_file_row = Row::new()
            .align_items(Alignment::Start)
            .spacing(10)
            .push(Text::new("Save to"))
            .push(save_file_line);

        let save_reset_cancel_row = Row::new()
            .push(save_button)
            .push(reset_button)
            .push(ok_button)
            .spacing(15)
            .align_items(Alignment::End);

        let selection_column = Column::new()
            .push(port_baud_row)
            .push(framerate_row)
            .push(save_file_row)
            .push(save_reset_cancel_row)
            .spacing(20);

        Container::new(selection_column)
            .center_x()
            .center_y()
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        match self.get_values().config_state{
            ConfigState::UnsavedChanges => {
                String::from("Configure Ambient Lighting Settings | Unsaved Changes")
            },
            ConfigState::NoChanges => {
                String::from("Configure Ambient Lighting Settings")
            }
        }
    }

    fn update(&mut self, message: Self::Message) {
        let mut new_state = self.get_values();
        match message {
            Message::PortSelected(port) => {
                new_state.selected_port = port;
            }
            Message::BaudrateSelected(baudrate) => {
                new_state.selected_baudrate = baudrate;
            }
            Message::RefreshrateSelected(refreshrate) => {
                new_state.desired_refreshrate = refreshrate;
            }
            Message::FilePathChanged(path) => {
                new_state.save_file_path = path;
            }

            Message::SaveFile => {
                self.save_config();
                // the current state is the new default
                self.previous_states = self.previous_states.split_off(self.current_values_index);
                self.current_values_index = 0;
            }

            Message::ResetSettings => {
                // the 0th index is the last setting configuration that can be reset to
                self.current_values_index = 0;
                // remove the extra values
                self.previous_states.truncate(1);
                // the 0th index should by default be saved already
                return
            }
            Message::SaveAndExit => {
                self.save_config();
                exit(0);
            }
        }
        new_state.config_state = ConfigState::NoChanges;
        if new_state != self.get_values()
        {
            new_state.config_state = ConfigState::UnsavedChanges;
        }

        self.current_values_index+=1;
        self.previous_states.truncate(self.current_values_index);
        self.previous_states.push(new_state);
    }
}

impl SettingsConfigurer {
    fn get_values(&self) -> FieldValues{
        self.previous_states[self.current_values_index].clone()
    }

    fn save_config(&mut self) -> bool{
        /*
        Does not handle undo/redo stack, only manages file saving
        */
        let current_values = self.get_values();
        if !current_values.save_file_path.is_empty() {
            let current_config_state = self.get_current_configuration();
            current_config_state.save_to_file(Path::new(&current_values.save_file_path));
            true
        }
        else {
            false
        }
    }
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

    pub fn get_current_configuration(&self) -> program_config::ProgramConfiguration {
        let current_values = self.get_values();
        program_config::ProgramConfiguration {
            serial_port: current_values.selected_port,
            refresh_rate: current_values.desired_refreshrate.parse().unwrap(),
            baudrate: current_values.selected_baudrate
        }
    }

    pub fn default_window_settings(path_in_opt: Option<&str>) -> Settings<()> {
        let mut out = Settings::default();
        if path_in_opt.is_some(){
            SettingsConfigurer::try_set_icon(&mut out, path_in_opt.unwrap()).unwrap();
        }
        out.window.size = (570, 230);
        out.window.resizable = false;
        out.default_text_size = 25;
        out
    }

    fn try_set_icon(settings_in: &mut Settings<()>, icon_path: &str) -> Result<(), ImageError> {
        /*
        Attempts to load an image and set it as the icon
        Does nothing if an error occurs along the line of setting the image
        */
        let path = Path::new(icon_path);
        let image = image::open(path)?.to_rgb8();
        settings_in.window.icon = Icon::from_rgba(image.to_vec(), image.width(), image.height()).ok();

        Ok(())
    }
}