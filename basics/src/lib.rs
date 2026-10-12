//! Runnable examples live in `examples/` — try: `cargo run --example 03_variables`.
//!
//! This file shows chapter 11 (modules and tests): `cargo test`.

/// A module groups related code. `pub` makes things visible outside it.
pub mod shop {
    pub struct Cart {
        items: Vec<(String, u32)>, // private: only code in this module can touch it
    }

    impl Cart {
        pub fn new() -> Self {
            Cart { items: Vec::new() }
        }

        pub fn add(&mut self, name: &str, price_paise: u32) {
            self.items.push((name.to_string(), price_paise));
        }

        /// Total price, with 10% off when over ₹1000 (100000 paise).
        pub fn total(&self) -> u32 {
            let sum: u32 = self.items.iter().map(|(_, price)| price).sum();
            if sum > 100_000 { sum * 90 / 100 } else { sum }
        }
    }

    impl Default for Cart {
        fn default() -> Self {
            Self::new()
        }
    }

    // Unit tests live next to the code, in a module only compiled for `cargo test`.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn empty_cart_costs_nothing() {
            assert_eq!(Cart::new().total(), 0);
        }

        #[test]
        fn small_orders_have_no_discount() {
            let mut cart = Cart::new();
            cart.add("pen", 5_000);
            cart.add("book", 30_000);
            assert_eq!(cart.total(), 35_000);
        }

        #[test]
        fn big_orders_get_ten_percent_off() {
            let mut cart = Cart::new();
            cart.add("phone", 200_000);
            assert_eq!(cart.total(), 180_000);
        }
    }
}
