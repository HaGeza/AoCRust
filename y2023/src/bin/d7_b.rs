use y2023::util::d7::get_total_winnings;

fn main() {
    let card_values = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    match get_total_winnings("data/d7/a.txt", &card_values, Some('J')) {
        Ok(winnings) => println!("{}", winnings),
        Err(e) => println!("Error: {}", e),
    }
}
