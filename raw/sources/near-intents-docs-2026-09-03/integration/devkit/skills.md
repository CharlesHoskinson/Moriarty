> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills

> Enable AI Agents to build cross-chain apps

Most modern AI Agents support the use of `skills`, which are small knowledge bases that explain how to use a particular API or tool.

By providing the `near-intents` skill to your agent, you can enable it to build cross-chain swap applications using the 1Click Swap API.

<Info>
  Skill repo: <a href="https://github.com/near/agent-skills/tree/main/skills/near-intents">near/agent-skills → skills/near-intents</a>
</Info>

***

## Using the skill

<Steps>
  <Step title="Install the skill">
    Install the `near-intents` skill:

    ```bash theme={null}
    npx skills add near/agent-skills --skill near-intents
    ```
  </Step>

  <Step title="Ask your agent to build a swap app">
    Your agent should automatically pick up the `near-intents` skill, otherwise you can explicitly ask the agent to read the `SKILL.md` file to learn how to use NEAR Intents.
  </Step>
</Steps>
