use azul::{prelude::*, widgets::{label::Label, button::Button}};

pub struct TuringMachineApp {
	
}

impl Layout for TuringMachineApp {
	fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
		let label = Label::new(format!("Hello World")).dom();

		Dom::new(NodeType::Div)
			.with_child(label)
	}
}