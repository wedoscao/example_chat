use actix_web::{get, HttpResponse, Responder};
use leptos::*;

#[get("/")]
pub async fn index() -> impl Responder {
    let html = ssr::render_to_string(|cx| {
        view! {cx,
            <html class="w-full h-full box-border" lang="en">
                <head>
                    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css" integrity="sha512-NhSC1YmyruXifcj/KFRWoC561YpHpc5Jtzgvbuzx5VozKpWvQ+4nXhPdFgmx8xqexRcpAglTj9sIBWINXa8x5w==" crossorigin="anonymous" referrerpolicy="no-referrer" />
                    <script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.5/htmx.min.js" integrity="sha512-2NwoAICmYEIEuayBZdfd/cEvYGevbb1jezvQli/Iw052KfAA3NGrXAH2AY02cjt3gMdui5Q8nKauPYgVOE+pmg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
                    <link rel="stylesheet" href="/public/style/tailwind.css"/>
                </head>
                <body class="h-full w-full">
                    <App/>
                </body>
            </html>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <main class="bg-black w-full h-full grid grid-cols-12 grid-rows-16">
            <Header/>
        </main>
    }
}

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <header class="col-span-12 row-span-1 text-white font-bold grid grid-cols-12">
            <span class="col-span-1 col-start-1 p-2 flex items-center justify-center">Example Chat</span>
        </header>
    }
}
