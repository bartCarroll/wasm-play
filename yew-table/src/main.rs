use yew::prelude::*;
mod components;

use crate::components::{
   table::ItemTable, menu_top::MenuTop, menu_left::MenuLeft,
};


enum Msg{
    None,
    ModalShow,
    ModalHide
}

struct App {
    is_modal_shown: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        Self {
            is_modal_shown: false
        }
        
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ModalShow => {
                self.is_modal_shown = true;
                return true;
            }
            Msg::ModalHide => {
                self.is_modal_shown = false;
                return true;
            }
           Msg::None => {
               // Do nothing
           }
        }
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container is-max-widescreen">
                <h1 class="title is-1">{"Numasist"}</h1>
                <MenuTop />
                <div class="columns">
                    <div class="column is-one-fifth">
                        <MenuLeft />
                    </div>
                    <div class="column">
                        <ItemTable />
                    </div>
                </div>
                <footer class="footer">
                    <div class="content is-normal has-text-centered">
                        <p>
                            <strong>{"Numasist "}</strong> {"by "} <a href="http://iambort.io">{"Bart Carroll"}</a>{". The source code is licensed "}
                            <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">{"CC BY NC SA 4.0"}</a>{"."}
                        </p>
                    </div>
                </footer>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}