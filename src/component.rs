use actix_web::{get, HttpResponse, Responder};
use leptos::{component, ssr, view, Children, IntoView, Scope};

#[get("/")]
pub async fn index() -> impl Responder {
    let html = ssr::render_to_string(|cx| {
        view! {cx,
            <DefaultTemplate>
                <Header/>
            </DefaultTemplate>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn default() -> impl Responder {
    let html = ssr::render_to_string(|cx| {
        view! {cx,
            <DefaultTemplate>
                <span 
                    class="text-white font-black row-span-6 row-start-6 col-span-6 col-start-4 flex items-center justify-center text-6xl"
                >Not Found Page</span>
            </DefaultTemplate>
        }
    });
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[component]
fn DefaultTemplate(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <html class="w-full h-full box-border" lang="en">
            <head>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css" integrity="sha512-NhSC1YmyruXifcj/KFRWoC561YpHpc5Jtzgvbuzx5VozKpWvQ+4nXhPdFgmx8xqexRcpAglTj9sIBWINXa8x5w==" crossorigin="anonymous" referrerpolicy="no-referrer" />
                <script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.5/htmx.min.js" integrity="sha512-2NwoAICmYEIEuayBZdfd/cEvYGevbb1jezvQli/Iw052KfAA3NGrXAH2AY02cjt3gMdui5Q8nKauPYgVOE+pmg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
                <link rel="icon" href="/public/icon.svg" sizes="any" type="image/svg+xml"/>
                <link rel="stylesheet" href="/public/tailwind.css"/>
            </head>
            <body class="h-full w-full">
                <main class="bg-black w-full h-full grid grid-cols-12 grid-rows-16">
                    {children(cx)}
                </main>
            </body>
        </html>
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
