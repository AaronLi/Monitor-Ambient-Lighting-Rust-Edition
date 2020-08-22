extern crate systray;

use std::sync::Condvar;
use std::io::{stdout, Write};
use std::thread;
use std::thread::JoinHandle;
use systray::Error;

pub fn setup_application(app: &'static mut systray::Application) -> JoinHandle<Result<(), Error>> {
    // The app won't do any events unless you tell it to wait for messages
    // TODO: run monitor scanning and output segments in separate thread
    // app.quit() quits the taskbar process and it will have to be reconstructed
    match app.set_icon_from_file("assets/icon.ico"){
        Ok(_) => {},
        Err(error) => {println!("Could not set icon: {}", error)}
    }

    app.add_menu_item("Blah", |v|{
        println!("Click");
        Ok::<_, systray::Error>(())
    });

    thread::spawn(move ||{app.wait_for_message()})
}