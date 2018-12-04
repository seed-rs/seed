//! Modelled after the todomvc project's Typescript-React example:
//! https://github.com/tastejs/todomvc/tree/gh-pages/examples/typescript-react

use wasm_bindgen::prelude::*;

use rebar::prelude::*;


const ENTER_KEY: i16 = 13;
const ESCAPE_KEY: i16 = 27;

enum Visible {
    All,
    Active,
    Completed,
}

fn class_names() {

}


// Model

struct Item {
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
    items: Vec<Item>,
}

impl Model {
    fn completed_count(&self) -> i32 { 
        self.items.filter(|i| i.completed == true).collect().len()
    }

    fn active_count(&self) -> i32 {
        // By process of elimination; active means not completed.
        self.items.len() - self.completed_count()
    }


}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            items: Vec::new(),
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
}

//fn update(msg: &Msg, model: Rc<Model>) -> Model {
fn update(msg: &Msg, model: Rc<RefCell<Model>>) -> Model {
    // todo msg probably doesn't need to be a ref.
//    let model2 = model.clone(); // todo deal with this.
    match msg {
        &Msg::Increment => {
//            Model {clicks: model.clicks + 1, ..model.unwrap()}
            Model {clicks: model.borrow().clicks + 1, what_we_count: String::from("test")}
        },
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

    li![ attrs!{"class" => classNames({
        completed: item.completed,
        editing: item.editing
        })}, vec![

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
    let active_todo_word = pluralize(model.items.len(), 'item');

    if model.completed_count() > 0 {
        let clear_button = button![ 
            attrs!{"class" => "clear-completed"},
            events!{"click", |_| Msg::ClearCompleted},
            "Clear completed"
        ];
    } else { let clear_button = div![] };

    footer![ attrs!{"class" => "footer"}, vec![
        span![ attrs!{"class" => "todo-count"}, vec![
            strong![ &model.items.len().to_string() ]
        ], model.active_todo_word + "left" ],

        ul![ attrs!{"class" => "filters"}, vec![
            li![ vec![
                a![ attrs!{"href" => "#/", "class" => class_names({selected: nowShowing === ALL_TODOS})}, "All" ]
            ] ],
            li![ vec![
                a![ attrs!{"href" => "#/active", "class" => class_names({selected: nowShowing === ACTIVE_TODOS})}, "All" ]
            ] ],
            li![ vec![
                a![ attrs!{"href" => "#/completed", "class" => class_names({selected: nowShowing === COMPLETED_TODOS})}, "All" ]
            ] ],
        ] ],

        clear_button    
    
    ] ]

}


// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn todo_app(model: &Model) -> El<Msg> {
    if (model.)
}


#[wasm_bindgen]
pub fn render() {
    run(Model::default(), update, todo_app, "main");
}