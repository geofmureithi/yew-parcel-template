use yew::prelude::*;
use yew_router::{prelude::*, Switch, route::Route};

pub struct App;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to= "/!"]
    RootPath,
    #[to= "/hello!"]
    HelloPath,
    #[to= "/awasome!"]
    AwasomePath,
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{<h2>{"this is root"}</h2>},
                            AppRouter::HelloPath => html!{<h2>{"Hello world"}</h2>},
                            AppRouter::AwasomePath => html!{<h2>{"My awesome Yew with Yew-Router and Parcel application!"}</h2>},
                            AppRouter::PageNotFound(None) => html!{"Page not found"},
                            AppRouter::PageNotFound(Some(missed_route)) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Some(route.route))
                    })
                />
            </div>
        }
    }
 }