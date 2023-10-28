use std::fs;
use std::io;
use std::io::Read;

fn get_file() -> fs::File {
    return match fs::File::open("ranking.txt") {
        Ok(file) => file,
        Err(_) => {
            return fs::File::create("ranking.txt").expect("Failed creating file");
        }
    };
}

fn main() {
    let mut stored_movies_file = get_file();

    let mut movies_file_as_string = String::new();
    let _ = stored_movies_file.read_to_string(&mut movies_file_as_string);

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
            println!("Is {movie} better than {movie_compare}:");
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
        println!("{movies:?}");
    }
}
