[Global insights](/global-map)

Funds

Get involved

News

[Docs](https://docs.projectcatalyst.io/)

Back

* [Stats](/funds/13/cardano-use-cases-concept/marlowe-2025-marlowe-v2#stats)
* [About](/funds/13/cardano-use-cases-concept/marlowe-2025-marlowe-v2#about)
* [Team](/funds/13/cardano-use-cases-concept/marlowe-2025-marlowe-v2#team)

[Home](/)[Funds overview](/funds)[Fund13](/funds/13)[Cardano Use Cases: Concept](/funds/13/cardano-use-cases-concept)

[Cardano Use Cases: Concept](/funds/13/cardano-use-cases-concept)

Last updated a year ago

Share:

[Share this on X](https://x.com/intent/tweet?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&text=&via=Catalyst_onX&via=%40InputOutputHK)[Share this on Facebook](https://www.facebook.com/share.php?u=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&t=)[Share this on LinkedIn](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2)[Share this on Reddit](https://reddit.com/submit?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&title=)

Marlowe 2025: Marlowe V2
========================

[Status:**Not approved**](/search?q=&status=NotApproved)

Share:

[Share this on X](https://x.com/intent/tweet?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&text=&via=Catalyst_onX&via=%40InputOutputHK)[Share this on Facebook](https://www.facebook.com/share.php?u=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&t=)[Share this on LinkedIn](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2)[Share this on Reddit](https://reddit.com/submit?url=https%3A%2F%2Fprojectcatalyst.io%2Ffunds%2F13%2Fcardano-use-cases-concept%2Fmarlowe-2025-marlowe-v2&title=)

Problem
-------

**As the Marlowe ecosystem is put into use, it is timely to review the constructs and behaviour of the core language and, based on community feedback, to scope out and design its successor, Marlowe V2.**

Total to date
-------------

This is the total amount allocated to Marlowe 2025: Marlowe V2.

**150,000 $ADA**

Total funds requested

**183**

Total votes cast

**57.7M**

Votes yes

**43.4M**

Votes abstain

About this idea
---------------

### [GENERAL] Name and surname of main applicant

Simon Thompson

### [GENERAL] Are you delivering this project as an individual or as an entity (whether formally incorporated or not)

Entity (Incorporated)

### [GENERAL] Requested funds in ada

150000

### [GENERAL] Please specify how many months you expect your project to last (from 2-12 months)

12

### [GENERAL] Please indicate if your proposal has been auto-translated into English from another language

No

### [GENERAL] Summarize your solution to the problem (200-character limit including spaces)

**Working with the community we will collect and prioritise proposals for Marlowe V2, prototype them in the reference Agda implementation, and based on the results, design the revised language.**

### [GENERAL] Does your project have any dependencies on other organizations, technical or otherwise?

Yes

### [GENERAL] If YES, please describe what the dependency is and why you believe it is essential for your project’s delivery. If NO, please write “No dependencies.”

This project depends on the Marlowe project being sustained, and in particular on the CF 13 proposal ADD LINK to support the maintenance and enhancement of Marlowe being funded.

### [GENERAL] Will your project’s output/s be fully open source?

Yes

### [GENERAL] Please provide here more information on the open source status of your project outputs

**All the code is and will be licensed under permissive “Apache License Version 2.0”.**

### [METADATA] Horizons

Smart Contracts

### [SOLUTION] Please describe your proposed solution

**Marlowe 2025**

This is one of a set of proposals that together consolidate the Marlowe TypeScript infrastructure, enhance Marlowe with an innovative oracle protocol, developed in collaboration with leading Cardano oracle providers, and design the successor core language, Marlowe V2, working with the Marlowe developer ecosystem.

**The proposal**

As the Marlowe ecosystem is put into use, it is timely to review the constructs and behaviour of the core language and, based on community feedback, to scope out and design its successor, Marlowe V2. Working with the community we will collect and prioritise proposals for Marlowe V2, prototype them in the reference Agda implementation, and based on the results, design the revised language.

We envisage discussing improvements in a number of general categories:

* **Simplification** of aspects of the original language. This might include simplifying the “When” construct so that the contract will by default close on timeout. Generalising some constructs, such as a “When” that awaits a set of actions, could substantially simplify the expression of a common programming pattern.
* Making the language more **expressive**. This could include adding new control constructs, such as bounded loops, and primitives, e.g. for crypto operations. It would also include conceptual additions, such as sub-languages for time arithmetic or token minting.
* Improving the **scalability** of the language: an example of this would be to change the behaviour of a contract on close to optimise performance with larger numbers of contract participants.
* Addressing the **security** of the language, which could be improved by checking types of values, so that e.g. addition only takes place between the same “kind” of number.

Former members of the Marlowe team, including Pablo Lamela and Brian Bush, have agreed to act as reviewers of proposals made during the project.

**The process**

The work will proceed in a series of steps.

1. Develop a preliminary draft list of proposed changes, based on the categories of simplification, expressiveness, scalability and security.
2. Use this draft as the basis for community consultation in a short series of online workshops. Participants would be asked first for additional proposals, and then to prioritise the resulting suggestions. A report of this constitutes **Milestone 1**.
3. Implement prototypes of the high(est) priority suggestions using the Agda model of Marlowe. If that proves to be too difficult it would be possible to default to using the Haskell interpreter.
4. Based on the implementation, write a report containing a preliminary assessment of the proposals. The Agda code and report constitute **Milestone 2**.
5. At this point the community, would be consulted for their views of the report and it should be revised on the basis of that.
6. On the basis of the consultation, write the design document.
7. The report should include an assessment of the impact of these changes on the existing infrastructure (Runtime, Playground etc). That revised design document, plus revised Agda code, constitute **Milestone 3**.
8. Write outline proposal to fund the implementation of the changes, to be included in the final report.
9. Deliver final report: **Final Milestone**.

### [IMPACT] Please define the positive impact your project will have on the wider Cardano community

Marlowe is an existing project in the Cardano ecosystem, initially conceived as a financial DSL providing a local account-based model like Ethereum, with the potential to evolve into a smart contract technology complementary to PlutusTx, Aiken and Scalus. etc. The introduction of marlowe-ts-sdk and the Marlowe Runtime, which integrates with familiar REST APIs and frameworks, makes it possible to build end-to-end DApps incorporating Marlowe on- and off-chain together with traditional web frameworks.

Other proposals in the Marlowe 2025 portfolio address specific enhancements (e.g. in the TypeScript SDK) and capabilities (such as oracles) that will deliver immediate, short-term results. This proposal takes a longer-term perspective to examine the core technology at the heart of the Marlowe Platform, to review its strengths and weaknesses, and to design and plan the next version of Marlowe, working with the wider Marlowe community.

**Working with the community**

In the project we will work closely with community members to understand their priorities for the evolution of the language and its ecosystem, and in particular to elicit concrete suggestions for simplifying and enhancing Marlowe. We will do this through a series of online workshops to elicit input and discussion on concrete suggestions for language improvement, and so to ensure the relevance of the work done in the project. In doing this we will leverage our previous experience of participating in local and international Cardano summits, meetups, and workshops, and, currently, engaging with the Marlowe Special Interest Group in Discord.

**Beneficiaries**

The project will benefit *existing users of the language*, who will be able to plan work and the evolution of their systems to take account of changes that they can expect to be delivered in the medium term. The work should also *broaden the base of Marlowe usage*, through enhancing the expressiveness and scalability of the system. It will also *bring users back to Marlowe* who might have been deterred from using it by some of the complexities of the existing language, as well as those who were unable fully to exploit it because of the lack of some features.

**Impact and dissemination**

We will record the engagement of the community in the workshops, and in a final online dissemination workshop to report the work more widely to users in the Cardano ecosystem.

### [CAPABILITY & FEASIBILITY] What is your capability to deliver your project with high levels of trust and accountability? How do you intend to validate if your approach is feasible?

**Unique Experience in Developing Marlowe Technology**

Simon Thompson is one of the original designers of the Marlowe language, and the other applicants are core maintainers of the Marlowe language and core team members of the Marlowe Project under IOG.

**Review and design processes will be public and open-source**

The review will be community-based, and the final report, together with the Agda implementations will be public and open-source. This approach provides an easy way for the Catalyst team and the Cardano community to evaluate the progress at each step of the process.

### [PROJECT MILESTONES] What are the key milestones you need to achieve in order to complete your project successfully?

**Consultation phase**

**Outputs:** report evaluating Marlowe V1 and outline proposals for Marlowe V2, based on the ecosystem consultation

**Acceptance criteria:** the report contains evidence of broad community consultation, and a range of suggestions from the community.

**Evidence of completion:** availability of report on Marlowe website.

**Implementation phase**

**Outputs:** Agda implementation of proposal prototypes, and a report on their evaluation.

**Acceptance criteria:** the implementation covers the highest priority proposals for Marlowe V2, as evidenced by support from phase 1.

**Evidence of completion:** availability of report on Marlowe website, and Agda implementation on Marlowe github repository.

**Design phase**

**Outputs:** Final design document and revised Agda code

**Acceptance criteria:** the design reflects the consultation in phase 1 and its evaluation in the implementation phase, phase 2.

**Evidence of completion:** availability of design on Marlowe website, and revised Agda implementation on Marlowe github repository.

**Project Close-out Report and Video.**

### [RESOURCES] Who is in the project team and what are their roles?

Simon Thompson, who is one of the original designers of Marlowe, will lead the project, planning the work, coordinating interactions with the community, taking part in the prototyping activity in Agda, and taking the lead in writing the design of Marlowe V2. He will be responsible for delivering the milestones, and reporting in general.

Tomasz Rybarczyk, who was an experienced member of the Marlowe team for a number of years, and who is part of the Marlowe continuity team, will take part in the protyping activity in Agda, and will take the lead on assessing the impact of language change proposals on the Marlowe Platform.

Other former members of the IOG Marlowe team (including Pablo Lamela) have agreed to act as reviewers of the proposals for Marlowe V2.

### [BUDGET & COSTS] Please provide a cost breakdown of the proposed work and resources

The budget consists of payment to the researcher and developer for 100 days work (50 days each) over the period of the project at the cost of ₳1500/day.

### [VALUE FOR MONEY] How does the cost of the project represent value for money for the Cardano ecosystem?

Investment in planning the next version of Marlowe will ensure that when the next version is implemented, effort will be directed appropriately and effectively, since the changes will have been prototyped in advance, and validated through community involvement in the process.

Our costings are based on prevailing rates for researchers and developers in Europe, and we are charging the same rate for all contributors. We have a unique combination of skills in the team: one of the designers of the original Marlowe language, plus a team familiar with the systems of the Marlowe Platform, from the Marlowe validator. The team is highly experienced having previously worked as members of the core Marlowe team; there is therefore no time needed for the team to get up to speed: we hit the ground running.

+ Read more

Team
----

[![Simon Thompson](/_next/image?url=https%3A%2F%2Fcardano.ideascale.com%2Fa%2Fuser-files%2Fmember%2F85435%2Fuploaded-avatars%2Favatar-a10ae8.jpg&w=128&q=75)

Simon Thompson](/proposers/simonthompsonmarlowe)[![Tomasz Rybarczyk](/_next/image?url=https%3A%2F%2Fcardano.ideascale.com%2Fa%2Fuser-files%2Fmember%2F69688%2Fuploaded-avatars%2Favatar-3cfa65.jpeg&w=128&q=75)

Tomasz Rybarczyk](/proposers/paluh)

Website:<https://github.com/marlowe-lang>

Follow us
---------

About
-----

[The project](/blog)[Docs](http://docs.projectcatalyst.io/)[Submit an idea](/get-involved/submit-an-idea)[Browse funded ideas](/search)[Media kit](/media-kit)

Contact
-------

[FAQ & tech support](https://catalystiog.zendesk.com/hc/en-us/requests/new)[Contact moderator](https://catalystiog.zendesk.com/hc/en-us/requests/new)

Resources
---------

[Cardano.org](https://www.cardano.org)[Cardano Foundation](https://cardanofoundation.org/)[Essential Cardano](https://www.essentialcardano.io/)

Legal
-----

[Privacy Policy](https://docs.projectcatalyst.io/open-funding/fine-print/privacy-policy)[Terms and Conditions](https://docs.projectcatalyst.io/open-funding/fine-print/terms-and-conditions)

Sign up to receive news and updates on all things Project Catalyst
------------------------------------------------------------------

We collect personal data in accordance with our [Privacy Policy](https://docs.projectcatalyst.io/open-funding/fine-print/privacy-policy)

Subscribe

© 2026 Cardano Foundation · All Rights Reserved