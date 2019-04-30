use azul::{prelude::*, widgets::{button::Button}};


pub struct TuringMachineApp {
    
}

impl Layout for TuringMachineApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::from_file("gui.xml", &mut XmlComponentMap::default());
        let mut buttons = Vec::new();

        for i in 0..20 {
            buttons.push(Button::with_label(i.to_string()).dom()
                .with_class("tape_element")
                .with_css_override("left_distance", CssProperty::Left(LayoutLeft::px((10 + i*35) as f32))));
        }

        dom
    }
}

pub fn gui() {
    let mut app = App::new(TuringMachineApp {}, AppConfig::default()).unwrap();
    /*let window = {
        let hot_reloader = css::hot_reload_override_native("gui.css", Duration::from_millis(500));
        app.create_hot_reload_window(WindowCreateOptions::default(), hot_reloader).unwrap()
    };*/
    let css = css::override_native(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gui.css"))).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Turing RS".to_string();
    let window = app.create_window(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
}