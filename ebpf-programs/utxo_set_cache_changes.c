// We don't care about the contents of the utxocache tracepoints here.
// It's only relevant that the tracepoint is being called so we can
// increase the respective prometheus counter.

const u8 UTXOCACHE_ADD = 0;
const u8 UTXOCACHE_SPENT = 1;
const u8 UTXOCACHE_UNCACHE = 2;

struct utxo_cache_event {
  u8 event;
};

BPF_PERF_OUTPUT(perf_utxocache_events);

int trace_utxocache_add(struct pt_regs *ctx) {
    struct utxo_cache_event e = {};
    e.event = UTXOCACHE_ADD;
    perf_utxocache_events.perf_submit(ctx, &e, sizeof(e));
    return 0;
};

int trace_utxocache_uncache(struct pt_regs *ctx) {
    struct utxo_cache_event e = {};
    e.event = UTXOCACHE_UNCACHE;
    perf_utxocache_events.perf_submit(ctx, &e, sizeof(e));
    return 0;
};

int trace_utxocache_spent(struct pt_regs *ctx) {
    struct utxo_cache_event e = {};
    e.event = UTXOCACHE_SPENT;
    perf_utxocache_events.perf_submit(ctx, &e, sizeof(e));
    return 0;
};
