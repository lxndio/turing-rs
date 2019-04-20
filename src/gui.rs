use azul::{prelude::*, widgets::{label::Label}};

pub struct TuringMachineApp {
    
}

impl Layout for TuringMachineApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("Hello World")).dom()
            .with_class("tape_elem");

        Dom::new(NodeType::Div)
            .with_class("main_window")
            .with_child(label)
    }
}

pub fn gui() {
    let mut app = App::new(TuringMachineApp {}, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::override_native(WIN_CSS).unwrap()).unwrap();
    app.run(window).unwrap();
}

// TODO probably outsource to seperate file
const WIN_CSS: &str = "\
.main_window {
    background-color: rgb(250, 250, 250);
}

.tape_elem {
    border: 1px solid rgb(5, 5, 5);
}
";