use iced::{Sandbox, Settings};

//mod model;
use crate::model::menu_gui::MyApp;

pub fn main() -> iced::Result {
    MyApp::run(Settings {
      window: iced::window::Settings {
         size: iced::Size::new(800 as f32, 700 as f32),  // Imposta la dimensione della finestra
         resizable: true,   // Permette di ridimensionare la finestra
         ..Default::default()
      },
      ..Settings::default()
   }).expect("Errore");

    Ok(())
}