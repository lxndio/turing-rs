use azul::{prelude::*};
use std::time::Duration;

pub struct TuringMachineApp {
    
}

impl Layout for TuringMachineApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        Dom::from_file("gui.xml", &mut XmlComponentMap::default())
    }
}

pub fn gui() {
    let mut app = App::new(TuringMachineApp {}, AppConfig::default()).unwrap();
    let window = {
        let hot_reloader = css::hot_reload_override_native("gui.css", Duration::from_millis(500));
        app.create_hot_reload_window(WindowCreateOptions::default(), hot_reloader).unwrap()
    };
    // let window = app.create_window(WindowCreateOptions::default(), css::override_native("gui.css").unwrap()).unwrap();
    app.run(window).unwrap();
}