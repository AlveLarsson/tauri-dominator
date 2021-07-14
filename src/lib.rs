use dominator::{Dom, class, html, clone, events};
use wasm_bindgen::prelude::*;

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
