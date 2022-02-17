use yew::prelude::*;

pub struct MenuTop {
    //
}

pub struct Msg{

}

impl Component for MenuTop {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <nav class="level">
                <div class="level-left">
                    <p class="level-item"><strong>{"Gold: "}</strong> {"$1,825.40"}</p>
                    <p class="level-item"><strong>{"Silver: "}</strong> {"$23.07"}</p>
                    <p class="level-item"><strong>{"Platinum: "}</strong> {"$984.55"}</p>
                </div>
                <div class="level-right">
                <p class="level-item"><strong>{"All"}</strong></p>
                <p class="level-item"><a>{"Published"}</a></p>
                <p class="level-item"><a>{"Drafts"}</a></p>
                <p class="level-item"><a>{"Deleted"}</a></p>
                <p class="level-item"><a class="button is-small is-success">{"New"}</a></p>
                </div>
            </nav>
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