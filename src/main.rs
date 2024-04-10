use core::fmt;
use std::io;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive)]
enum Gem {
    Diamond,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "diamond"),
            Gem::Sapphire => write!(f, "sapphire"),
            Gem::Ruby => write!(f, "ruby"),
            Gem::Topaz => write!(f, "topaz"),
            Gem::Onyx => write!(f, "onyx"),
            Gem::Jade => write!(f, "jade"),
        }
    }
}

// impl Gem {
//     fn from_number(num: u8) -> Option<Gem> {
//         match num {
//             1 => Some(Gem::Diamond),
//             2 => Some(Gem::Sapphire),
//             3 => Some(Gem::Ruby),
//             4 => Some(Gem::Topaz),
//             5 => Some(Gem::Onyx),
//             6 => Some(Gem::Jade),
//             _ => None,
//         }
//     }
// }

fn main() {
    // let gems = [
    //     (Gem::Onyx, 25.00),
    //     (Gem::Diamond, 100.00),
    //     (Gem::Onyx, 50.00),
    //     (Gem::Ruby, 10.00),
    // ];

    // for gem in gems {
    //     println!("This {:?} is worth ${}.", gem.0, gem.1)
    // }

    let mut map = [[0; 5]; 5];
    println!("{map:?}");

    map[4][2] = 1;
    map[1][2] = 2;
    map[3][3] = 3;
    map[0][2] = 4;
    map[1][4] = 5;

    for row in map {
        println!("{row:?}")
    }

    let found: Vec<Gem> = Vec::new();

    println!("Search an X Y position (example input: 5 3)");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read line.")
        }
    };

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        println!("Two numbers required.");
        return;
    }

    let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            println!("Something is wrong with the input.");
            return;
        }
    };

    let data = map[x as usize][y as usize];

    let gem = match Gem::from_u8(data) {
        Some(gem) => gem,
        None => {
            println!("No gem found at this position.");
            return;
        }
    };

    // found.push(Gem::from_u8(data).expect("No gem found."));

    println!("{found:?}");
}
