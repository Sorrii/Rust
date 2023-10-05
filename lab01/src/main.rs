fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    } else if x != 2 && x % 2 == 0 {
        return false;
    } else {
        for i in 3..x - 1 {
            if x % i == 0 {
                return false;
            }
        }
    }

    return true;
}

fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let rest: u32 = b;
        b = a % b;
        a = rest;
    }

    return a;
}

fn sing_99_bottles() -> () {
    let mut bottles: u8 = 99;
    let mut flag: u8 = 0; //am folosit un flag pentru a repeta cantecul doar de doua pentru a putea citi
                          //si raspunsurile la ex 1 si ex 2

    loop {
        let plural = if bottles > 1 { "s" } else { "" };

        if bottles > 0 {
            println!("{} bottle{} of beer of beer on the wall,", bottles, plural);
            println!("{} bottle{} of beer.", bottles, plural);
            println!("Take one down, pass it around,");
            println!(
                "{} bottle{} of beer on the wall!\n",
                bottles - 1,
                if bottles - 1 > 1 || bottles - 1 == 0 {
                    "s"
                } else {
                    ""
                }
            );

            bottles -= 1;
        } else {
            println!("No more bottles of beer on the wall,");
            println!("No more bottles of beer.");
            println!("Go to the store and buy some more,");
            println!("99 bottles of beer on the wall!\n");

            bottles = 99;
            flag += 1;

            if flag == 2 {
                return;
            }
        }
    }
}

fn main() {
    sing_99_bottles();

    print!("\n1. The prime numbers between 0 and 100 are: ");

    for i in 0..=100 {
        if is_prime(i) == true {
            print!("{} ", i);
        }
    }

    println!("\n");

    println!("2. Coprime numbers are: ");

    // for i in 0..100 {
    //     for j in 1..=100 {
    //         if cmmdc(i, j) == 1 {
    //             println!("{} {} ", i, j);
    //         }
    //     }
    // }
    //am comentat partea asta^ de cod pentru readability

    for i in 0..100 {
        if cmmdc(i, i + 1) == 1 {
            println!("{} {}", i, i + 1);
        }
    }
}
