use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <html lang="en">
        <head>
            <title>HTMX</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.min.js" />
            <link rel="icon" href="data:," />
        </head>
        <body>
            <h1>THIS IS A HEADER</h1>
            {children()}
        </body>
        </html>
    }
}
