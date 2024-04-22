use yew::prelude::*;

mod components;
use components::index_page::IndexPage;
use components::wiki_page::WikiPage;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:slug")]
    Post { slug: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <IndexPage></IndexPage> },
        Route::Post { slug } => {
            html! {<WikiPage slug={slug}></WikiPage>}
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
