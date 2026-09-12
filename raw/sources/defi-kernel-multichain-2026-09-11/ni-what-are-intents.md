> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# What are Intents?

> Define desired outcomes instead of managing execution steps

An intent expresses **what you want** to achieve, **not how to get it**. Instead of writing code to bridge assets, find quotes, and execute swaps, you submit an intent such as *"I have 1 ETH on Ethereum, I want USDC on Arbitrum"*.

<div style={{ position: "relative", paddingBottom: "56.25%", height: 0, overflow: "hidden" }}>
  <iframe src="https://www.youtube.com/embed/mOGD2gzZJqE" title="NEAR 101: Introduction to Intents" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerPolicy="strict-origin-when-cross-origin" allowFullScreen style={{ position: "absolute", top: 0, left: 0, width: "100%", height: "100%" }} />
</div>

***

## How it works

<Columns cols={2}>
  <Steps>
    <Step title="You create an intent">
      Specify your desired outcome: the tokens you have, the tokens you want, and where you want them delivered.
    </Step>

    <Step title="Market makers compete">
      Multiple market makers bid to fulfill your intent (quote request), competing on price, speed, and execution quality.
    </Step>

    <Step title="Smart contracts verify">
      Approved quotes are submitted to a smart contract ensuring atomic execution; your swap either completes fully or you get automatically refunded.
    </Step>

    <Step title="You receive your tokens">
      Your requested tokens arrive at the destination address you choose, secured by on-chain verification.
    </Step>
  </Steps>

  <div className="intent-diagram-desktop-only" style={{ alignItems: "center", height: "100%" }}>
    <img height="100%" src="https://mintcdn.com/defuselabsltd/7Xebxgu7H2OpX1la/images/diagrams/what-are-intents.png?fit=max&auto=format&n=7Xebxgu7H2OpX1la&q=85&s=b6dc50f7f81fc45f06162f00032e3c9b" alt="NEAR Intents Architecture" data-path="images/diagrams/what-are-intents.png" />
  </div>
</Columns>

***

## Key benefits

<CardGroup cols={2}>
  <Card title="Simple Integration" icon="sparkles" horizontal>
    Express desired outcomes instead of managing individual steps
  </Card>

  <Card title="Fast Settlement" icon="bolt" horizontal>
    Solvers compete to fill orders quickly
  </Card>

  <Card title="Use Your Wallets" icon="wallet" horizontal>
    Use your existing wallets on any supported chain
  </Card>

  <Card title="Competitive Pricing" icon="chart-line" horizontal>
    Multiple solvers compete on price and execution quality
  </Card>

  <Card title="Automatic Refunds" icon="shield-check" horizontal>
    Funds are returned if the intent cannot be executed
  </Card>

  <Card title="Non-Custodial Security" icon="lock-keyhole" horizontal>
    You maintain control of your assets throughout the process
  </Card>
</CardGroup>

***

## Learn more

<AccordionGroup>
  <Accordion title="How does NEAR Intents ensure security?">
    NEAR Intents uses atomic execution enforced by smart contracts on NEAR Protocol. Transactions either complete with all conditions met, or they are reverted and funds returned to the refund address. All transactions are verified on-chain before settlement.
  </Accordion>

  <Accordion title="What chains and tokens are supported?">
    NEAR Intents supports major blockchains including Ethereum, NEAR, and others. The list of supported chains and tokens is continuously expanding. Check our [Chain Support page](/resources/chain-support) for the current list.
  </Accordion>

  <Accordion title="What are the fees?">
    The 1Click API charges 0.2% (20 basis points) for unauthenticated requests. Authenticated users (with JWT tokens) pay no platform fees—only network gas costs and market maker spreads. [Learn more about fees](/resources/fees).
  </Accordion>

  <Accordion title="How do market makers compete?">
    When you create an intent, it is broadcast to multiple market makers who bid to fulfill it. They compete on price, speed, and execution quality. The best execution path is selected while ensuring the intent's terms are met.
  </Accordion>
</AccordionGroup>

***

## Additional Resources

<Card title="Read the deep dive" icon="book-open" href="https://www.near.org/blog/unpacking-near-intents-a-deep-dive">
  Explore the technical architecture and design philosophy behind NEAR Intents in this comprehensive blog post.
</Card>

<Card title="Watch NEAR 101: Introduction to Intents" icon="youtube" href="https://www.youtube.com/watch?v=mOGD2gzZJqE">
  Learn the basics of what NEAR Intents are and how they work in this video overview.
</Card>
