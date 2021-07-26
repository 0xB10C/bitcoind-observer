struct block_connected
{
    int32_t   height;
    u64     transactions;
    int32_t   inputs;
    u64     sigops;
    u64     connection_time;
};

BPF_PERF_OUTPUT(perf_block_connected);

int trace_block_connected(struct pt_regs *ctx) {
    struct block_connected bc = {};

    bpf_usdt_readarg(2, ctx, &bc.height);
    bpf_usdt_readarg(3, ctx, &bc.transactions);
    bpf_usdt_readarg(4, ctx, &bc.inputs);
    bpf_usdt_readarg(5, ctx, &bc.sigops);
    bpf_usdt_readarg(6, ctx, &bc.connection_time);

    perf_block_connected.perf_submit(ctx, &bc, sizeof(bc));
    return 0;
};