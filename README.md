# fast-factor

A fast integer factorisation library for unsigned integers in Rust.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
fast-factor = "0.1.3"
```

## Functions

### `factor(n)`
Returns all factors of `n`, including 1 and `n` itself. Returns an empty vector for 0.

```rust
assert_eq!(fast_factor::factor(12_u32), vec![1, 2, 3, 4, 6, 12]);
assert_eq!(fast_factor::factor(1_u32),  vec![1]);
assert_eq!(fast_factor::factor(0_u32),  vec![]);
```

### `proper_factor(n)`
Returns all factors of `n`, excluding `n` itself.

```rust
assert_eq!(fast_factor::proper_factor(12_u32), vec![1, 2, 3, 4, 6]);
assert_eq!(fast_factor::proper_factor(1_u32),  vec![]);
```

### `exclusive_factor(n)`
Returns all factors of `n`, excluding both 1 and `n` itself.

```rust
assert_eq!(fast_factor::exclusive_factor(12_u32), vec![2, 3, 4, 6]);
assert_eq!(fast_factor::exclusive_factor(7_u32),  vec![]);
```

### `is_prime(n)`
Returns `true` if `n` is prime. 0 and 1 are not considered prime.

```rust
assert!(fast_factor::is_prime(7_u32));
assert!(!fast_factor::is_prime(12_u32));
assert!(!fast_factor::is_prime(1_u32));
```

## Generics

All functions are generic over any type implementing `PrimInt + Unsigned + Roots`, so they work with `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

```rust
fast_factor::factor(100_u64);
fast_factor::factor(255_u8);
```

## Algorithm

All functions use trial division up to `sqrt(n)`, then derive complementary factors by division.

| Function             | Time complexity |
|----------------------|-----------------|
| `factor`             | O(√n)           |
| `proper_factor`      | O(√n)           |
| `exclusive_factor`   | O(√n)           |
| `is_prime`           | O(√n)           |

## License

MIT
