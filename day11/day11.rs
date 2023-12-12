use std::io;

struct Point {
    x: usize,
    y: usize,
}

fn find_galaxies(image: &[Vec<char>], expansion: usize) -> Vec<Point> {
    let no_galaxy_rows: Vec<bool> = image
        .iter()
        .map(|row| row.iter().all(|&c| c == '.'))
        .collect();
    let no_galaxy_columns: Vec<bool> = (0..image[0].len())
        .map(|x| (0..image.len()).map(|y| image[y][x]).all(|c| c == '.'))
        .collect();
    let mut galaxies: Vec<Point> = Vec::new();

    let mut y_offset = 0;
    for y in 0..image.len() {
        let mut x_offset = 0;
        if no_galaxy_rows[y] {
            y_offset += expansion;
            continue;
        }

        for x in 0..image[y].len() {
            if no_galaxy_columns[x] {
                x_offset += expansion;
                continue;
            }

            if image[y][x] == '#' {
                galaxies.push(Point {
                    x: x + x_offset,
                    y: y + y_offset,
                });
            }
        }
    }

    galaxies
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn distances_sum(galaxies: &[Point]) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += manhattan_distance(&galaxies[i], &galaxies[j]);
        }
    }
    sum
}

fn main() {
    let image: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let galaxies = find_galaxies(&image, 1);
    println!("part 1: {}", distances_sum(&galaxies));

    let galaxies = find_galaxies(&image, 1_000_000 - 1);
    println!("part 2: {}", distances_sum(&galaxies));
}
