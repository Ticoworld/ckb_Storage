use ckb_app_config::DBConfig;
use ckb_db::RocksDB;
use ckb_db_schema::{COLUMNS, COLUMN_CELL, COLUMN_CELL_DATA, COLUMN_CELL_DATA_HASH};
use ckb_types::{packed, prelude::*};
use std::{env, fs, path::{Path, PathBuf}, time::Instant};

const CKB: u64 = 100_000_000;
const OPTIONS: &str = "evidence/wo04/default.db-options";

fn tree_bytes(root: &Path) -> u64 {
    fs::read_dir(root).unwrap().map(|entry| {
        let path = entry.unwrap().path();
        if path.is_dir() { tree_bytes(&path) } else { path.metadata().unwrap().len() }
    }).sum()
}

fn cell(i: u32) -> (packed::OutPoint, packed::CellEntry) {
    let mut args = [0u8; 20];
    for (j, b) in args.iter_mut().enumerate() {
        *b = i.wrapping_mul(0x9e3779b9).rotate_left(j as u32).wrapping_add(j as u32) as u8;
    }
    let lock = packed::Script::new_builder()
        .code_hash(packed::Byte32::from_slice(&[0x42; 32]).unwrap())
        .hash_type(0u8)
        .args(args.to_vec().pack())
        .build();
    let output = packed::CellOutput::new_builder()
        .capacity(ckb_types::prelude::Pack::<packed::Uint64>::pack(&(61 * CKB)))
        .lock(lock).build();
    let block_number = i / 1_000;
    let mut block_hash = [0u8; 32];
    for (j, b) in block_hash.iter_mut().enumerate() {
        *b = block_number.wrapping_mul(0x85ebca6b).rotate_left(j as u32).wrapping_add(j as u32) as u8;
    }
    let entry = packed::CellEntry::new_builder()
        .output(output)
        .block_hash(packed::Byte32::from_slice(&block_hash).unwrap())
        .block_number(ckb_types::prelude::Pack::<packed::Uint64>::pack(&(block_number as u64)))
        .block_epoch(ckb_types::prelude::Pack::<packed::Uint64>::pack(&((block_number / 100) as u64)))
        .index(ckb_types::prelude::Pack::<packed::Uint32>::pack(&0u32))
        .data_size(ckb_types::prelude::Pack::<packed::Uint64>::pack(&0u64))
        .build();
    let mut hash = [0u8; 32];
    for (j, b) in hash.iter_mut().enumerate() {
        *b = i.wrapping_mul(0x27d4eb2d).rotate_left(j as u32).wrapping_add((j * 17) as u32) as u8;
    }
    let point = packed::OutPoint::new_builder()
        .tx_hash(packed::Byte32::from_slice(&hash).unwrap())
        .index(ckb_types::prelude::Pack::<packed::Uint32>::pack(&0u32))
        .build();
    (point, entry)
}

fn open(path: &Path) -> RocksDB {
    fs::create_dir_all(path).unwrap();
    let cache_size = env::var("WO04_CACHE_SIZE_BYTES").ok()
        .map(|v| v.parse::<usize>().expect("WO04_CACHE_SIZE_BYTES must be an integer"))
        .unwrap_or(268_435_456);
    let config = DBConfig {
        path: path.to_path_buf(),
        cache_size: Some(cache_size),
        options_file: Some(PathBuf::from(OPTIONS)),
        ..Default::default()
    };
    RocksDB::open(&config, COLUMNS)
}

fn put_cell(tx: &ckb_db::RocksDBTransaction, i: u32) {
    let (point, entry) = cell(i);
    let key = point.to_cell_key();
    tx.put(COLUMN_CELL, &key, entry.as_slice()).unwrap();
    tx.put(COLUMN_CELL_DATA, &key, &[]).unwrap();
    tx.put(COLUMN_CELL_DATA_HASH, &key, &[]).unwrap();
}

fn delete_cell(tx: &ckb_db::RocksDBTransaction, i: u32) {
    let key = cell(i).0.to_cell_key();
    tx.delete(COLUMN_CELL, &key).unwrap();
    tx.delete(COLUMN_CELL_DATA, &key).unwrap();
    tx.delete(COLUMN_CELL_DATA_HASH, &key).unwrap();
}

fn write_range(db: &RocksDB, start: u32, count: u32, delete_start: Option<u32>) {
    for base in (0..count).step_by(1_000) {
        let len = (count - base).min(1_000);
        let tx = db.transaction();
        for off in 0..len { put_cell(&tx, start + base + off); }
        if let Some(old_start) = delete_start {
            for off in 0..len { delete_cell(&tx, old_start + base + off); }
        }
        tx.commit().unwrap(); drop(tx);
    }
}

