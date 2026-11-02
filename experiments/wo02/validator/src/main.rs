use ckb_chain_spec::consensus::{Consensus, ConsensusBuilder};
use ckb_hash::blake2b_256 as hash;
use ckb_script::{TransactionScriptsVerifier, TxVerifyEnv};
use ckb_traits::{CellDataProvider, ExtensionProvider, HeaderProvider};
use ckb_types::{
    bytes::Bytes,
    core::{
        Capacity, EpochNumberWithFraction, HeaderView, ScriptHashType, TransactionView,
        cell::{
            CellMeta, CellMetaBuilder, CellProvider, CellStatus, HeaderChecker,
            ResolvedTransaction, resolve_transaction,
        },
        error::OutPointError,
    },
    packed::{self, Byte32, CellDep, CellInput, CellOutput, OutPoint, Script},
    prelude::*,
};
use ckb_verification::{CapacityVerifier, NonContextualTransactionVerifier};
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
const CKB: u64 = 100_000_000;
const MAX_CYCLES: u64 = 100_000_000;
const RESERVE: u64 = 147 * CKB;
const FEE: u64 = 10_000;
#[derive(Clone, Default)]
struct Store {
    cells: HashMap<OutPoint, CellMeta>,
}
impl CellDataProvider for Store {
    fn get_cell_data(&self, p: &OutPoint) -> Option<Bytes> {
        self.cells.get(p)?.mem_cell_data.clone()
    }
    fn get_cell_data_hash(&self, p: &OutPoint) -> Option<Byte32> {
        self.cells.get(p)?.mem_cell_data_hash.clone()
    }
}
impl HeaderProvider for Store {
    fn get_header(&self, _: &Byte32) -> Option<HeaderView> {
        None
    }
}
impl ExtensionProvider for Store {
    fn get_block_extension(&self, _: &Byte32) -> Option<packed::Bytes> {
        None
    }
}
impl CellProvider for Store {
    fn cell(&self, p: &OutPoint, _: bool) -> CellStatus {
        self.cells
            .get(p)
            .cloned()
            .map(CellStatus::Live)
            .unwrap_or(CellStatus::Unknown)
    }
}
impl HeaderChecker for Store {
    fn check_valid(&self, h: &Byte32) -> Result<(), OutPointError> {
        Err(OutPointError::InvalidHeader(h.clone()))
    }
}
struct Lab {
    store: Store,
    consensus: Arc<Consensus>,
    deps: Vec<CellMeta>,
    support: Vec<CellMeta>,
    counter: u32,
    rows: Vec<Value>,
}
impl Lab {
    fn new() -> Self {
        Self {
            store: Store::default(),
            consensus: Arc::new(ConsensusBuilder::default().build()),
            deps: vec![],
            support: vec![],
            counter: 1,
            rows: vec![],
        }
    }
    fn cell(&mut self, o: CellOutput, d: Vec<u8>) -> CellMeta {
        self.counter += 1;
        let op = OutPoint::new_builder()
            .tx_hash(hash(&self.counter.to_le_bytes()).pack())
            .index(0u32)
            .build();
        let c = CellMetaBuilder::from_cell_output(o, Bytes::from(d))
            .out_point(op.clone())
            .build();
        self.store.cells.insert(op, c.clone());
        c
    }
    fn code(&mut self, data: Vec<u8>, args: Vec<u8>) -> Script {
        let script = Script::new_builder()
            .code_hash(hash(&data).pack())
            .hash_type(ScriptHashType::Data2)
            .args(Bytes::from(args))
            .build();
        let output = CellOutput::new_builder()
            .lock(Script::default())
            .capacity(Capacity::bytes(data.len() + 41).unwrap())
            .build();
        let c = self.cell(output, data);
        self.deps.push(c);
        script
    }
    fn tx(
        &self,
        inputs: &[CellMeta],
        outputs: Vec<CellOutput>,
        data: Vec<Vec<u8>>,
        payload: Vec<u8>,
    ) -> TransactionView {
        let mut hashes = HashSet::new();
        for c in inputs {
            hashes.insert(c.cell_output.lock().code_hash());
            if let Some(t) = c.cell_output.type_().to_opt() {
                hashes.insert(t.code_hash());
            }
        }
        for o in &outputs {
            if let Some(t) = o.type_().to_opt() {
                hashes.insert(t.code_hash());
            }
        }
        let deps = self
            .deps
            .iter()
            .filter(|c| hashes.contains(c.mem_cell_data_hash.as_ref().unwrap()))
            .chain(self.support.iter())
            .map(|c| {
                CellDep::new_builder()
                    .out_point(c.out_point.clone())
                    .build()
            })
            .collect::<Vec<_>>();
        let w = packed::WitnessArgs::new_builder()
            .input_type(Some(Bytes::from(payload)))
            .build();
        let mut witnesses = vec![w.as_bytes().pack()];
        for _ in 1..inputs.len() {
            witnesses.push(Bytes::new().pack());
        }
        TransactionView::new_advanced_builder()
            .inputs(
                inputs
                    .iter()
                    .map(|c| {
                        CellInput::new_builder()
                            .previous_output(c.out_point.clone())
                            .build()
                    })
                    .collect::<Vec<_>>(),
            )
            .outputs(outputs)
            .outputs_data(
                data.into_iter()
                    .map(|d| Bytes::from(d).pack())
                    .collect::<Vec<_>>(),
            )
            .cell_deps(deps)
            .set_witnesses(witnesses)
            .build()
    }
    fn resolved(&self, tx: &TransactionView, _input: &CellMeta) -> ResolvedTransaction {
        resolve_transaction(
            tx.clone(),
            &mut HashSet::<OutPoint>::new(),
            &self.store,
            &self.store,
        )
        .expect("fixture resolution")
    }
    fn check(
        &mut self,
        name: &str,
        tx: TransactionView,
        input: &CellMeta,
        expected: bool,
        stage: Option<&str>,
    ) -> Option<u64> {
        let rtx = Arc::new(self.resolved(&tx, input));
        let header = HeaderView::new_advanced_builder()
            .epoch(EpochNumberWithFraction::new(100_000, 0, 1))
            .build();
        let env = Arc::new(TxVerifyEnv::new_commit(&header));
        let result = NonContextualTransactionVerifier::new(&tx, &self.consensus)
            .verify()
            .map_err(|e| ("noncontextual", e.to_string()))
            .and_then(|_| {
                CapacityVerifier::new(rtx.clone(), self.consensus.dao_type_hash())
                    .verify()
                    .map_err(|e| ("capacity", e.to_string()))
            })
            .and_then(|_| {
                TransactionScriptsVerifier::new(
                    rtx,
                    self.store.clone(),
                    self.consensus.clone(),
                    env,
                )
                .verify(MAX_CYCLES)
                .map_err(|e| ("scripts", e.to_string()))
            });
        let (ok, cycles, failed_stage, error) = match result {
            Ok(c) => (true, Some(c), None, None),
            Err((s, e)) => (false, None, Some(s), Some(e)),
        };
        let passed = ok == expected && (expected || stage.is_none() || stage == failed_stage);
        println!(
            "{} {} accepted={} cycles={:?} stage={:?} {}",
            if passed { "PASS" } else { "FAIL" },
            name,
            ok,
            cycles,
            failed_stage,
            error.as_deref().unwrap_or("")
        );
        let input_cap: u64 = tx
            .inputs()
            .into_iter()
            .map(|i| {
                let c: u64 = self
                    .store
                    .cells
                    .get(&i.previous_output())
                    .unwrap()
                    .cell_output
                    .capacity()
                    .into();
                c
            })
            .sum();
        let output_cap: u64 = tx
            .outputs()
            .into_iter()
            .map(|o| {
                let c: u64 = o.capacity().into();
                c
            })
            .sum();
        self.rows.push(json!({"name":name,"test_passed":passed,"expected_accept":expected,"accepted":ok,"cycles":cycles,"failure_stage":failed_stage,"error":error,"serialized_transaction_bytes":tx.data().as_slice().len(),"transaction_size_for_fee_note":"raw packed transaction bytes; fixture fee is consensus-valid, not a mempool admission test","fee_shannons":input_cap.checked_sub(output_cap)}));
        assert!(passed, "unexpected result: {name}");
        cycles
    }
    fn resolver_case(
        &mut self,
        name: &str,
        tx: TransactionView,
        seen: &mut HashSet<OutPoint>,
        expected: bool,
    ) {
        let r = resolve_transaction(tx, seen, &self.store, &self.store);
        let ok = r.is_ok();
        println!(
            "{} {name} resolver_accept={ok} {:?}",
            if ok == expected { "PASS" } else { "FAIL" },
            r.as_ref().err()
        );
        self.rows.push(json!({"name":name,"test_passed":ok==expected,"expected_accept":expected,"accepted":ok,"failure_stage":if ok{None}else{Some("resolver")},"error":r.err().map(|e|e.to_string())}));
        assert_eq!(ok, expected);
    }
}
fn output(lock: &Script, capacity: u64) -> CellOutput {
    CellOutput::new_builder()
        .lock(lock.clone())
        .capacity(capacity)
        .build()
}
fn key(n: u8) -> SigningKey {
    SigningKey::from_bytes(&[n; 32])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    hash: [u8; 32],
    sum: u64,
}
fn empty() -> Node {
    Node {
        hash: hash([2]),
        sum: 0,
    }
}
fn leaf(k: &SigningKey, a: u64) -> Node {
    if a == 0 {
        return empty();
    }
    let mut b = vec![0];
    b.extend(k.verifying_key().as_bytes());
    b.extend(a.to_le_bytes());
    Node {
        hash: hash(b),
        sum: a,
    }
}
fn parent(l: Node, r: Node) -> Node {
    let mut b = vec![1];
    b.extend(l.hash);
    b.extend(l.sum.to_le_bytes());
    b.extend(r.hash);
    b.extend(r.sum.to_le_bytes());
    Node {
        hash: hash(b),
        sum: l.sum.checked_add(r.sum).unwrap(),
    }
}
fn tree(leaves: &[Node]) -> Node {
    if leaves.len() == 1 {
        return leaves[0];
    }
    let m = leaves.len() / 2;
    parent(tree(&leaves[..m]), tree(&leaves[m..]))
}
fn proof(leaves: &[Node], mut i: usize) -> Vec<Node> {
    let mut layer = leaves.to_vec();
    let mut p = vec![];
    while layer.len() > 1 {
        p.push(layer[i ^ 1]);
        layer = layer.chunks_exact(2).map(|c| parent(c[0], c[1])).collect();
        i >>= 1;
    }
    p
}
fn fold(mut n: Node, mut i: usize, p: &[Node]) -> Node {
    for s in p {
        n = if i & 1 == 0 {
            parent(n, *s)
        } else {
            parent(*s, n)
        };
        i >>= 1;
    }
    n
}
fn encode(n: Node, depth: usize) -> Vec<u8> {
    let mut b = vec![depth as u8];
    b.extend(n.hash);
    b.extend(n.sum.to_le_bytes());
    b
}
fn manifest(keys: &[SigningKey], amounts: &[u64]) -> Vec<u8> {
    let mut b = vec![0];
    for (k, a) in keys.iter().zip(amounts) {
        if *a == 0 {
            b.extend([0; 32]);
        } else {
            b.extend(k.verifying_key().as_bytes());
        }
        b.extend(a.to_le_bytes());
    }
    b
}
fn payload(tx: &TransactionView) -> Vec<u8> {
    packed::WitnessArgs::from_slice(&tx.witnesses().get(0).unwrap().raw_data())
        .unwrap()
        .input_type()
        .to_opt()
        .unwrap()
        .raw_data()
        .to_vec()
}
fn set_payload(tx: TransactionView, b: Vec<u8>) -> TransactionView {
    let w = packed::WitnessArgs::from_slice(&tx.witnesses().get(0).unwrap().raw_data())
        .unwrap()
        .as_builder()
        .input_type(Some(Bytes::from(b)))
        .build();
    let mut ws = tx.witnesses().into_iter().collect::<Vec<_>>();
    ws[0] = w.as_bytes().pack();
    tx.as_advanced_builder().set_witnesses(ws).build()
}
fn sign_claim(tx: TransactionView, k: &SigningKey) -> TransactionView {
    let mut b = payload(&tx);
    let len = b.len();
    b[len - 64..].copy_from_slice(&k.sign(tx.hash().as_slice()).to_bytes());
    set_payload(tx, b)
}
fn sign_secp(tx: TransactionView, key: &ckb_crypto::secp::Privkey) -> TransactionView {
    let w = packed::WitnessArgs::from_slice(&tx.witnesses().get(0).unwrap().raw_data()).unwrap();
    let zero = w
        .clone()
        .as_builder()
        .lock(Some(Bytes::from(vec![0; 65])))
        .build();
    let mut h = ckb_hash::new_blake2b();
    h.update(tx.hash().as_slice());
    h.update(&(zero.as_slice().len() as u64).to_le_bytes());
    h.update(zero.as_slice());
    let mut msg = [0; 32];
    h.finalize(&mut msg);
    let sig = key.sign_recoverable(&msg.into()).unwrap();
    let w = w
        .as_builder()
        .lock(Some(Bytes::from(sig.serialize())))
        .build();
    let mut ws = tx.witnesses().into_iter().collect::<Vec<_>>();
    ws[0] = w.as_bytes().pack();
    tx.as_advanced_builder().set_witnesses(ws).build()
}
fn committed(lab: &mut Lab, tx: &TransactionView, i: usize) -> CellMeta {
    let c = CellMetaBuilder::from_cell_output(
        tx.outputs().get(i).unwrap(),
        tx.outputs_data().get(i).unwrap().raw_data(),
    )
    .out_point(OutPoint::new(tx.hash(), i as u32))
    .build();
    lab.store.cells.insert(c.out_point.clone(), c.clone());
    c
}
fn instance(template: &Script, funder: &CellMeta) -> Script {
    let input = CellInput::new_builder()
        .previous_output(funder.out_point.clone())
        .build();
    let mut seed = input.as_slice().to_vec();
    seed.extend(0u64.to_le_bytes());
    template
        .clone()
        .as_builder()
        .args(Bytes::from(hash(seed).to_vec()))
        .build()
}
fn admission(
    lab: &Lab,
    funder: &CellMeta,
    typ: &Script,
    lock: &Script,
    n: Node,
    depth: usize,
    w: Vec<u8>,
    capacity: u64,
    secp: &ckb_crypto::secp::Privkey,
) -> TransactionView {
    sign_secp(
        lab.tx(
            &[funder.clone()],
            vec![
                output(lock, capacity)
                    .as_builder()
                    .type_(Some(typ.clone()))
                    .build(),
            ],
            vec![encode(n, depth)],
            w,
        ),
        secp,
    )
}
fn withdraw(
    lab: &Lab,
    pool: &CellMeta,
    target: Option<&CellMeta>,
    fresh: &Script,
    index: usize,
    amount: u64,
    key: &SigningKey,
    p: &[Node],
) -> TransactionView {
    // This local builder has no global tree/manifest or operator state.
    let old = fold(leaf(key, amount), index, p);
    let raw = pool.mem_cell_data.as_ref().unwrap();
    assert_eq!(encode(old, p.len()).as_slice(), raw.as_ref());
    let next = fold(empty(), index, p);
    let closing = next.sum == 0;
    let owed = amount + if closing { RESERVE } else { 0 };
    let mut outs = vec![];
    let mut data = vec![];
    if !closing {
        outs.push(
            pool.cell_output
                .clone()
                .as_builder()
                .capacity(next.sum + RESERVE)
                .build(),
        );
        data.push(encode(next, p.len()));
    }
    let old_target = target
        .map(|c| {
            let n: u64 = c.cell_output.capacity().into();
            n
        })
        .unwrap_or(0);
    outs.push(output(
        &target
            .map(|c| c.cell_output.lock())
            .unwrap_or(fresh.clone()),
        old_target + owed - FEE,
    ));
    data.push(vec![]);
    let mut inputs = vec![pool.clone()];
    if let Some(t) = target {
        inputs.push(t.clone());
    }
    let mut b = vec![if target.is_some() { 2 } else { 1 }];
    b.extend((index as u32).to_le_bytes());
    b.extend(amount.to_le_bytes());
    b.extend(key.verifying_key().as_bytes());
    for s in p {
        b.extend(s.hash);
        b.extend(s.sum.to_le_bytes());
    }
    b.extend([0; 64]);
    sign_claim(lab.tx(&inputs, outs, data, b), key)
}
fn refresh(own: usize, old: &[Node], public: &[u8]) -> Vec<Node> {
    let spent = u32::from_le_bytes(public[1..5].try_into().unwrap()) as usize;
    assert_ne!(own, spent);
    let level = (usize::BITS - 1 - (own ^ spent).leading_zeros()) as usize;
    let mut subtree = empty();
    for i in 0..level {
        let off = 45 + 40 * i;
        let other = Node {
            hash: public[off..off + 32].try_into().unwrap(),
            sum: u64::from_le_bytes(public[off + 32..off + 40].try_into().unwrap()),
        };
        subtree = if (spent >> i) & 1 == 0 {
            parent(subtree, other)
        } else {
            parent(other, subtree)
        }
    }
    let mut result = old.to_vec();
    result[level] = subtree;
    result
}
fn permutations(prefix: Vec<usize>, remaining: Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if remaining.is_empty() {
        out.push(prefix);
        return;
    }
    for i in 0..remaining.len() {
        let mut p = prefix.clone();
        p.push(remaining[i]);
        let mut r = remaining.clone();
        r.remove(i);
        permutations(p, r, out);
    }
}
fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut lab = Lab::new();
    let success_bytes =
        std::fs::read(root.join("../wo01-sources/ckb/script/testdata/always_success")).unwrap();
    let lock = lab.code(success_bytes.clone(), vec![]);
    let code = std::fs::read(root.join("experiments/wo02/bin/sum_type")).unwrap();
    let template = lab.code(code.clone(), vec![0; 32]);
    let secp = ckb_crypto::secp::Privkey::from_slice(&[7; 32]);
    let secpdata = ckb_system_scripts::BUNDLED_CELL
        .get("specs/cells/secp256k1_data")
        .unwrap()
        .into_owned();
    let c = lab.cell(
        output(&lock, (secpdata.len() as u64 + 41) * CKB),
        secpdata.clone(),
    );
    lab.support.push(c);
    let standard_bytes = ckb_system_scripts::BUNDLED_CELL
        .get("specs/cells/secp256k1_blake160_sighash_all")
        .unwrap()
        .into_owned();
    let standard = lab
        .code(
            standard_bytes.clone(),
            hash(secp.pubkey().unwrap().serialize())[..20].to_vec(),
        )
        .as_builder()
        .hash_type(ScriptHashType::Data)
        .build();
    let acp_bytes = std::fs::read(root.join("experiments/wo02/bin/anyone_can_pay")).unwrap();
    assert_eq!(
        hex::encode(hash(&acp_bytes)),
        "cd69ba816f7471e59110058aa37387c362ed9a240cd178f7bb1ecee386cb31e6"
    );
    let acp_template = lab
        .code(acp_bytes.clone(), vec![0; 20])
        .as_builder()
        .hash_type(ScriptHashType::Data)
        .build();
    let recipient_keys: Vec<_> = (41..45)
        .map(|n| ckb_crypto::secp::Privkey::from_slice(&[n; 32]))
        .collect();
    let recipients: Vec<_> = recipient_keys
        .iter()
        .map(|k| {
            let s = acp_template
                .clone()
                .as_builder()
                .args(Bytes::from(
                    hash(k.pubkey().unwrap().serialize())[..20].to_vec(),
                ))
                .build();
            lab.cell(output(&s, 61 * CKB), vec![])
        })
        .collect();
    let keys: Vec<_> = (1..5).map(key).collect();
    let amounts = [CKB, 2 * CKB, 3 * CKB, 4 * CKB];
    let leaves: Vec<_> = keys.iter().zip(amounts).map(|(k, a)| leaf(k, a)).collect();
    let initial = tree(&leaves);
    let funder = lab.cell(output(&standard, initial.sum + RESERVE + FEE), vec![]);
    let typ = instance(&template, &funder);
    let create = admission(
        &lab,
        &funder,
        &typ,
        &lock,
        initial,
        2,
        manifest(&keys, &amounts),
        initial.sum + RESERVE,
        &secp,
    );
    lab.check(
        "admit_fully_backed_four_independent_claims",
        create.clone(),
        &funder,
        true,
        None,
    );
    let pool = committed(&mut lab, &create, 0);
    // Admission attacks pass capacity/noncontextual stages; the type must reject them.
    let under = admission(
        &lab,
        &funder,
        &typ,
        &lock,
        initial,
        2,
        manifest(&keys, &amounts),
        initial.sum + RESERVE - 1,
        &secp,
    );
    lab.check(
        "admission_underfunded_one_shannon",
        under,
        &funder,
        false,
        Some("scripts"),
    );
    let big_amounts = [100 * CKB; 4];
    let big_leaves: Vec<_> = keys.iter().map(|k| leaf(k, 100 * CKB)).collect();
    let big_root = tree(&big_leaves);
    let big_funder = lab.cell(output(&standard, big_root.sum + RESERVE + FEE), vec![]);
    let big_type = instance(&template, &big_funder);
    lab.check(
        "admission_principal_underfunded_400_claims_200_backing",
        admission(
            &lab,
            &big_funder,
            &big_type,
            &lock,
            big_root,
            2,
            manifest(&keys, &big_amounts),
            200 * CKB,
            &secp,
        ),
        &big_funder,
        false,
        Some("scripts"),
    );
    let lying = Node {
        hash: initial.hash,
        sum: CKB,
    };
    let tx = admission(
        &lab,
        &funder,
        &typ,
        &lock,
        lying,
        2,
        manifest(&keys, &amounts),
        CKB + RESERVE,
        &secp,
    );
    lab.check(
        "admission_forged_total",
        tx,
        &funder,
        false,
        Some("scripts"),
    );
    let forged = Node {
        hash: [0; 32],
        sum: initial.sum,
    };
    lab.check(
        "admission_forged_root",
        admission(
            &lab,
            &funder,
            &typ,
            &lock,
            forged,
            2,
            manifest(&keys, &amounts),
            initial.sum + RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let mut bad_manifest = manifest(&keys, &amounts);
    bad_manifest.pop();
    lab.check(
        "admission_missing_manifest_data",
        admission(
            &lab,
            &funder,
            &typ,
            &lock,
            initial,
            2,
            bad_manifest,
            initial.sum + RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let mut overflow = vec![0];
    for k in &keys {
        overflow.extend(k.verifying_key().as_bytes());
        overflow.extend(u64::MAX.to_le_bytes());
    }
    lab.check(
        "admission_sum_overflow",
        admission(
            &lab,
            &funder,
            &typ,
            &lock,
            Node {
                hash: [0; 32],
                sum: 0,
            },
            2,
            overflow,
            RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let huge = leaf(&keys[0], u64::MAX);
    lab.check(
        "admission_liabilities_plus_reserve_overflow",
        admission(
            &lab,
            &funder,
            &typ,
            &lock,
            huge,
            0,
            manifest(&keys[..1], &[u64::MAX]),
            RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    lab.check(
        "admission_depth_over_bound",
        admission(
            &lab,
            &funder,
            &typ,
            &lock,
            initial,
            9,
            manifest(&keys, &amounts),
            initial.sum + RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let false_id = typ
        .clone()
        .as_builder()
        .args(Bytes::from(vec![0; 32]))
        .build();
    lab.check(
        "admission_wrong_instance_id",
        admission(
            &lab,
            &funder,
            &false_id,
            &lock,
            initial,
            2,
            manifest(&keys, &amounts),
            initial.sum + RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let bad_lock = standard.clone().as_builder().args(Bytes::new()).build();
    lab.check(
        "admission_wrong_pool_lock",
        admission(
            &lab,
            &funder,
            &typ,
            &bad_lock,
            initial,
            2,
            manifest(&keys, &amounts),
            initial.sum + RESERVE,
            &secp,
        ),
        &funder,
        false,
        Some("scripts"),
    );
    let fresh = withdraw(
        &lab,
        &pool,
        None,
        &standard,
        0,
        CKB,
        &keys[0],
        &proof(&leaves, 0),
    );
    lab.check(
        "one_ckb_exit_fresh_standard_fails_capacity",
        fresh,
        &pool,
        false,
        Some("capacity"),
    );
    let valid = withdraw(
        &lab,
        &pool,
        Some(&recipients[0]),
        &standard,
        0,
        CKB,
        &keys[0],
        &proof(&leaves, 0),
    );
    lab.check(
        "one_ckb_exit_existing_real_acp",
        valid.clone(),
        &pool,
        true,
        None,
    );
    let credit = committed(&mut lab, &valid, 1);
    let owner_tx = sign_secp(
        lab.tx(
            &[credit.clone()],
            vec![output(&standard, 62 * CKB - 2 * FEE)],
            vec![vec![]],
            vec![],
        ),
        &recipient_keys[0],
    );
    lab.check(
        "acp_owner_independently_spends_credited_cell",
        owner_tx,
        &credit,
        true,
        None,
    );
    // A direct existing-ACP deposit is the competitive baseline (no pool, no type).
    let donor = lab.cell(output(&standard, 100 * CKB), vec![]);
    let direct = sign_secp(
        lab.tx(
            &[donor.clone(), recipients[0].clone()],
            vec![
                output(&standard, 99 * CKB - FEE),
                output(&recipients[0].cell_output.lock(), 62 * CKB),
            ],
            vec![vec![], vec![]],
            vec![],
        ),
        &secp,
    );
    lab.check(
        "existing_acp_direct_one_ckb_deposit",
        direct,
        &donor,
        true,
        None,
    );

    let same_budget = sign_secp(
        lab.tx(
            &[donor.clone(), recipients[0].clone()],
            vec![
                output(&standard, 99 * CKB),
                output(&recipients[0].cell_output.lock(), 62 * CKB - FEE),
            ],
            vec![vec![], vec![]],
            vec![],
        ),
        &secp,
    );
    lab.check(
        "existing_acp_direct_same_one_ckb_gross_budget",
        same_budget,
        &donor,
        true,
        None,
    );
    let direct_fresh = sign_secp(
        lab.tx(
            &[donor.clone()],
            vec![output(&standard, 99 * CKB - FEE), output(&standard, CKB)],
            vec![vec![], vec![]],
            vec![],
        ),
        &secp,
    );
    lab.check(
        "direct_one_ckb_new_standard_also_fails",
        direct_fresh,
        &donor,
        false,
        Some("capacity"),
    );
    // Forged proofs/signatures and correctly authorized malicious transactions.
    for (name, off) in [
        ("withdraw_forged_signature", 45 + 80),
        ("withdraw_forged_leaf_key", 13),
        ("withdraw_forged_sibling_hash", 45),
        ("withdraw_forged_sibling_sum", 77),
        ("withdraw_forged_index", 1),
        ("withdraw_forged_amount", 5),
    ] {
        let mut p = payload(&valid);
        p[off] ^= 1;
        lab.check(
            name,
            set_payload(valid.clone(), p),
            &pool,
            false,
            Some("scripts"),
        );
    }
    let mut p = payload(&valid);
    p[77..85].copy_from_slice(&u64::MAX.to_le_bytes());
    lab.check(
        "withdraw_sum_overflow",
        set_payload(valid.clone(), p),
        &pool,
        false,
        Some("scripts"),
    );
    let mut p = payload(&valid);
    p.pop();
    lab.check(
        "withdraw_truncated_public_proof",
        set_payload(valid.clone(), p),
        &pool,
        false,
        Some("scripts"),
    );
    let mut outs = valid.outputs().into_iter().collect::<Vec<_>>();
    outs[0] = outs[0]
        .clone()
        .as_builder()
        .capacity(RESERVE + 9 * CKB - 1)
        .build();
    outs[1] = outs[1]
        .clone()
        .as_builder()
        .capacity(62 * CKB - FEE + 1)
        .build();
    let evil = sign_claim(
        valid.as_advanced_builder().set_outputs(outs).build(),
        &keys[0],
    );
    lab.check(
        "signed_theft_from_remaining_claims",
        evil,
        &pool,
        false,
        Some("scripts"),
    );
    let mut ds = valid.outputs_data().into_iter().collect::<Vec<_>>();
    let mut d = ds[0].raw_data().to_vec();
    d[33..41].copy_from_slice(&(8 * CKB).to_le_bytes());
    ds[0] = Bytes::from(d).pack();
    let evil = sign_claim(
        valid.as_advanced_builder().set_outputs_data(ds).build(),
        &keys[0],
    );
    lab.check(
        "signed_successor_liability_reduction",
        evil,
        &pool,
        false,
        Some("scripts"),
    );
    let mut outs = valid.outputs().into_iter().collect::<Vec<_>>();
    outs[0] = outs[0].clone().as_builder().type_(None::<Script>).build();
    let evil = sign_claim(
        valid.as_advanced_builder().set_outputs(outs).build(),
        &keys[0],
    );
    lab.check(
        "signed_strip_type_while_claims_remain",
        evil,
        &pool,
        false,
        Some("scripts"),
    );
    let mut outs = valid.outputs().into_iter().collect::<Vec<_>>();
    outs[1] = outs[1].clone().as_builder().lock(standard.clone()).build();
    let evil = sign_claim(
        valid.as_advanced_builder().set_outputs(outs).build(),
        &keys[0],
    );
    lab.check(
        "signed_redirect_existing_acp_capital",
        evil,
        &pool,
        false,
        Some("scripts"),
    );
    let premature = sign_claim(
        lab.tx(
            &[pool.clone(), recipients[0].clone()],
            vec![output(&recipients[0].cell_output.lock(), 218 * CKB - FEE)],
            vec![vec![]],
            payload(&valid),
        ),
        &keys[0],
    );
    lab.check(
        "signed_premature_full_closure",
        premature,
        &pool,
        false,
        Some("scripts"),
    );
    let mut seen = HashSet::new();
    lab.resolver_case("first_pool_exit_resolves", valid.clone(), &mut seen, true);
    lab.resolver_case(
        "replay_pool_exit_dead_input",
        valid.clone(),
        &mut seen,
        false,
    );
    let competing = withdraw(
        &lab,
        &pool,
        Some(&recipients[1]),
        &standard,
        1,
        2 * CKB,
        &keys[1],
        &proof(&leaves, 1),
    );
    lab.check(
        "competing_exit_individually_valid",
        competing.clone(),
        &pool,
        true,
        None,
    );
    lab.resolver_case("competing_exit_conflicts", competing, &mut seen, false);
    // 24 complete orders x 4 independent withdrawals. Each user retains only its initial local proof;
    // subsequent proofs use PUBLIC payloads, never the global tree in the local builder.
    let mut orders = vec![];
    permutations(vec![], vec![0, 1, 2, 3], &mut orders);
    let mut reconstructed = 0;
    let mut sequences = vec![];
    for order in &orders {
        let mut current = pool.clone();
        let mut local: Vec<_> = (0..4).map(|i| proof(&leaves, i)).collect();
        let mut live = vec![true; 4];
        let mut remaining = initial.sum;
        for (step, &i) in order.iter().enumerate() {
            let tx = withdraw(
                &lab,
                &current,
                Some(&recipients[i]),
                &standard,
                i,
                amounts[i],
                &keys[i],
                &local[i],
            );
            lab.check(
                &format!(
                    "order_{}{}_{}{}_step_{step}",
                    order[0], order[1], order[2], order[3]
                ),
                tx.clone(),
                &current,
                true,
                None,
            );
            let public = payload(&tx);
            remaining -= amounts[i];
            live[i] = false;
            for j in 0..4 {
                if live[j] {
                    local[j] = refresh(j, &local[j], &public);
                    reconstructed += 1;
                }
            }
            if remaining > 0 {
                let c = committed(&mut lab, &tx, 0);
                let cap: u64 = c.cell_output.capacity().into();
                assert_eq!(cap, RESERVE + remaining);
                let d = c.mem_cell_data.as_ref().unwrap();
                assert_eq!(u64::from_le_bytes(d[33..41].try_into().unwrap()), remaining);
                current = c;
            } else {
                assert!(
                    tx.outputs()
                        .into_iter()
                        .all(|o| o.type_().to_opt().is_none())
                );
            }
        }
        sequences.push(json!({"order":order,"remaining_liabilities_shannons":remaining,"all_claims_paid":remaining==0}));
    }
    // Separate process receives only retained user proof and serialized PUBLIC transactions.
    let public_dir = root.join("evidence/wo02/operator-gone");
    std::fs::create_dir_all(&public_dir).unwrap();
    let initial_local = json!({"index":0,"amount_shannons":CKB,"pubkey":hex::encode(keys[0].verifying_key().as_bytes()),"proof":proof(&leaves,0).iter().map(|n|json!({"hash":hex::encode(n.hash),"sum":n.sum})).collect::<Vec<_>>(),"initial_root_data":hex::encode(pool.mem_cell_data.as_ref().unwrap()),"initial_outpoint":hex::encode(pool.out_point.as_slice()),"type_script":hex::encode(typ.as_slice())});
    std::fs::write(
        public_dir.join("retained-user-state.json"),
        serde_json::to_string_pretty(&initial_local).unwrap() + "\n",
    )
    .unwrap();
    let mut stream = vec![];
    let mut public_transactions = vec![];
    let mut cur = pool.clone();
    let mut witness_state = leaves.clone();
    for i in [1usize, 3, 2] {
        let tx = withdraw(
            &lab,
            &cur,
            Some(&recipients[i]),
            &standard,
            i,
            amounts[i],
            &keys[i],
            &proof(&witness_state, i),
        );
        lab.check(
            &format!("public_archive_valid_exit_{i}"),
            tx.clone(),
            &cur,
            true,
            None,
        );
        stream.push(hex::encode(tx.data().as_slice()));
        public_transactions.push(tx.clone());
        witness_state[i] = empty();
        cur = committed(&mut lab, &tx, 0);
    }
    let archive = json!({"packed_transactions":stream,"independently_authenticated_live_outpoint":hex::encode(cur.out_point.as_slice()),"anchor_scope":"isolated real-validator run supplies trusted current cell; no production light-client proof included"});
    std::fs::write(
        public_dir.join("public-archive.json"),
        serde_json::to_string_pretty(&archive).unwrap() + "\n",
    )
    .unwrap();
    let client = root.join("experiments/wo02/proof_client.py");
    let result = std::process::Command::new("python3")
        .arg(&client)
        .arg(public_dir.join("retained-user-state.json"))
        .arg(public_dir.join("public-archive.json"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    std::fs::write(
        public_dir.join("reconstructed-user-state.json"),
        &result.stdout,
    )
    .unwrap();
    let restored: Value = serde_json::from_slice(&result.stdout).unwrap();
    let reconstructed_proof: Vec<Node> = restored["proof"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| Node {
            hash: hex::decode(n["hash"].as_str().unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            sum: n["sum"].as_u64().unwrap(),
        })
        .collect();
    lab.check(
        "operator_gone_separate_client_reconstructed_exit",
        withdraw(
            &lab,
            &cur,
            Some(&recipients[0]),
            &standard,
            0,
            CKB,
            &keys[0],
            &reconstructed_proof,
        ),
        &cur,
        true,
        None,
    );
    let mut client_attacks = vec![];
    for attack in [
        "missing_middle",
        "forged_witness",
        "forged_signature",
        "wrong_tip",
    ] {
        let mut bad = archive.clone();
        if attack == "missing_middle" {
            bad["packed_transactions"].as_array_mut().unwrap().remove(1);
        } else if attack == "wrong_tip" {
            bad["independently_authenticated_live_outpoint"] = json!("00".repeat(36));
        } else {
            let first = public_transactions[0].clone();
            let mut bytes = payload(&first);
            let off = if attack == "forged_signature" {
                bytes.len() - 1
            } else {
                45
            };
            bytes[off] ^= 1;
            let changed = set_payload(first, bytes);
            bad["packed_transactions"][0] = json!(hex::encode(changed.data().as_slice()));
        }
        let path = public_dir.join(format!("attack-{attack}.json"));
        std::fs::write(&path, serde_json::to_string_pretty(&bad).unwrap() + "\n").unwrap();
        let result = std::process::Command::new("python3")
            .arg(&client)
            .arg(public_dir.join("retained-user-state.json"))
            .arg(&path)
            .output()
            .unwrap();
        assert!(!result.status.success(), "client accepted {attack}");
        client_attacks.push(json!({"attack":attack,"rejected":true,"stderr":String::from_utf8_lossy(&result.stderr)}));
        println!("PASS independent_client_rejects_{attack}");
    }
    std::fs::write(
        public_dir.join("client-attack-results.json"),
        serde_json::to_string_pretty(&client_attacks).unwrap() + "\n",
    )
    .unwrap();
    // Isolate stale/reused proof failures against the committed successor.
    let successor = committed(&mut lab, &valid, 0);
    let mut changed = leaves.clone();
    changed[0] = empty();
    let good_second = withdraw(
        &lab,
        &successor,
        Some(&recipients[1]),
        &standard,
        1,
        2 * CKB,
        &keys[1],
        &proof(&changed, 1),
    );
    let mut stale = payload(&good_second);
    for (i, s) in proof(&leaves, 1).iter().enumerate() {
        stale[45 + 40 * i..77 + 40 * i].copy_from_slice(&s.hash);
        stale[77 + 40 * i..85 + 40 * i].copy_from_slice(&s.sum.to_le_bytes());
    }
    lab.check(
        "stale_proof_rejected_with_fresh_authorization",
        sign_claim(set_payload(good_second, stale), &keys[1]),
        &successor,
        false,
        Some("scripts"),
    );
    let mut repeated = payload(&valid);
    let mut repeated_outputs = valid.outputs().into_iter().collect::<Vec<_>>();
    repeated_outputs[0] = repeated_outputs[0]
        .clone()
        .as_builder()
        .capacity(155 * CKB)
        .build();
    let tx = lab.tx(
        &[successor.clone(), recipients[0].clone()],
        repeated_outputs,
        valid
            .outputs_data()
            .into_iter()
            .map(|b| b.raw_data().to_vec())
            .collect(),
        std::mem::take(&mut repeated),
    );
    lab.check(
        "reused_claim_on_successor",
        sign_claim(tx, &keys[0]),
        &successor,
        false,
        Some("scripts"),
    );
    // Do not assert an availability guarantee from an opaque root: empty public input cannot unlock.
    lab.check(
        "missing_public_proof_cannot_exit",
        set_payload(valid.clone(), vec![]),
        &pool,
        false,
        Some("scripts"),
    );
    // Ensure a copied instance cannot be readmitted using a different first funding input.
    let other_funder = lab.cell(output(&standard, initial.sum + RESERVE + FEE), vec![]);
    lab.check(
        "cannot_recreate_same_instance_from_new_funder",
        admission(
            &lab,
            &other_funder,
            &typ,
            &lock,
            initial,
            2,
            manifest(&keys, &amounts),
            initial.sum + RESERVE,
            &secp,
        ),
        &other_funder,
        false,
        Some("scripts"),
    );
    let max_keys: Vec<_> = (0u16..256)
        .map(|i| {
            let seed = hash(i.to_le_bytes());
            SigningKey::from_bytes(&seed)
        })
        .collect();
    let max_amounts = vec![CKB; 256];
    let max_leaves: Vec<_> = max_keys.iter().map(|k| leaf(k, CKB)).collect();
    let max_root = tree(&max_leaves);
    let max_funder = lab.cell(output(&standard, max_root.sum + RESERVE + 2 * FEE), vec![]);
    let max_type = instance(&template, &max_funder);
    let max_create = admission(
        &lab,
        &max_funder,
        &max_type,
        &lock,
        max_root,
        8,
        manifest(&max_keys, &max_amounts),
        max_root.sum + RESERVE,
        &secp,
    );
    lab.check(
        "admission_256_actual_independent_keys",
        max_create,
        &max_funder,
        true,
        None,
    );
    let required = pool
        .cell_output
        .occupied_capacity(Capacity::bytes(41).unwrap())
        .unwrap()
        .as_u64();
    assert_eq!(required, RESERVE);
    let report = json!({"base_wo01":"ac79f830c896b8b431de9ce869df0883ffc416d5","scope":"actual CKB noncontextual/capacity/script validators and resolver with in-memory cells; no node, mempool, PoW or censorship benchmark","max_admission_depth":8,"max_admission_claims":256,"fixture_fee_shannons":FEE,"root_occupied_shannons":required,"root_data_bytes":41,"pool_reserve_shannons":RESERVE,"type_elf_bytes":code.len(),"type_code_data_hash":hex::encode(hash(&code)),"type_deployment_occupied_ckb_standard_lock":code.len()+61,"always_success_bytes":success_bytes.len(),"always_success_deployment_ckb_standard_lock":success_bytes.len()+61,"acp_elf_bytes":acp_bytes.len(),"acp_data_hash":hex::encode(hash(&acp_bytes)),"acp_deployment_occupied_ckb_standard_lock":acp_bytes.len()+61,"standard_elf_bytes":standard_bytes.len(),"secp_data_bytes":secpdata.len(),"independent_client_archive_attacks":client_attacks,"exhaustive_orders":sequences,"public_proof_refreshes":reconstructed,"cases":lab.rows});
    std::fs::write(
        root.join("evidence/wo02/validator-results.json"),
        serde_json::to_string_pretty(&report).unwrap() + "\n",
    )
    .unwrap();
    println!(
        "ALL {} CASES PASSED; {} COMPLETE EXIT ORDERS; {} PUBLIC PROOF REFRESHES",
        report["cases"].as_array().unwrap().len(),
        orders.len(),
        reconstructed
    );
}
