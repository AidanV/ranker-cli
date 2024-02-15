use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, SetTitle},
    Command, ExecutableCommand, QueueableCommand,
};
use std::{
    borrow::BorrowMut,
    io::{self, stdout, Stdout, Write},
};

enum Mode {
    ADD,
    SORT,
    QUIT,
}

struct State {
    mode: Mode,
}

fn handle_add(out: &mut Stdout, mut state: State) -> State {
    disable_raw_mode().expect("Failed to disable raw");
    out.execute(Clear(terminal::ClearType::All))
        .expect("Failed to clear");

    out.execute(MoveTo(0, 0)).unwrap();
    print!("Enter a movie to rank: ");

    Stdout::flush(out).expect("Failed flush");
    match read().expect("failed read") {
        Event::FocusGained => todo!(),
        Event::FocusLost => todo!(),
        Event::Key(event) => {
            match event.code {
                KeyCode::Char('q') => state.mode = Mode::QUIT,
                KeyCode::Char('a') => state.mode = Mode::SORT,
                _ => {
                    // out.queue(Clear(terminal::ClearType::All))
                    //     .expect("Failed to clear");
                    // let terminal::WindowSize { rows, .. } =
                    //     terminal::window_size().expect("Failed to get window size");

                    // out.queue(MoveTo(20, rows / 2)).unwrap();
                    // println!("we in here :{:?}: \n columns = {}", event, rows)
                }
            }
        }
        Event::Mouse(_) => todo!(),
        Event::Paste(_) => todo!(),
        Event::Resize(_, _) => todo!(),
    }
    state
}

fn handle_sort(out: &mut std::io::Stdout, mut state: State) -> State {
    out.execute(Clear(terminal::ClearType::All))
        .expect("Failed to clear");

    let terminal::WindowSize { rows, .. } =
        terminal::window_size().expect("Failed to get window size");

    out.execute(MoveTo(20, rows / 2)).unwrap();
    print!("Sort: ");
    match read().expect("failed read") {
        Event::FocusGained => todo!(),
        Event::FocusLost => todo!(),
        Event::Key(event) => {
            match event.code {
                KeyCode::Char('q') => state.mode = Mode::QUIT,
                KeyCode::Char('a') => state.mode = Mode::ADD,
                _ => {
                    // out.queue(Clear(terminal::ClearType::All))
                    //     .expect("Failed to clear");
                    // let terminal::WindowSize { rows, .. } =
                    //     terminal::window_size().expect("Failed to get window size");

                    // out.queue(MoveTo(20, rows / 2)).unwrap();
                    // println!("we in here :{:?}: \n columns = {}", event, rows)
                }
            }
        }
        Event::Mouse(_) => todo!(),
        Event::Paste(_) => todo!(),
        Event::Resize(_, _) => todo!(),
    }
    state
}

fn main() -> io::Result<()> {
    let mut out = stdout();
    let mut state = State { mode: Mode::ADD };
    enable_raw_mode().expect("Failed to enter raw mode");
    println!("here");
    loop {
        state = match state.mode {
            Mode::ADD => handle_add(out.borrow_mut(), state),
            Mode::SORT => {
                handle_sort(out.borrow_mut(), state)
                // out.queue(Clear(terminal::ClearType::All))
                //     .expect("Failed to clear");

                // let terminal::WindowSize { rows, .. } =
                //     terminal::window_size().expect("Failed to get window size");

                // out.queue(MoveTo(20, rows / 2)).unwrap();
                // print!("Is it better");
                // match read()? {
                //     Event::FocusGained => todo!(),
                //     Event::FocusLost => todo!(),
                //     Event::Key(event) => {
                //         match event.code {
                //             KeyCode::Char('q') => break,
                //             _ => {
                //                 // out.queue(Clear(terminal::ClearType::All))
                //                 //     .expect("Failed to clear");
                //                 // let terminal::WindowSize { rows, .. } =
                //                 //     terminal::window_size().expect("Failed to get window size");

                //                 // out.queue(MoveTo(20, rows / 2)).unwrap();
                //                 // println!("we in here :{:?}: \n columns = {}", event, rows)
                //             }
                //         }
                //     }
                //     Event::Mouse(_) => break,
                //     Event::Paste(_) => todo!(),
                //     Event::Resize(_, _) => todo!(),
                // }
            }
            Mode::QUIT => break,
        }
    }
    disable_raw_mode().expect("Failed to exit raw mode");
    Ok(())
}

// let movies_file_as_string =
//     fs::read_to_string("ranking.txt").expect("Failed to read as string");

// let mut movies: Vec<String> = movies_file_as_string
//     .split('\n')
//     .map(|elem| elem.to_string())
//     .filter(|elem| elem != "")
//     .collect();

// println!("Enter a movie (enter empty string to quit):");
// let mut movie = String::new();
// io::stdin()
//     .read_line(&mut movie)
//     .expect("Failed to read line");
// let movie: String = movie.trim().to_string();
// if movie == "" {
//     break;
// }
// if movies.len() == 0 {
//     movies.push(movie);
// } else {
//     let mut left: usize = 0;
//     let mut right: usize = movies.len() - 1;
//     let mut target: usize = 0;

//     while left <= right {
//         target = left + (right - left) / 2;
//         // println!("left {left} right {right} target {target}");
//         let movie_compare = movies[target].clone();
//         println!("Is {movie} better than {movie_compare} (yes or no):");
//         let mut is_better = String::new();
//         io::stdin()
//             .read_line(&mut is_better)
//             .expect("Failed to read line");

//         let is_better: String = is_better.trim().to_lowercase();
//         if right == left {
//             left += 1;
//             target += if is_better == "yes" { 1 } else { 0 };
//         } else if is_better == "yes" {
//             left = target + 1;
//         } else {
//             right = target;
//             if left == right {
//                 break;
//             }
//         }
//     }
//     movies.insert(movies.len() - target, movie);
// }

// fs::write("ranking.txt", movies.join("\n")).expect("failed to write");

// println!("{movies:?}");
