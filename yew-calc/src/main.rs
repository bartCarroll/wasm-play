use yew::prelude::*;


#[derive(Debug)]
enum Msg {
    Add,
    Subtract,
    Clear,
    Num(i64),
    Equals,
    None
}

#[derive(Debug)]
enum Op{
    Add,
    Subtract,
    Equals,
    None
}

struct Model {
    value: i64,
    total: i64,
    op_pressed: bool,
    operator: Op
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            total: 0,
            op_pressed: true,
            operator: Op::None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.operator = Op::Add;
                self.op_pressed = true;
                self.total+=self.value;
                true
            }
            Msg::Subtract => {
                self.operator = Op::Subtract;
                self.op_pressed = true;
                self.total-=self.value;
                true
            }
            Msg::Clear => {
                self.value = 0;
                self.operator = Op::None;
                self.total = 0;
                self.op_pressed = false;
                true
            }
            Msg::Num(n) =>{
                log::info!("Num: {:?} | Value: {} | Operator {:?}",n, self.value, self.operator );
                if self.value == 0 || self.op_pressed{
                    log::info!("IN IF");
                    self.value = n;
                }
                else{
                    log::info!("IN ELSE");
                    let mut val_str: String = self.value.to_string().to_owned();
                    val_str.push_str(&n.to_string());
                    self.value = val_str.parse::<i64>().unwrap();
                }
                self.op_pressed = false;
                true
            }
            Msg::Equals =>{
                self.op_pressed = false;
                match self.operator {
                    Op::Add => {
                        self.operator = Op::Add;
                        self.op_pressed = true;
                        self.total+=self.value;
                    }
                    Op::Subtract => {
                        self.operator = Op::Subtract;
                        self.op_pressed = true;
                        self.total-=self.value;
                    }
                    Op::Equals => {

                    }
                    Op::None => {

                    }
                }
                self.operator = Op::None;
                self.value = self.total;
                true 
            }
            Msg::None =>{
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container is-max-desktop">
                <article  class="panel">
                <div class="panel-header">
                <p style="align:right;border:1px">{ self.value }</p>
                <p style="align:right;border:1px">{ self.total }</p>
                </div>
                <div class="block-content">
                <div class="field is-grouped">
                    <button class="button" onclick={link.callback(|_| Msg::Num(7))}>{ "7"}</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(8))}>{ "8"}</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(9))}>{ "9" }</button>
                    <button class="button" onclick={link.callback(|_| Msg::Add)}>{"+"}</button>
                </div>
                <div class="field is-grouped">
                    <button class="button" onclick={link.callback(|_| Msg::Num(4))}>{ "4"}</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(5))}>{ "5" }</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(6))}>{ "6" }</button>
                    <button class="button" onclick={link.callback(|_| Msg::Subtract)}>{"-"}</button>
                </div>
                <div class="field is-grouped">
                    <button class="button" onclick={link.callback(|_| Msg::Num(1))}>{ "1"}</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(2))}>{ "2" }</button>
                    <button class="button" onclick={link.callback(|_| Msg::Num(3))}>{ "3" }</button>
                    <button class="button" onclick={link.callback(|_| Msg::Equals)}>{"="}</button>
                </div>
                <button class="button" onclick={link.callback(|_| Msg::Clear)}>{"clear"}</button>
                </div>    
                </article>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}