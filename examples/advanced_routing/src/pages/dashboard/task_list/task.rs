use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub task_no: u32,
    pub task_title: String,
    pub task_description: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    ClickTask,
}
pub fn update(msg: Msg, _: &mut Model) {
    match msg {
        Msg::ClickTask => {}
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    div![
        "Title",
        h3![model.task_title.to_string()],
        p![model.task_description.to_string()]
    ]
}
