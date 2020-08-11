#![allow(non_snake_case)]

//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gio;
extern crate glib;
extern crate gtk;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::Builder;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("hackerify.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Couldn't add from string");

    let window: gtk::Window = builder.get_object("mainWindow").expect("Couldn't get window");
    window.set_application(Some(application));
    let saveButton: gtk::Button = builder
        .get_object("saveButton")
        .expect("Couldn't get builder");
    let textView: gtk::TextView = builder
        .get_object("mainText")
        .expect("Couldn't get text_view");
	let textBuffer: gtk::TextBuffer = builder
        .get_object("textBuffer")
        .expect("Couldn't get text buffer");

	saveButton.connect_clicked(clone!(@weak window => move |_| {
		// let startBuffer @ endBuffer = buffer.get_bounds();
		// let actualText = buffer.get_text(startBuffer, endBuffer, false);
		let finalText = textBuffer.get_text(&textBuffer.get_start_iter(), &textBuffer.get_end_iter(), true);
		println!("{:#?}", finalText);
	}));

	//
    // open_button.connect_clicked(clone!(@weak window => move |_| {
    //     // TODO move this to a impl?
    //     let file_chooser = gtk::FileChooserDialog::new(
    //         Some("Open File"),
    //         Some(&window),
    //         gtk::FileChooserAction::Open,
    //     );
    //     file_chooser.add_buttons(&[
    //         ("Open", gtk::ResponseType::Ok),
    //         ("Cancel", gtk::ResponseType::Cancel),
    //     ]);
    //     file_chooser.connect_response(clone!(@weak text_view => move |file_chooser, response| {
    //         if response == gtk::ResponseType::Ok {
    //             let filename = file_chooser.get_filename().expect("Couldn't get filename");
    //             let file = File::open(&filename).expect("Couldn't open file");
	//
    //             let mut reader = BufReader::new(file);
    //             let mut contents = String::new();
    //             let _ = reader.read_to_string(&mut contents);
	//
    //             text_view
    //                 .get_buffer()
    //                 .expect("Couldn't get window")
    //                 .set_text(&contents);
    //         }
    //         file_chooser.close();
    //     }));
	//
    //     file_chooser.show_all();
    // }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.daniel071.hackerify"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
