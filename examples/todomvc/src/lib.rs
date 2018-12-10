//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys;


const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

#[derive(Clone)]
enum Visible {
    All,
    Active,
    Completed,
}

fn pluralize(count: usize, word: &str) -> String {
    let mut result = word.to_string();
    if count != 1 { result += "s" };
    result
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
    fn visible(&self, visible: Visible) -> bool {
        match visible {
            Visible::All => true,
            Visible::Active => !self.completed,
            Visible:: Completed => self.completed,
        }
    }
}

//#[derive(Clone)]
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

    fn shown_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|t| t.visible(self.visible)).collect()
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
    NewTodo(u32), // keycode
    Submit(u32),
}

fn update(msg: Msg, model: &Model) -> Model {
    let mut todos = model.todos.clone();
    match msg {
        Msg::KeyDown(code) => {
            if code == ESCAPE_KEY {
//                Model {..model}
                // todo props.oncancel?
            } else if code == ENTER_KEY {
//                update()
            }
        },
        Msg::NewTodo(code) => {
            if code == ENTER_KEY {



            }
        },
        Msg::Submit() => {

        }

        _ => ()
    };
    Model{todos, visible: model.visible.clone()}
}

// View

fn todo_item(item: Todo) -> El<Msg> {
//       public componentDidUpdate(prevProps : ITodoItemProps) {
//     if (!prevProps.editing && this.props.editing) {
//       var node = React.findDOMNode<HTMLInputElement>(this.refs["editField"]);
//       node.focus();
//       node.setSelectionRange(node.value.length, node.value.length);
//     }
// }

//    let att =attrs!{"class" => classNames({
//        completed: item.completed,
//        editing: item.editing
//        })}
    let att = attrs!{};

    li![ att, vec![
        div![ attrs!{"class" => "view"}, vec![
            input![ 
                attrs!{"class" => "toggle"; "type" => "checkbox"; "checked" => &item.completed.to_string() },
                vec![simple_ev("change", Msg::Toggle(item.id))]
            ],

            label![ vec![simple_ev("doubleclick", Msg::EditItem(item.id))], item.name ],
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

fn selection_li(text: &str, path: &str, visible: &Visible, highlighter: Visible) -> El<Msg> {
    li![ vec![
        a![ attrs!{"href" => path; "class" => match visible {
            highlighter => "selected",
            _ => ""
        }}, text ]
    ] ]
}

fn footer(model: &Model) -> El<Msg> {
    let active_todo_word = pluralize(model.todos.len(), "item");

    let clear_button = if model.completed_count() > 0 {
        button![
            attrs!{"class" => "clear-completed"},
            vec![simple_ev("click", Msg::ClearCompleted)],
            "Clear completed"
        ]
    } else { div![] };

    footer![ attrs!{"class" => "footer"}, vec![
        span![ attrs!{"class" => "todo-count"}, vec![
            strong![ &model.todos.len().to_string() ]
        ], format!("{} left", active_todo_word) ],

        ul![ attrs!{"class" => "filters"}, vec![
            selection_li("All", "#/", &model.visible, Visible::All),
            selection_li("Active", "#/active", &model.visible, Visible::Active),
            selection_li("Completed", "#/completed", &model.visible, Visible::Completed),
        ] ],
        clear_button
    ] ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn todo_app(model: &Model) -> El<Msg> {
    let mut todo_items: Vec<El<Msg>> = model.shown_todos().iter().map(|todo| todo_item(todo.clone())).collect();

    let main = section![ attrs!{"class" => "main"}, vec![
        input![
            attrs!{"id" => "toggle-all"; "class" => "toggle-all"; "type" => "checkbox";
                   "checked" => &(model.active_count() == 0).to_string()},
            vec![simple_ev("change", Msg::ToggleAll)]
        ],
        label![ attrs!{"for" => "toggle-all"}, "Mark all as complete"],
        ul![ attrs!{"class" => "todo-list"}, todo_items ],
    ] ];

    div![ vec![
        header![ attrs!{"class" => "header"}, vec![
            h1![ "todos" ],
            input![
                attrs!{"class" => "new-todo"; "placeholder" => "What needs to be done?";
                       "auto-focus" => &true.to_string()},
                vec![ keyboard_ev("keydown", |ev| Msg::NewTodo(ev.key_code())) ]
            ]
        ] ],
        main,
        footer(model)
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, todo_app, "main");
}