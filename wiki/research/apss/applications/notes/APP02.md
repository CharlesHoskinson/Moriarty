---
id: apss.applications.app02
title: "Combinators versus a fixed product catalog"
status: draft
source_id: APP02
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Combinators versus a fixed product catalog

Source: [Composing contracts: an adventure in financial engineering](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/contracts-icfp.pdf). Publication/version: 2000-08-23 (manuscript); ICFP September 2000. Retrieved 2026-09-19T16:51:07.943034+00:00.

SHA-256: `dc92534929a60841d58858b44a73ad6b53d788935142353300f46b4e6cf0829b`. Capture: [`captures/APP02.pdf`](../../../../attachments/apss/applications/APP02.pdf). Independence key: `peyton-jones-eber-seward`.

Evidence locator: PDF pp. 1–2; introduction and sections 2.1–2.2. PDF pages visually read: 1, 2.

**Source claims.** Peyton Jones, Eber and Seward propose defining financial contracts compositionally rather than expanding a fixed catalog. The paper describes a Haskell combinator library and compositional valuation semantics. The examples combine payments and reverse parties' rights/obligations with `give`.

**Moriarty inference.** ACTUS/DeFi coverage should qualify primitives and libraries without becoming a deployment allowlist. A new composition of supported constructors should be expressible and analyzable without maintainer registration.

**Limitation.** Valuation semantics is not execution correctness, custody enforcement or a ledger correspondence proof. The historical examples use representations such as floating-point dates that should not be copied into an exact bounded financial kernel. The application is the contract description; enforcing and funding that description remains a separate obligation. Text extraction damages some glyphs; the reviewed page images establish the actual title and argument.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
