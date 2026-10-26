# Proof server | Midnight Docs
> Source: https://docs.midnight.network/guides/run-proof-server (requested as /develop/tutorial/using/proof-server; redirected)
> Collected: 2026-10-26
> Published: Unknown

Publication metadata note: Unknown (page footer: last updated Oct 23, 2026)

[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Midnight Logo](/img/midnight-header-logo-dark.svg)](/)

[API reference](/api-reference)[SDKs](/sdks)[Compact](/compact)[Dev Diaries](/blog)

AI search

Search

* [Overview](/)
* [What is Midnight?](/what-is-midnight)
* [Getting started](/getting-started)
* [Examples](/examples)
* [Tutorials](/tutorials)
* [Concepts](/concepts)
* [Guides](/guides)

  + [Networks and environments](/guides/networks-and-environments)
  + [Fund a wallet](/guides/acquire-tokens)
  + [DUST sponsorship](/guides/dust-sponsorship)
  + [Manage DUST](/guides/manage-dust)
  + [Run a wallet in Node.js](/guides/node-server-wallet)
  + [Build a private smart contract](/guides/build-a-private-smart-contract)
  + [Security and best practices](/guides/security-best-practices)
  + [Run the proof server](/guides/run-proof-server)
  + [Prove transactions locally](/guides/local-proving)
  + [Use Compact contracts from JavaScript](/guides/compact-javascript-runtime)
  + [Manage private state with witnesses](/guides/private-state-witnesses)
  + [Deploy and operate a contract](/guides/deploy-and-operate)
  + [Index contract state with EffectStream](/guides/index-state-with-effectstream)
  + [Build a cross-chain DApp with EffectStream](/guides/build-cross-chain-dapp-with-effectstream)
  + [Create a React wallet connector](/guides/react-wallet-connect)
  + [Create a Next.js wallet connector](/guides/nextjs-wallet-connect)
  + [Windows Compact setup](/guides/windows-compact-setup)
  + [Set up Bun for Midnight development](/how-to/bun-runtime-midnight)
* [Tokens](/tokens)
* [AI integration](/ai-integration)
* [Compact](/compact)
* [Nodes](/nodes)
* [Release notes](/relnotes/overview)
* [Troubleshoot](/troubleshoot)
* [Glossary](/glossary)

* [Guides](/guides)
* Run the proof server

Version: v1

Explore with… ▾

Copy page as MarkdownCopied

On this page

> For the complete documentation index, see [llms.txt](/llms.txt)

Proof server
============

Midnight uses zero-knowledge (ZK) cryptography to enable shielded
transactions and data protection. An essential element of this
architecture is ZK functionality provided by a Midnight *proof
server*, which generates proofs locally that are verified on-chain.

The information that a DApp sends to the proof server includes
private data, such as details of token ownership or a DApp's
private state. To protect your data, you should access only a local
proof server, or perhaps one on a remote machine that you control,
over an encrypted channel.

This guide shows you how to run the proof server locally in order
to process transactions on the Midnight Network. The wallet communicates with the proof
server to invoke ZK functionality and generate ZK proofs for your
transactions.

This guide is aimed at Midnight Network users, rather than DApp Builders.

Install Docker Desktop[​](#install-docker-desktop "Direct link to Install Docker Desktop")
------------------------------------------------------------------------------------------

If you don't have Docker, then download and install Docker for your operating system (macOS, Windows, or Linux):
<https://www.docker.com/products/docker-desktop/>

New users might need to set up an account.

Install the proof server[​](#install-the-proof-server "Direct link to Install the proof server")
------------------------------------------------------------------------------------------------

Docker hosts completed code in Images. The Midnight proof server is available as a Docker Image.

1. Inside Docker Desktop, use the search bar to locate `midnightntwrk/proof-server`.
2. Pull the tag listed for the proof server in the [compatibility matrix](/relnotes/support-matrix). Do not rely on `latest`; that tag is republished less often than the versioned tags and can lag the matrix.

After pulling the image, the code for the proof server is hosted in a Container in Docker Desktop.

Start the proof server[​](#start-the-proof-server "Direct link to Start the proof server")
------------------------------------------------------------------------------------------

Start the proof server by clicking **Run** button in the same search result, or by navigating to your
Containers and clicking the **Run** button there.

To inspect the proof server, navigate to Containers and **View Details** -- there is no action required here,
but you should see some output indicating that the server has started.

The proof server listens on port 6300 by default. Leave it there unless you also update every wallet and DApp configuration that points at it; see [Local proving](/guides/local-proving) for how to change the port.

Stop the proof server[​](#stop-the-proof-server "Direct link to Stop the proof server")
---------------------------------------------------------------------------------------

To stop the proof server, simply stop the container.

For processing transactions on the Midnight network, the proof server must be
running, so if you have stopped it, start it again now.

Your privacy[​](#your-privacy "Direct link to Your privacy")
------------------------------------------------------------

The proof server exists to protect your privacy. It makes one outbound
request, at startup, to download the public proving parameters from
<https://srs.midnight.network/>. That data is public and identical for every
user; your witness data is never part of it. After that it only listens on
its assigned port for requests from your wallet. To avoid that request on
later starts, keep the downloaded parameters in a volume. To download from
your own mirror instead, set `MIDNIGHT_PARAM_SOURCE`. See [Local proving](/guides/local-proving) for details.

[Edit this page](https://github.com/midnightntwrk/midnight-docs/edit/main/docs/guides/run-proof-server.mdx)

Last updated on **Oct 23, 2026**

[Previous

Security and best practices](/guides/security-best-practices)[Next

Prove transactions locally](/guides/local-proving)

* [Install Docker Desktop](#install-docker-desktop)
* [Install the proof server](#install-the-proof-server)
* [Start the proof server](#start-the-proof-server)
* [Stop the proof server](#stop-the-proof-server)
* [Your privacy](#your-privacy)

![Midnight Logo](/img/midnight-header-logo-dark.svg)![Midnight Logo](/img/midnight-header-logo-dark.svg)

© 2026 Input Output Global, Inc. All Rights Reserved.

Resources

* [Midnight Foundation](https://midnight.network/)
* [Dev Diaries](/blog)
* [Glacier Drop](https://www.midnight.gd/)
* [Careers](https://midnight.network/careers)

Legal

* [Cookie Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20cookie-policy.pdf)
* [Privacy Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20%20-%20Privacy%20Notice.pdf)
* [Terms and Conditions](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20-%20Website%20Terms%20of%20Use.pdf)

Social

* [![YouTube](/img/youtube.svg)](https://www.youtube.com/channel/UCy3oZ64F3FOtjZ5sZGQNgkA)
* [![X/Twitter](/img/x.svg)](https://x.com/MidnightNtwrk)
* [![Discord](/img/discord.svg)](https://discord.com/invite/midnightnetwork)
* [![LinkedIn](/img/linkedin.svg)](https://www.linkedin.com/showcase/midnight-ntwrk/)
