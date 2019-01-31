//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

#[macro_use]
extern crate seed;
use seed::prelude::*;
use serde::{Deserialize, Serialize};

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone, Copy, PartialEq)]
enum Visible {
    All,
    Active,
    Completed,
}

impl Visible {
    fn to_string(self) -> String {
        match self {
            Visible::All => "".into(),
            Visible::Active => "active".into(),
            Visible::Completed => "completed".into(),
        }
    }
}

// Model

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool,
    editing: bool,
}

impl Todo {
    fn visible(&self, visible: &Visible) -> bool {
        match visible {
            Visible::All => true,
            Visible::Active => !self.completed,
            Visible::Completed => self.completed,
        }
    }
}

#[derive(Clone)]
struct Model {
    todos: Vec<Todo>,
    visible: Visible,
    entry_text: String,
    edit_text: String,
    local_storage: web_sys::Storage,
}

impl Model {
    fn completed_count(&self) -> i32 {
        let completed: Vec<&Todo> = self.todos.iter().filter(|i| i.completed).collect();
        completed.len() as i32
    }

    fn active_count(&self) -> i32 {
        // By process of elimination; active means not completed.
        self.todos.len() as i32 - self.completed_count()
    }

    fn sync_storage(&self) {
        // todo: Every item that adds, deletes, or changes a today re-serializes and stores
        // todo the whole model. Effective, but probably quite slow!
        seed::storage::store_data(&self.local_storage, "seed-todo-data", &self.todos);
    }
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let local_storage = seed::storage::get_storage().unwrap();

        //        let todos: Vec<Todo> = match local_storage.get_item("seed-todo-data") {
        //            Some(Ok(tds)) => {
        //                serde_json::from_str(&tds).unwrap()
        //            },
        //            None => Vec::new(),
        //        };

        //        let x: String = local_storage.get_item("seed-todo-data").unwrap().unwrap();
        //        let todos: Vec<Todo> = serde_json::from_str(&x).unwrap();
        //
        //
        let todos = Vec::new();

        Self {
            todos,
            visible: Visible::All,
            entry_text: String::new(),
            edit_text: String::new(),
            local_storage,
        }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    // usize here corresponds to indicies of todos in the Vec they live in.
    ClearCompleted,
    Destroy(usize),
    Toggle(usize),
    ToggleAll,
    NewTodo(web_sys::Event),
    SetVisibility(seed::App<Msg, Model>, Visible),
    EditEntry(String),

    EditItem(usize),
    EditSubmit(usize),
    EditChange(String),
    EditKeyDown(usize, u32), // item position, keycode

    RoutePage(Visible),
}

fn update(msg: Msg, model: Model) -> Update<Model> {
    // We take a verbose immutable-design/functional approach in this example.
    // Alternatively, you could re-declare model as mutable at the top, and mutate
    // what we need in each match leg. See the Update section of the guide for details.
    model.sync_storage(); // Doing it here will miss the most recent update...
    match msg {
        Msg::ClearCompleted => {
            let todos = model.todos.into_iter().filter(|t| !t.completed).collect();
            Render(Model { todos, ..model })
        }
        Msg::Destroy(posit) => {
            let todos = model
                .todos
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i != &posit)
                // We only used the enumerate to find the right todo; remove it.
                .map(|(_, t)| t)
                .collect();
            Render(Model { todos, ..model })
        }
        Msg::Toggle(posit) => {
            let mut todos = model.todos;
            let mut todo = todos.remove(posit);
            todo.completed = !todo.completed;
            todos.insert(posit, todo);

            Render(Model { todos, ..model })
        }
        Msg::ToggleAll => {
            // Mark all as completed, unless all are: mark all as not completed.
            let completed = model.active_count() != 0;
            let todos = model
                .todos
                .into_iter()
                .map(|t| Todo { completed, ..t })
                .collect();
            Render(Model { todos, ..model })
        }
        Msg::NewTodo(ev) => {
            // Add a todo_, if the enter key is pressed.
            // We handle text input after processing a key press, hence the
            // raw event logic here.
            let code = seed::to_kbevent(&ev).key_code();
            if code != ENTER_KEY {
                return Render(model);
            }
            ev.prevent_default();

            let target = ev.target().unwrap();
            let input_el = seed::to_input(&target);
            let title = input_el.value().trim().to_string();

            if !title.is_empty() {
                let mut todos = model.todos.clone();
                todos.push(Todo {
                    title,
                    completed: false,
                    editing: false,
                });
                input_el.set_value("");
                Render(Model { todos, ..model })
            } else {
                Render(model)
            }
        }
        Msg::SetVisibility(state, visible) => {
            Render(seed::push_route(state, &visible.to_string(), Msg::RoutePage(visible)))
        }
        Msg::EditEntry(entry_text) => Render(Model{ entry_text, ..model }),

        Msg::EditItem(posit) => {
            let mut todos: Vec<Todo> = model
                .todos
                .into_iter()
                .map(|t| Todo {
                    editing: false,
                    ..t
                })
                .collect();

            let mut todo = todos.remove(posit);
            todo.editing = true;
            todos.insert(posit, todo.clone());

            Render(Model {
                todos,
                edit_text: todo.title,
                ..model
            })
        }
        Msg::EditSubmit(posit) => {
            if model.edit_text.is_empty() {
                update(Msg::Destroy(posit), model)
            } else {
                let mut todos = model.todos;
                let mut todo = todos.remove(posit);
                todo.editing = false;
                todo.title = model.edit_text.clone();
                todos.insert(posit, todo);

                Render(Model {
                    todos,
                    edit_text: model.edit_text.trim().to_string(),
                    ..model
                })
            }
        }
        Msg::EditChange(edit_text) => Render(Model { edit_text, ..model }),
        Msg::EditKeyDown(posit, code) => {
            if code == ESCAPE_KEY {
                let todos = model
                    .todos
                    .clone()
                    .into_iter()
                    .map(|t| Todo {
                        editing: false,
                        ..t
                    })
                    .collect();
                Render(Model {
                    todos,
                    edit_text: model.todos[posit].title.clone(),
                    ..model
                })
            } else if code == ENTER_KEY {
                update(Msg::EditSubmit(posit), model)
            } else {
                Render(model)
            }
        }

        Msg::RoutePage(visible) => Render(Model { visible, ..model }),
    }
}

