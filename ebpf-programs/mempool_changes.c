// The longest rejection reason is 118 chars and is generated in case of SCRIPT_ERR_EVAL_FALSE by
// strprintf("mandatory-script-verify-flag-failed (%s)", ScriptErrorString(check.GetScriptError()))
#define MAX_REJECT_REASON_LENGTH        118
// The longest string returned by RemovalReasonToString() is 'sizelimit'
#define MAX_REMOVAL_REASON_LENGTH       9
#define HASH_LENGTH                     32

struct added_event
{
  u64   vsize;
  s64   fee;
};

struct removed_event
{
  char  reason[MAX_REMOVAL_REASON_LENGTH];
  u64   vsize;
  s64   fee;
};

struct rejected_event
{
  char  reason[MAX_REJECT_REASON_LENGTH];
};

struct replaced_event
{
  /*
   * We currently don't use these as we don't have metrics for them.
  u64   replaced_vsize;
  s64   replaced_fee;
  u64   replacement_vsize;
  s64   replacement_fee;
  */
};

// BPF perf buffer to push the data to user space.
BPF_PERF_OUTPUT(mempool_added_events);
BPF_PERF_OUTPUT(mempool_removed_events);
BPF_PERF_OUTPUT(mempool_rejected_events);
BPF_PERF_OUTPUT(mempool_replaced_events);

int trace_mempool_added(struct pt_regs *ctx) {
  struct added_event added = {};

  bpf_usdt_readarg(2, ctx, &added.vsize);
  bpf_usdt_readarg(3, ctx, &added.fee);

  mempool_added_events.perf_submit(ctx, &added, sizeof(added));
  return 0;
}

int trace_mempool_removed(struct pt_regs *ctx) {
  struct removed_event removed = {};

  bpf_usdt_readarg_p(2, ctx, &removed.reason, MAX_REMOVAL_REASON_LENGTH);
  bpf_usdt_readarg(3, ctx, &removed.vsize);
  bpf_usdt_readarg(4, ctx, &removed.fee);

  mempool_removed_events.perf_submit(ctx, &removed, sizeof(removed));
  return 0;
}

int trace_mempool_rejected(struct pt_regs *ctx) {
  struct rejected_event rejected = {};

  bpf_usdt_readarg_p(2, ctx, &rejected.reason, MAX_REJECT_REASON_LENGTH);

  mempool_rejected_events.perf_submit(ctx, &rejected, sizeof(rejected));
  return 0;
}

int trace_mempool_replaced(struct pt_regs *ctx) {
  struct replaced_event replaced = {};

  /* Unused. See replaced struct comment.
  bpf_usdt_readarg(2, ctx, &replaced.replaced_vsize);
  bpf_usdt_readarg(3, ctx, &replaced.replaced_fee);
  bpf_usdt_readarg(4, ctx, &replaced.replaced_entry_time);
  bpf_usdt_readarg(6, ctx, &replaced.replacement_vsize);
  bpf_usdt_readarg(7, ctx, &replaced.replacement_fee);
  */

  mempool_replaced_events.perf_submit(ctx, &replaced, sizeof(replaced));
  return 0;
}
