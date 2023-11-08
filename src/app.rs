use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::clone::Clone;
use std::sync::atomic::{AtomicUsize, Ordering};

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
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
    let (compare, set_compare) = create_signal(Option::None::<usize>);
    let add_movie = move |_| {
        if movies.get().len() == 0 {
            set_movies.update(|movies| movies.push(movie.get()));
        } else {
            set_compare(Some(movies.get().len() / 2));
            set_is_ranking(true);
        }
    };
    let up_movie = move |_| {
        let Find {
            left: mut l,
            right: r,
        } = find.get();
        l = 1 + l + (r - l) / 2;
        if l >= r {
            set_movies.update(|movies| movies.insert(movies.len() - l, movie.get()));
            set_is_ranking(false);
            set_find(Find {
                left: 0,
                right: movies.get().len(),
            });
            set_compare(None);
            return;
        }
        let t = l + (r - l) / 2;
        set_compare(Some(t));
        set_find(Find { left: l, right: r });
    };

    let down_movie = move |_| {
        let Find {
            left: l,
            right: mut r,
        } = find.get();

        r = l + (r - l) / 2;

        if l >= r {
            set_movies.update(|movies| movies.insert(movies.len() - l, movie.get()));
            set_is_ranking(false);
            set_find(Find {
                left: 0,
                right: movies.get().len(),
            });
            set_compare(None);
            return;
        }
        // a b c d e f
        let t = l + (r - l) / 2;
        set_compare(Some(t));
        set_find(Find { left: l, right: r });
    };

    let delete_movie =
        move |_, movie_id| set_movies.update(|movies| movies.retain(|m| m.id != movie_id));

    view! {
        <p>{move || match compare.get() {
                    Option::Some(t) => movies.get()[movies.get().len() - 1 - t].name.clone(),
                    Option::None => "Leptos".to_string()
                }}</p>
        <input on:input=move |ev|{
            set_movie({
                Movie {name: event_target_value(&ev), id: get_id()}
            }
            );
        }></input>
        <button on:click=add_movie>"Click Me"</button>
        <For
            each = movies
            key = |movie| movie.id.clone()
            children = move |m| {
                view!{
                    <div style="flex-direction: row;display: flex;justify-content: space-around;">
                    <p>{m.name.clone()}</p>
                    <button on:click=move |ev| delete_movie(ev, m.id)>delete</button>
                    </div>
                }
            }
        />
        <div>{move || if is_ranking() {
            view!{
                <div>
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
