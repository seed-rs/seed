//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

//use std::cmp::Ordering;

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone, PartialEq)]
enum Visible {
    All,
    Active,
    Completed,
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
            Visible:: Completed => self.completed,
        }
    }
}

#[derive(Clone)]
struct Model {
    todos: Vec<Todo>,
    visible: Visible,
    edit_text: String,
    local_storage: web_sys::Storage,
}

impl Model {
    fn completed_count(&self) -> i32 {
        let completed: Vec<&Todo> = self.todos.iter().filter(|i| i.completed == true).collect();
        completed.len() as i32
    }

    fn active_count(&self) -> i32 {
        // By process of elimination; active means not completed.
        self.todos.len() as i32 - self.completed_count()
    }

    fn shown_todos(&self) -> Vec<Todo> {
        let mut todos = self.todos.clone();
        todos = todos.into_iter().filter(|t| t.visible(&self.visible)).collect();
        todos.sort();  // I'm not sure what criteria this is using... Id?
        todos
    }

    fn add_todo(&mut self, name: String) {
        self.todos.push( Todo {
            title: name,
//            edit_text: String::new(), // what is this? todo
            completed: false,
            editing: false,
        });
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
    SetVisibility(Visible),

    EditItem(usize),
    EditSubmit(usize),
    EditChange(String),
    EditKeyDown(usize, u32),  // item position, keycode
}

fn update(msg: Msg, model: &Model) -> Model {
    let mut model = model.clone();


    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter().filter(|t| t.completed == false).collect();
            model.sync_storage();
        },
        Msg::Destroy(posit) => {
            model.todos.remove(posit);
            model.sync_storage();
        },
        Msg::Toggle(posit) => model.todos[posit].completed = !model.todos[posit].completed,
        Msg::ToggleAll => {
            // Mark all as completed, unless all are... then mark all as note completed.
            let setting = if model.active_count() == 0 { false } else { true };
            for todo in &mut model.todos {
                todo.completed = setting;
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
                let text = input_el.value().trim().to_string();

                if text.len() > 0 {
                    model.add_todo(text);
                    input_el.set_value("");
                    model.sync_storage();
                }
            }
        },
        Msg::SetVisibility(vis) => model.visible = vis,

        Msg::EditItem(posit) => {
            for todo in &mut model.todos {
                todo.editing = false;
            }
            model.todos[posit].editing = true;
            model.edit_text = (&model.todos[posit].title).to_string();
        },
        Msg::EditSubmit(posit) => {
            if model.edit_text.len() > 0 {
                model.todos[posit].title = model.edit_text.clone();
                model.todos[posit].editing = false;
                model.edit_text = model.edit_text.trim().to_string();
                model.sync_storage();
            } else {
                model.todos.remove(posit);
            }
        },
        Msg::EditChange(text) => model.edit_text = text,
        Msg::EditKeyDown(posit, code) => {
            if code == ESCAPE_KEY {
                model.edit_text = model.todos[posit].title.clone();
                for todo in &mut model.todos {
                    todo.editing = false;
                }
            } else if code == ENTER_KEY {
                model = update(Msg::EditSubmit(posit), &model);
            }
        },
    };
    model
}

// View

fn todo_item(item: Todo, posit: usize, edit_text: String) -> El<Msg> {
    let mut att = attrs!{};
    if item.completed { att.add("class", "completed"); }
    if item.editing { att.add("class", "editing"); }
    att.add("key", &item.title);

    li![ att, vec![
        div![ attrs!{"class" => "view"}, vec![
            input![ 
                attrs!{"class" => "toggle"; "type" => "checkbox"; "checked" => item.completed },
                vec![simple_ev("click", Msg::Toggle(posit))]
            ],

            label![ vec![simple_ev("dblclick", Msg::EditItem(posit))], item.title ],
            button![ attrs!{"class" => "destroy"}, vec![simple_ev("click", Msg::Destroy(posit))] ]
        ] ],

        if item.editing == true {
            input![
                attrs!{"class" => "edit"; "value" => edit_text},
                vec![
                    simple_ev("blur", Msg::EditSubmit(posit)),
                    input_ev("input", |text| Msg::EditChange(text)),
                    keyboard_ev("keydown", move |ev| Msg::EditKeyDown(posit, ev.key_code())),
                ]
            ]
        } else { seed::empty() }
    ] ]
}

fn selection_li(text: &str, path: &str, visible: Visible, highlighter: Visible) -> El<Msg> {
    li![ vec![
        a![ attrs!{"href" => path; "class" => if visible == highlighter {"selected"} else {""}},
            vec![ simple_ev("click", Msg::SetVisibility(highlighter)) ], text
            ]
    ] ]
}

fn footer(model: &Model) -> El<Msg> {
    let optional_s = if model.todos.len() == 1 {""} else {"s"};

    let clear_button = if model.completed_count() > 0 {
        button![
            attrs!{"class" => "clear-completed"},
            vec![simple_ev("click", Msg::ClearCompleted)],
            "Clear completed"
        ]
    } else { seed::empty() };

    footer![ attrs!{"class" => "footer"}, vec![
        span![ attrs!{"class" => "todo-count"}, vec![
            strong![ model.active_count().to_string() ],
            span![ format!(" item{} left", optional_s) ]
        ]  ],

        ul![ attrs!{"class" => "filters"}, vec![
            // todo fix cloning here.
            selection_li("All", "#/", model.visible.clone(), Visible::All),
            selection_li("Active", "#/active", model.visible.clone(), Visible::Active),
            selection_li("Completed", "#/completed", model.visible.clone(), Visible::Completed),
        ] ],
        clear_button
    ] ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn todo_app(model: Model) -> El<Msg> {
    // We use the item's position in its Vec to identify it, because this allows
    // simple in-place modification through indexing.
    let items: Vec<El<Msg>> = model.shown_todos()
        .into_iter()
        .enumerate()
        .map(|(posit, todo)| todo_item(todo.clone(), posit, model.edit_text.clone())).collect();

    let main = if !model.todos.is_empty() {

        section![ attrs!{"class" => "main"}, vec![
            input![
                attrs!{"id" => "toggle-all"; "class" => "toggle-all"; "type" => "checkbox";
                       "checked" => model.active_count() == 0},
                vec![simple_ev("change", Msg::ToggleAll)]
            ],
            label![ attrs!{"for" => "toggle-all"}, "Mark all as complete"],
            ul![ attrs!{"class" => "todo-list"}, items ],
        ] ]

    } else { seed::empty() };

    div![ vec![
        header![ attrs!{"class" => "header"}, vec![
            h1![ "todos" ],
            input![
                attrs!{
                    "class" => "new-todo";
                    "placeholder" => "What needs to be done?";
                    "auto-focus" => true
                },
                vec![ raw_ev("keydown", |ev| Msg::NewTodo(ev)) ]
            ]
        ] ],
        main,
        if model.active_count() > 0 || model.completed_count() > 0
            { footer(&model) } else { seed::empty() }
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, todo_app, "main");
}