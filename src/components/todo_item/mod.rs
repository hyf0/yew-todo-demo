use yew::prelude::*;
use crate::model;

#[derive(Properties, Clone)]
pub struct TodoItemProps {
    pub item: model::TodoItem,
    pub delete: Callback<i32>,
}

pub struct TodoItem {
    link: ComponentLink<Self>,
    props: TodoItemProps,
}

pub enum Msg {
    OnClick,
}

impl Component for TodoItem {   
    type Message = Msg;
    type Properties = TodoItemProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // 近似于 constructor
        Self {
            link,
            props,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // 单单针对 state 变化的 shouldComponentUpdate
        // 同时起到一个局部 reducer 的作用
        match msg {
            Msg::OnClick => {
                let id = self.props.item.id.clone();
                self.props.delete.emit(id);
                return  false;
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // 单单针对 props 变化的 shouldComponentUpdate
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        // render 函数
        html! {
            <div class="ToDoItem">
                <p class="ToDoItem-Text">{&self.props.item.text}</p>
                <button 
                    onclick={self.link.callback(|_| Msg::OnClick)}
                    class="ToDoItem-Delete"
                >
                { "-" } 
                </button>
            </div>
        }
    }
}
