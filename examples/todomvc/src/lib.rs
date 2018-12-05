//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

use wasm_bindgen::prelude::*;

use rebar::prelude::*;


const ENTER_KEY: u16 = 13;
const ESCAPE_KEY: u16 = 27;

enum Visible {
    All,
    Active,
    Completed,
}

fn class_names() {

}


// Model

struct Item {
    id: u16,
    title: &'static str,
    edit_text: String,
    completed: bool,
    editing: bool,
}

impl Item {
    fn visible(&self, visible: Visible) -> bool {
        match visible {
            Visible::All => true,
            Visible::Active => !self.completed,
            Visible:: Completed => self.completed,
        }
    }
}

#[derive(Clone)]
struct Model {
    todos: Vec<Item>,
    visible: Visible,
    // todo: key and on_changes ??
}

impl Model {
    fn completed_count(&self) -> i32 { 
        self.todos.filter(|i| i.completed == true).collect().len()
    }

    fn active_count(&self) -> i32 {
        // By process of elimination; active means not completed.
        self.todos.len() - self.completed_count()
    }


}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
        }
    }
}

// Update

enum Msg {
    ClearCompleted,
    EditItem(&Item),
    KeyDownItem(&Item),
    Destroy(&Item),
    Toggle(&Item),
    NewTodoKeydown(e),
}

//fn update(msg: &Msg, model: Rc<Model>) -> Model {
fn update(msg: &Msg, model: &model) -> Model {
    // todo msg probably doesn't need to be a ref.
    let model = model.clone();
    match msg {
        &Msg::Increment => {
//            Model {clicks: model.clicks + 1, ..model.unwrap()}
            Model {clicks: model.borrow().clicks + 1, what_we_count: String::from("test")}
        },
        &Msg::EditItem(item, event) => {
            let updated_item = Item{ title: event.target.value, ..item.clone() };
            let mut updated = &model.todos.filter(|todo| todo.id != updated_item.id).collect();
            updated.push(updated_item);
            Model {todos: updated}
        }
    }
}

// View

fn todo_item(item: &todo_item) -> El<Msg> {
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
        div![ attrs!{"class" => "view"} ], vec![
            input![ 
                attrs!{"class" => "toggle", "type" => "checkbox", "checked" => item.completed },
                events!{"change" => |_| Msg::Toggle(item)} 
            ],

            label![ events!{"doubleclick" => |e| Msg::EditItem(&item)}, item.title ],
            button![ attrs!{"class" => "destroy"}, events!{"click" => |_| Msg::Destroy(item)} ]
        ],

        // todo ?? ref? state.editText?
        input![ 
            attrs!{"class" => "edit", "value" => item.name},
            events!{"blur" => |e| Msg::Submit(e), "change" => |e| Msg::Change(e), 
                    "keydown" => |e| Msg::KeyDown(item)}
        ]
    ] ]
}

fn footer(model: &Model) -> El<Msg> {
    let active_todo_word = pluralize(model.todos.len(), "item");

    if model.completed_count() > 0 {
        let clear_button = button![ 
            attrs!{"class" => "clear-completed"},
            events!{"click", |_| Msg::ClearCompleted},
            "Clear completed"
        ];
    } else { let clear_button = div![]; };

    footer![ attrs!{"class" => "footer"}, vec![
        span![ attrs!{"class" => "todo-count"}, vec![
            strong![ &model.items.len().to_string() ]
        ], model.active_todo_word + "left" ],

        ul![ attrs!{"class" => "filters"}, vec![
            li![ vec![
                a![ attrs!{"href" => "#/";
                "class" => if model.visible == Visible::All {"selected"} else {""}}, "All" ]
            ] ],
            li![ vec![
                a![ attrs!{"href" => "#/active";
                "class" => if model.visible == Visible::Active {"selected"} else {""}}, "Active" ]
            ] ],
            li![ vec![
                a![ attrs!{"href" => "#/completed";
                "class" => if model.visible == Visible::Selected {"selected"} else {""}}, "Completed" ]
            ] ],
        ] ],

        clear_button    
    
    ] ]

}


// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn todo_app(model: &Model) -> El<Msg> {
    let todo_items = &model.todos.map(|todo| todo_item(todo)).collect();

    let main = section![ attrs!{"class" => "main"}, vec![
        input![
            attrs!{"id" => "toggle-all"; "class" => "toggle-all"; "type" => "checkbox",
                   "checked" => model.active_count() == 0},
            events!{"change" => Msg::ToggleAll}
        ],
        label![ attrs!{"for" => "toggle-all"} "Mark all as complete"],
        ul![ attrs!{"class" => "todo-list"}, todo_items ],
    ] ];

    div![ vec![
        header![ attrs!{"class" => "header"}, vec![
            h1![ "todos" ],
            input![
                attrs!{"class" => "new-todo"; "placeholder" => "What needs to be done?";
                       "auto-focus" => true.as_str()}
                events!{"keydown" => NewTodoKeydown}
           ]
        ]
        ],
        main,
        footer(model)
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    run(Model::default(), update, todo_app, "main");
}