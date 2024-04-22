use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(IndexPage)]
pub fn index_page(_props: &Props) -> Html {
    let pages = read_links_from_files();

    log::info!("Some info");

    html! {
        <div class="homepage">
            <h1>{"All Wiki Pages"}</h1>
            <ul>
                { pages.iter().map(|page| render_page_link(page)).collect::<Html>() }
            </ul>
        </div>
    }
}

pub fn read_links_from_files() -> Vec<WikiPageLink> {
    let mut links = Vec::new();

    // FIXME: Parse these from the file dir
    let slug = "btrees".to_string();
    let title = "BTrees".to_string();

    links.push(WikiPageLink { slug, title });

    // FIXME: Parse these from the file dir
    let slug = "about".to_string();
    let title = "About".to_string();

    links.push(WikiPageLink { slug, title });

    links
}

pub fn render_page_link(page: &WikiPageLink) -> Html {
    html! {
        <li><a href={ format!("/{}", page.slug) }>{&page.title}</a></li>
    }
}

pub struct WikiPageLink {
    slug: String,
    title: String,
}
