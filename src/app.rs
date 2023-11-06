use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::clone::Clone;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone)]
struct Movie {
    name: String,
    id: usize,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (movie, set_movie) = create_signal("".to_string());
    let (movies, set_movies) = create_signal(Vec::new());
    let add_movie = move |_| {
        let m: Movie = Movie {
            name: movie.get(),
            id: movies.get().len(),
        };
        set_movies.update(|movies| movies.push(m));
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <input on:input=move |ev|{
            set_movie(event_target_value(&ev));
        }></input>
        <button on:click=add_movie>"Click Me: "  {movie}</button>
        <For
            each = movies
            key = |movie| movie.id
            children = move |m| {
                view!{
                    <p>{m.name}</p>
                }
            }
        />
    }
}

#[component]
fn MovieList() -> impl IntoView {
    // return view! {<text> test </text>};
    // let movies = use_context::<ReadSignal<Vec<String>>>().expect("failed context");
    // return view! { each=movies
    //     children=move |(id, (movie, set_movie))| {
    //         view! {<text>{movie}</text>}
    //     }
    // };
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
