#![no_std]
#![no_main]

use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, high_level::*};
use ed25519_dalek::{Signature, VerifyingKey};
ckb_std::entry!(entry);
ckb_std::default_alloc!();

fn hash(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    let mut h = blake2b_ref::Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    h.update(data);
    h.finalize(&mut out);
    out
}
fn verify(key: &[u8], sig: &[u8]) -> Result<(), i8> {
    let key: [u8; 32] = key.try_into().map_err(|_| 10)?;
    let vk = VerifyingKey::from_bytes(&key).map_err(|_| 11)?;
    let signature = Signature::from_slice(sig).map_err(|_| 12)?;
    vk.verify_strict(&load_tx_hash().map_err(|_| 13)?, &signature)
        .map_err(|_| 14)
}
fn leaf(key: &[u8], amount: &[u8]) -> [u8; 32] {
    let mut b = [0; 41];
    b[1..33].copy_from_slice(key);
    b[33..].copy_from_slice(amount);
    hash(&b)
}
fn node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut b = [1; 65];
    b[1..33].copy_from_slice(left);
    b[33..].copy_from_slice(right);
    hash(&b)
}
fn root(mut h: [u8; 32], mut index: u32, proof: &[u8]) -> [u8; 32] {
    for s in proof.chunks_exact(32) {
        let s: [u8; 32] = s.try_into().unwrap();
        h = if index & 1 == 0 {
            node(&h, &s)
        } else {
            node(&s, &h)
        };
        index >>= 1;
    }
    h
}
fn run() -> Result<(), i8> {
    let args = load_script().map_err(|_| 2)?.args().raw_data();
    let w = load_witness(0, Source::GroupInput).map_err(|_| 3)?;
    #[cfg(feature = "policy")]
    {
        if args.len() != 32 || w.len() != 129 || w[0] > 1 {
            return Err(20);
        }
        if hash(&w[1..65])[..] != args[..] {
            return Err(21);
        }
        let start = 1 + w[0] as usize * 32;
        verify(&w[start..start + 32], &w[65..])?;
    }
    #[cfg(feature = "embedded")]
    {
        // RFC 8032 test-vector public key; private seed exists only in host test fixture.
        const KEY: [u8; 32] = [
            0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64,
            0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68,
            0xf7, 0x07, 0x51, 0x1a,
        ];
        if !args.is_empty() || w.len() != 64 {
            return Err(30);
        }
        verify(&KEY, &w)?;
    }
    #[cfg(feature = "pool")]
    {
        if !args.is_empty() || w.len() < 109 {
            return Err(40);
        }
        let index = u32::from_le_bytes(w[0..4].try_into().unwrap());
        let depth = w[44] as usize;
        if depth > 20 || (index as u64) >= (1u64 << depth) || w.len() != 109 + 32 * depth {
            return Err(41);
        }
        let amount = u64::from_le_bytes(w[4..12].try_into().unwrap());
        let proof = &w[45..45 + 32 * depth];
        let input_root = load_cell_data(0, Source::Input).map_err(|_| 42)?;
        if input_root.len() != 32
            || root(leaf(&w[12..44], &w[4..12]), index, proof)[..] != input_root[..]
        {
            return Err(43);
        }
        verify(&w[12..44], &w[45 + 32 * depth..])?;
        // Narrow prototype: exactly one input, no type scripts; all fees paid by exiting claim.
        if load_cell(1, Source::Input).is_ok()
            || load_cell_type(0, Source::Input).map_err(|_| 44)?.is_some()
        {
            return Err(44);
        }
        let cap = load_cell_capacity(0, Source::Input).map_err(|_| 45)?;
        let remaining = cap.checked_sub(amount).ok_or(46)?;
        let next_root = root(hash(&[2]), index, proof);
        let mut empty = hash(&[2]);
        for _ in 0..depth {
            empty = node(&empty, &empty)
        }
        let closing = next_root == empty;
        let payout_index = if closing { 0 } else { 1 };
        if !closing {
            if remaining < 73 * 100_000_000 {
                return Err(47);
            }
            if load_cell_capacity(0, Source::Output).map_err(|_| 48)? != remaining
                || load_cell_data(0, Source::Output).map_err(|_| 49)?[..] != next_root[..]
                || load_cell_lock_hash(0, Source::Input).map_err(|_| 50)?
                    != load_cell_lock_hash(0, Source::Output).map_err(|_| 51)?
            {
                return Err(52);
            }
            if load_cell_type(0, Source::Output).map_err(|_| 53)?.is_some() {
                return Err(53);
            }
        }
        if load_cell(payout_index + 1, Source::Output).is_ok()
            || load_cell_type(payout_index, Source::Output)
                .map_err(|_| 54)?
                .is_some()
        {
            return Err(54);
        }
        let payout = if closing { cap } else { amount };
        if payout < 1000
            || load_cell_capacity(payout_index, Source::Output).map_err(|_| 55)? != payout - 1000
            || !load_cell_data(payout_index, Source::Output)
                .map_err(|_| 56)?
                .is_empty()
        {
            return Err(55);
        }
    }
    Ok(())
}
fn entry() -> i8 {
    match run() {
        Ok(()) => 0,
        Err(e) => e,
    }
}
