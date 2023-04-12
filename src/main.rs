use rand::{thread_rng, Rng};
use std::{
    f64::consts::PI,
    io::{stdin, stdout, BufRead, BufReader, Write},
    process::exit,
};

// Python-like input function that returns the float of whatever input we give
// Panics if the input is not a number
fn input(prompt: &str) -> f64 {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut reader = BufReader::new(stdin());
    let mut buf = "".to_owned();

    match reader.read_line(&mut buf) {
        Ok(_) => {
            if let Some(res) = buf.strip_suffix("\r\n") {
                res.parse().unwrap()
            } else {
                println!("[-] No input was supplied. Exiting...");
                exit(0);
            }
        }
        Err(_) => panic!("[-] Could not read from STDIN"),
    }
}

fn get_percentages(n: f64) -> Vec<f64> {
    fn coor(x: f64) -> f64 {
        (x * 2. * PI).cos() + 1.
    }

    if n == 1. {
        return vec![1.];
    }

    let percentages: Vec<f64> = (0..n.round() as usize)
        .map(|i| coor(1. / (n - 1.) * i as f64))
        .collect();
    let s: f64 = percentages.iter().sum();

    percentages.iter().map(|d| *d * 1. / s).collect()
}

// Random selection shuffle
fn random_payment(days: &f64, pay_per_day: &f64, limit: &f64) -> Vec<f64> {
    let mut t = thread_rng();
    let mut payments: Vec<f64> = (0..days.round() as u64).map(|_| *pay_per_day).collect();

    // Check if we have gone too far or not enough and balance the amounts so it matches the desired quantity
    let dif = limit - payments.iter().sum::<f64>();

    if dif < 0. {
        payments[0] -= dif;
    } else if dif > 0. {
        payments[0] += dif;
    }

    // For n iterations, select a random value between 0 and 1 and substract it from an index at random and add it to another at random.
    // Do this for the whole length of the array per iteration
    const ITERS: u8 = 5;
    const MUL_BY: f64 = 4.;

    let arr_len = payments.len() as f64;

    for _ in 0..ITERS {
        for _ in 0..arr_len as usize {
            let random_amount: f64 = t.gen::<f64>() * MUL_BY;
            let random_indexes: Vec<usize> = (0..2)
                .map(|_| (t.gen::<f64>() * arr_len).floor() as usize)
                .collect();

            if payments[random_indexes[0]] > random_amount {
                payments[random_indexes[0]] -= random_amount;
                payments[random_indexes[1]] += random_amount;
            }
        }
    }

    // Round up the payment digits to two
    payments = payments.iter().map(|d| (d * 100.).round() / 100.).collect();

    // Check if we are paying too much or too little and adjust quantity accordingly
    let mut owe_or_get = limit - payments.iter().sum::<f64>() * 100.;

    if owe_or_get < 0.001 || owe_or_get > -0.001 {
        owe_or_get = 0.;
    }

    if owe_or_get > 0. {
        for (i, item) in payments.iter().enumerate() {
            if item - owe_or_get > 0. {
                payments[i] -= owe_or_get;
                break;
            }
        }
    } else if owe_or_get < 0. {
        payments[0] += owe_or_get;
    }

    payments
}

// Random selection shuffle
fn by_groups_of_n_payment(days: &f64, pay_per_day: &f64, limit: &f64, n: usize) -> Vec<f64> {
    let mut payments: Vec<f64> = (0..days.round() as u64).map(|_| *pay_per_day).collect();
    let mut n = n;

    // Check if we have gone too far or not enough and balance the amounts so it matches the desired quantity
    let dif = limit - payments.iter().sum::<f64>();

    if dif < 0. {
        payments[0] -= dif;
    } else if dif > 0. {
        payments[0] += dif;
    }

    // Spread by percenteges
    let item_len = payments.len();
    let percentages: Vec<f64> = get_percentages(n as f64);
    let iters = item_len / n * n;

    for j in 0..iters {
        payments[j] *= percentages[j % n] * n as f64;
    }

    if iters < item_len {
        n = item_len - iters;
        let percentages = get_percentages(n as f64);

        for j in iters..iters + n {
            payments[j] *= percentages[j - iters] * n as f64;
        }
    }

    // Round 'em all up
    payments = payments
        .iter()
        .map(|d| (*d * 100.).round() / 100.)
        .collect();

    // Check if we are paying too much or too little and adjust quantity accordingly
    let owe_or_get = payments.iter().sum::<f64>() - limit;

    if owe_or_get < 0. {
        payments[0] -= owe_or_get;
    } else {
        for (i, item) in payments.iter().enumerate() {
            if item - owe_or_get > 0. {
                payments[i] -= owe_or_get;
                break;
            }
        }
    }

    // Return rounded n
    payments
        .iter()
        .map(|d| (*d * 100.).round() / 100.)
        .collect()
}

fn main() {
    let limit = input("What is the amount you want to pay? ");
    let days = input("Over how many days? ");

    if limit <= 0. || days <= 0. {
        println!("[-] The amount nor the days can be 0 or less than 0");
        exit(1);
    }

    // Define pay per day variable so we dont have to do the calculation per iteration
    let pay_per_day = limit / days;

    // Get the payments
    let technique = input("Would you like to get payments randomly or by groups? [0, 1]: ");
    let payments: Vec<f64> = match technique.round() as usize {
        0 => random_payment(&days, &pay_per_day, &limit),
        1 => {
            let groups = input("By groups of n, n being: ");
            by_groups_of_n_payment(&days, &pay_per_day, &limit, groups as usize)
        }
        _ => vec![],
    };

    // If this panics then something went wrong
    assert!(
        limit - payments.iter().sum::<f64>() < 0.01 || limit - payments.iter().sum::<f64>() > -0.01,
        "[-] More than a cent is left to pay or we are paying more than 1 cent over the limit so the algorithm did not work"
    );

    // Print out the results
    println!("Payments to make:\n");

    for (i, n) in payments.iter().enumerate() {
        println!("Day {}: {}€", i, n);
    }

    println!(
        "\nSum of Transactions: {}€",
        (payments.iter().sum::<f64>() * 100.).round() / 100.
    )
}
