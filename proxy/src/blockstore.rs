use solana_ledger::blockstore::Blockstore;
use rocksdb::Options;
use tempfile::TempDir;

pub fn create_in_memory_blockstore() -> (Blockstore, TempDir) {
    // Create a temporary directory that will be deleted when it goes out of scope
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path();

    // Set up RocksDB options for in-memory operation
    let mut db_options = Options::default();
    db_options.create_if_missing(true);
    db_options.set_paranoid_checks(false);
    db_options.set_max_open_files(-1);

    // Open the database
    let blockstore = Blockstore::open_with_options(db_path, db_options, true, None).unwrap();

    // Return both Blockstore and TempDir
    (blockstore, temp_dir)
}

/*pub struct Blockstore {
    ledger_path: PathBuf,
    meta_cf: HashMap<Slot, SlotMeta>,
    dead_slots_cf: HashMap<Slot, bool>,
    duplicate_slots_cf: HashMap<Slot, DuplicateSlotProof>,
    roots_cf: HashMap<Slot, bool>,
    erasure_meta_cf: HashMap<(Slot, u64), ErasureMeta>,
    orphans_cf: HashMap<Slot, bool>,
    index_cf: HashMap<Slot, Index>,
    data_shred_cf: HashMap<(Slot, u64), Vec<u8>>,
    code_shred_cf: HashMap<(Slot, u64), Vec<u8>>,
    transaction_status_cf: HashMap<(u64, Signature, Slot), TransactionStatusMeta>,
    address_signatures_cf: HashMap<(u64, Pubkey, Slot, Signature), AddressSignatureMeta>,
    transaction_memos_cf: HashMap<Signature, String>,
    transaction_status_index_cf: HashMap<Slot, TransactionStatusIndexMeta>,
    highest_primary_index_slot: RwLock<Option<Slot>>,
    rewards_cf: HashMap<Slot, Rewards>,
    blocktime_cf: HashMap<Slot, UnixTimestamp>,
    perf_samples_cf: HashMap<Slot, PerfSample>,
    block_height_cf: HashMap<Slot, u64>,
    program_costs_cf: HashMap<Pubkey, ProgramCost>,
    bank_hash_cf: HashMap<Slot, FrozenHashVersioned>,
    optimistic_slots_cf: HashMap<Slot, OptimisticSlotMetaVersioned>,
    max_root: AtomicU64,
    merkle_root_meta_cf: HashMap<Slot, MerkleRootMeta>,
    insert_shreds_lock: Mutex<()>,
    new_shreds_signals: Mutex<Vec<Sender<bool>>>,
    completed_slots_senders: Mutex<Vec<CompletedSlotsSender>>,
    pub shred_timing_point_sender: Option<PohTimingSender>,
    pub lowest_cleanup_slot: RwLock<Slot>,
    pub slots_stats: SlotsStats,
    rpc_api_metrics: BlockstoreRpcApiMetrics,
}*/