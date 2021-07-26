
// Tor v3 addresses are 62 chars + 6 chars for the port (':12345').
// I2P addresses are 60 chars + 6 chars for the port (':12345').
#define MAX_PEER_ADDR_LENGTH 62 + 6
#define MAX_PEER_CONN_TYPE_LENGTH 20
#define MAX_MSG_TYPE_LENGTH 20

struct p2p_message
{
    u64     peer_id;
    char    peer_addr[MAX_PEER_ADDR_LENGTH];
    char    peer_conn_type[MAX_PEER_CONN_TYPE_LENGTH];
    char    msg_type[MAX_MSG_TYPE_LENGTH];
    u64     msg_size;
};


// Two BPF perf buffers for pushing data (here P2P messages) to user space.
BPF_PERF_OUTPUT(inbound_messages);
BPF_PERF_OUTPUT(outbound_messages);

int trace_inbound_message(struct pt_regs *ctx) {
    struct p2p_message msg = {};

    bpf_usdt_readarg(1, ctx, &msg.peer_id);
    bpf_usdt_readarg_p(2, ctx, &msg.peer_addr, MAX_PEER_ADDR_LENGTH);
    bpf_usdt_readarg_p(3, ctx, &msg.peer_conn_type, MAX_PEER_CONN_TYPE_LENGTH);
    bpf_usdt_readarg_p(4, ctx, &msg.msg_type, MAX_MSG_TYPE_LENGTH);
    bpf_usdt_readarg(5, ctx, &msg.msg_size);

    inbound_messages.perf_submit(ctx, &msg, sizeof(msg));
    return 0;
};

int trace_outbound_message(struct pt_regs *ctx) {
    struct p2p_message msg = {};

    bpf_usdt_readarg(1, ctx, &msg.peer_id);
    bpf_usdt_readarg_p(2, ctx, &msg.peer_addr, MAX_PEER_ADDR_LENGTH);
    bpf_usdt_readarg_p(3, ctx, &msg.peer_conn_type, MAX_PEER_CONN_TYPE_LENGTH);
    bpf_usdt_readarg_p(4, ctx, &msg.msg_type, MAX_MSG_TYPE_LENGTH);
    bpf_usdt_readarg(5, ctx, &msg.msg_size);

    outbound_messages.perf_submit(ctx, &msg, sizeof(msg));
    return 0;
};