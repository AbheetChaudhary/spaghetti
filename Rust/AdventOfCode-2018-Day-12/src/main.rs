use std::collections::VecDeque;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const GENERATIONS_V1: usize = 20;
const GENERATIONS_V2: usize = 50_000_000_000;
const SURROUNDING: usize = 5;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <inputfile>", args[0]);
        std::process::exit(-1);
    }

    let input_untrimmed = std::fs::read_to_string(&args[1])?;
    let input = input_untrimmed.trim();

    let intermediate = input.split("\n\n").collect::<Vec<&str>>();

    let (initial_state_text, table_text) = (intermediate[0], intermediate[1]);

    let pots = pots_from_str(initial_state_text.split(':').skip(1).next().unwrap());

    let mut table = [false; 32];

    for line in table_text.trim().lines() {
        let intermediate = line.split(" => ").collect::<Vec<&str>>();
        let index = intermediate[0].as_bytes().iter().enumerate().fold(0usize, |acc, (i, &val)| {
            if val == b'#' {
                acc + 2usize.pow(i as u32)
            } else {
                acc
            }
        });

        table[index] = match intermediate[1].trim() {
            "#" => true,
            "." => false,
            _ => unreachable!(),
        };
    }

    // println!("{:?}", table);

    part1(pots.clone(), &table, GENERATIONS_V1)?;
    part2(pots.clone(), &table)?;

    Ok(())
}

fn pots_from_str(state: &str) -> Vec<Pot> {
    let mut pots = Vec::with_capacity(state.trim().as_bytes().len());
    for &c in state.trim().as_bytes().iter() {
        if c == b'#' {
            pots.push(true);
        } else {
            pots.push(false);
        }
    }

    pots
}

type Pot = bool;

fn part1(mut state: Vec<Pot>, table: &[bool], generations: usize) -> Result<()> {
    // start and end indices on the numberline that represent the beginning and end of the 'state'
    // vec
    let mut start = 0i32;
    let mut end = state.len() as i32 - 1;

    // dump(VecDeque::from(state.clone()), start, end, 0, 0, 24);
    for gen in 1..=generations {
        // find index where a planted pot occurs
        let idx_first_plant = state.iter().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        // last index where a planted pot occurs
        let idx_last_plant = state.len() - 1 - state.iter().rev().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        // start checking pots from first pot index minus 2 to last pot index plus 2
        let idx_start = idx_first_plant as i32 - 2;
        let idx_end = idx_last_plant as i32 + 2;

        // fertility for next generation
        let mut fertility = Vec::with_capacity((idx_end - idx_start) as usize + 1);

        for idx in idx_start..=idx_end {
            // println!("{idx}");
            fertility.push(check_fertility(&state, table, idx));
        }

        state = fertility;
        start += idx_first_plant as i32 - 2;
        end = idx_end;

        // dump(VecDeque::from(state.clone()), start, end, gen, idx_first_plant, idx_last_plant);
    }

    let sum = state.iter().enumerate().fold(0i32, |acc, (i, &val)| {
        if val {
            acc + i as i32 + start
        } else {
            acc
        }

    });

    println!("{sum}");

    Ok(())
}

