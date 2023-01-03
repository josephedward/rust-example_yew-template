// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}


// use yew::prelude::*;
// use yew_router::prelude::*;


// // ===================================================================================
// // for {username}.github.io/{repo_name}
// // replace 'yew-template-for-github.io' to your repo name

// #[derive(Clone, Routable, PartialEq)]
// enum RootRoute {
//     #[at("/rust-example_yew-template/")]
//     Home,
//     #[at("/rust-example_yew-template/:s")]
//     Route,
// }

// #[derive(Clone, Routable, PartialEq)]
// enum Route {
//     #[at("/rust-example_yew-template/about")]
//     About,
//     #[not_found]
//     #[at("/rust-example_yew-template/404")]
//     NotFound,
// }

// fn root_route(routes: &RootRoute) -> Html {
//     match routes {
//         RootRoute::Home => html! { 
//             //render an iframe
//             <iframe src="https://josephedward.github.io/sci-fi_portfolio/" width="100%vw" height="950px"  />
//          },
//         RootRoute::Route =>
//             html! {
//             <Switch<Route> render={Switch::render(switch)} />
//         },
//     }
// }


// fn switch(routes: &Route) -> Html {
//     match routes {
//         Route::About => html! { <p>{ "About" }</p> },
//         Route::NotFound => html! { <p>{ "Not Found" }</p> },
//     }
// }

// // ===================================================================================
// // for {username}.github.io

// // #[derive(Clone, Routable, PartialEq)]
// //  enum RootRoute {
// //      #[at("/")]
// //      Home,
// //      #[at("/about")]
// //      About,
// //      #[not_found]
// //      #[at("/404")]
// //      NotFound,
// //  }

// //  fn root_route(routes: &Route) -> Html {
// //      match routes {
// //          RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template" }</p> },
// //          RootRoute::About => html! { <p>{ "About" }</p> },
// //          RootRoute::NotFound => html! { <p>{ "Not Found" }</p> },
// //      }
// //  }

// // ===================================================================================

// /// main root
// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         // ********************************************************
//         // **    basename is not supported on yew 0.19.0 yet.    **
//         // <BrowserRouter basename="/rust-example_yew-template/">
//         //     <Switch<Route> render={Switch::render(switch)} />
//         // </BrowserRouter>
//         // ********************************************************
//         <BrowserRouter>
//             <Switch<RootRoute> render={Switch::render(root_route)} />
//         </BrowserRouter>
//     }
// }

// /// entry point
// fn main() {
//     yew::start_app::<App>();
// }