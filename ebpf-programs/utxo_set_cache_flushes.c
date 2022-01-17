
struct utxo_cache_flush {
  u64 duration;
  u32 mode;
  u64 coins_count;
  u64 coins_memusage;
  bool flush_for_prune;
};

BPF_PERF_OUTPUT(perf_utxocache_flushes);

int trace_utxocache_flush(struct pt_regs *ctx) {
    struct utxo_cache_flush f = {};
    bpf_usdt_readarg(1, ctx, &f.duration);
    bpf_usdt_readarg(2, ctx, &f.mode);
    bpf_usdt_readarg(3, ctx, &f.coins_count);
    bpf_usdt_readarg(4, ctx, &f.coins_memusage);
    bpf_usdt_readarg(5, ctx, &f.flush_for_prune);
    perf_utxocache_flushes.perf_submit(ctx, &f, sizeof(f));
    return 0;
};