// View

fn todo_item(item: Todo, posit: usize, edit_text: String) -> El<Msg> {
    let mut att = attrs! {};
    if item.completed {
        att.add(At::Class, "completed");
    }
    if item.editing {
        att.add(At::Class, "editing");
    }

    li![
        att,
        div![
            class!["view"],
            input![
                attrs! {At::Class => "toggle"; At::Type => "checkbox"; At::Checked => item.completed },
                simple_ev(Ev::Click, Msg::Toggle(posit))
            ],
            label![simple_ev(Ev::DblClick, Msg::EditItem(posit)), item.title],
            button![
                class!["destroy"],
                simple_ev(Ev::Click, Msg::Destroy(posit))
            ]
        ],
        if item.editing {
            input![
                attrs! {At::Class => "edit"; At::Value => edit_text},
                simple_ev(Ev::Blur, Msg::EditSubmit(posit)),
                input_ev(Ev::Input, Msg::EditChange),
                keyboard_ev(Ev::KeyDown, move |ev| Msg::EditKeyDown(posit, ev.key_code())),
            ]
        } else {
            seed::empty()
        }
    ]
}

fn selection_li(state: seed::App<Msg, Model>, text: &str, visible: Visible, highlighter: Visible) -> El<Msg> {
    li![a![
        class![if visible == highlighter {
            "selected"
        } else {
            ""
        }],
        style! {"cursor" => "pointer"},
        simple_ev(Ev::Click, Msg::SetVisibility(state, highlighter)),
        text
    ]]
}

fn footer(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let optional_s = if model.todos.len() == 1 { "" } else { "s" };

    let clear_button = if model.completed_count() > 0 {
        button![
            class!["clear-completed"],
            simple_ev(Ev::Click, Msg::ClearCompleted),
            "Clear completed"
        ]
    } else {
        seed::empty()
    };

    footer![
        class!["footer"],
        span![
            class!["todo-count"],
            strong![model.active_count().to_string()],
            span![format!(" item{} left", optional_s)]
        ],
        ul![
            class!["filters"],
            selection_li(state.clone(), "All", model.visible, Visible::All),
            selection_li(state.clone(), "Active", model.visible, Visible::Active),
            selection_li(state, "Completed", model.visible, Visible::Completed)
        ],
        clear_button
    ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn todo_app(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    // We use the item's position in model.todos to identify it, because this allows
    // simple in-place modification through indexing. This is different from its
    // position in visible todos, hence the two-step process.
    let todo_els: Vec<El<Msg>> = model
        .todos
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_posit, todo)| todo.visible(&model.visible))
        .map(|(posit, todo)| todo_item(todo, posit, model.edit_text.clone()))
        .collect();

    let main = if !model.todos.is_empty() {
        section![
            class!["main"],
            input![
                attrs! {At::Id => "toggle-all"; At::Class => "toggle-all"; At::Type => "checkbox";
                At::Checked => model.active_count() == 0},
                simple_ev(Ev::Click, Msg::ToggleAll)
            ],
            label![attrs! {At::For => "toggle-all"}, "Mark all as complete"],
            ul![class!["todo-list"], todo_els]
        ]
    } else {
        seed::empty()
    };

    div![
        header![
            class!["header"],
            h1!["todos"],
            input![
                attrs! {
                    At::Class => "new-todo";
                    At::PlaceHolder => "What needs to be done?";
                    At::AutoFocus => true;
                    At::Value => model.entry_text;
                },
                raw_ev(Ev::KeyDown, Msg::NewTodo),
                input_ev(Ev::Input, Msg::EditEntry),
            ]
        ],
        main,
        if model.active_count() > 0 || model.completed_count() > 0 {
            footer(state, &model)
        } else {
            seed::empty()
        }
    ]
}

#[wasm_bindgen]
pub fn render() {
    let routes = routes! {
        "" => Msg::RoutePage(Visible::All),
        "active" => Msg::RoutePage(Visible::Active),
        "completed" => Msg::RoutePage(Visible::Completed),
    };

//    let mut routes2 = HashMap::new();
//    routes2.insert(vec![])


    seed::App::build(Model::default(), update, todo_app)
        .routes(routes)
        .finish()
        .run();
}
