use super::BoardVisualizer;

struct TerminalVisualizer {}
impl BoardVisualizer for TerminalVisualizer {
    fn draw_field(field: &crate::field::Field) {
        coordinate_header();
        field
            .field
            .iter()
            .enumerate()
            .for_each(|(i, [a, b, c, d])| {
                print_field_row(Some(i), to_char(a), to_char(b), to_char(c), to_char(&d));
                if i != 3 {
                    print_spacer();
                }
            });
    }

    fn players_turn(player_turn: bool) {
        println!("\n");
        match player_turn {
            false => println!("It's player one's turn..."),
            true => println!("It's player two's turn..."),
        }
        println!("\n");
    }

    fn display_result(result: crate::field::GameResult) {
        println!("{}", result);
    }
}

fn to_char(field_item: &Option<bool>) -> char {
    match field_item {
        Some(false) => 'X',
        Some(true) => 'O',
        None => ' ',
    }
}

fn coordinate_header() {
    println!("    0 1 2 3");
}

fn print_field_row(line_num: Option<usize>, a: char, b: char, c: char, d: char) {
    match line_num {
        None => println!("    {}|{}|{}|{}", a, b, c, d),
        Some(num) => println!("{}: {}|{}|{}|{}", num, a, b, c, d),
    }
}

fn print_spacer() {
    println!("   -+-+-+-");
}
