#define MAX_LOCK_NAME 16
#define MAX_FILE_NAME 32

struct sync_event {
  u64  mutex;
  char lock_name[MAX_LOCK_NAME];
  char file_name[MAX_FILE_NAME];
  u64  line_number;
};

BPF_PERF_OUTPUT(sync_enter);
BPF_PERF_OUTPUT(sync_locked);
BPF_PERF_OUTPUT(sync_try_locked);
BPF_PERF_OUTPUT(sync_unlocked);

int trace_sync_enter(struct pt_regs *ctx) {
    struct sync_event e = {};
    bpf_usdt_readarg(1, ctx, &e.mutex);
    bpf_usdt_readarg_p(2, ctx, &e.lock_name, MAX_LOCK_NAME);
    bpf_usdt_readarg_p(3, ctx, &e.file_name, MAX_FILE_NAME);
    bpf_usdt_readarg(4, ctx, &e.line_number);
    sync_enter.perf_submit(ctx, &e, sizeof(e));
    return 0;
};

int trace_sync_locked(struct pt_regs *ctx) {
    struct sync_event e = {};
    bpf_usdt_readarg(1, ctx, &e.mutex);
    bpf_usdt_readarg_p(2, ctx, &e.lock_name, MAX_LOCK_NAME);
    bpf_usdt_readarg_p(3, ctx, &e.file_name, MAX_FILE_NAME);
    bpf_usdt_readarg(4, ctx, &e.line_number);
    sync_locked.perf_submit(ctx, &e, sizeof(e));
    return 0;
};

int trace_sync_try_locked(struct pt_regs *ctx) {
    struct sync_event e = {};
    bpf_usdt_readarg(1, ctx, &e.mutex);
    bpf_usdt_readarg_p(2, ctx, &e.lock_name, MAX_LOCK_NAME);
    bpf_usdt_readarg_p(3, ctx, &e.file_name, MAX_FILE_NAME);
    bpf_usdt_readarg(4, ctx, &e.line_number);
    sync_try_locked.perf_submit(ctx, &e, sizeof(e));
    return 0;
};

int trace_sync_unlocked(struct pt_regs *ctx) {
    struct sync_event e = {};
    bpf_usdt_readarg(1, ctx, &e.mutex);
    bpf_usdt_readarg_p(2, ctx, &e.lock_name, MAX_LOCK_NAME);
    bpf_usdt_readarg_p(3, ctx, &e.file_name, MAX_FILE_NAME);
    //bpf_usdt_readarg(4, ctx, &e.line_number);
    sync_unlocked.perf_submit(ctx, &e, sizeof(e));
    return 0;
};
