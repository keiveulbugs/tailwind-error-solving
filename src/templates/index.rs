use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::header::Layout;


fn index_page<G: Html>(cx: Scope) -> View<G> {
    // view! { cx,
    //     Layout(title = "My site") {
    //         div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
    //             h1 { "Welcome to the site!" }
    //             p {
    //                 "A lot more pages are needed"
                
    //             }
    //             a ( href="/about") {"about"}
                
    //         }
    //     }
    // };

    view! { cx,
        
        button(class = "btn text-red-500") { "test" }
        button(class = "btn bg-purple-800") { "test 2" }
        
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}