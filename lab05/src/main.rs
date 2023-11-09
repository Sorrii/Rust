use std::fs;
mod bonus;
mod canvas;
use anyhow::Result;
use bonus::Cell;
use bonus::Gameboard;
use canvas::Canvas;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}

fn p1() -> () {
    let content = fs::read_to_string("input_for_p1").expect("Could not open input file!\n");

    let mut name1 = String::new();
    let mut name2 = String::new();
    let mut phone1 = String::new();
    let mut phone2 = String::new();
    let mut max_age = 0;
    let mut min_age = 255;

    for line in content.lines() {
        let data: Vec<&str> = line.split(',').collect();
        if let Ok(age) = data[2].trim().parse::<u8>() {
            let student = Student {
                name: String::from(data[0].trim()),
                phone: String::from(data[1].trim()),
                age,
            };

            if student.age > max_age {
                max_age = student.age;
                name1 = String::from(&student.name);
                phone1 = String::from(&student.phone);
            }

            if student.age < min_age {
                min_age = student.age;
                name2 = String::from(&student.name);
                phone2 = String::from(&student.phone);
            }
        } else {
            println!(
                "Error occured while trying to parse the age for line {}",
                line
            );
        }
    }

    let oldest_student = Student {
        name: name1,
        phone: phone1,
        age: max_age,
    };

    let youngest_student = Student {
        name: name2,
        phone: phone2,
        age: min_age,
    };

    println!("Oldest student is {:?}", oldest_student);
    println!("Youngest student is {:?}", youngest_student);
}

fn p2() -> () {
    let mut canvas = Canvas::new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(
        c,
        &[
            (2, 16, 95),
            (5, 35, 92),
            (6, 3, 95),
            (2, 11, 95),
            (5, 3, 95),
        ],
    );
    set_pixels(
        c,
        &[
            (2, 38, 95),
            (4, 9, 40),
            (3, 41, 124),
            (2, 37, 95),
            (2, 25, 124),
        ],
    );
    set_pixels(
        c,
        &[
            (5, 27, 124),
            (2, 27, 124),
            (4, 0, 124),
            (3, 35, 47),
            (2, 18, 95),
        ],
    );
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(
        c,
        &[
            (6, 30, 95),
            (5, 45, 95),
            (4, 31, 124),
            (4, 7, 124),
            (3, 43, 39),
        ],
    );
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(
        c,
        &[
            (3, 44, 95),
            (3, 19, 92),
            (5, 23, 95),
            (3, 8, 47),
            (2, 10, 95),
        ],
    );
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 10, 95),
            (4, 44, 95),
            (2, 9, 95),
            (0, 32, 95),
            (5, 2, 95),
        ],
    );
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(
        c,
        &[
            (3, 46, 92),
            (5, 25, 44),
            (1, 43, 124),
            (5, 46, 47),
            (3, 15, 47),
        ],
    );
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(
        c,
        &[
            (5, 44, 95),
            (2, 2, 95),
            (5, 10, 95),
            (5, 9, 95),
            (4, 43, 124),
        ],
    );
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(
        c,
        &[
            (4, 40, 124),
            (3, 3, 95),
            (4, 4, 124),
            (6, 31, 47),
            (3, 4, 96),
        ],
    );
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 12, 92),
            (2, 32, 95),
            (5, 37, 95),
            (5, 26, 95),
            (5, 39, 47),
        ],
    );
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(
        c,
        &[
            (5, 36, 95),
            (7, 30, 95),
            (6, 4, 47),
            (4, 24, 95),
            (1, 32, 95),
        ],
    );
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);
}

fn p3() -> Result<(), anyhow::Error> {
    let content = fs::read_to_string("src/students.json")?;
    let students: Vec<Student> = serde_json::from_str(&content)?;

    let mut name1 = String::new();
    let mut name2 = String::new();
    let mut phone1 = String::new();
    let mut phone2 = String::new();
    let mut max_age = 0;
    let mut min_age = 255;

    for student in &students {
        if student.age > max_age {
            max_age = student.age;
            name1 = String::from(&student.name);
            phone1 = String::from(&student.phone);
        }

        if student.age < min_age {
            min_age = student.age;
            name2 = String::from(&student.name);
            phone2 = String::from(&student.phone);
        }
    }

    let oldest_student = Student {
        name: name1,
        phone: phone1,
        age: max_age,
    };

    let youngest_student = Student {
        name: name2,
        phone: phone2,
        age: min_age,
    };

    println!("Oldest student is {:?}", oldest_student);
    println!("Youngest student is {:?}", youngest_student);

    Ok(())
}

fn parse_content(content: String) -> Result<[[Cell; 11]; 11], anyhow::Error> {
    let mut matrix: Vec<Vec<Cell>> = Vec::new();

    for line in content.lines() {
        let row: Vec<Cell> = line
            .chars()
            .map(|c| if c == '1' { Cell::Alive } else { Cell::Dead })
            .collect();

        matrix.push(row);
    }
    let mut board: [[Cell; 11]; 11] = [[Cell::Dead; 11]; 11];
    let mut r = 0;
    let mut c = 0;

    for row in &matrix {
        for &cell in row {
            if r < 11 && c < 11 {
                board[r][c] = cell;
            r += 1;
            }
        }
        c += 1;
    }

    Ok(board)
}

fn play_bonus_game() -> () {
    let content = fs::read_to_string("src/bonus").expect("Could not open input file!\n");
    match parse_content(content) {
        Ok(matrix) => {
            let mut game = Gameboard::new_board(matrix);

            for generation in 1..=3 {
                game.create_next_gen();
                println!("Generation {}: ", generation);
                game.print();
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

fn main() {
    println!("\nExercise 1: ");
    p1();

    println!("\nExercise 2: ");
    p2();
    say_ty();

    println!("\nExercise 3: ");
    let _ = p3();

    println!("\nBonus: ");
    play_bonus_game();
}
fn set_pixels(canvas: &mut Canvas, pixels: &[(usize, usize, u8)]) {
    canvas.set_pixels(pixels);
}

fn print(canvas: Canvas) {
    canvas.print_canvas();
}

fn say_ty() -> () {
    //am vrut inital sa fac tot cu set_pixel.. dar aveam si test la lfac urmatoarea zi :c
    let thank_you = r#"
 _   _                 _                        
| | | |               | |                       
| |_| |__   __ _ _ __ | | __  _   _  ___  _   _ 
| __| '_ \ / _` | '_ \| |/ / | | | |/ _ \| | | |
| |_| | | | (_| | | | |   <  | |_| | (_) | |_| |
 \__|_| |_|\__,_|_| |_|_|\_\  \__, |\___/ \__,_|
                               __/ |            
                              |___/                      
"#;

    let matrix: Vec<Vec<char>> = thank_you
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in &matrix {
        for &c in row {
            print!("{}", c);
        }
        print!("\n");
    }
}
