#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(bigint_helper_methods)]
#![feature(const_bigint_helper_methods)]
#![allow(incomplete_features)]

pub mod bigfield;
pub mod bigint;

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
}
