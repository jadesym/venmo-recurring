use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VenmoRequest {
    _amount_in_cents: u32,
    _friend_id: String,
    _note_text: String,
}

fn main() {
    println!("Hello, world!");
    let example_venmo_request = VenmoRequest {
        _amount_in_cents: 1000,
        _friend_id: String::from("example_friend_id"),
        _note_text: String::from("test text"),
    };
    println!("{:#?}", example_venmo_request);
}
