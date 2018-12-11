//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

use std::cmp::Ordering;

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;


const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone, PartialEq)]
enum Visible {
    All,
    Active,
    Completed,
}

// Model

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Todo {
    id: u32,
    title: String,
    edit_text: String,
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
    editing: Option<u32>,
//    local_storage: web_sys::Storage,
    // todo: key and on_changes ??
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
        let ids: Vec<u32> = self.todos.iter().map(|t| t.id).collect();
        // max() will fail if there are no todos.
        let id = if let Some(id_) = ids.into_iter().max() {id_ + 1} else {0};

        self.todos.push( Todo {
            id,
            title: name,
            edit_text: String::new(), // what is this? todo
            completed: false,
            editing: false,
        })
    }

}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
//        local_storage.fetch_local_storage();

        Self {
            todos: Vec::new(),
            visible: Visible::All,
            edit_text: String::new(),
            editing: None,
//            local_storage,
        }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    ClearCompleted,
    KeyDownItem(web_sys::KeyboardEvent),  // todo
    Destroy(u32),
    Toggle(u32),
    ToggleAll,  // todo
    NewTodo(web_sys::Event), // keycode
    SetVisibility(Visible),

    EditItem(u32),
    EditSubmit(u32),  // todo
    EditChange(u32, String),  // todo
    EditKeyDown(u32),  // todo

}

fn update(msg: Msg, model: &Model) -> Model {
    let mut model = model.clone();

    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter().filter(|t| t.completed == false).collect();
        }

        Msg::EditKeyDown(code) => {
            if code == ESCAPE_KEY {
//                Model {..model}
                // todo props.oncancel?
            } else if code == ENTER_KEY {
//                update()
            }
        },
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
            let keyboard_ev = ev.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            let code = keyboard_ev.key_code();

            if code == ENTER_KEY {
                ev.prevent_default();
                let target = ev.target().unwrap();
                let input_el = target.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                let text = input_el.value().trim().to_string();

                input_el.set_value("");

                if text.len() > 0 {
                    model.add_todo(text);
                    model.edit_text = String::new();
                }
            }
        },

        Msg::EditSubmit(id) => {

        },
        Msg::Toggle(id) => {
            // todo This works, but is a mess of cloning. Clean up!
            let mut todo = model.clone().todos.into_iter().find(|t| t.id == id).unwrap();
            todo.completed = !todo.completed;

            let new_todos: Vec<Todo> = model.clone().todos.into_iter().filter(|t| t.id != id).collect();
            model.todos = new_todos;
            model.todos.push(todo);
        },
        Msg::SetVisibility(vis) => model.visible = vis,
        Msg::Destroy(id) => {
            // todo broken, sort of
            log!("ID:", id);
            for id1 in &model.todos {
                log!("listing them: ", id1.id);
            }
            model.todos = model.todos.into_iter().filter(|t| t.id != id).collect()
        },
        Msg::EditItem(id) => {
            model.editing = Some(id);
            let mut todo = model.clone().todos.into_iter().find(|t| t.id == id).unwrap();
            model.edit_text = todo.title;
        }

        _ => ()
    };
    model
}

// View

fn todo_item(item: Todo, edit_text: String) -> El<Msg> {
    let mut att = attrs!{};
    if item.completed { att.add("class", "completed"); }
    if item.editing { att.add("class", "editing"); }

    li![ att, vec![
        div![ attrs!{"class" => "view"}, vec![
            input![ 
                attrs!{"class" => "toggle"; "type" => "checkbox"; "checked" => item.completed },
                vec![simple_ev("change", Msg::Toggle(item.id))]
            ],

            label![ vec![simple_ev("dblclick", Msg::EditItem(item.id))], item.title ],
            button![ attrs!{"class" => "destroy"}, vec![simple_ev("click", Msg::Destroy(item.id))] ]
        ] ],

        input![
            attrs!{"class" => "edidt"; "value" => edit_text},
            vec![
                simple_ev("blur", Msg::EditSubmit(item.id)),
                input_ev("change", |text| Msg::EditChange(1, text)),  // todo item id
                keyboard_ev("keydown", |ev| Msg::EditKeyDown(ev.key_code())),
            ]
        ]
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
    let mut items: Vec<El<Msg>> = model.shown_todos().into_iter()
        .map(|todo| todo_item(todo.clone(), model.edit_text.clone())).collect();

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
                attrs!{"class" => "new-todo"; "placeholder" => "What needs to be done?";
                       "auto-focus" => true},
                vec![ raw_ev("keydown", |ev| Msg::NewTodo(ev)) ]
            ]
        ] ],
        main,
        if model.active_count() > 0 || model.completed_count() > 0 { footer(&model) } else { seed::empty() }
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, todo_app, "main");
}