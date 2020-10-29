use crate::Routes as Root;

use crate::pages::dashboard::Routes as Parent;
use seed::{prelude::*, *};
use seed_routing::*;

pub mod task;
pub fn init(
    _: Url,
    model: &mut Model,
    query: &IndexMap<String, String>,
    _: &Routes,
    _: &mut impl Orders<Msg>,
) -> Model {
    if !model.is_default {
        Model {
            tasks: get_dummy_data(),
            checked_tasks_no: model.checked_tasks_no.clone(),
            is_default: false,
        }
    } else {
        let mut selected_no: Vec<u32> = vec![];
        for selected in query.iter() {
            if selected.0.contains("select") {
                let no: u32 = selected
                    .1
                    .parse()
                    .expect("expect value from query parameters");
                selected_no.push(no)
            }
        }

        let init = Model {
            tasks: get_dummy_data(),
            checked_tasks_no: selected_no,
            is_default: false,
        };
        init
    }
}
#[derive(Clone)]
pub struct Model {
    pub tasks: Vec<task::Model>,
    pub checked_tasks_no: Vec<u32>,
    pub is_default: bool,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            checked_tasks_no: vec![],
            tasks: vec![],
            is_default: true,
        }
    }
}
#[derive(Debug, PartialEq, Clone, AsUrl)]
pub enum Routes {
    Task { id: String },
    //     #[as_path = ""] this makes run time error
    Root,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    ClickTask(u32, bool),
    Task(task::Msg),
    LoadTasks,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClickTask(task_no, will_uncheck) => {
            if will_uncheck {
                if let Some(index) = model.checked_tasks_no.iter().position(|no| no == &task_no) {
                    model.checked_tasks_no.remove(index);
                }
            } else if let Some(_) = model.checked_tasks_no.iter().position(|no| no == &task_no) {
            } else {
                model.checked_tasks_no.push(task_no)
            }
        }
        Msg::LoadTasks => model.tasks = get_dummy_data(),
        Msg::Task(_) => {}
    }
}

fn render_tasks(model: &Model) -> Node<Msg> {
    ul![list(&model.tasks, &model.checked_tasks_no)]
}

pub fn list(tasks: &[task::Model], list: &[u32]) -> Vec<Node<Msg>> {
    let mut tasks_list = Vec::new();
    for t in tasks {
        tasks_list.push(render_task(t, list.contains(&t.task_no)));
    }
    tasks_list
}

pub fn render_task(task: &task::Model, is_checked: bool) -> Node<Msg> {
    let task_url = Root::Dashboard(Parent::Tasks {
        children: Routes::Task {
            id: task.task_no.to_string(),
        },
        query: IndexMap::new(),
    })
    .to_url();

    let task_no = task.task_no;
    li![div![
        input![
            attrs! {
            At::Checked => is_checked.as_at_value(),
            At::Id=>  &task.task_no.to_string(),
            At::Type=> "checkbox"
                    },
            ev(Ev::Click, move |_| Msg::ClickTask(task_no, is_checked)),
        ],
        a![
            C![
                "route",
                IF!(is_current_url(task_url.clone()) => "active-route")
            ],
            attrs! { At::Href => task_url},
            task.task_title.to_string(),
        ]
    ]]
}
fn is_current_url(url: Url) -> bool {
    Url::current() == url
}
pub fn get_dummy_data() -> Vec<task::Model> {
    vec![
        task::Model {
            task_no: 0,
            task_title: "Nested Url".to_string(),
            task_description: "Try to find an easy way to manipulate nested route".to_string(),
        },
        task::Model {
            task_no: 1,
            task_title: "Guard & permission".to_string(),
            task_description: "FInd a way to set Guard for protected routes".to_string(),
        },
        task::Model {
            task_no: 2,
            task_title: "Stuff".to_string(),
            task_description: "Additional stuff to do".to_string(),
        },
    ]
}
pub fn view(task_routes: &Routes, model: &Model) -> Node<Msg> {
    div![vec![
        render_tasks(model),
        match task_routes {
            Routes::Task { id } => {
                let task = model.tasks.iter().find(|t| t.task_no.to_string() == *id);
                task::view(task.unwrap()).map_msg(Msg::Task)
            }
            Routes::Root => div!["no task selected"],
        },
    ]]
}
