# Reproduction of issue: https://github.com/anza-xyz/agave/issues/1186

When running the tests against local validator of solana 1.18.15 the tests runs fine.

When running the tests against a validator without feature EenyoWx9UMXYKpR8mW5Jmfmy2fRjzUtM7NduYMY8bx33 the tests fail with at stackframe error:

```bash
solana-test-validator -r  --deactivate-feature EenyoWx9UMXYKpR8mW5Jmfmy2fRjzUtM7NduYMY8bx33
anchor test --skip-local-validator
```

results in:

```bash
     Simulation failed.
Message: Transaction simulation failed: Error processing Instruction 0: Program failed to complete.
Logs:
[
  "Program YMzmnDGoxtfXB68r2rdrFWhnCykFyYUWEj62NkvTsbE invoke [1]",
  "Program log: Instruction: Initialize",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program YMzmnDGoxtfXB68r2rdrFWhnCykFyYUWEj62NkvTsbE consumed 5031 of 200000 compute units",
  "Program YMzmnDGoxtfXB68r2rdrFWhnCykFyYUWEj62NkvTsbE failed: Access violation in stack frame 11 at address 0x20000bfd0 of size 8"
].
Catch the `SendTransactionError` and call `getLogs()` on it for full details.
```
