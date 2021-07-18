use dominator::{Dom, class, html, clone, events};
use wasm_bindgen::prelude::*;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {}

impl App {
    fn new() -> App{
        App{}
    }

    fn render(self) -> Dom {
        html!("div", {
            .children(&mut [

                html!("h1", {
                    .text("Hello, World!")
                }),

                html!("img", {
                    .property("src", "assets/mycat.jpg")
                    .property("height", "420")
                })
            ])
        })
    }
}

/// Application entry point to wasm
#[wasm_bindgen(start)]
pub fn main() {
    
    let app = App::new();
    dominator::append_dom(&dominator::body(), app.render());
}

