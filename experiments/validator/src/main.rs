use ckb_chain_spec::consensus::{Consensus, ConsensusBuilder};
use ckb_hash::blake2b_256 as hash;
use ckb_script::{TransactionScriptsVerifier, TxVerifyEnv};
use ckb_traits::{CellDataProvider, HeaderProvider, ExtensionProvider};
use ckb_types::{bytes::Bytes, core::{Capacity, TransactionView, HeaderView, EpochNumberWithFraction, ScriptHashType, cell::{CellMeta, CellMetaBuilder, ResolvedTransaction, CellProvider, CellStatus, HeaderChecker, resolve_transaction}, error::OutPointError}, packed::{self, Byte32, OutPoint, CellOutput, CellInput, CellDep, Script}, prelude::*};
use ckb_verification::{CapacityVerifier, NonContextualTransactionVerifier};
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{json, Value};
use std::{collections::{HashMap, HashSet}, sync::Arc};
const CKB:u64=100_000_000;
const MAX_CYCLES:u64=100_000_000;
#[derive(Clone, Default)]
struct Store { cells: HashMap<OutPoint, CellMeta> }
impl CellDataProvider for Store {
 fn get_cell_data(&self,p:&OutPoint)->Option<Bytes>{self.cells.get(p)?.mem_cell_data.clone()}
 fn get_cell_data_hash(&self,p:&OutPoint)->Option<Byte32>{self.cells.get(p)?.mem_cell_data_hash.clone()}
}
impl HeaderProvider for Store {fn get_header(&self,_:&Byte32)->Option<HeaderView>{None}}
impl ExtensionProvider for Store {fn get_block_extension(&self,_:&Byte32)->Option<packed::Bytes>{None}}
impl CellProvider for Store {fn cell(&self,p:&OutPoint,_:bool)->CellStatus {self.cells.get(p).cloned().map(CellStatus::Live).unwrap_or(CellStatus::Unknown)}}
impl HeaderChecker for Store {fn check_valid(&self,h:&Byte32)->Result<(),OutPointError>{Err(OutPointError::InvalidHeader(h.clone()))}}
struct Lab {store:Store, consensus:Arc<Consensus>, deps:Vec<CellMeta>, support:Vec<CellMeta>, counter:u32, rows:Vec<Value>}
impl Lab {
 fn new()->Self {Self{store:Store::default(),consensus:Arc::new(ConsensusBuilder::default().build()),deps:vec![],support:vec![],counter:1,rows:vec![]}}
 fn cell(&mut self,o:CellOutput,d:Vec<u8>)->CellMeta {
  self.counter+=1; let op=OutPoint::new_builder().tx_hash(hash(&self.counter.to_le_bytes()).pack()).index(0u32).build();
  let c=CellMetaBuilder::from_cell_output(o,Bytes::from(d)).out_point(op.clone()).build();self.store.cells.insert(op,c.clone());c
 }
 fn code(&mut self, data:Vec<u8>,args:Vec<u8>)->Script {
  let script=Script::new_builder().code_hash(hash(&data).pack()).hash_type(ScriptHashType::Data2).args(Bytes::from(args)).build();
  let output=CellOutput::new_builder().lock(Script::default()).capacity(Capacity::bytes(data.len()+41).unwrap()).build();
  let c=self.cell(output,data);self.deps.push(c);script
 }
 fn tx(&self,input:&CellMeta,outputs:Vec<CellOutput>,data:Vec<Vec<u8>>,w:Vec<u8>)->TransactionView {
  TransactionView::new_advanced_builder().input(CellInput::new_builder().previous_output(input.out_point.clone()).build())
   .outputs(outputs).outputs_data(data.into_iter().map(|d|Bytes::from(d).pack()).collect::<Vec<_>>())
   .cell_deps(self.deps.iter().filter(|c| c.mem_cell_data_hash.as_ref()==Some(&input.cell_output.lock().code_hash())).chain(if input.cell_output.lock().hash_type()==ScriptHashType::Data.into() {self.support.iter()} else {self.support[0..0].iter()}).map(|c|CellDep::new_builder().out_point(c.out_point.clone()).build()).collect::<Vec<_>>())
   .witness(Bytes::from(w)).build()
 }
 fn resolved(&self,tx:&TransactionView,_input:&CellMeta)->ResolvedTransaction {resolve_transaction(tx.clone(),&mut HashSet::<OutPoint>::new(),&self.store,&self.store).expect("fixture resolution")}
 fn check(&mut self,name:&str,tx:TransactionView,input:&CellMeta,expected:bool,stage:Option<&str>)->Option<u64>{
  let rtx=Arc::new(self.resolved(&tx,input));
  let header=HeaderView::new_advanced_builder().epoch(EpochNumberWithFraction::new(100_000,0,1)).build();
  let env=Arc::new(TxVerifyEnv::new_commit(&header));
  let result=NonContextualTransactionVerifier::new(&tx,&self.consensus).verify().map_err(|e|("noncontextual",e.to_string()))
   .and_then(|_|CapacityVerifier::new(rtx.clone(),self.consensus.dao_type_hash()).verify().map_err(|e|("capacity",e.to_string())))
   .and_then(|_|TransactionScriptsVerifier::new(rtx,self.store.clone(),self.consensus.clone(),env).verify(MAX_CYCLES).map_err(|e|("scripts",e.to_string())));
  let (ok,cycles,failed_stage,error)=match result{Ok(c)=>(true,Some(c),None,None),Err((s,e))=>(false,None,Some(s),Some(e))};
  let passed=ok==expected && (expected || stage.is_none() || stage==failed_stage);
  println!("{} {} accepted={} cycles={:?} stage={:?} {}",if passed{"PASS"}else{"FAIL"},name,ok,cycles,failed_stage,error.as_deref().unwrap_or(""));
  let input_cap:u64=input.cell_output.capacity().into();let output_cap:u64=tx.outputs().into_iter().map(|o|{let c:u64=o.capacity().into();c}).sum();
  self.rows.push(json!({"name":name,"test_passed":passed,"expected_accept":expected,"accepted":ok,"cycles":cycles,"failure_stage":failed_stage,"error":error,"serialized_transaction_bytes":tx.data().as_slice().len(),"fee_shannons":input_cap.checked_sub(output_cap)}));
  assert!(passed,"unexpected result: {name}");cycles
 }
 fn resolver_case(&mut self,name:&str,tx:TransactionView,seen:&mut HashSet<OutPoint>,expected:bool){
  let r=resolve_transaction(tx,seen,&self.store,&self.store);let ok=r.is_ok();
  println!("{} {name} resolver_accept={ok} {:?}",if ok==expected{"PASS"}else{"FAIL"},r.as_ref().err());
  self.rows.push(json!({"name":name,"test_passed":ok==expected,"expected_accept":expected,"accepted":ok,"failure_stage":if ok{None}else{Some("resolver")},"error":r.err().map(|e|e.to_string())}));assert_eq!(ok,expected);
 }
}
fn output(lock:&Script,capacity:u64)->CellOutput {CellOutput::new_builder().lock(lock.clone()).capacity(capacity).build()}
fn key(n:u8)->SigningKey {SigningKey::from_bytes(&[n;32])}
fn policy(a:&SigningKey,b:&SigningKey)->Vec<u8>{[a.verifying_key().as_bytes().as_slice(),b.verifying_key().as_bytes().as_slice()].concat()}
fn sign_policy(tx:TransactionView,p:&[u8],mode:u8,k:&SigningKey)->TransactionView {let mut w=vec![mode];w.extend(p);w.extend(k.sign(tx.hash().as_slice()).to_bytes());tx.as_advanced_builder().set_witnesses(vec![Bytes::from(w).pack()]).build()}
fn leaf(k:&SigningKey,amount:u64)->[u8;32]{let mut b=vec![0];b.extend(k.verifying_key().as_bytes());b.extend(amount.to_le_bytes());hash(b)}
fn node(a:[u8;32],b:[u8;32])->[u8;32]{hash([vec![1],a.to_vec(),b.to_vec()].concat())}
fn tree_root(leaves:&[[u8;32]])->[u8;32]{if leaves.len()==1 {return leaves[0]}let m=leaves.len()/2;node(tree_root(&leaves[..m]),tree_root(&leaves[m..]))}
fn proof(leaves:&[[u8;32]],mut index:usize)->Vec<u8>{let mut layer=leaves.to_vec();let mut out=vec![];while layer.len()>1{out.extend(layer[index^1]);layer=layer.chunks_exact(2).map(|p|node(p[0],p[1])).collect();index>>=1;}out}
fn sign_pool(tx:TransactionView,leaves:&[[u8;32]],index:usize,amount:u64,k:&SigningKey)->TransactionView {let mut w=vec![];w.extend((index as u32).to_le_bytes());w.extend(amount.to_le_bytes());w.extend(k.verifying_key().as_bytes());w.push(leaves.len().trailing_zeros() as u8);w.extend(proof(leaves,index));w.extend(k.sign(tx.hash().as_slice()).to_bytes());tx.as_advanced_builder().set_witnesses(vec![Bytes::from(w).pack()]).build()}
fn mutate_witness(tx:&TransactionView,index:usize)->TransactionView {let mut w=tx.witnesses().get(0).unwrap().raw_data().to_vec();w[index]^=1;tx.as_advanced_builder().set_witnesses(vec![Bytes::from(w).pack()]).build()}
fn sign_secp(tx:TransactionView,config:&[u8],key:&ckb_crypto::secp::Privkey)->TransactionView {
 let mut lock=config.to_vec();lock.extend([0;65]);
 let w=packed::WitnessArgs::new_builder().lock(Some(Bytes::from(lock))).build();
 let mut h=ckb_hash::new_blake2b();h.update(tx.hash().as_slice());h.update(&(w.as_slice().len() as u64).to_le_bytes());h.update(w.as_slice());
 let mut msg=[0;32];h.finalize(&mut msg);
 let sig=key.sign_recoverable(&msg.into()).unwrap();let mut lock=config.to_vec();lock.extend(sig.serialize());
 let w=packed::WitnessArgs::new_builder().lock(Some(Bytes::from(lock))).build();
 tx.as_advanced_builder().set_witnesses(vec![w.as_bytes().pack()]).build()
}
fn main(){
 let mut lab=Lab::new();
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
 let fixture=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../wo01-sources/ckb/script/testdata/always_success");
 let success=lab.code(std::fs::read(fixture).unwrap(),vec![]);
 let (a,b,attacker)=(key(1),key(2),key(99));let p=policy(&a,&b);
 let policy_code=std::fs::read(root.join("experiments/bin/policy")).unwrap();
 let policy_lock=lab.code(policy_code.clone(),hash(&p).to_vec());
 let pool_code=std::fs::read(root.join("experiments/bin/pool")).unwrap();let pool_lock=lab.code(pool_code.clone(),vec![]);
 let embedded_code=std::fs::read(root.join("experiments/bin/embedded")).unwrap();let embedded_lock=lab.code(embedded_code.clone(),vec![]);
 let mut capacity_rows=vec![];
 for (name,args,data_len,type_args) in [("empty_args",0,0,None),("args_20_standard_shape",20,0,None),("args_32_policy",32,0,None),("args_64_two_keys",64,0,None),("pool_root",0,32,None),("args_20_type_32_data_16",20,16,Some(32)),("omnilock_base_shape",22,0,None),("acp_base_shape",20,0,None)] {
  let l=success.clone().as_builder().args(Bytes::from(vec![0;args])).build();
  let mut o=output(&l,1000*CKB);if let Some(n)=type_args{o=o.as_builder().type_(Some(Script::new_builder().args(Bytes::from(vec![0;n])).build())).build();}
  let min=o.occupied_capacity(Capacity::bytes(data_len).unwrap()).unwrap().as_u64();
  capacity_rows.push(json!({"name":name,"args_bytes":args,"data_bytes":data_len,"type_args_bytes":type_args,"occupied_shannons":min,"occupied_ckb":min/CKB,"serialized_output_bytes":o.as_slice().len()}));
  let input=lab.cell(output(&success,1000*CKB),vec![]);
  // Typed shapes measure occupied capacity here; no deployed type verifier is implied.
  if type_args.is_none(){
   for (suffix,c,expected) in [("exact",min,true),("minus_one_shannon",min-1,false)] {
    let tx=lab.tx(&input,vec![o.clone().as_builder().capacity(c).build()],vec![vec![0;data_len]],vec![]);
    lab.check(&format!("capacity_{name}_{suffix}"),tx,&input,expected,Some("capacity"));
   }
  }
 }
 // Existing system locks are the strongest reusable owner/recovery baseline.
 let secp_data=ckb_system_scripts::BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap().into_owned();
 let support=lab.cell(output(&success,(secp_data.len() as u64+41)*CKB),secp_data.clone());lab.support.push(support);
 let secp=ckb_crypto::secp::Privkey::from_slice(&[7;32]);let recover=ckb_crypto::secp::Privkey::from_slice(&[8;32]);let bad=ckb_crypto::secp::Privkey::from_slice(&[9;32]);
 let standard_bin=ckb_system_scripts::BUNDLED_CELL.get("specs/cells/secp256k1_blake160_sighash_all").unwrap().into_owned();
 let standard=lab.code(standard_bin.clone(),hash(secp.pubkey().unwrap().serialize())[0..20].to_vec()).as_builder().hash_type(ScriptHashType::Data).build();
 let standard_cell=lab.cell(output(&standard,100*CKB),vec![]);
 let standard_tx=lab.tx(&standard_cell,vec![output(&standard,100*CKB-1000)],vec![vec![]],vec![]);
 lab.check("standard_secp_owner_spend",sign_secp(standard_tx.clone(),&[],&secp),&standard_cell,true,None);
 lab.check("standard_secp_wrong_signer",sign_secp(standard_tx,&[],&bad),&standard_cell,false,Some("scripts"));
 let mut multi_policy=vec![0,0,1,2];multi_policy.extend(&hash(secp.pubkey().unwrap().serialize())[0..20]);multi_policy.extend(&hash(recover.pubkey().unwrap().serialize())[0..20]);
 let multi_bin=ckb_system_scripts::BUNDLED_CELL.get("specs/cells/secp256k1_blake160_multisig_all").unwrap().into_owned();
 let multi=lab.code(multi_bin.clone(),hash(&multi_policy)[0..20].to_vec()).as_builder().hash_type(ScriptHashType::Data).build();
 let multi_cell=lab.cell(output(&multi,100*CKB),vec![]);
 let multi_tx=lab.tx(&multi_cell,vec![output(&standard,100*CKB-1000)],vec![vec![]],vec![]);
 lab.check("existing_multisig_1of2_owner",sign_secp(multi_tx.clone(),&multi_policy,&secp),&multi_cell,true,None);
 lab.check("existing_multisig_1of2_independent_recovery",sign_secp(multi_tx.clone(),&multi_policy,&recover),&multi_cell,true,None);
 lab.check("existing_multisig_1of2_unauthorized",sign_secp(multi_tx,&multi_policy,&bad),&multi_cell,false,Some("scripts"));
 // A valid signature authorizes all raw transaction fields; signatures do not authenticate unrelated witness data.
 let asset=lab.cell(output(&policy_lock,200*CKB),vec![]);
 let base=lab.tx(&asset,vec![output(&success,200*CKB-1000)],vec![vec![]],vec![]);
 let normal=sign_policy(base.clone(),&p,0,&a);lab.check("policy_owner_spend",normal.clone(),&asset,true,None);
 lab.check("policy_independent_recovery",sign_policy(base.clone(),&p,1,&b),&asset,true,None);
 lab.check("policy_unauthorized_signer",sign_policy(base.clone(),&p,0,&attacker),&asset,false,Some("scripts"));
 lab.check("policy_forged_signature",mutate_witness(&normal,100),&asset,false,Some("scripts"));
 lab.check("policy_forged_policy",sign_policy(base.clone(),&policy(&attacker,&b),0,&attacker),&asset,false,Some("scripts"));
 lab.check("policy_wrong_recovery_path",sign_policy(base.clone(),&p,1,&a),&asset,false,Some("scripts"));
 let altered=normal.clone().as_advanced_builder().set_outputs(vec![output(&success,199*CKB)]).build();lab.check("policy_output_tamper",altered,&asset,false,Some("scripts"));
 let new_policy=policy(&key(3),&key(4));let rotated_lock=policy_lock.clone().as_builder().args(Bytes::from(hash(&new_policy).to_vec())).build();
 let rotation=sign_policy(lab.tx(&asset,vec![output(&rotated_lock,200*CKB-1000)],vec![vec![]],vec![]),&p,1,&b);
 lab.check("policy_recovery_rotates_lock",rotation.clone(),&asset,true,None);
 let rotated=lab.cell(rotation.outputs().get(0).unwrap(),vec![]);
 let rotated_spend=lab.tx(&rotated,vec![output(&success,200*CKB-2000)],vec![vec![]],vec![]);
 lab.check("policy_old_owner_after_rotation",sign_policy(rotated_spend.clone(),&p,0,&a),&rotated,false,Some("scripts"));
 lab.check("policy_new_owner_after_rotation",sign_policy(rotated_spend,&new_policy,0,&key(3)),&rotated,true,None);
 // Late deposits retain their original lock; recovery of a different cell cannot revoke it.
 let late=lab.cell(output(&policy_lock,100*CKB),vec![]);
 lab.check("policy_old_owner_can_spend_late_old_lock_deposit",sign_policy(lab.tx(&late,vec![output(&success,100*CKB-1000)],vec![vec![]],vec![]),&p,0,&a),&late,true,None);
 let mut seen=HashSet::new();lab.resolver_case("policy_first_spend_resolves",normal.clone(),&mut seen,true);lab.resolver_case("policy_replay_dead_input",normal,&mut seen,false);
 // Empty args is possible when identity moves into code, with per-identity deployment cost.
 let embedded_seed:[u8;32]=hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60").unwrap().try_into().unwrap();
 let embedded_key=SigningKey::from_bytes(&embedded_seed);let embedded_cell=lab.cell(output(&embedded_lock,41*CKB),vec![]);
 let eb=lab.tx(&embedded_cell,vec![output(&success,41*CKB-1000)],vec![vec![]],vec![]);
 // Payout needs at least 41 CKB even with the deliberately insecure zero-args fixture.
 lab.check("embedded_exit_cannot_pay_fee_at_minimum",eb.clone(),&embedded_cell,false,Some("capacity"));
 let embedded_cell=lab.cell(output(&embedded_lock,100*CKB),vec![]);let eb=lab.tx(&embedded_cell,vec![output(&success,100*CKB-1000)],vec![vec![]],vec![]);
 let es=embedded_key.sign(eb.hash().as_slice()).to_bytes();let eb=eb.as_advanced_builder().set_witnesses(vec![Bytes::from(es.to_vec()).pack()]).build();lab.check("embedded_authorized_owner",eb.clone(),&embedded_cell,true,None);
 let es=attacker.sign(eb.hash().as_slice()).to_bytes();lab.check("embedded_wrong_owner",eb.as_advanced_builder().set_witnesses(vec![Bytes::from(es.to_vec()).pack()]).build(),&embedded_cell,false,Some("scripts"));
 // Shared pool: four independent keys; reserve 73, liabilities 4*100 CKB. No operator signature.
 let keys=[key(10),key(11),key(12),key(13)];let amounts=[100*CKB;4];let leaves:Vec<_>=keys.iter().zip(amounts).map(|(k,a)|leaf(k,a)).collect();
 let pool=lab.cell(output(&pool_lock,473*CKB),tree_root(&leaves).to_vec());
 let payout_lock=standard.clone(); // 61-byte secure recipient; creation does not execute recipient lock.
 let make_withdraw=|lab:&Lab,input:&CellMeta,leaves:&[[u8;32]],index:usize,k:&SigningKey,amount:u64|{
  let mut updated=leaves.to_vec();updated[index]=hash([2]);let closing=updated.iter().all(|x|*x==hash([2]));
  let cap:u64=input.cell_output.capacity().into();
  let (outs,data)=if closing{(vec![output(&payout_lock,cap-1000)],vec![vec![]])}else{(vec![output(&pool_lock,cap-amount),output(&payout_lock,amount-1000)],vec![tree_root(&updated).to_vec(),vec![]])};
  sign_pool(lab.tx(input,outs,data,vec![]),leaves,index,amount,k)
 };
 let withdrawal=make_withdraw(&lab,&pool,&leaves,0,&keys[0],100*CKB);
 lab.check("pool_independent_exit_no_operator",withdrawal.clone(),&pool,true,None);
 lab.check("pool_forged_signature",mutate_witness(&withdrawal,withdrawal.witnesses().get(0).unwrap().raw_data().len()-1),&pool,false,Some("scripts"));
 lab.check("pool_forged_sibling",mutate_witness(&withdrawal,45),&pool,false,Some("scripts"));
 lab.check("pool_forged_index",mutate_witness(&withdrawal,0),&pool,false,Some("scripts"));
 lab.check("pool_unauthorized_key",make_withdraw(&lab,&pool,&leaves,0,&attacker,100*CKB),&pool,false,Some("scripts"));
 lab.check("pool_excessive_claim",make_withdraw(&lab,&pool,&leaves,0,&keys[0],101*CKB),&pool,false,Some("scripts"));
 let mut changed=withdrawal.outputs().into_iter().collect::<Vec<_>>();changed[0]=changed[0].clone().as_builder().capacity(372*CKB).build();changed[1]=changed[1].clone().as_builder().capacity(101*CKB-1000).build();
 let theft=withdrawal.as_advanced_builder().set_outputs(changed).build();lab.check("pool_reauthorized_overwithdraw",sign_pool(theft,&leaves,0,100*CKB,&keys[0]),&pool,false,Some("scripts"));
 let mut changed_data=withdrawal.outputs_data().into_iter().collect::<Vec<_>>();changed_data[0]=Bytes::from(vec![0;32]).pack();let bad_root=withdrawal.as_advanced_builder().set_outputs_data(changed_data).build();lab.check("pool_reauthorized_wrong_next_root",sign_pool(bad_root,&leaves,0,100*CKB,&keys[0]),&pool,false,Some("scripts"));
 let mut seen=HashSet::new();lab.resolver_case("pool_first_exit_resolves",withdrawal.clone(),&mut seen,true);
 let parallel=make_withdraw(&lab,&pool,&leaves,1,&keys[1],100*CKB);lab.check("pool_concurrent_exit_individually_valid",parallel.clone(),&pool,true,None);lab.resolver_case("pool_concurrent_exit_conflicts",parallel.clone(),&mut seen,false);lab.resolver_case("pool_repeated_exit_dead_input",withdrawal.clone(),&mut seen,false);
 let next=lab.cell(withdrawal.outputs().get(0).unwrap(),withdrawal.outputs_data().get(0).unwrap().raw_data().to_vec());
 let mut new_leaves=leaves.clone();new_leaves[0]=hash([2]);
 // Correct new input signature with OLD proof isolates proof staleness from signature replay.
 let stale=make_withdraw(&lab,&next,&leaves,1,&keys[1],100*CKB);lab.check("pool_stale_proof_on_successor",stale,&next,false,Some("scripts"));
 lab.check("pool_updated_proof_on_successor",make_withdraw(&lab,&next,&new_leaves,1,&keys[1],100*CKB),&next,true,None);
 // Leaf zeroing prevents a repeated claim even with a fresh signature and new outpoint.
 lab.check("pool_repeat_claim_on_successor",make_withdraw(&lab,&next,&new_leaves,0,&keys[0],100*CKB),&next,false,Some("scripts"));
 // Withholding: no proof/witness cannot unlock a real resolved pool.
 lab.check("pool_missing_state_proof",lab.tx(&pool,withdrawal.outputs().into_iter().collect(),withdrawal.outputs_data().into_iter().map(|b|b.raw_data().to_vec()).collect(),vec![]),&pool,false,Some("scripts"));
 let mut cur=next;let mut state=new_leaves;
 for i in 1..4 {
  let t=make_withdraw(&lab,&cur,&state,i,&keys[i],100*CKB);lab.check(&format!("pool_sequential_exit_{i}"),t.clone(),&cur,true,None);state[i]=hash([2]);
  if i<3{cur=lab.cell(t.outputs().get(0).unwrap(),t.outputs_data().get(0).unwrap().raw_data().to_vec())}
 }
 // Cheap operating claims do not imply cheap independent L1 exits.
 let dust_leaves=vec![leaf(&keys[0],CKB),leaf(&keys[1],CKB)];let dust=lab.cell(output(&pool_lock,75*CKB),tree_root(&dust_leaves).to_vec());
 lab.check("pool_one_ckb_claim_exit_below_recipient_floor",make_withdraw(&lab,&dust,&dust_leaves,0,&keys[0],CKB),&dust,false,Some("capacity"));
 // Missing contract data blocks execution, even with otherwise valid auth.
 let saved=lab.deps.clone();lab.deps.retain(|c|c.mem_cell_data.as_ref().unwrap().as_ref()!=policy_code.as_slice());
 let missing=sign_policy(lab.tx(&asset,vec![output(&success,200*CKB-1000)],vec![vec![]],vec![]),&p,0,&a);lab.check("policy_missing_code_dependency",missing,&asset,false,Some("scripts"));lab.deps=saved;
 let deployment:Vec<_>=[("policy",policy_code),("pool",pool_code),("embedded",embedded_code),("standard_secp",standard_bin),("standard_multisig",multi_bin),("standard_secp_data",secp_data)].into_iter().map(|(n,b)|json!({"contract":n,"elf_bytes":b.len(),"code_data_hash":hex::encode(hash(&b)),"deployment_occupied_ckb_with_20_byte_lock":b.len()+61})).collect();
 let report=json!({"scope":"real CKB noncontextual, capacity and VM script verifiers; real outpoint resolver with in-memory cells; no live chain, PoW, network, mempool or mainnet deployment", "max_test_cycles":MAX_CYCLES,"consensus_max_block_cycles":lab.consensus.max_block_cycles(),"consensus_max_block_bytes":lab.consensus.max_block_bytes(),"capacity_measurements":capacity_rows,"deployment":deployment,"cases":lab.rows});
 let report_path=root.join("evidence/validator-results.json");std::fs::write(report_path,serde_json::to_string_pretty(&report).unwrap()+"\n").unwrap();
 println!("ALL {} CASES PASSED",report["cases"].as_array().unwrap().len());
}
