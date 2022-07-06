mod venmo;

fn main() {
    println!("Hello, world!");
    let example_venmo_request = venmo::create_venmo_request();
    println!("{:#?}", example_venmo_request);
}
