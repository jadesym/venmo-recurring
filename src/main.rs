mod venmo;

fn main() {
    let created_venmo_payment_charge_result =
        venmo::create_venmo_payment_charge(venmo::VenmoCharge {
            charge_amount_in_cents: 1000,
            target_unique_id: venmo::VenmoPaymentTargetUniqueId::UserId(String::from(
                "test_user_id",
            )),
            note_text: String::from("example note text"),
        });
    println!("{:#?}", created_venmo_payment_charge_result);
}
