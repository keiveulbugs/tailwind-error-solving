mod templates;
mod error_views;
mod components;


use perseus::prelude::*;
//use perseus_tailwind::*;
use sycamore::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use perseus::{i18n::TranslationsManager, stores::MutableStore};
use perseus::{plugins::Plugins, prelude::*};

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(crate::error_views::get_error_views())
        .index_view(|cx| {
            
            
            view! { cx,
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = "dist/static/tailwind.css")
                    }
                    body {
                        PerseusRoot()
                    }
                }
            }
        })
        .plugins(
            Plugins::new()
                .plugin(
                    perseus_tailwind::get_tailwind_plugin,
                    perseus_tailwind::TailwindOptions {
                        in_file: "src/tailwind.css".into(),
                        out_file: "generated/tailwind.css".into(),
                    },
                )
        )
}
