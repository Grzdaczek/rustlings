// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [Some(0); 5];
    for (i, n) in numbers.iter_mut().enumerate() {
        let number_to_add: u16 = {
            ((i as u16  * 1235) + 2) / (4 * 16)
        };

        *n = Some(number_to_add);
    }
}
