use std::{
    collections::HashSet,
    iter::{repeat, zip},
    str::FromStr,
};

#[derive(Debug)]
enum MoveParseError {
    InvalidMove,
    InvalidValue,
}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_at(1);
        let n = match value.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(MoveParseError::InvalidValue),
        };

        match direction {
            "U" => Ok(Move::Up(n)),
            "D" => Ok(Move::Down(n)),
            "R" => Ok(Move::Right(n)),
            "L" => Ok(Move::Left(n)),
            _ => Err(MoveParseError::InvalidMove),
        }
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<Move>> {
    input
        .lines()
        .map(|l| l.split(',').map(|m| m.parse::<Move>().unwrap()).collect())
        .collect()
}

#[aoc(day3, part1)]
fn p1(input: &[Vec<Move>]) -> Option<i32> {
    let start = (0, 0);
    path_intersections(&input[0], &input[1])
        .into_iter()
        .filter(|i| *i != start)
        .map(|p| manhattan_distance(start, p))
        .min()
}

#[aoc(day3, part2)]
fn p2(input: &[Vec<Move>]) -> Option<usize> {
    let start = (0, 0);
    let path1: Vec<Point> = create_path(&input[0]).into_iter().collect();
    let path2: Vec<Point> = create_path(&input[1]).into_iter().collect();

    path_intersections(&input[0], &input[1])
        .into_iter()
        .filter(|i| *i != start)
        .map(|p| {
            // Add 1 starting position.
            let d1 = path_distance(p, &path1).unwrap() + 1;
            let d2 = path_distance(p, &path2).unwrap() + 1;
            d1 + d2
        })
        .min()
}

fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

fn path_distance(p: Point, path: &[Point]) -> Option<usize> {
    path.iter().position(|point| point == &p)
}

fn path_intersections(first: &[Move], second: &[Move]) -> Vec<Point> {
    let first_path: HashSet<Point> = create_path(first).into_iter().collect();
    let second_path: HashSet<Point> = create_path(second).into_iter().collect();
    first_path.intersection(&second_path).copied().collect()
}

fn create_path(moves: &[Move]) -> Vec<Point> {
    let mut x = 0;
    let mut y = 0;
    moves
        .iter()
        .flat_map(|m| travel(&mut x, &mut y, *m))
        .collect()
}

/**
 * Compute a destination given the current coordinates and a move.
 *
 * Return the path of the points traveled.
 */
fn travel(x: &mut i32, y: &mut i32, m: Move) -> Vec<Point> {
    let start_x = *x;
    let start_y = *y;

    match m {
        Move::Up(n) => *y += n,
        Move::Down(n) => *y -= n,
        Move::Right(n) => *x += n,
        Move::Left(n) => *x -= n,
    }

    generate_point_range((start_x, start_y), (*x, *y))
}

/**
 * Generate the points in the path traveled from start to end.
 */
fn generate_point_range(start: Point, end: Point) -> Vec<Point> {
    let x_range: Box<dyn Iterator<Item = i32>> = match start.0.cmp(&end.0) {
        std::cmp::Ordering::Less => Box::new(start.0 + 1..=end.0),
        std::cmp::Ordering::Equal => Box::new(repeat(start.0)),
        std::cmp::Ordering::Greater => Box::new((end.0..start.0).rev()),
    };

    let y_range: Box<dyn Iterator<Item = i32>> = match start.1.cmp(&end.1) {
        std::cmp::Ordering::Less => Box::new(start.1 + 1..=end.1),
        std::cmp::Ordering::Equal => Box::new(repeat(start.1)),
        std::cmp::Ordering::Greater => Box::new((end.1..start.1).rev()),
    };

    zip(x_range, y_range).collect()
}
