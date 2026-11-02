#![no_std]
#![no_main]

use ckb_std::{ckb_constants::Source, ckb_types::prelude::Entity, high_level::*};
use ed25519_dalek::{Signature, VerifyingKey};
ckb_std::entry!(entry);
ckb_std::default_alloc!();
const CKB: u64 = 100_000_000;
// 8 capacity + 33 lock + (33+32) type + (1+32+8) data = 147 bytes.
const RESERVE: u64 = 147 * CKB;
const FEE: u64 = 10_000; // Research fixture fee, not a dynamic fee policy.
const MAX_DEPTH: usize = 8; // Full-manifest admission; deliberately not a million-user design.
const SUCCESS: [u8; 32] = [
    0x28, 0xe8, 0x3a, 0x12, 0x77, 0xd4, 0x8a, 0xdd, 0x8e, 0x72, 0xfa, 0xda, 0xa9, 0x24, 0x85, 0x59,
    0xe1, 0xb6, 0x32, 0xba, 0xb2, 0xbd, 0x60, 0xb2, 0x79, 0x55, 0xeb, 0xc4, 0xc0, 0x38, 0x00, 0xa5,
];
#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    hash: [u8; 32],
    sum: u64,
}
fn hash(b: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    let mut h = blake2b_ref::Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    h.update(b);
    h.finalize(&mut out);
    out
}
fn u64le(b: &[u8]) -> Result<u64, i8> {
    Ok(u64::from_le_bytes(b.try_into().map_err(|_| 10)?))
}
fn leaf(k: &[u8], a: u64) -> Result<Node, i8> {
    if a == 0 {
        if k.iter().any(|x| *x != 0) {
            return Err(11);
        }
        return Ok(empty());
    }
    if a <= FEE {
        return Err(55);
    } // Every nonempty admitted claim must fund its exit fee.
    let mut b = [0; 41];
    b[1..33].copy_from_slice(k);
    b[33..].copy_from_slice(&a.to_le_bytes());
    Ok(Node {
        hash: hash(&b),
        sum: a,
    })
}
fn empty() -> Node {
    Node {
        hash: hash(&[2]),
        sum: 0,
    }
}
fn parent(l: Node, r: Node) -> Result<Node, i8> {
    let sum = l.sum.checked_add(r.sum).ok_or(12)?;
    let mut b = [1; 81];
    b[1..33].copy_from_slice(&l.hash);
    b[33..41].copy_from_slice(&l.sum.to_le_bytes());
    b[41..73].copy_from_slice(&r.hash);
    b[73..].copy_from_slice(&r.sum.to_le_bytes());
    Ok(Node {
        hash: hash(&b),
        sum,
    })
}
fn parse(data: &[u8]) -> Result<(usize, Node), i8> {
    if data.len() != 41 || data[0] as usize > MAX_DEPTH {
        return Err(13);
    }
    Ok((
        data[0] as usize,
        Node {
            hash: data[1..33].try_into().unwrap(),
            sum: u64le(&data[33..])?,
        },
    ))
}
fn fold(mut n: Node, mut index: usize, proof: &[u8]) -> Result<Node, i8> {
    for s in proof.chunks_exact(40) {
        let other = Node {
            hash: s[..32].try_into().unwrap(),
            sum: u64le(&s[32..])?,
        };
        n = if index & 1 == 0 {
            parent(n, other)?
        } else {
            parent(other, n)?
        };
        index >>= 1;
    }
    Ok(n)
}
fn cell(index: usize, src: Source) -> Result<(), i8> {
    let lock = load_cell_lock(index, src).map_err(|_| 14)?;
    if lock.code_hash().raw_data()[..] != SUCCESS[..]
        || lock.hash_type().as_slice() != [4]
        || !lock.args().raw_data().is_empty()
    {
        return Err(15);
    }
    Ok(())
}
fn main_result() -> Result<(), i8> {
    let script = load_script().map_err(|_| 16)?;
    let args = script.args().raw_data();
    if args.len() != 32 {
        return Err(17);
    }
    let in_group = load_cell(0, Source::GroupInput).is_ok();
    let out_group = load_cell(0, Source::GroupOutput).is_ok();
    if load_cell(1, Source::GroupInput).is_ok() || load_cell(1, Source::GroupOutput).is_ok() {
        return Err(18);
    }
    let w = load_witness_args(0, Source::Input)
        .map_err(|_| 19)?
        .input_type()
        .to_opt()
        .ok_or(19)?
        .raw_data();
    if !in_group {
        // Creation ID binds type instance to first funding input and absolute output zero.
        if !out_group || load_cell(1, Source::Input).is_ok() || load_cell(1, Source::Output).is_ok()
        {
            return Err(20);
        }
        if load_cell_type_hash(0, Source::Output).map_err(|_| 21)?
            != Some(load_script_hash().map_err(|_| 21)?)
        {
            return Err(21);
        }
        let first = load_input(0, Source::Input).map_err(|_| 22)?;
        let mut seed = alloc::vec::Vec::from(first.as_slice());
        seed.extend_from_slice(&0u64.to_le_bytes());
        if hash(&seed)[..] != args[..] {
            return Err(23);
        }
        cell(0, Source::Output)?;
        let (depth, committed) = parse(&load_cell_data(0, Source::Output).map_err(|_| 24)?)?;
        let count = 1usize << depth;
        if w.len() != 1 + 40 * count || w[0] != 0 {
            return Err(25);
        }
        let mut nodes = alloc::vec::Vec::with_capacity(count);
        for c in w[1..].chunks_exact(40) {
            nodes.push(leaf(&c[..32], u64le(&c[32..])?)?)
        }
        while nodes.len() > 1 {
            let mut next = alloc::vec::Vec::with_capacity(nodes.len() / 2);
            for p in nodes.chunks_exact(2) {
                next.push(parent(p[0], p[1])?)
            }
            nodes = next
        }
        if nodes[0] != committed || committed.sum == 0 {
            return Err(26);
        }
        if load_cell_capacity(0, Source::Output).map_err(|_| 27)?
            != committed.sum.checked_add(RESERVE).ok_or(28)?
        {
            return Err(29);
        }
        return Ok(());
    }
    // Input pool is absolute input zero, so no attacker can redirect group-local indexes.
    if load_cell_type_hash(0, Source::Input).map_err(|_| 30)?
        != Some(load_script_hash().map_err(|_| 30)?)
    {
        return Err(30);
    }
    cell(0, Source::Input)?;
    let (depth, old) = parse(&load_cell_data(0, Source::Input).map_err(|_| 31)?)?;
    if w.len() != 109 + 40 * depth || (w[0] != 1 && w[0] != 2) {
        return Err(32);
    }
    let index = u32::from_le_bytes(w[1..5].try_into().unwrap()) as usize;
    if index >= 1usize << depth {
        return Err(33);
    }
    let amount = u64le(&w[5..13])?;
    if amount < FEE {
        return Err(34);
    }
    let proof = &w[45..45 + 40 * depth];
    let n = leaf(&w[13..45], amount)?;
    if fold(n, index, proof)? != old {
        return Err(35);
    }
    let vk = VerifyingKey::from_bytes(w[13..45].try_into().map_err(|_| 36)?).map_err(|_| 36)?;
    let sig = Signature::from_slice(&w[45 + 40 * depth..]).map_err(|_| 37)?;
    vk.verify_strict(&load_tx_hash().map_err(|_| 38)?, &sig)
        .map_err(|_| 39)?;
    let next = fold(empty(), index, proof)?;
    if next.sum != old.sum.checked_sub(amount).ok_or(40)? {
        return Err(40);
    }
    let cap = load_cell_capacity(0, Source::Input).map_err(|_| 41)?;
    if cap != old.sum.checked_add(RESERVE).ok_or(28)? {
        return Err(42);
    }
    let closing = next.sum == 0;
    let mut e = empty();
    for _ in 0..depth {
        e = parent(e, e)?
    }
    if closing && next != e {
        return Err(43);
    }
    let payout_index = if closing { 0 } else { 1 };
    if closing {
        if out_group {
            return Err(44);
        }
    } else {
        if !out_group
            || load_cell_type_hash(0, Source::Output).map_err(|_| 45)?
                != Some(load_script_hash().map_err(|_| 45)?)
        {
            return Err(45);
        }
        cell(0, Source::Output)?;
        if parse(&load_cell_data(0, Source::Output).map_err(|_| 46)?)? != (depth, next)
            || load_cell_capacity(0, Source::Output).map_err(|_| 47)?
                != next.sum.checked_add(RESERVE).ok_or(28)?
        {
            return Err(48);
        }
    }
    if load_cell(payout_index + 1, Source::Output).is_ok()
        || load_cell_type(payout_index, Source::Output)
            .map_err(|_| 49)?
            .is_some()
        || !load_cell_data(payout_index, Source::Output)
            .map_err(|_| 49)?
            .is_empty()
    {
        return Err(49);
    }
    let owed = amount
        .checked_add(if closing { RESERVE } else { 0 })
        .ok_or(28)?
        .checked_sub(FEE)
        .ok_or(34)?;
    let previous = if w[0] == 2 {
        if load_cell(2, Source::Input).is_ok()
            || load_cell_type(1, Source::Input).map_err(|_| 50)?.is_some()
            || !load_cell_data(1, Source::Input).map_err(|_| 50)?.is_empty()
        {
            return Err(50);
        }
        if load_cell_lock_hash(1, Source::Input).map_err(|_| 51)?
            != load_cell_lock_hash(payout_index, Source::Output).map_err(|_| 51)?
        {
            return Err(51);
        }
        load_cell_capacity(1, Source::Input).map_err(|_| 52)?
    } else {
        if load_cell(1, Source::Input).is_ok() {
            return Err(53);
        }
        0
    };
    if load_cell_capacity(payout_index, Source::Output).map_err(|_| 54)?
        != previous.checked_add(owed).ok_or(28)?
    {
        return Err(54);
    }
    Ok(())
}
fn entry() -> i8 {
    match main_result() {
        Ok(()) => 0,
        Err(e) => e,
    }
}
