use package::front_of_house as package1;
fn main() {
    assert_eq!(package1::hosting::seat_at_table(), "sit down please");
    assert_eq!(package::eat_at_restaurant(),"yummy yummy!");
    println!("Success");
}