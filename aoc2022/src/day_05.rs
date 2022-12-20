use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::{anychar, char, digit1, newline},
    combinator::map_res,
    multi::{count, many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

// (count, from, to)
type Move = (u32, u32, u32);

fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(i)
}

fn existing_crate(i: &str) -> IResult<&str, Option<char>> {
    delimited(char('['), anychar, char(']'))(i).map(|(next, res)| (next, Some(res)))
}

fn empty_space(i: &str) -> IResult<&str, Option<char>> {
    count(char(' '), 3)(i).map(|(next, _)| (next, None))
}

fn cargo_crate(i: &str) -> IResult<&str, Option<char>> {
    alt((existing_crate, empty_space))(i)
}

fn stack_row(i: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list1(char(' '), cargo_crate), newline)(i)
}

fn stacks(i: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (next, rows) = many1(stack_row)(i)?;

    let num_stacks = rows[0].len();
    let mut stacks: Vec<Vec<char>> = (0..num_stacks).map(|_| Vec::new()).collect();

    // Populate our stacks from the bottom up.
    rows.into_iter().rev().for_each(|row| {
        for (i, cargo) in row.iter().enumerate() {
            if let Some(c) = cargo {
                stacks[i].push(*c);
            }
        }
    });

    Ok((next, stacks))
}

fn remove_unnecessary(i: &str) -> IResult<&str, &str> {
    take_until("move")(i)
}

fn crate_move(i: &str) -> IResult<&str, Move> {
    tuple((
        tag("move "),
        number,
        tag(" from "),
        number,
        tag(" to "),
        number,
    ))(i)
    .map(|(next, res)| (next, (res.1, res.3, res.5)))
}

fn moves(i: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, crate_move)(i)
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (next, stacks) = stacks(input).expect("could not parse stacks");
    let (move_start, _) = remove_unnecessary(next)
        .expect("could not remove unnecessary data between stacks and moves");
    let (_, moves) = moves(move_start).expect("could not parse moves");

    (stacks, moves)
}

fn make_move(stacks: &mut [Vec<char>], m: &Move) {
    let (count, from, to) = *m;

    for _ in 0..count {
        match stacks[from as usize - 1].pop() {
            Some(c) => stacks[to as usize - 1].push(c),
            None => panic!("invalid move"),
        }
    }
}

#[aoc(day5, part1)]
fn p1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let (mut stacks, moves) = input.to_owned();
    moves.into_iter().for_each(|m| make_move(&mut stacks, &m));
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn make_move2(stacks: &mut [Vec<char>], m: &Move) {
    let (count, from, to) = *m;

    for i in 0..count as usize {
        let to_stack_len = stacks[to as usize - 1].len();

        match stacks[from as usize - 1].pop() {
            Some(c) => stacks[to as usize - 1].insert(to_stack_len - i, c),
            None => panic!("invalid move"),
        }
    }
}

#[aoc(day5, part2)]
fn p2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let (mut stacks, moves) = input.to_owned();
    moves.into_iter().for_each(|m| make_move2(&mut stacks, &m));
    stacks.iter().map(|s| s.last().unwrap()).collect()
}
