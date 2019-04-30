use azul::{prelude::*, widgets::{button::Button}, window::WindowCreateOptions};

use crate::turing_machine::TuringMachine;
use crate::tape::{Tape, Tapeable};

pub struct TuringMachineApp {
    turing_machine: TuringMachine<bool>,
}

impl Layout for TuringMachineApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::from_file("gui.xml", &mut XmlComponentMap::default());
        /*let mut buttons = Vec::new();

        for i in 0..20 {
            buttons.push(Button::with_label(i.to_string()).dom()
                .with_class("tape_element")
                .with_css_override("left_distance", CssProperty::Left(LayoutLeft::px((10 + i*35) as f32))));
        }*/

        let mut cells_dom = self.turing_machine.tape().contents_trim_blanks().iter().enumerate().map(|(i, cell)| NodeData {
            node_type: NodeType::Div,
            classes: vec!["tape_element".into(), match cell {
                Some(true) => "true_cell".into(),
                Some(false) => "false_cell".into(),
                None => "none_cell".into(),
            }],
            dynamic_css_overrides: vec![("left_distance".into(), CssProperty::Left(LayoutLeft::px((10 + i*49) as f32)))],
            .. Default::default()
        }).collect::<Dom<Self>>();

        Dom::new(NodeType::Div)
            .with_child(dom)
            .with_child(cells_dom)
    }
}

pub fn gui() {
    let vec: Vec<Option<bool>> = vec![Some(true); 20];
    let mut app = App::new(TuringMachineApp {
        turing_machine: TuringMachine::new(Box::new(Tape::tape(vec))),
    }, AppConfig::default()).unwrap();
    /*let window = {
        let hot_reloader = css::hot_reload_override_native("gui.css", Duration::from_millis(500));
        app.create_hot_reload_window(WindowCreateOptions::default(), hot_reloader).unwrap()
    };*/
    let css = css::override_native(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gui.css"))).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Turing RS".to_string();
    let window = app.create_window(window_options, css).unwrap();
    app.run(window).unwrap();
}