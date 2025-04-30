use itertools::Itertools;

const REQUIRED_RESULT: isize = 10;

fn apply_operation(number_left: isize, operation: char, number_right: isize) -> isize {
    match operation {
        '+' => number_left + number_right,
        '-' => number_left - number_right,
        '*' => number_left * number_right,
        '/' => number_left / number_right,
        _ => {
            panic!("invalid operation");
        }
    }
}

/// Prints "n1 + n2 + n3 + n4 + n5"
/// where the numbers and operations are actually defined by the parameter
fn applied_operations_string(numbers: &[usize], operations: &[char]) -> String {
    let n = numbers[0];
    let mut out_str = format!("{n}");
    for (i, &op) in operations.into_iter().enumerate() {
        out_str += &format!(" {op} {}", numbers[i + 1]);
    }
    out_str
}

fn main() {
    let numbers: Vec<_> = std::env::args()
        .skip(1)
        .take(5)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // operations
    let ops = ['-', '+', '/', '*'];
    // cartesian product can be used to obtain permutations with repetitions
    let ops_all: Vec<_> = itertools::repeat_n(ops, 4)
        .multi_cartesian_product()
        .collect();

    let n_numbers = numbers.len();
    let numbers_permutations: Vec<_> = numbers.into_iter().permutations(n_numbers).collect();

    // --- not parallel
    let mut results = vec![];
    for numbers in numbers_permutations {
        for ops in &ops_all {
            let mut res = numbers[0] as isize;
            for (i, &op) in ops.iter().enumerate() {
                res = apply_operation(res, op, numbers[i + 1] as isize);
            }
            if res == REQUIRED_RESULT {
                let s = applied_operations_string(&numbers, &ops);
                results.push(format!("{s} = {res}"));
            }
        }
    }
    for x in results {
        println!("{x}");
    }
    // dbg!(results.len());

    // --- now in parallel
    // let ops_all = &ops_all;
    // let results = std::thread::scope(|s| {
    //     let mut handles = vec![];
    //     for chunk in numbers_permutations.chunks(numbers_permutations.len() / 8) {
    //         handles.push(s.spawn(move || {
    //             let mut results = vec![];
    //             for numbers in chunk {
    //                 for ops in ops_all {
    //                     let mut res = numbers[0] as isize;
    //                     for (i, &op) in ops.iter().enumerate() {
    //                         res = apply_operation(res, op, numbers[i + 1] as isize);
    //                     }
    //                     if res == REQUIRED_RESULT {
    //                         let s = applied_operations_string(&numbers, &ops);
    //                         results.push(format!("{s} = {res}"));
    //                     }
    //                 }
    //             }
    //             results
    //         }));
    //     }
    //     handles
    //         .into_iter()
    //         .flat_map(|handle| handle.join().unwrap().into_iter())
    //         .collect::<Vec<String>>()
    // });
    // for line in results {
    //     println!("{line}");
    // }
}
