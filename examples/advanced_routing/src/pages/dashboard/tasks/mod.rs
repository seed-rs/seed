use crate::{pages::dashboard::DashboardRoutes, Routes};
use seed::{prelude::*, *};
use seed_routing::*;
pub mod task;
pub fn init(
    _: Url,
    model: &mut Model,
    query: &IndexMap<String, String>,
    _: &TasksRoutes,
    _: &mut impl Orders<Msg>,
) -> Model {
    if !model.is_default {
        Model {
            tasks: get_dummy_data(),
            selected_task_no: model.selected_task_no.clone(),
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
            selected_task_no: selected_no,
            is_default: false,
        };
        init
    }
}
#[derive(Clone)]
pub struct Model {
    pub tasks: Vec<task::Model>,
    pub selected_task_no: Vec<u32>,
    pub is_default: bool,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            selected_task_no: vec![],
            tasks: vec![],
            is_default: true,
        }
    }
}
#[derive(Debug, PartialEq, Clone, AsUrl)]
pub enum TasksRoutes {
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClickTask(task_no, will_uncheck) => {
            if will_uncheck {
                if let Some(index) = model.selected_task_no.iter().position(|no| no == &task_no) {
                    model.selected_task_no.remove(index);
                }
            } else if let Some(index) = model.selected_task_no.iter().position(|no| no == &task_no)
            {
            } else {
                model.selected_task_no.push(task_no)
            }
            //
            // let mut iter = &model.selected_task_no.iter().map(|i| i.to_string());
            // let string_vec: Vec<String> = iter.clone().collect();
            // let query: UrlSearch = UrlSearch::new(vec![("select", string_vec)]);
            // orders.notify(subs::UrlRequested::new(Url::current().set_search(query)));
        }
        Msg::LoadTasks => model.tasks = get_dummy_data(),
        Msg::Task(task) => {

            // let index: usize = model.selected_task_no.unwrap() as usize;
            // task::update(task, model.tasks.get_mut(index).unwrap())
        }
    }
}
// pub fn view(model: &Model, router: &SuperRouter<Routes>) -> Node<Msg> {
//     div!["my tasks", render_tasks(model, router),]
// }

fn render_tasks(model: &Model) -> Node<Msg> {
    ul![list(&model.tasks, &model.selected_task_no)]
}

pub fn list(tasks: &[task::Model], list: &[u32]) -> Vec<Node<Msg>> {
    let mut tasks_list = Vec::new();
    for t in tasks {
        tasks_list.push(render_task(t, list.contains(&t.task_no)));
    }
    tasks_list
}

pub fn render_task(task: &task::Model, is_checked: bool) -> Node<Msg> {
    let task_url = Routes::Dashboard(DashboardRoutes::Tasks {
        children: TasksRoutes::Task {
            id: task.task_no.to_string(),
        },
        query: IndexMap::new(),
    })
    .to_url();
    let task_no = task.task_no;
    // let route =
    // Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Task(task.task_no)));
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
pub fn view(task_routes: &TasksRoutes, model: &Model) -> Node<Msg> {
    div![vec![
        render_tasks(model),
        match task_routes {
            TasksRoutes::Task { id } => {
                let task = model.tasks.iter().find(|t| t.task_no.to_string() == *id);
                task::view(task.unwrap()).map_msg(Msg::Task)
            }
            TasksRoutes::Root => div!["no task selected"],
        },
    ]]
}
