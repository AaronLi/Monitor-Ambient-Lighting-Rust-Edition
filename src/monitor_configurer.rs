extern crate iced;

use self::iced::{Sandbox, Element};


struct MonitorConfigurer {
    sides_present: [bool; 4]
}

#[derive(Debug, Clone)]
enum Message{
}

impl Default for MonitorConfigurer {
    fn default() -> Self {
        MonitorConfigurer{
            sides_present: [false; 4]
        }
    }
}


impl Sandbox for MonitorConfigurer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Monitor LED Setup Wizard".into()
    }

    fn update(&mut self, _: Self::Message) {
        unimplemented!()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        unimplemented!()
    }

}