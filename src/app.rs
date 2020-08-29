extern crate systray;

use std::thread;

pub fn setup_application(app: &mut systray::Application){
    // The app won't do any events unless you tell it to wait for messages
    // TODO: run monitor scanning and output segments in separate thread
    // app.quit() quits the taskbar process and it will have to be reconstructed
    app.set_icon_from_file("assets/icon.ico").expect("Unable to set icon for menu");

    app.add_menu_item("Quit", |application| {
        application.quit();
        Ok::<_, systray::Error>(())
    }).expect("Unable to add quit button to menu");
}