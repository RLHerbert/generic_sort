#[allow(dead_code)] // Don't worry about this.
type Number = f64;
// This doesn't actually work in Rust. Because of Rust's module system,
// there is no way (as far as I know) to implement Ord on f64. Instead we need a newtype:

// struct _number(f64);

// Which is more verbose and requires more boilerplate to set up. This is the price you
// pay for correctness in programming languages :)
