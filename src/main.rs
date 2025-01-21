fn main() {
    //parse input



    let start_inf = 16;
    let end_inf = 23;
    if start_inf == end_inf || end_inf < start_inf {
        return;
    }
    if end_inf > 25 {
        above_inf_25(start_inf, end_inf);
    } else {
        below_inf_25(start_inf, end_inf);
    }
}
fn above_inf_25(start_inf: i32, end_inf: i32) {
    let start_inf = start_inf; //cloned
    let end_inf = end_inf; //cloned
    let mut total = 0;

    //block
    let mut start_price = 20_000_000;
    {
        if start_inf > 25 {
            for _i in 25..start_inf { //gen start price
                start_price += 10_000_000; //price is 20m
            }
        }
    }
    //block 2
    {
        total += 10_000_000; //starts off at 30m :)
        for _i in 25..end_inf - 1{
            total += start_price;
            start_price += 10_000_000;
        }
        let mut diff = end_inf - start_inf;
        diff *= 20_000_000;
        total += diff;
    }
    print!("{total}");
}
fn below_inf_25(start_inf: i32, end_inf: i32) {
    let start_inf = start_inf;
    let end_inf = end_inf;
    let mut total = end_inf - start_inf;
    total *= 20_000_000;
    println!("{total}");
}