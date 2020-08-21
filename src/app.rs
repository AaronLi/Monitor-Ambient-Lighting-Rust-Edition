extern crate systray;

use std::io::{stdout, Write};

pub fn setup_application(app: &mut systray::Application){
    app.set_icon_from_file("C:\\Users\\dumpl\\IdeaProjects\\monitor_ambient_lighting_rust_edition\\assets\\icon.png");

    app.add_menu_item("Blah", |_|{
        println!("Click");
        Ok::<_, systray::Error>(())
    });
}