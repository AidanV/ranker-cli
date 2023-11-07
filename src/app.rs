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

#[derive(Clone)]
struct Find {
    left: usize,
    right: usize,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (movie, set_movie) = create_signal(Movie {
        name: "".to_string(),
        id: 0,
    });
    let (movies, set_movies) = create_signal(Vec::<Movie>::new());
    let (is_ranking, set_is_ranking) = create_signal(false);
    let (find, set_find) = create_signal(Find { left: 0, right: 0 });
    let add_movie = move |_| {
        if movies.get().len() == 0 {
            log!("movies was 0 {}", movie.get().name);
            set_movies.update(|movies| movies.push(movie.get()));
        } else {
            log!("{}", movies.get()[0].name);
            set_is_ranking(true);
        }
    };
    let up_movie = move |_| {
        log!("movie: {}", movie.get().name);
        // set_find.update(|f| {
        //     f.left = 0;
        //     f.right = movies.get().len() - 1;
        //     f.target = f.left + (f.right - f.left) / 2;
        // });
        let Find {
            left: mut l,
            right: r,
        } = find.get();
        if l > r {
            set_movies.update(|movies| movies.insert(movies.len() - l, movie.get()));
            set_is_ranking(false);
            set_find(Find {
                left: 0,
                right: movies.get().len() - 1,
            });
            return;
        }
        let t = l + (r - l) / 2;
        log!("l{} r{} t{}", l, r, t);
        if l == r {
            l += 1;
        } else {
            l = t + 1;
        }
        set_find(Find { left: l, right: r });
    };

    let down_movie = move |_| {
        log!("movie: {}", movie.get().name);
        // set_find.update(|f| {
        //     f.left = 0;
        //     f.right = movies.get().len() - 1;
        //     f.target = f.left + (f.right - f.left) / 2;
        // });
        let Find {
            left: mut l,
            right: mut r,
        } = find.get();
        if l > r {
            set_movies.update(|movies| movies.insert(movies.len() - l, movie.get()));
            set_is_ranking(false);
            set_find(Find {
                left: 0,
                right: movies.get().len() - 1,
            });
            return;
        }
        let t = l + (r - l) / 2;
        log!("l{} r{} t{}", l, r, t);
        if l == r {
            l += 1;
        } else {
            r = t;
        }
        set_find(Find { left: l, right: r });
    };
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <input on:input=move |ev|{
            set_movie({
                log!("set movie");
                Movie {name: event_target_value(&ev), id: movies.get().len()}
            }
            );//(Movie {name: event_target_value(&ev), id: 0});
        }></input>
        <button on:click=add_movie>"Click Me:"{movie.get().name}</button>
        <For
            each = movies
            key = |movie| movie.id.clone()
            children = move |m| {
                view!{
                    <p>{m.name.clone()}</p>
                }
            }
        />
        // <Rank should_rank=&move || is_ranking()/>
        <div>{move || if is_ranking() {
            view!{
                <div>
                // <p>{if movies.get().len() > 0 {movies.get()[find.get().left].name.clone()} else {"hi".to_string()}}</p>
                <button on:click=up_movie>Up</button>
                <button on:click=down_movie>Down</button>
                </div>
            }
        } else {
            view!{
                <div></div>
            }
        }}</div>
    }
}

#[component]
fn Rank(should_rank: &'static dyn Fn() -> bool) -> impl IntoView {
    if should_rank() {
        view! {
            <p>testing</p>
        }
    } else {
        view! {
        <p>tested</p>
        }
    }
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
