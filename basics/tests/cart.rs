// Integration test: uses the library from the outside, like a real user would.
use basics::shop::Cart;

#[test]
fn checkout_flow() {
    let mut cart = Cart::new();
    cart.add("headphones", 150_000);
    assert_eq!(cart.total(), 135_000);
}