fn delete_range(db: &RocksDB, start: u32, count: u32) {
    for base in (0..count).step_by(1_000) {
        let len = (count - base).min(1_000);
        let tx = db.transaction();
        for off in 0..len { delete_cell(&tx, start + base + off); }
        tx.commit().unwrap(); drop(tx);
    }
}

fn compact_cell_columns(db: &RocksDB) {
    db.compact_range(COLUMN_CELL, None, None).unwrap();
    db.compact_range(COLUMN_CELL_DATA, None, None).unwrap();
    db.compact_range(COLUMN_CELL_DATA_HASH, None, None).unwrap();
}

fn empty_db_bytes(root: &Path) -> u64 {
    drop(open(root));
    tree_bytes(root)
}

fn run_one(root: &Path, n: u32, churn_rounds: u32) -> (u64, u64, u64, u64, u64, u128, u128, u64, u64) {
    let value_bytes = cell(1).1.as_slice().len() as u64;
    let db = open(root);
    let t0 = Instant::now();
    write_range(&db, 0, n, None);
    let insert_ms = t0.elapsed().as_millis();
    assert!(db.get_pinned(COLUMN_CELL, &cell(0).0.to_cell_key()).unwrap().is_some());
    drop(db); // close/flush before measuring persistent file bytes
    let db = open(root); compact_cell_columns(&db); drop(db);
    let inserted_bytes = tree_bytes(root);

    let mut churn_ms = 0u128;
    for round in 0..churn_rounds {
        let old_start = n * round;
        let new_start = n * (round + 1);
        let db = open(root);
        let t = Instant::now();
        write_range(&db, new_start, n, Some(old_start));
        churn_ms += t.elapsed().as_millis();
        assert!(db.get_pinned(COLUMN_CELL, &cell(new_start).0.to_cell_key()).unwrap().is_some());
        assert!(db.get_pinned(COLUMN_CELL, &cell(old_start).0.to_cell_key()).unwrap().is_none());
        drop(db);
    }
    let before_delete = tree_bytes(root);
    let live_start = n * churn_rounds;
    let db = open(root);
    compact_cell_columns(&db);
    drop(db);
    let compacted_live_bytes = tree_bytes(root);
    let db = open(root);
    let t = Instant::now();
    delete_range(&db, live_start, n);
    let final_delete_ms = t.elapsed().as_millis();
    assert!(db.get_pinned(COLUMN_CELL, &cell(live_start).0.to_cell_key()).unwrap().is_none());
    compact_cell_columns(&db);
    drop(db);
    let after_delete = tree_bytes(root);
    (value_bytes, inserted_bytes, before_delete, compacted_live_bytes, after_delete, insert_ms, churn_ms + final_delete_ms, n as u64 * (churn_rounds as u64 + 1), n as u64 * (churn_rounds as u64 + 1))
}

fn main() {
    let base = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/tmp/wo04-ckb-store"));
    let baseline_path = base.join("empty-baseline");
    let empty_bytes = empty_db_bytes(&baseline_path);
    println!("record_type,live_cells,churn_rounds,total_inserted,total_deleted,cell_entry_value_bytes,raw_column_key_value_bytes,empty_db_bytes,db_bytes_after_live_insert_compacted,db_bytes_before_live_compaction,db_bytes_after_live_compaction,db_bytes_after_delete_compacted,initial_insert_ms,churn_and_delete_ms");
    println!("empty,0,0,0,0,0,{empty_bytes},{empty_bytes},{empty_bytes},{empty_bytes},0,0");
    fs::remove_dir_all(&baseline_path).unwrap();
    for n in [1_000u32, 10_000, 100_000] {
        for churn in [0u32, 1, 5] {
            let path = base.join(format!("cells-{n}-churn-{churn}"));
            let (cell_bytes, after_first, before_delete, live_compacted, after_delete, insert_ms, delete_ms, total_inserted, total_deleted) = run_one(&path, n, churn);
            let raw_column_bytes = n as u64 * 293;
            println!("no_data_cell,{n},{churn},{total_inserted},{total_deleted},{cell_bytes},{raw_column_bytes},{empty_bytes},{after_first},{before_delete},{live_compacted},{after_delete},{insert_ms},{delete_ms}");
            fs::remove_dir_all(path).unwrap();
        }
    }
    fs::remove_dir_all(base).ok();
}
