extern crate systray;

use std::sync::Condvar;
use std::io::{stdout, Write};
use systray::{Error};
use orbtk::prelude::*;
use std::thread;

pub fn setup_application(app: &mut systray::Application){
    // The app won't do any events unless you tell it to wait for messages
    // TODO: run monitor scanning and output segments in separate thread
    // app.quit() quits the taskbar process and it will have to be reconstructed

}