# Benchmarking Cubi as a local-LLM coding agent

What we picked, what got measured, what the measurement showed, and what is
still outstanding.

## Which benchmark

Cubi already ships harnesses for the two that matter, so the recommendation is
to use what's here rather than adopt a third:

| Benchmark | Status in Cubi | What it measures | Cost to run |
| --- | --- | --- | --- |
| **Cubi quick suite** (`cubi bench`) | built in, 6 tasks | agent plumbing + tool-calling on tiny fix-the-bug tasks; zero network, zero Docker | seconds per task |
| **SWE-bench-Lite** (`cubi swebench`) | built in, official schema | real GitHub issues; the standard number people quote | hours + Docker for scoring |
| Aider Polyglot | not wired | 225 Exercism problems, 6 languages, hidden tests; two attempts with test feedback | moderate; Docker recommended |
| Terminal-Bench | not wired | long-horizon shell work | high |

**Recommendation.** Use `cubi bench` as the fast pre-commit signal and
`cubi swebench` for anything quotable. [Aider Polyglot](https://github.com/Aider-AI/polyglot-benchmark)
is the best *addition* if we want cross-language edit quality — it is designed
around exactly the edit-a-file-then-fix-your-mistake loop Cubi's agent
implements, and it discriminates well among small local models. Note the
widely-reported caveat that top-of-leaderboard SWE-bench scores are partly
memorized and don't predict out-of-distribution behavior, which is another
argument for keeping the internal suite as the primary regression guard.

## What was measured

The run happened in a container with **no GPU and no model weights**: this
session's egress policy denies `ollama.com`, `registry.ollama.ai`, and
`huggingface.co` (403 on CONNECT). So no live-model scores were produced —
see "Outstanding" below. What *is* measurable without a model is the harness
itself, and that turned out to be where the problem was.

### Result 1 — the suite is correctly calibrated

Both ends of the suite were checked directly, per task:

- **Floor.** With the pristine `repo/`, all 6 `verify.sh` scripts fail
  (rc 1 for pytest, 101 for cargo). No task is a free pass.
- **Ceiling.** With the known-correct fix applied, all 6 pass. No task is
  unsolvable or mis-specified.

### Result 2 — the harness could never score above 0% (fixed)

Driving the real agent loop through the harness with a scripted tool call
(the `Fake` backend, so the result is deterministic) exposed three independent
defects, each sufficient on its own to make every task fail for every model:

1. **All writes were denied.** `run_task` gives the agent an isolated `HOME`,
   which starts with an empty trust store. Cubi refuses writes outside a
   trusted root, and headless `-p` *auto-denies* instead of prompting
   (`src/cli/agent.rs:136`), so every `edit_file` returned
   `[tool denied] user declined approval`. `cubi swebench` already solved this
   with a `seed_trust` helper; `cubi bench` never got it.
2. **`verify.sh` was never executed.** `tasks_root` defaults to the relative
   `bench/tasks`, but verify runs with the throwaway workdir as cwd — so `sh`
   exited **2** ("can't open") without running any test. The existence check
   passed because it resolves from the harness cwd, which masked the bug.
3. **Event logs were discarded.** `--events` was likewise relative, so each
   run's JSONL was written *inside* the workdir and deleted with it.

Fixes: a shared `permissions::seed_trust_file` (now used by both harnesses, so
the on-disk trust format has one writer), plus canonicalizing `tasks_root` and
the output dir. After the fix, the same end-to-end run scores as it should:

```
task                 status  verify_rc  agent_s
rust-typo-fix        pass    0          0.28
rust-off-by-one      pass    0          0.23
py-bug-off-by-one    pass    0          0.28
py-import-fix        pass    0          0.21
py-leap-year         pass    0          0.22
rust-fizzbuzz        pass    0          0.27
----
end-to-end ceiling: 6/6 passed (100.0%)
```

The negative control still fails correctly: an agent that only *reads* the
file scores 0% with `verify_exit_code: 101` — the real `cargo test` failure,
not the old harness-level rc 2.

**Impact.** Every nightly `summary.json` produced before this change reports
0% regardless of model, so there is no usable historical baseline — and the
`qwen3:8b` → `qwen3.5:9b` default bump could not have been measured by this
harness even in principle. `tests/bench.rs` now guards all three defects with
a model-free pass/fail pair that runs in ordinary CI.

## Outstanding

**Live-model scores were not produced** — blocked on model weights, not on the
harness. On any machine with Ollama, the comparison the default-model bump
actually calls for is:

```sh
ollama pull qwen3:8b && ollama pull qwen3.5:9b
cargo build --release

for m in qwen3:8b qwen3.5:9b; do
  for run in 1 2 3; do
    ./target/release/cubi bench --suite quick --model "$m" --json \
      --output "bench/results/$m-$run"
  done
done
```

Three runs per model, because the quick suite is 6 binary tasks: a one-task
swing is ±16.7 points and a single run cannot separate two similar models
(see "Interpreting a score" in [`bench/README.md`](../bench/README.md)).
Two caveats for that run: the tasks' `time_cap_seconds` (120–180s) assume
GPU-class throughput and will need raising for CPU-only inference, and the
Python tasks need `pytest` installed.

Worth doing next, in order:

1. Run the comparison above and establish a real baseline for both models.
2. Grow the quick suite past 6 tasks so it can resolve a model delta at all.
3. Consider wiring Aider Polyglot for cross-language edit quality.
