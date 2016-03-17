extern crate stockfighter;


use stockfighter::{test_api,test_venue, quote};

fn main() {
    test_api();
    test_venue("TESTEX");
    test_venue("NOTANEX");

    println!("Response: {}", quote("ZTPEX", "UELJ"));

    println!("Response: {}", buy_market("ZTPEX", "UELJ", 55, 100));

}
