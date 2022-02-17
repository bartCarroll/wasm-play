use yew::prelude::*;

pub struct MenuLeft {
    //
}

pub struct Msg{

}

impl Component for MenuLeft {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <aside class="menu">
                <p class="menu-label">
                    {"General"}
                </p>
                <ul class="menu-list">
                    <li><a>{"Home"}</a></li>
                    <li><a>{"Inventory"}</a></li>
                    <li><a>{"Cost"}</a></li>
                </ul>
                <p class="menu-label">
                    {"Administration"}
                </p>
                <ul class="menu-list">
                    <li><a>{"Accounts"}</a></li>
                    <li><a>{"Authentication"}</a></li>
                </ul>
                </aside>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            //
        }
    }
}