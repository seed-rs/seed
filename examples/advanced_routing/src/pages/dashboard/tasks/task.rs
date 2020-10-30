use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub task_no: u32,
    pub task_title: String,
    pub task_description: String,
}

impl Clone for Model {
    fn clone(&self,) -> Self {
        Model {
            task_no: self.task_no,
            task_title: self.task_title.clone(),
            task_description: self.task_description.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    ClickTask,
}
pub fn view(model: &Model,) -> Node<Msg,> {
    div![
        "Title",
        h3![model.task_title.to_string()],
        p![model.task_description.to_string()]
    ]
}
