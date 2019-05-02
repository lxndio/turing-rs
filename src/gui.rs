use std::time::Duration;

use azul::{prelude::*, window::WindowCreateOptions};

use crate::turing_machine::{TuringMachine, Transitionable};
use crate::tape::{Tape, Direction};

pub struct TuringMachineApp {
    turing_machine: TuringMachine<bool>,
    tape_elements: Vec<Option<bool>>,
    elements_qty: usize,
    running: bool,
    speed: usize,
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
                    .with_callback(On::MouseUp, Callback(start_stop)))
                .with_child(Dom::label("Stop")
                    .with_class("button")
                    .with_id("btn_stop")
                    .with_callback(On::MouseUp, Callback(start_stop)))
                .with_child(Dom::label("Step")
                    .with_class("button")
                    .with_id("btn_step")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
                .with_child(Dom::label(self.speed.to_string() + "ms")
                    .with_class("button")
                    .with_id("btn_speed")
                    .with_callback(On::MouseUp, Callback(button_pressed)))
                .with_child(Dom::label("Reset")
                    .with_class("button")
                    .with_id("btn_reset")
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
            classes: vec!["tape_element".into(), match self.tape_elements[i] {
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

fn step(state: &mut TuringMachineApp, _: &mut AppResources) -> (UpdateScreen, TerminateTimer) {
    if state.running && state.turing_machine.step() {
        let radius = if state.elements_qty % 2 == 0 { state.elements_qty / 2 } else { (state.elements_qty-1) / 2 };
        state.tape_elements = state.turing_machine.tape().contents_around_head(radius);

        (Redraw, TerminateTimer::Continue)
    } else {
        state.running = false;

        (Redraw, TerminateTimer::Terminate)
    }
}

fn button_pressed(app_state: &mut AppState<TuringMachineApp>, event: &mut CallbackInfo<TuringMachineApp>) -> UpdateScreen {
    let state = &mut app_state.data.lock().ok().unwrap();

    if event.target_has_id("btn_step") {
        state.turing_machine.step();

        let radius = if state.elements_qty % 2 == 0 { state.elements_qty / 2 } else { (state.elements_qty-1) / 2 };
        state.tape_elements = state.turing_machine.tape().contents_around_head(radius);
    } else if event.target_has_id("btn_speed") {
        match state.speed {
            250  => state.speed = 500,
            500  => state.speed = 750,
            750  => state.speed = 1000,
            1000 => state.speed = 1500,
            1500 => state.speed = 2000,
            2000 => state.speed = 250,
            _    => state.speed = 500,
        }
    } else if event.target_has_id("btn_reset") {
        state.running = false;

        reset_turing_machine(state);
    } else if event.target_has_id("btn_element_qty_plus") {
        state.elements_qty += 2;

        let radius = if state.elements_qty % 2 == 0 { state.elements_qty / 2 } else { (state.elements_qty-1) / 2 };
        state.tape_elements = state.turing_machine.tape().contents_around_head(radius);
    } else if event.target_has_id("btn_element_qty_minus") {
        state.elements_qty -= 2;

        let radius = if state.elements_qty % 2 == 0 { state.elements_qty / 2 } else { (state.elements_qty-1) / 2 };
        state.tape_elements = state.turing_machine.tape().contents_around_head(radius);
    }

    Redraw
}

fn start_stop(app_state: &mut AppState<TuringMachineApp>, event: &mut CallbackInfo<TuringMachineApp>) -> UpdateScreen {
    if event.target_has_id("btn_start") {
        if let Some(timer) = {
            let state = &mut app_state.data.lock().ok().unwrap();

            if state.running {
                None
            } else {
                let timer = Timer::new(step).with_interval(Duration::from_millis(state.speed as u64));

                state.running = true;
                Some(timer)
            }
        } {
            app_state.add_timer(TimerId::new(), timer)
        }
    } else if event.target_has_id("btn_stop") {
        let state = &mut app_state.data.lock().ok().unwrap();
        state.running = false;
    }

    Redraw
}

fn reset_turing_machine(state: &mut std::sync::MutexGuard<'_, TuringMachineApp>) {
    let vec: Vec<Option<bool>> = vec![Some(true); 10];

    let mut turing_machine = TuringMachine::new(Box::new(Tape::tape(vec)));
    turing_machine.add_transition((0, Some(true)), (0, Some(false), Direction::Right));
    turing_machine.add_transition((0, Some(false)), (0, Some(true), Direction::Right));
    turing_machine.add_transition((0, None), (0, None, Direction::Hold));

    let tape_elements = turing_machine.tape().contents_around_head(11);

    state.turing_machine = turing_machine;
    state.tape_elements = tape_elements;
    state.elements_qty = 11;
}

pub fn gui() {
    let vec: Vec<Option<bool>> = vec![Some(true); 10];

    let mut turing_machine = TuringMachine::new(Box::new(Tape::tape(vec)));
    turing_machine.add_transition((0, Some(true)), (0, Some(false), Direction::Right));
    turing_machine.add_transition((0, Some(false)), (0, Some(true), Direction::Right));
    turing_machine.add_transition((0, None), (0, None, Direction::Hold));

    let tape_elements = turing_machine.tape().contents_around_head(11);

    let mut app = App::new(TuringMachineApp {
        turing_machine,
        tape_elements,
        elements_qty: 11,
        running: false,
        speed: 500,
    }, AppConfig::default()).unwrap();

    let css = css::override_native(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gui.css"))).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Turing RS".to_string();

    let window = app.create_window(window_options, css).unwrap();
    app.run(window).unwrap();
}