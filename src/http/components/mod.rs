use leptos::IntoView;
use regex::Regex;

pub mod layout;

pub fn htmlify<F, N>(f: F) -> String
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let ssr = leptos::ssr::render_to_string(f).to_string();

    let data_hk = Regex::new(r#"data-hk="?\d+-\d+-\d+-\d+"?"#).unwrap();
    let trimmed = data_hk.replace_all(ssr.as_str(), "").into_owned();

    let comments = Regex::new(r"(?s)<!--.*?-->").unwrap();
    let decomented = comments.replace_all(trimmed.as_str(), "").into_owned();

    return decomented.to_string();
}
