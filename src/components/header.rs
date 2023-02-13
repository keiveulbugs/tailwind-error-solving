use sycamore::prelude::*;


#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // header(style = "position: fixed; top: 0; left: 0; margin: 0; width: 100%; z-index: 99; background-color: #f8a90c; color: white; padding: 1rem;") {
        //     p { (title.to_string()) }
        // }
        header(class="fixed top-0 z-50 w-full bg-yellow-300") {
            p { ("hello ".to_string() + &title.to_string()) }
        
        }
        main(style = "padding: 1rem; left: 0; grid-area: main;") {
            (children)
        }
        footer(style = "grid-area: footer; left: 0; background-color: #f8a90c; color: white; margin: 0; width: 100%;  padding: 1rem;") {
            p { "Hey there, I'm a footer!" }
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: &'a str,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
}