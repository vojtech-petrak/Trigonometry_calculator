use std::{io::{stdout, stdin, Write}, f32::consts::PI};
use crossterm::{cursor::MoveUp, QueueableCommand};

enum Properties {
    Hypotenuse,
    Angle0,
    Pendant0,
    Pendant1,
    Area,
}

fn main() {
    const COUNT: usize = Properties::Area as usize + 1;
    let properties: [&str; COUNT] = [
        "Hypotenuse",
        "Angle 0",
        "Pendant 0",
        "Pendant 1",
        "Area",
    ];
    let mut values: [f32; COUNT] = [0.0; COUNT];
    let mut known_count: usize = 0;
    let mut known: [bool; COUNT] = [false; COUNT];

    // input
    println!("Trigonometry calculator by Vojtěch Petrák\n");
    println!("Please enter known values, else leave blank:");
    for i in 0..COUNT {
        while values[i] == 0.0 {
            print!("{} = ", properties[i]);
            stdout().flush().unwrap();

            let mut value: String = String::new();
            stdin().read_line(&mut value).expect("Failed to read line");

            let value = value.trim();
            if value == "" {
                stdout().queue(MoveUp(1)).unwrap();
                print!("            \r");
                stdout().flush().unwrap();
                break;
            }
            values[i] = match value.parse() {
                Ok(number) => { known_count += 1; known[i] = true; number },
                Err(_) => { println!("{value} is an invalid value"); 0.0 },
            }
        }
        if known_count == 2 { break; }
    }
    if known_count < 2 { println!("\nNot enough values!"); return; }

    // unwrap
    let [
        mut hypotenuse,
        mut angle0,
        mut pendant0,
        mut pendant1,
        mut area,
    ]: [f32; COUNT] = values;
    angle0 = angle0 / 180.0 * PI;

    // value swap
    if (known[Properties::Hypotenuse as usize] && known[Properties::Pendant0 as usize])
    || (known[Properties::Area as usize] && known[Properties::Pendant0 as usize])
    || (known[Properties::Angle0 as usize] && known[Properties::Pendant1 as usize]) {
        (pendant0, pendant1) = (pendant1, pendant0);
        angle0 = PI / 2.0 - angle0;
    }

    // calculation
    if known[Properties::Area as usize] {
        if known[Properties::Hypotenuse as usize] {
            angle0 = (4.0 * area / hypotenuse.powi(2)).asin() / 2.0;
            pendant1 = (2.0 * area / angle0.tan()).sqrt();
        } else if known[Properties::Angle0 as usize] {
            pendant1 = (2.0 * area / angle0.tan()).sqrt();
            hypotenuse = (2.0 * area / angle0.sin()).sqrt();
        } else {
            angle0 = (2.0 * area / pendant1.powi(2)).atan();
            hypotenuse = (2.0 * area / angle0.sin()).sqrt();
        }
        pendant0 = hypotenuse * angle0.sin();

    } else if known[Properties::Hypotenuse as usize] {
        if known[Properties::Angle0 as usize] {
            pendant1 = hypotenuse * angle0.cos();
        } else {
            angle0 = (pendant1 / hypotenuse).acos();
        }
        pendant0 = hypotenuse * angle0.sin();
        area = pendant1.powi(2) * angle0.tan() / 2.0;

    } else {
        if known[Properties::Angle0 as usize] {
            pendant1 = pendant0 / angle0.tan();
        } else {
            angle0 = (pendant0 / pendant1).atan();
        }
        hypotenuse = pendant0 / angle0.sin();
        area = pendant1.powi(2) * angle0.tan() / 2.0;
    }

    // value swap
    if (known[Properties::Hypotenuse as usize] && known[Properties::Pendant0 as usize])
    || (known[Properties::Angle0 as usize] && known[Properties::Pendant1 as usize])
    || (known[Properties::Area as usize] && known[Properties::Pendant0 as usize]) {
        (pendant0, pendant1) = (pendant1, pendant0);
        angle0 = PI / 2.0 - angle0;
    }

    // wrap
    angle0 = angle0 / PI * 180.0;
    values = [
        hypotenuse,
        angle0,
        pendant0,
        pendant1,
        area,
    ];

    // print
    println!("\nResults:");
    for (i, property) in properties.iter().enumerate() {
        println!("{} = {}", property, values[i]);
    }
}
