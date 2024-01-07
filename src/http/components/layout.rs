use leptos::*;

// https://fonts.googleapis.com/css?family=Zen%20Maru%20Gothic:400,700&subset=latin
const FONTSTYLE: &str = r###"
@font-face {
    font-family: 'Zen Maru Gothic';
    font-style: normal;
    font-weight: 400;
    src: local("Zen Maru Gothic"), url(/static/fonts/ZenMaruGothic400.woff2) format('woff2');
    font-display: swap;
}
@font-face {
    font-family: 'Zen Maru Gothic';
    font-style: normal;
    font-weight: 700;
    src: local("Zen Maru Gothic"), url(/static/fonts/ZenMaruGothic700.woff2) format('woff2');
    font-display: swap;
}
"###;

const TAILWINDCONFIG: &str = r###"
tailwind.config = {
    theme: {
        extend: {
            colors: {
                // "light-weakest": "#737373",
                // "light-weak": "#a3a3a3",
                light: "#e5e5e5",
                dark: "#171717",
                // "dark-weak": "#262626",
                accent: "#84cc16",
                // "accent-weak": "#65a30d",
                // "accent-weakest": "#4d7c0f",
            },
            keyframes: {
                flickerin: {
                    "0%": { opacity: 0.1 },
                    "20%": { opacity: 0.5 },

                    // Slow flicker (4%)
                    // "24%, 32%, 40%": {opacity: .3},
                    // "28%, 36%, 44%": {opacity: .7},
                    // "48%": {opacity: .5},

                    // Medium flicker (3%)
                    "23%, 29%, 35%": { opacity: 0.3 },
                    "26%, 32%, 38%": { opacity: 0.7 },
                    "41%": { opacity: 0.5 },

                    // Fast flicker (2%)
                    // "22%, 28%, 32%": {opacity: .3},
                    // "24%, 26%, 30%": {opacity: .7},
                    // "34%": {opacity: .5},

                    "100%": { opacity: 1 },
                },
                flicker: {
                    "0%": { opacity: 1 },
                    "20%": { opacity: 1 },

                    // Slow flicker (4%)
                    // "24%, 32%, 40%": {opacity: .3},
                    // "28%, 36%, 44%": {opacity: .7},
                    // "48%": {opacity: .5},

                    // Medium flicker (3%)
                    "23%, 29%, 35%": { opacity: 0.7 },
                    "26%, 32%, 38%": { opacity: 1 },
                    "41%": { opacity: 1 },

                    // Fast flicker (2%)
                    // "22%, 28%, 32%": {opacity: .3},
                    // "24%, 26%, 30%": {opacity: .7},
                    // "34%": {opacity: .5},

                    "100%": { opacity: 1 },
                },
            },
            animation: {
                flicker: "flickerin 1.5s, flicker 1.5s infinite 1.5s",
            },
            dropShadow: {
                "colored-sm": "0 0 1px var(--tw-shadow-color)",
                "colored": "0 0 2px var(--tw-shadow-color)",
                "colored-md": "0 0 3px var(--tw-shadow-color)",
                "colored-lg": "0 0 8px var(--tw-shadow-color)",
            }
        },
        fontFamily: {
            sans: ["Zen Maru Gothic", "Lato", "sans-serif"],
            fancy: ["Videotype", "Raleway", "sans-serif"],
        }
    }
};"###;

#[component]
fn NavLink(#[prop(into)] title: String, #[prop(into)] url: String) -> impl IntoView {
    return view! {
        <a href={url} class="text-gray-300 hover:text-gray-100">{title}</a>
    };
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    return view! {
        <html lang="en">
            <head>
                <title>CTFr</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="/static/favicon.svg"/>
                <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.min.js" />
                <script src="https://cdn.tailwindcss.com/3.3.5"></script>
                <script inner_html={TAILWINDCONFIG.to_string()}></script>
                <style inner_html={FONTSTYLE.to_string()}></style>
            </head>

            <body class="flex flex-col h-screen justify-between bg-dark text-light text-base font-sans m-0">
                <header class="py-3 bg-neutral-800 shadow-md">
                    <nav class="container flex mx-auto justify-between items-center px-10 xl:max-w-screen-xl lg:max-w-screen-lg md:max-w-screen-md">
                        <div class="flex items-center">
                            <a href="/" class="text-accent text-center text-2xl font-fancy animate-flicker shadow-accent">
                                <img
                                    src="/static/favicon.svg"
                                    class="max-h-8 inline mr-2 drop-shadow-colored"
                                /><span class="drop-shadow-colored">CTFr</span>
                            </a>

                            <div class="flex space-x-4 ml-5">
                                <NavLink title="Users" url="#" />
                                <NavLink title="Scoreboad" url="#" />
                                <NavLink title="Challenges" url="#" />
                            </div>
                        </div>
                        // Spacing: 1fr
                        <div class="flex center-items space-x-4">
                                <NavLink title="Login" url="#" />
                                <NavLink title="Register" url="#" />
                        </div>
                    </nav>
                </header>

                <main class="mb-auto mx-auto mt-8 p-5 max-w-screen-md md:text-lg text-base">
                    {children()}
                </main>

                <footer class="footer">
                    <div class="container mx-auto text-center">
                        <a href="https://github.com/MNThomson/CTFr" class="text-secondary" target="_blank" rel="noopener noreferrer">
                            <small class="text-muted">Powered by CTFr | Made with {"♥️"}</small>
                        </a>
                    </div>
                </footer>
            </body>
        </html>
    };
}