fn part2(mut state: Vec<Pot>, table: &[bool]) -> Result<()> {
    let state_backup = state.clone();
    let mut cache: HashSet<Vec<bool>> = HashSet::new();
    let mut period: usize = 1;
    // start and end indices on the numberline that represent the beginning and end of the 'state'
    // vec
    let mut start = 0i32;
    let mut end = state.len() as i32 - 1;

    let gen_zero_first_plant = state.iter().enumerate().find(|(_, &val)| {
        val
    }).unwrap().0 as isize;

    // first plant location of final generation before repeating
    let mut gen_zero_again_first_plant: isize = 0;

    // dump(VecDeque::from(state.clone()), start, end, 0, 0, 24);
    for gen in 1..=GENERATIONS_V2 {
        // find index where a planted pot occurs
        let idx_first_plant = state.iter().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        gen_zero_again_first_plant = start as isize + idx_first_plant as isize;
        // last index where a planted pot occurs
        let idx_last_plant = state.len() - 1 - state.iter().rev().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        // start checking pots from first pot index minus 2 to last pot index plus 2
        let idx_start = idx_first_plant as i32 - 2;
        let idx_end = idx_last_plant as i32 + 2;

        start += idx_first_plant as i32 - 2;
        end = idx_end;


        if !cache.insert((&state[idx_first_plant..=idx_last_plant]).to_vec()) {
            break;
        }

        // fertility for next generation
        let mut fertility = Vec::with_capacity((idx_end - idx_start) as usize + 1);

        for idx in idx_start..=idx_end {
            // println!("{idx}");
            fertility.push(check_fertility(&state, table, idx));
        }

        state = fertility;

        // dump(VecDeque::from(state.clone()), start, end, gen, idx_first_plant, idx_last_plant);
        period += 1;
        if period & 0xffff == 0 {
            println!("processing: {}", period);
        }
    }
    println!("period: {period}");

    let gen_shift_amount = gen_zero_again_first_plant - gen_zero_first_plant;
    let repetition_count = (GENERATIONS_V2 / period) as isize;
    let shift = gen_shift_amount * repetition_count;

    state = state.clone();

    for gen in 1..=(GENERATIONS_V2 % period) {
        // find index where a planted pot occurs
        let idx_first_plant = state.iter().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        // last index where a planted pot occurs
        let idx_last_plant = state.len() - 1 - state.iter().rev().enumerate().find(|(_, &val)| {
            val
        }).unwrap().0;

        // start checking pots from first pot index minus 2 to last pot index plus 2
        let idx_start = idx_first_plant as i32 - 2;
        let idx_end = idx_last_plant as i32 + 2;

        // fertility for next generation
        let mut fertility = Vec::with_capacity((idx_end - idx_start) as usize + 1);

        for idx in idx_start..=idx_end {
            // println!("{idx}");
            fertility.push(check_fertility(&state, table, idx));
        }

        state = fertility;
        start += idx_first_plant as i32 - 2;
        end = idx_end;

        // dump(VecDeque::from(state.clone()), start, end, gen, idx_first_plant, idx_last_plant);
    }
    // find index where a planted pot occurs
    let idx_first_plant = state.iter().enumerate().find(|(_, &val)| {
        val
    }).unwrap().0;

    // last index where a planted pot occurs
    let idx_last_plant = state.len() - 1 - state.iter().rev().enumerate().find(|(_, &val)| {
        val
    }).unwrap().0;

    let mod_shift = start as isize + idx_last_plant as isize - 0 - gen_zero_first_plant;

    let tree_count = state.iter().fold(0usize, |acc, &val| {
        if val {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", index_sum(&state, shift + start as isize));

    let sum = &state[idx_first_plant..=idx_last_plant].iter().enumerate().fold(0isize, |acc, (i, &val)| {
        if val {
            acc + i as isize + shift + mod_shift
        } else {
            acc
        }

    });


    println!("{}", sum);

    Ok(())
}

fn index_sum(state: &Vec<bool>, start: isize) -> isize {
    state.iter().enumerate().fold(0isize, |acc, (i, &val)| {
        if val {
            acc + start + i as isize
        } else {
            acc
        }
    })
}

fn _dump(mut state: VecDeque<bool>, start: i32, end: i32, generation: u32, first_idx: usize, last_idx: usize) {
    let start_goal = -3;
    let end_goal = 35;

    if start_goal < start {
        for _ in 0..(start - start_goal) {
            state.push_front(false);
        }
    }

    if end_goal > end {
        for _ in 0..(end_goal - end) {
            state.push_back(false);
        }
    }

    let state_string = state.iter().map(|&val| {
        if val {
            '#'
        } else {
            '.'
        }
    }).collect::<String>();

    println!("{generation:0>2}: {state_string}  {first_idx} {last_idx} {}", state_string.len());
}

/// check fertitily at given index
fn check_fertility(state: &Vec<Pot>, table: &[bool], idx: i32) -> bool {
    if idx < 2 { // surrounding window extends too backward
        // create the later part of surrounding window
        let mut surrounding_later = state.iter()
            .take((idx + 2) as usize + 1)
            .map(|&b| b)
            .collect::<Vec<bool>>();

        // create initial part of surrounding window and fill it with false
        let mut surrounding = vec![false; SURROUNDING - surrounding_later.len()];

        // complete surrounding window
        surrounding.append(&mut surrounding_later);

        assert_eq!(surrounding.len(), 5);
        // get index from surrounding window and find fertility
        table[calculate_table_index(&surrounding)]

    } else if idx as usize >= state.len() - 2 { // surrounding window extends too far
        // create the initial part of surrounding window
        let mut surrounding = state.iter()
            .skip((idx - 2) as usize)
            .map(|&b| b)
            .collect::<Vec<bool>>();

        // create later part of surrounding window and fill it with false
        let mut surrounding_later = vec![false; SURROUNDING - surrounding.len()];

        // complete surrounding window
        surrounding.append(&mut surrounding_later);

        assert_eq!(surrounding.len(), 5);
        // get index from surrounding window and find fertility
        table[calculate_table_index(&surrounding)]

    } else {
        let surrounding = state.iter()
            .skip((idx - 2) as usize)
            .take(5)
            .map(|&b| b)
            .collect::<Vec<bool>>();

        assert_eq!(surrounding.len(), 5);
        // get index from surrounding window and find fertility
        table[calculate_table_index(&surrounding)]

    }
}

#[inline]
/// calculate the index in table using surrounding window as a binary representation
fn calculate_table_index(surrounding: &[bool]) -> usize {
    surrounding[..SURROUNDING].into_iter()
        .enumerate()
        .fold(0usize, |acc, (i, &val)| {
            if val {
                acc + 2usize.pow(i as u32)
            } else {
                acc
            }
        })
}
