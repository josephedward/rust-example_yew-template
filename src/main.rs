
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
        RootRoute::Home => html! { 
            
            <ul>
                <li class="listitem"> <a href="https://josephedward.vercel.app" target="_blank" rel="noopener noreferrer">{"Click here to view my portfolio"}</a></li>
                //leetcode 
                <li class="listitem"> <a href="https://leetcode.com/josephedward/" target="_blank" rel="noopener noreferrer">{"Click here to view my leetcode"}</a></li>
                //hackerrank
                <li class="listitem"> <a href="https://www.hackerrank.com/josephedward" target="_blank" rel="noopener noreferrer">{"Click here to view my hackerrank"}</a></li>
                //codewars
                <li class="listitem"> <a href="https://www.codewars.com/users/josephedward" target="_blank" rel="noopener noreferrer">{"Click here to view my codewars"}</a></li>
                //github
                <li class="listitem"> <a href="github.com/josephedward" target="_blank" rel="noopener noreferrer">{"Click here to view my github"}</a></li>
                //linkedin
                <li class="listitem"> <a href="https://www.linkedin.com/in/joseph-e-ab7355185/" target="_blank" rel="noopener noreferrer">{"Click here to view my linkedin"}</a></li>
                //about.me
                <li class="listitem"> <a href="https://about.me/joseph_edward" target="_blank" rel="noopener noreferrer">{"Click here to view my about.me"}</a></li>
                //dev.to
                <li class="listitem"> <a href="https://dev.to/josephedward" target="_blank" rel="noopener noreferrer">{"Click here to view my dev.to"}</a></li>
                //youtube
                <li class="listitem"> <a href="https://www.youtube.com/channel/UCthtlhbzBDgS8DneOYIVH5g" target="_blank" rel="noopener noreferrer">{"Click here to view my youtube"}</a></li>
                //codepen
                <li class="listitem"> <a href="https://codepen.io/josephedward" target="_blank" rel="noopener noreferrer">{"Click here to view my codepen"}</a></li>
                //credly
                <li class="listitem"> <a href="https://www.credly.com/users/joseph-edward.7a7a47e9/badges" target="_blank" rel="noopener noreferrer">{"Click here to view my credly"}</a></li>
                //angel.co
                <li class="listitem"> <a href="https://angel.co/u/josephedward01" target="_blank" rel="noopener noreferrer">{"Click here to view my angel.co"}</a></li>
                
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