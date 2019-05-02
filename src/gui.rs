use azul::{prelude::*, window::WindowCreateOptions};

use crate::turing_machine::TuringMachine;
use crate::tape::{Tape};

pub struct TuringMachineApp {
    turing_machine: TuringMachine<bool>,
    elements_qty: usize,
}

impl Layout for TuringMachineApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        //let dom = Dom::from_file("gui.xml", &mut XmlComponentMap::default())
        //    .with_callback(On::MouseUp, Callback(button_pressed));

        let dom = Dom::new(NodeType::Div).with_id("main_screen")
            .with_child(Dom::new(NodeType::Div).with_id("top_bar")
                .with_child(Dom::label("Start")
                    .with_class("button")
                    .with_id("btn_start")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
            .with_child(Dom::label("Stop")
                    .with_class("button")
                    .with_id("btn_stop")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
            .with_child(Dom::label("Step")
                    .with_class("button")
                    .with_id("btn_step")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
            .with_child(Dom::label("Elements +")
                    .with_class("button")
                    .with_id("btn_element_qty_plus")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
            .with_child(Dom::label("Elements -")
                    .with_class("button")
                    .with_id("btn_element_qty_minus")
                    .with_callback(On::MouseUp, Callback(button_pressed))));

        let cells_dom = (0..self.elements_qty).map(|i| NodeData {
            node_type: NodeType::Div,
            classes: vec!["tape_element".into(), "none_cell".into()],
            dynamic_css_overrides: vec![("left_distance".into(), CssProperty::Left(LayoutLeft::px((10 + i*49) as f32)))],
            .. Default::default()
        }).collect::<Dom<Self>>();

        Dom::new(NodeType::Div)
            .with_child(dom)
            .with_child(cells_dom)
    }
}

fn button_pressed(app_state: &mut AppState<TuringMachineApp>, event: &mut CallbackInfo<TuringMachineApp>) -> UpdateScreen {
    if event.target_has_id("btn_element_qty_plus") {
        app_state.data.modify(|state| state.elements_qty += 1)?;
    } else if event.target_has_id("btn_element_qty_minus") {
        app_state.data.modify(|state| state.elements_qty -= 1)?;
    }

    Redraw
}

pub fn gui() {
    let vec: Vec<Option<bool>> = vec![Some(true); 35];

    let mut app = App::new(TuringMachineApp {
        turing_machine: TuringMachine::new(Box::new(Tape::tape(vec))),
        elements_qty: 10,
    }, AppConfig::default()).unwrap();

    let css = css::override_native(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gui.css"))).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Turing RS".to_string();

    let window = app.create_window(window_options, css).unwrap();
    app.run(window).unwrap();
}