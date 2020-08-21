extern crate systray;

use std::io::{stdout, Write};

pub fn setup_application(app: &mut systray::Application){
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
}

pub fn capture_send_loop(){
    // Runs indefinitely until a signal is given for it to exit
    loop{
        
    }
}