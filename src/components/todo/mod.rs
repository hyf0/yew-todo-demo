use yew::prelude::*;

use crate::model;
use crate::components::todo_item::TodoItem;

pub struct Todo {
  link: ComponentLink<Self>,
  list: Vec<model::TodoItem>,
  input: String,
  show_error: bool,
}

pub enum Msg {
  UpdateInput(String),
  AddTodoItem,
  DeleteItem(i32),
  None,
}

impl Component for Todo {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    use model::TodoItem;
    Self {
      link,
      input: String::new(),
      show_error: false,
      list: vec![
        TodoItem {
          id: 1,
          text: "eat".to_owned(),
        },
        TodoItem {
          id: 2,
          text: "work".to_owned(),
        },
      ],
    }
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::UpdateInput(input) => {
        self.input = input;
        true
      },
      Msg::AddTodoItem => {
        if self.input.trim().len() == 0 {
          self.show_error = true;
        } else {
          self.list.push(model::TodoItem {
            id: self.list.len() as i32,
            text: self.input.clone(),
          });
        }
        self.input = String::new();
        true
      },
      Msg::DeleteItem(id) => {
        self.list = self.list.clone().into_iter().filter(|item| item.id != id).collect();
        true
      }
      _ => true
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // 如果有新的不同屬性，應該只能回傳 true
    // 若是這個元件沒有任何屬性，那就可以只回傳 false
    false
  }


  fn view(&self) -> Html {
    html! {
        <div class="ToDo">
        <img class="Logo" src={"./assets/rust.svg"} alt="Rust logo" />
        <img class="Logo" src={"./assets/yew.svg"} alt="Yew logo" />
        <h1 class="ToDo-Header">{"Yew(by Rust) To Do"}</h1>
        <div class="ToDo-Container">
          <div class="ToDo-Content">
            {self.list
              .iter()
              .map(|item| html! { 
                <TodoItem 
                  delete={self.link.callback(|id: i32| Msg::DeleteItem(id))}
                  item={item} 
                /> 
              })
              .collect::<Html>()}
          </div>

          <div class="ToDoInput">
            <input
              type="text"
              placeholder="I need to..."
              value={&self.input}
              oninput=self.link.callback(|evt: yew::events::InputData| Msg::UpdateInput(evt.value))
              onkeypress=self.link.callback(|evt: yew::events::KeyboardEvent| if evt.key() == "Enter" { Msg::AddTodoItem } else { Msg::None })
            />
            <button
              onclick=self.link.callback(|_| Msg::AddTodoItem)
              class="ToDo-Add"
            >
              {"+"}
            </button>
          </div>
          <div class="ToDo-ErrorContainer">{
            if self.show_error {
              html! { <p>{ "Please enter a todo!" }</p> }
            } else {
              html! {}
            }
          }</div>
        </div>
      </div>
    }
  }
}
