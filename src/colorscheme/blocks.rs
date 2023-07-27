use nu_ansi_term::{Color, Style};
use std::ops::RangeInclusive;

const BLOCK: &str = "    ";

pub fn print() {
    println!();
    print_blocks(0..=7);
    print_blocks(8..=15);
    println!();
}

fn print_blocks(range: RangeInclusive<u8>) {
    for i in range {
        print_block(i);
    }
    println!();
}

fn print_block(i: u8) {
    let block = Style::new().on(Color::Fixed(i));
    print!("{}", block.paint(BLOCK));
}
