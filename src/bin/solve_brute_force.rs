use four_win_three_lose::field::Field;

fn main() {
    let mut field = Field::new();
    let res =
        field.brute_force_game_state(false, false, &Field::possible_moves_symmetrical_if_sparse);
    println!("{:?}", res);
}
