// use comrak::{markdown_to_html, Options};

use yew::prelude::*;
// use yew::{html, AttrValue, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

#[function_component]
pub fn WikiPage(props: &Props) -> Html {
    let Props { slug } = props;

    // let content = include_str!(path);
    // let halfway_parsed_markdown = markdown_to_html(&content, &Options::default());
    // let parsed_markdown = Html::from_html_unchecked(AttrValue::from(halfway_parsed_markdown));

    html! {
        <div class="wiki-page">
            <p><a href="/">{ "home" }</a> {" > "} <a href={slug.to_string()}>{slug}</a></p>
            <div>
                // { parsed_markdown }
            </div>
        </div>
    }
}
