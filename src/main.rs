
use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::prelude::*;
// use yew_hooks::use_counter;


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
        RootRoute::Home => html! { 
            //render a list of external links
            <div class="flex flex-col items-center justify-center h-screen">
                <p class="text-4xl">{ "Yew Template" }</p>
                <ul class="flex flex-col items-center justify-center">
                    <li class="text-2xl"><a href="www.google.com">{"Google"}</a></li>
                    <li class="text-2xl"><a href="www.github.com">{"GitHub"}</a></li>
                    <li class="text-2xl"><a href="www.rust-lang.org">{"Rust"}</a></li>
                </ul>
            </div>
        
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

// ===================================================================================
// for {username}.github.io

// #[derive(Clone, Routable, PartialEq)]
//  enum RootRoute {
//      #[at("/")]
//      Home,
//      #[at("/about")]
//      About,
//      #[not_found]
//      #[at("/404")]
//      NotFound,
//  }

//  fn root_route(routes: &Route) -> Html {
//      match routes {
//          RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template" }</p> },
//          RootRoute::About => html! { <p>{ "About" }</p> },
//          RootRoute::NotFound => html! { <p>{ "Not Found" }</p> },
//      }
//  }

// ===================================================================================

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        // ********************************************************
        // **    basename is not supported on yew 0.19.0 yet.    **
        // <BrowserRouter basename="/rust-example_yew-template/">
        //     <Switch<Route> render={Switch::render(switch)} />
        // </BrowserRouter>
        // ********************************************************
        <BrowserRouter>
            <Switch<RootRoute> render={Switch::render(root_route)} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}