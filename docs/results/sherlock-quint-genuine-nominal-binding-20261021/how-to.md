# How-to: reproduce the selected positive checks

Use a local Node/Quint installation; the historical checks used the selected Node24.21.0 binary and Quint0.32.0 package. The portable source closure has thirteen local Quint modules. From the repository root, set `QUINT_CLI` to your installed Quint `dist/src/cli.js`, then enter `models/sherlock/quint/public-nominal-source-cases-20261020`.

First run the Python adapter with explicit paths to this result directory's two original observations, `--candidate-dir .`, and a new absent `--out` directory:

```sh
python3 -I nominal_positive_data.py --map-capture <result-dir>/data/map-queue.observation.json --mirror-capture <result-dir>/data/mirror-arbor.observation.json --candidate-dir . --out <new-output-dir>
```

After inspecting both generated and authored-candidate inverse receipts, the corresponding frontend and exact two-test commands are:

```sh
node --max-old-space-size=512 "$QUINT_CLI" typecheck q11ActualNominalBindingTest.qnt
node --max-old-space-size=512 "$QUINT_CLI" test q11ActualNominalBindingTest.qnt --main q11ActualNominalBindingTest --match '^(q11ActualMapQueueFinalRootTest|q11ActualMirrorArborFinalRootTest)$' --backend typescript --max-samples 1 --seed 1
```

These are portable instructions, not extra executions recorded by publication. Historical operations used the existing capture mechanism with60s wall and4MiB per stream, saving full raw results before classification. Keep local resource/output limits appropriate to your environment and preserve diagnostics. A repeat is a new observation, not a retroactive refresh of these receipts. No project reviewer record is a public developer prerequisite; unsupported profiles and objective proof/target obligations remain separate.
