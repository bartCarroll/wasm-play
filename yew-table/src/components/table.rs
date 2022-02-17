use yew::prelude::*;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ItemType{
    Coin,
    Bar
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ItemType::Coin => write!(f, "Coin"),
           ItemType::Bar => write!(f, "Bar"),
       }
    }
}

pub enum Metal{
    Silver,
    Gold,
    Platinum
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Metal::Gold => write!(f, "Gold"),
           Metal::Silver => write!(f, "Silver"),
           Metal::Platinum => write!(f, "Platinum"),
       }
    }
}

pub enum Mint{
    Us,
    Royal,
    Ca,
    Pamp
}

impl fmt::Display for Mint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Mint::Us => write!(f, "United States Mint"),
           Mint::Ca => write!(f, "Royal Canadian Mint"),
           Mint::Pamp => write!(f, "Pamp Suisse"),
           Mint::Royal => write!(f, "Royal Mint"),
       }
    }
}

pub struct Item{
    id: u64,
    name: String,
    i_type: ItemType,
    mint: Mint,
    metal: Metal,
    year: String
}

#[derive(Debug)]
pub enum Msg {
    Remove(u64),
}

pub struct ItemTable {
    items: HashMap<u64, Item>
}

impl Component for ItemTable {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let init_map = HashMap::from([(1, Item{ id: 1, name: "Queen's Beast".to_string(), i_type: ItemType::Coin, mint: Mint::Royal, metal: Metal::Gold, year: "1995".to_string()}),
                          (2, Item{ id: 2, name: "American Silver Eagle".to_string(), i_type: ItemType::Coin, mint: Mint::Us, metal: Metal::Silver, year: "1998".to_string()}),
                          (3, Item{ id: 3, name: "American Silver Eagle".to_string(), i_type: ItemType::Coin, mint: Mint::Us, metal: Metal::Silver, year: "1998".to_string()}),
                          (4, Item{ id: 4, name: "".to_string(), i_type: ItemType::Bar, mint: Mint::Pamp, metal: Metal::Gold, year: "".to_string()}),
                          (5, Item{ id: 5, name: "American Silver Eagle".to_string(), i_type: ItemType::Coin, mint: Mint::Us, metal: Metal::Silver, year: "2001".to_string()}),
                          ]);

        Self {
            items: init_map
        }
        
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
           Msg::Remove(id) => {
               self.items.remove(&id);
           }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let render_item = |item: (&u64, &Item)| {
            let id= *item.0;
            html! {
                <>
                    <tr>
                        <td>{ id }</td>
                        <td>{ &item.1.name}</td>
                        <td>{ &item.1.i_type}</td>
                        <td>{ &item.1.metal}</td>
                        <td>{ &item.1.mint}</td>
                        <td>{ &item.1.year}</td>
                        <td>
                            <div class="buttons">
                                <span class="icon has-text-info">
                                    <i class="fas fa-info-circle"></i>
                                </span>
                                <span class="icon has-text-warning">
                                    <i class="fas fa-edit"></i>
                                </span>
                                <span class="icon has-text-danger">
                                    <i class="fas fa-minus-circle"  onclick={link.callback(move |_| Msg::Remove(id))}></i>
                                </span>

                            </div>
                        </td>
                    </tr>
                </>
            }
        };

        html! {
            <div class="box">
                <div class="tabs is-boxed">
                    <ul>
                    <li class="is-active">
                        <a>
                        <span class="icon is-small"><i class="fas fa-image" aria-hidden="true"></i></span>
                        <span>{"All"}</span>
                        </a>
                    </li>
                    <li>
                        <a>
                        <span class="icon is-small"><i class="fas fa-coins" aria-hidden="true"></i></span>
                        <span>{"Coins"}</span>
                        </a>
                    </li>
                    <li>
                        <a>
                        <span class="icon is-small"><i class="fas fa-money-bill" aria-hidden="true"></i></span>
                        <span>{"Bars"}</span>
                        </a>
                    </li>
                    </ul>
                </div>
                <table class="table is-striped">
                    <thead>
                    <tr>
                        <th>{"Id"}</th>
                        <th>{"Name"}</th>
                        <th>{"Type"}</th>
                        <th>{"Metal"}</th>
                        <th>{"Mint"}</th>
                        <th>{"Year"}</th>
                        <th>{"Operations"}</th>
                    </tr>
                    </thead>
                    <tbody>
                        {for self.items.iter().map(render_item)}
                    </tbody>
                </table>
            </div>
        }
    }
}