mod app;

use app::WordClockComponent;

use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/ch")]
    ChBern,
    #[at("/en")]
    EnUk,
    #[at("/de")]
    DeDe,
    #[at("/")]
    Default,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::ChBern => html! { <WordClockComponent language="ch-bern" /> },
        Route::EnUk => html! { <WordClockComponent language="en-uk" /> },
        Route::DeDe => html! { <WordClockComponent language="de-de" /> },
        _ => html! { <WordClockComponent language="en-uk" /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
