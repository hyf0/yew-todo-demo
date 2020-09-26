use yew::prelude::*;

use crate::components::todo::Todo;

pub struct App {
}

pub enum Msg {
}



impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // 如果有新的不同屬性，應該只能回傳 true
        // 若是這個元件沒有任何屬性，那就可以只回傳 false
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Todo />
            </div>
        }
    }
}