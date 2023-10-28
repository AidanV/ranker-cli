use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut stored_movies_file = fs::OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("ranking.txt")
        .expect("Failed opening file");

    let movies_file_as_string =
        fs::read_to_string("ranking.txt").expect("Failed to read as string");

    let mut movies: Vec<String> = movies_file_as_string
        .split('\n')
        .map(|elem| elem.to_string())
        .filter(|elem| elem != "")
        .collect();

    loop {
        println!("Enter a movie (q to quit):");
        let mut movie = String::new();
        io::stdin()
            .read_line(&mut movie)
            .expect("Failed to read line");
        let movie: String = movie.trim().to_string();
        if movie == "q" {
            break;
        }
        let mut left: usize = 0;
        let mut right: usize = movies.len() - 1;
        let mut target: usize = 0;
        while left <= right {
            target = left + (right - left) / 2;
            // println!("left {left} right {right} target {target}");
            let movie_compare = movies[target].clone();
            println!("Is {movie} better than {movie_compare} (yes or no):");
            let mut is_better = String::new();
            io::stdin()
                .read_line(&mut is_better)
                .expect("Failed to read line");

            let is_better: String = is_better.trim().to_lowercase();
            if right == left {
                left += 1;
                target += if is_better == "yes" { 1 } else { 0 };
            } else if is_better == "yes" {
                left = target + 1;
            } else {
                right = target;
            }
        }
        movies.insert(target, movie);

        // stored_movies_file.write(b"").expect("Failed clearing file");
        stored_movies_file.set_len(0).expect("Failed clearing file");

        for m in movies.clone() {
            let mut mv = m.into_bytes();
            let mut te = "\n".to_string().into_bytes();
            mv.append(&mut te);
            stored_movies_file.write_all(&mv).expect("failed write");
        }

        println!("{movies:?}");
    }
}
