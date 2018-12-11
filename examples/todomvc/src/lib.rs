//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;


const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone)]
enum Visible {
    All,
    Active,
    Completed,
}

// Model

#[derive(Clone)]
struct Todo {
    id: u32,
    name: String,
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
        let todos = self.todos.clone();
        todos.into_iter().filter(|t| t.visible(&self.visible)).collect()
    }

    fn add_todo(&mut self, name: String) {
        let ids: Vec<u32> = self.todos.iter().map(|t| t.id).collect();
        // max() will fail if there are no todos.
        let id = if let Some(id_) = ids.into_iter().max() {id_ + 1} else {0};

        self.todos.push( Todo {
            id,
            name,
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
//            local_storage,
        }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    ClearCompleted,
    Change(u32, String),
    EditItem(u32),
    KeyDown(u32),
    KeyDownItem(web_sys::KeyboardEvent),
    Destroy(u32),
    Toggle(u32),
    ToggleAll,
    NewTodo(web_sys::Event), // keycode
    Submit(u32),
}

fn update(msg: Msg, model: &Model) -> Model {
    let mut model = model.clone();
//    let mut todos = model.todos.clone();
    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter().filter(|t| t.completed == false).collect();
        }

        Msg::KeyDown(code) => {
            if code == ESCAPE_KEY {
//                Model {..model}
                // todo props.oncancel?
            } else if code == ENTER_KEY {
//                update()
            }
        },
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

                if text.len() > 0 {
                    model.add_todo(text);
                }
                // todo: Clear this field.
            }
        },
        Msg::ToggleAll => {

        },
        Msg::Submit(id) => {

        },
        Msg::Toggle(id) => {
            // todo This works, but is a mess of cloning. Clean up!
            let mut todo = model.clone().todos.into_iter().find(|t| t.id == id).unwrap();
            todo.completed = !todo.completed;

            let new_todos: Vec<Todo> = model.clone().todos.into_iter().filter(|t| t.id != id).collect();
            model.todos = new_todos;
            model.todos.push(todo);
        },

        _ => ()
    };
    model
}

// View

fn todo_item(item: Todo) -> El<Msg> {
    let mut att = attrs!{};
    if item.completed { att.add("class", "completed"); }
    if item.editing { att.add("class", "editing"); }

    li![ att, vec![
        div![ attrs!{"class" => "view"}, vec![
            input![ 
                attrs!{"class" => "toggle"; "type" => "checkbox"; "checked" => &item.completed.to_string() },
                vec![simple_ev("change", Msg::Toggle(item.id))]
            ],

            label![ vec![simple_ev("dblclick", Msg::EditItem(item.id))], item.name ],
            button![ attrs!{"class" => "destroy"}, vec![simple_ev("click", Msg::Destroy(item.id))] ]
        ] ],

        input![
            attrs!{"class" => "edit"; "value" => item.name},
            vec![
                simple_ev("blur", Msg::Submit(item.id)), 
                input_ev("change", |text| Msg::Change(1, text)),  // todo item id
                keyboard_ev("keydown", |ev| Msg::KeyDown(ev.key_code())),
            ]
        ]
    ] ]
}

fn selection_li(text: &str, path: &str, visible: Visible, highlighter: Visible) -> El<Msg> {
    li![ vec![
        a![ attrs!{"href" => path; "class" => match visible {
            highlighter => "selected",
            _ => ""
        }}, text ]
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
            strong![ &model.todos.len().to_string() ],
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
    let mut todo_items: Vec<El<Msg>> = model.shown_todos().into_iter().map(|todo| todo_item(todo.clone())).collect();

    let main = if !model.todos.is_empty() {

        section![ attrs!{"class" => "main"}, vec![
            input![
                attrs!{"id" => "toggle-all"; "class" => "toggle-all"; "type" => "checkbox";
                       "checked" => model.active_count() == 0},
                vec![simple_ev("change", Msg::ToggleAll)]
            ],
            label![ attrs!{"for" => "toggle-all"}, "Mark all as complete"],
            ul![ attrs!{"class" => "todo-list"}, todo_items ],
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