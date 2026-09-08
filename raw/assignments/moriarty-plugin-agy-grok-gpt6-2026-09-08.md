# Moriarty plugin implementation routing

The user redirected the current primary task to finishing the approved Moriarty development plugin:

> Use AGY Gemini 3.8 flash high to implement the plugin, grok for the first check then GPT 6 for the check after grok passes

The user also requested the Gemini 3.8 Flash model card and AGY calling documentation.

For this plugin, use AGY with `gemini-3.8-flash-high` to implement, Grok 4.6 high for the first independent check, and a fresh GPT-6 Astra check only after Grok passes the same candidate. This supersedes the older plugin model routing. Finish all four approved milestones; preserve their acceptance requirements and existing resource limits. Do not let ledger work displace the plugin again.

Google's model card and AGY headless documentation were acquired under `/home/charl/.local/state/moriarty/agy-gemini38-reference-20260908/acquisition.json`. The live `agy models` result includes the requested exact model slug. Run completion and actual test results must be checked separately from the process exit code.

The twelve Moriarty sprints remain required. This instruction changes the immediate focus and plugin routing; it does not claim any sprint or plugin milestone accepted.
