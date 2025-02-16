# BigField

Modulo Arithmetic over large integers.

To see a working example, see `main.rs`.

```
use bigfield::{Modulus, PrimeField};
use bigint::{BigInt, Concat};

create_special_modulus1!(P, BigInt<4>, 255, 19);
type F = PrimeField<BigInt<4>, P>;

fn main() {
    let a = F::from(BigInt::ZERO);
    let b = F::from(BigInt::ONE);
    let c = a - b;
    println!("P: {}", hex::encode(P::M.to_be_bytes()));
    println!("Result of 0 - 1 (mod P) = {}", hex::encode(c.to_be_bytes()));

    let d = F::from(BigInt::from_u32(10));
    let e = F::from(BigInt::from_u64(100));
    let f = d - e;
    println!("10 - 100 (mod P) = {}", hex::encode(f.to_be_bytes()));
}
```
