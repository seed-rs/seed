//! Modelled after the todomvc project's [Typescript-React example](https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react)

#[macro_use]
extern crate seed;
use seed::events::Event;
use seed::prelude::*;
use seed::storage::Storage;
use serde::{Deserialize, Serialize};

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone, Copy, PartialEq)]
enum Visible {
    All,
    Active,
    Completed,
}

impl ToString for Visible {
    fn to_string(&self) -> String {
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
    fn visible(&self, visible: Visible) -> bool {
        match visible {
            Visible::All => true,
            Visible::Active => !self.completed,
            Visible::Completed => self.completed,
        }
    }
}

struct Model {
    todos: Vec<Todo>,
    visible: Visible,
    entry_text: String,
    edit_text: String,
    local_storage: Storage,
}

impl Model {
    fn completed_count(&self) -> usize {
        let completed: Vec<&Todo> = self.todos.iter().filter(|i| i.completed).collect();
        completed.len()
    }

    fn active_count(&self) -> usize {
        // By process of elimination; active means not completed.
        self.todos.len() - self.completed_count()
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
    NewTodo(Event),
    EditEntry(String),

    EditItem(usize),
    EditSubmit(usize),
    EditChange(String),
    EditKeyDown(usize, u32), // item position, keycode

    ChangeVisibility(Visible),
}

/// Called by update function. Split into separate function since we use it twice.
fn edit_submit(posit: usize, model: &mut Model) {
    if model.edit_text.is_empty() {
        model.todos.remove(posit);
    } else {
        let mut todo = model.todos.remove(posit);
        todo.editing = false;
        todo.title = model.edit_text.clone();
        model.todos.insert(posit, todo);
        model.edit_text = model.edit_text.trim().to_string();
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    model.sync_storage(); // Doing it here will miss the most recent update...

    // todo has some bugs.
    match msg {
        Msg::ClearCompleted => {
            model.todos = model
                .todos
                .clone()
                .into_iter()
                .filter(|t| !t.completed)
                .collect();
        }
        Msg::Destroy(posit) => {
            model.todos.remove(posit);
        }

        Msg::Toggle(posit) => model.todos[posit].completed = !model.todos[posit].completed,

        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            for todo in &mut model.todos {
                todo.completed = completed;
            }
        }
        Msg::NewTodo(ev) => {
            // Add a todo_, if the enter key is pressed.
            // We handle text input after processing a key press, hence the
            // raw event logic here.
            let code = seed::to_kbevent(&ev).key_code();
            if code == ENTER_KEY {
                ev.prevent_default();

                let target = ev.target().unwrap();
                let input_el = seed::to_input(&target);
                let title = input_el.value().trim().to_string();

                if !title.is_empty() {
                    model.todos.push(Todo {
                        title,
                        completed: false,
                        editing: false,
                    });
                    input_el.set_value("");
                }
            }
        }
        Msg::EditEntry(entry_text) => model.entry_text = entry_text,

        Msg::EditItem(posit) => {
            for todo in &mut model.todos {
                todo.editing = false;
            }

            let mut todo = model.todos.remove(posit);
            todo.editing = true;
            model.todos.insert(posit, todo.clone());
            model.edit_text = todo.title;
        }
        Msg::EditSubmit(posit) => edit_submit(posit, model),
        Msg::EditChange(edit_text) => model.edit_text = edit_text,
        Msg::EditKeyDown(posit, code) => {
            if code == ESCAPE_KEY {
                for todo in &mut model.todos {
                    todo.editing = false;
                }
                model.edit_text = model.todos[posit].title.clone();
            } else if code == ENTER_KEY {
                edit_submit(posit, model)
            }
        }
        Msg::ChangeVisibility(visible) => model.visible = visible,
    }
}

// View

fn todo_item(item: &Todo, posit: usize, edit_text: &str) -> Node<Msg> {
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
                attrs! {
                   At::Class => "toggle",
                   At::Type => "checkbox",
                   At::Checked => item.completed.as_at_value()
                },
                simple_ev(Ev::Click, Msg::Toggle(posit))
            ],
            label![simple_ev(Ev::DblClick, Msg::EditItem(posit)), item.title],
            button![class!["destroy"], simple_ev(Ev::Click, Msg::Destroy(posit))]
        ],
        if item.editing {
            input![
                attrs! {At::Class => "edit", At::Value => edit_text},
                simple_ev(Ev::Blur, Msg::EditSubmit(posit)),
                input_ev(Ev::Input, Msg::EditChange),
                keyboard_ev(Ev::KeyDown, move |ev| Msg::EditKeyDown(
                    posit,
                    ev.key_code()
                )),
            ]
        } else {
            empty![]
        }
    ]
}

fn selection_li(text: &str, visible: Visible, highlighter: Visible) -> Node<Msg> {
    li![a![
        attrs! {
            At::Class => if visible == highlighter {"selected"} else {""}
            At::Href => "/".to_string() + &highlighter.to_string()
        },
        style! {"cursor" => "pointer"},
        text
    ]]
}

fn footer(model: &Model) -> Node<Msg> {
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
            selection_li("All", model.visible, Visible::All),
            selection_li("Active", model.visible, Visible::Active),
            selection_li("Completed", model.visible, Visible::Completed)
        ],
        clear_button
    ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn view(model: &Model) -> impl View<Msg> {
    // We use the item's position in model.todos to identify it, because this allows
    // simple in-place modification through indexing. This is different from its
    // position in visible todos, hence the two-step process.
    let todo_els: Vec<Node<Msg>> = model
        .todos
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(posit, todo)| {
            if todo.visible(model.visible) {
                Some(todo_item(&todo, posit, &model.edit_text))
            } else {
                None
            }
        })
        .collect();

    let main = if model.todos.is_empty() {
        seed::empty()
    } else {
        section![
            class!["main"],
            input![
                attrs! {
                    At::Id => "toggle-all"; At::Class => "toggle-all"; At::Type => "checkbox",
                    At::Checked => (model.active_count() == 0).as_at_value(),
                },
                simple_ev(Ev::Click, Msg::ToggleAll)
            ],
            label![attrs! {At::For => "toggle-all"}, "Mark all as complete"],
            ul![class!["todo-list"], todo_els]
        ]
    };

    vec![
        header![
            class!["header"],
            h1!["todos"],
            input![
                attrs! {
                    At::Class => "new-todo";
                    At::Placeholder => "What needs to be done?";
                    At::AutoFocus => true;
                    At::Value => model.entry_text;
                },
                raw_ev(Ev::KeyDown, Msg::NewTodo),
                input_ev(Ev::Input, Msg::EditEntry),
            ]
        ],
        main,
        if model.active_count() > 0 || model.completed_count() > 0 {
            footer(model)
        } else {
            seed::empty()
        },
    ]
}

#[allow(clippy::needless_pass_by_value)]
fn routes(url: seed::Url) -> Msg {
    match url.path.get(0).map(String::as_str) {
        Some("active") => Msg::ChangeVisibility(Visible::Active),
        Some("completed") => Msg::ChangeVisibility(Visible::Completed),
        _ => Msg::ChangeVisibility(Visible::All),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
