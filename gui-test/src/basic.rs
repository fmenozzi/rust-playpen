extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

pub fn run() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, GTK!");
    window.set_default_size(400, 200);

    let button = Button::new_with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
