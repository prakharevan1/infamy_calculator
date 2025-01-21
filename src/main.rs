use std::io;
fn main() {
    //parse input
    let mut start_inf = String::new();
    let mut end_inf = String::new();
    let mut cheaper_inf = String::new();

    //parse starting infamy input

    println!("Please enter your starting infamy:");
    io::stdin()
        .read_line(&mut start_inf)
        .expect("Failed to read line. Please restart or unwanted results may occur!");
    let start_inf: i32 = start_inf
        .trim()
        .parse()
        .expect("Failed to parse input. Please restart or unwanted results may occur!");

    //parse ending infamy input

    println!("Please enter your ending infamy:");
    io::stdin()
        .read_line(&mut end_inf)
        .expect("Failed to read line. Please restart or unwanted results may occur!");
    let end_inf: i32 = end_inf
        .trim()
        .parse()
        .expect("Failed to parse input. Please restart or unwanted results may occur!");

    //check if cheaper infamy or not

    println!("Do you have cheaper infamy? (Y/N):");
    io::stdin()
        .read_line(&mut cheaper_inf)
        .expect("Failed to get input. Please restart or unwanted results may occur!");
    let cheaper_inf = cheaper_inf.trim();
    let cheaper_inf = match cheaper_inf {
        "Y" => true,
        "N" => false,
        "y" => true,
        "n" => false,
        _ => false,
    };
    //check infamy values
    let mut cost: i128;
    if start_inf == end_inf || end_inf < start_inf {
        return;
    }
    if end_inf > 25 {
        cost = above_inf_25(start_inf, end_inf);
    } else {
        cost = below_inf_25(start_inf, end_inf);
    }
    if cheaper_inf == true {
        cost /= 2;
    }
    println!("The total cost to get from infamy {start_inf} to {end_inf} is {cost} dollars");
}
fn above_inf_25(start_inf: i32, end_inf: i32) -> i128 {
    let start_inf: i128 = start_inf as i128; //cloned
    let end_inf: i128 = end_inf as i128; //cloned, parses it as an i128
    let mut total: i128 = 0;

    //block
    let mut start_price = 20_000_000;
    {
        if start_inf > 25 {
            for _i in 25..start_inf {
                //gen start price
                start_price += 10_000_000; //price is 20m
            }
        }
    }
    //block 2
    {
        total += 10_000_000; //starts off at 30m :)
        for _i in 25..end_inf - 1 {
            total += start_price;
            start_price += 10_000_000;
        }
        let mut diff: i128 = end_inf - start_inf;
        diff *= 20_000_000;
        total += diff;
    }
    total
}
fn below_inf_25(start_inf: i32, end_inf: i32) -> i128 {
    let start_inf = start_inf as i128; //clones and parses as an i128
    let end_inf = end_inf as i128; //clones and parses as an 128
    let mut total: i128 = end_inf - start_inf;
    total *= 20_000_000;
    total
}
