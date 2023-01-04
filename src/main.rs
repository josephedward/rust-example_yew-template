use yew::prelude::*;
use yew_router::prelude::*;

// ===================================================================================
// for {username}.github.io/{repo_name}
// replace 'yew-template-for-github.io' to your repo name

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/rust-example_yew-template/")]
    Home,
    #[at("/rust-example_yew-template/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/rust-example_yew-template/about")]
    About,
    #[not_found]
    #[at("/rust-example_yew-template/404")]
    NotFound,
}

fn root_route(routes: &RootRoute) -> Html {
    match routes {
        RootRoute::Home =>
            html! { 
            <ul>
                <a class="listitem" href="https://josephedward.vercel.app" target="_blank" rel="noopener noreferrer">{"portfolio"}</a>
                <a class="listitem" href="https://leetcode.com/josephedward/" target="_blank" rel="noopener noreferrer">{"leetcode"}</a>
                <a class="listitem" href="https://www.hackerrank.com/josephedward" target="_blank" rel="noopener noreferrer">{"hackerrank"}</a>
                <a class="listitem" href="https://www.codewars.com/users/josephedward" target="_blank" rel="noopener noreferrer">{"codewars"}</a>
                <a class="listitem" href="https://github.com/josephedward" target="_blank" rel="noopener noreferrer">{"github"}</a>
                <a class="listitem" href="https://www.linkedin.com/in/joseph-e-ab7355185/" target="_blank" rel="noopener noreferrer">{"linkedin"}</a>
                <a class="listitem" href="https://about.me/joseph_edward" target="_blank" rel="noopener noreferrer">{"about.me"}</a>
                <a class="listitem" href="https://dev.to/josephedward" target="_blank" rel="noopener noreferrer">{"dev.to"}</a>
                <a class="listitem" href="https://www.youtube.com/channel/UCthtlhbzBDgS8DneOYIVH5g" target="_blank" rel="noopener noreferrer">{"youtube"}</a>
                <a class="listitem" href="https://codepen.io/josephedward" target="_blank" rel="noopener noreferrer">{"codepen"}</a>
                <a class="listitem" href="https://www.credly.com/users/joseph-edward.7a7a47e9/badges" target="_blank" rel="noopener noreferrer">{"credly"}</a>
                <a class="listitem" href="https://angel.co/u/josephedward01" target="_blank" rel="noopener noreferrer">{"angel.co"}</a>
            </ul>
        },
        RootRoute::Route =>
            html! {
            <Switch<Route> render={Switch::render(switch)} />
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<RootRoute> render={Switch::render(root_route)} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}