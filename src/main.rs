#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(bigint_helper_methods)]
#![feature(const_bigint_helper_methods)]
#![allow(incomplete_features)]

pub mod bigfield;
pub mod bigint;

use crypto_bigint::modular::ConstMontyForm;
use crypto_bigint::{impl_modulus, U256};
use hex_literal::hex;
use rand::Rng;

create_special_modulus1!(P0, BigInt<4>, 255, 19);
impl_modulus!(
    P1,
    U256,
    "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"
);

type F0 = PrimeField<BigInt<4>, P0>;
type F1 = ConstMontyForm<P1, { U256::LIMBS }>;

use bigfield::{Modulus, PrimeField};
use bigint::{BigInt, Concat};

fn main() {
    // let a = BigInt::<4>::from_le_bytes(hex!("000000000000000000000000000000000000000000000003ffffffffffffffff"));
    let a = BigInt::<4>::from_u64(1);
    let b = Concat::new(&BigInt::<4>::ZERO, &a);
    for i in 0..=255 {
        let c = b.shr(i);
        println!("{i}: {:?}", c.to_words());
    }
}
