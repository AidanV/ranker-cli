use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);
    let (movie, set_movie) = create_signal("".to_string());
    let initial_length = 1;
    let initial_movies = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (movies, set_movies) = create_signal(initial_movies);
    let mut next_counter_id = initial_length;
    let add_movie = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_movies.update(move |movies| {
            movies.push((next_counter_id, sig));
        });
        next_counter_id += 1;
    };
    // fn store(movies: Vec<String>, movie: String?) {
    //     movies.push(movie);
    //     for movie in movies.clone() {
    //         leptos::logging::log!("{movie}");
    //     }
    // }
    // let test = move |_| add_movie;
    // let counters = (1..=length).map(|idx| create_signal(idx));

    // provide_context(movies);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <input on:input=move |ev|{
            set_movie(event_target_value(&ev));
        }></input>
        <button on:click=add_movie>"Click Me: "  {movie}</button>
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
