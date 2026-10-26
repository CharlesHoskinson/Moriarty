# WLC MSM author slides — extracted text
> Source: https://iacr.org/submit/files/slides/2024/tches/tches2024/2_54/slides.pdf
> Collected: 2026-10-26
> Published: Unknown

Capture note: scrapling PDF conversion failed; public web PDF extraction used. Screenshot fetching failed, so plotted numeric coordinates are not transcribed.

 (https://iacr.org/submit/files/slides/2024/tches/tches2024/2_54/slides.pdf)
citeturn7view0 [wordlim: 200] Crawled: last week; Content type: application/pdf; Number of pages: 24; Source: open({"ref_id":"turn5view0","lineno":0}); Total lines: 288
L0@P0: WUHAN UNIVERSITY
L1@P0: Yutian Chen, Cong Peng, Yu Dai, Min Luo and Debiao He
L2@P0: Presenter: Yutian Chen
L3@P0: Load-Balanced Parallel Implementation on GPUs
L4@P0-1:  for Multi-Scalar Multiplication Algorithmzk-SNARK and its performance bottlenecks
L5@P1: • zk-SNARK, a cryptographic primitive,
L6@P1: allows the prover to generate a proof 𝜋 for NP relation 𝑅 = {(𝑥; 𝑤): 𝑃(𝑥, 𝑤)}.
L7@P1: Performance Bottleneck:
L8@P1: 𝑷𝒓𝒐𝒗𝒆 phase
L9@P1: NTT + MSM
L10@P1-2: Fast ImplementationGPU and its application in zk-SNARK
L11@P2: GPUs are good at
L12@P2: • parallel,
L13@P2: • arithmetically intense,
L14@P2: • streaming-memory problems
L15@P2: Zprize competition aims to
L16@P2: • improve baseline performance
L17@P2: of key algorithms of zk-SNARK
L18@P2: • accelerating MSM using GPUs
L19@P2-3: • Lu et al. proposed cuZK in CHES 23Multi-Scalar Multiplication (MSM)
L20@P3: Given
L21@P3: • 𝑛 scalars 𝑘𝑖
L22@P3: • 𝑛 points 𝑃𝑖for 𝑖 ∈ 𝑛
L23@P3: MSM is to compute
L24@P3: 𝑸 = ෍𝒊=𝟏
L25@P3: 𝒏
L26@P3: 𝒌𝒊𝑷𝒊
L27@P3: 𝜆 = log2𝑟 , 𝑟 𝑖𝑠 𝑡ℎ𝑒 𝑜𝑟𝑑𝑒𝑟
L28@P3: in the worst case:
L29@P3-4: 𝒏𝝀 times PADD + (𝝀 − 𝟏) times PDBLThe Pippenger Algorithm
L30@P4: Bucket Accumulation phase
L31@P4: “bucket” point 𝑩𝒕is the sum of
L32@P4: all points with the sub-scalar
L33@P4: equaled to 𝒕
L34@P4: 1 ≤ 𝑡 ≤ 2
L35@P4: 𝑐 − 1, where 𝑐 is
L36@P4-5: the window size of subtaskThe Pippenger Algorithm
L37@P5: Bucket Aggregation phase
L38@P5: sequentially computes
L39@P5: 𝑮𝒕−𝟏 = 𝑮𝒕 + 𝑩𝒕−𝟏
L40@P5: from 𝐺2
L41@P5: 𝑐−1 = 𝐵2𝑐−1 to 𝐺1
L42@P5: finally computes
L43@P5: ෍𝒕=𝟏
L44@P5: 𝟐
L45@P5: 𝒄−𝟏
L46@P5: 𝑮𝒕
L47@P5-6: The Pippenger Algorithm is the fastest serial MSM algorithmChallenges in parallel implementations
L48@P6: • In bucket accumulation:
L49@P6: Adjust the edges
L50@P6: ①
L51@P6: ②
L52@P6: Not adjust the edges
L53@P6: ⚫ Pre-perform radix sorting based on sub-scalar
L54@P6: ⚫ Each thread processes non-overlapping data
L55@P6: Load imbalance
L56@P6: ? Load balance
L57@P6: ⚫ Using atomic operations (lock in fact)
L58@P6: ⚫ Alloc independent buckets for each thread
L59@P6: (large memory &
L60@P6-7: cannot parallelize the bucket aggregation)Challenges in parallel implementations
L61@P7: • In bucket aggregation:
L62@P7: . . .
L63@P7: round-1 round-2
L64@P7: round-2
L65@P7: 𝑐 − 1 round-3
L66@P7: Serial computational process
L67@P7: Existed parallel methods
L68@P7-8: still involved small-scale MSMOur work  Overall architecture
L69@P8:  Scalar processing and
L70@P8: point precomputation
L71@P8:  Single-point and double-point
L72@P8: addition optimization
L73@P8:  Load-balanced accumulation
L74@P8: from original points to buckets
L75@P8:  A layered parallel reduction algorithm
L76@P8-9: for the bucket aggregation Our work  Scalar processing and point precomputation
L77@P9: • Scalar conversion formula:
L78@P9: 𝑘𝑖,𝑗𝑃𝑖
L79@P9: → 𝑘෨
L80@P9: 𝑖,𝑗
L81@P9: ⋅ −1
L82@P9: 𝑠𝑖,𝑗𝑃𝑖
L83@P9: → 𝑘ത
L84@P9: 𝑖,𝑗
L85@P9: ⋅ −1
L86@P9: 𝑠𝑖,𝑗
L87@P9: ⋅ 2
L88@P9: ℎ𝑖,𝑗𝑃𝑖
L89@P9: → 𝑘ത
L90@P9: 𝑖,𝑗
L91@P9: ⋅ −1
L92@P9: 𝑠𝑖,𝑗𝑃𝑖,ℎ𝑖,𝑗
L93@P9: ′
L94@P9: ◆Convert to the signed representation
L95@P9: （digit ∈ 0,2
L96@P9: 𝑐
L97@P9: 𝑡𝑜 ∈ [−𝟐
L98@P9: 𝒄−𝟏
L99@P9: , 𝟐
L100@P9: 𝒄−𝟏
L101@P9: ]）
L102@P9: ◆Convert to the float representation 𝒌෡
L103@P9: 𝒊,𝒋
L104@P9: Precomputed
L105@P9: points
L106@P9: 𝑶𝑫𝑫
L107@P9: 𝑬𝑿𝑷
L108@P9: 𝑺𝑰𝑮𝑵
L109@P9: The mapping: (𝑘𝑖,𝑗 → 𝑘෠
L110@P9: 𝑖,𝑗)
L111@P9-10: can be also precomputedOur work  Scalar processing and point precomputation
L112@P10: • Change the way to add points into the bucket:
L113@P10: • Original: 𝑃𝑖 ⇒ 𝐵𝑘𝑖,𝑗
L114@P10: Modified: −1
L115@P10: 𝑠𝑖,𝑗
L116@P10: ⋅ 2
L117@P10: ℎ𝑖,𝑗𝑃𝑖 ⇒ 𝐵𝑘ത
L118@P10: 𝑖,𝑗
L119@P10: • Given a threshold parameter 𝜏 ∈ [1, 𝑐 − 1], store {2
L120@P10: 1𝑃𝑖
L121@P10: , 2
L122@P10: 2𝑃𝑖
L123@P10: , ⋯ , 2
L124@P10: 𝜏𝑃𝑖
L125@P10: } in GPU
L126@P10: if ℎ𝑖,𝑗 ≤ 𝜏，the point can be looked up directly
L127@P10: else, 2
L128@P10: 𝜏𝑃𝑖 can be looked up and (ℎ𝑖,𝑗 − 𝜏) times PDBL are required ≈ 1/2
L129@P10: 𝜏+1
L130@P10: probability
L131@P10: The number of buckets: from 2
L132@P10: 𝑐
L133@P10: to 2
L134@P10: 𝑐−2
L135@P10-11: but need 𝜏 ⋅ 𝑛|𝔾| bytes memory for precomputed pointsOur work  Single-point and double-point
L136@P11: addition optimization
L137@P11: Cached Jacobian 𝑋, 𝑌, 𝑍𝑍, 𝑍𝑍𝑍
L138@P11: (𝑥 =
L139@P11: 𝑋
L140@P11: 𝑍𝑍 , 𝑦 =
L141@P11: 𝑌
L142@P11: 𝑍𝑍𝑍 , 𝑍𝑍
L143@P11: 3 = 𝑍𝑍𝑍2
L144@P11: )
L145@P11: Homogeneous 𝑋, 𝑌, 𝑍
L146@P11: (𝑥 =
L147@P11: 𝑋
L148@P11: 𝑍
L149@P11: , 𝑦 =
L150@P11: 𝑌
L151@P11: 𝑍
L152@P11: )
L153@P11: • Reduce the number of multiplications in the process of
L154@P11: adding one or two original points into one bucket.
L155@P11-12: singlepointdoublepointOur work  Single-point and double-point
L156@P12: addition optimization
L157@P12: • Single-point mode:
L158@P12: based on cached Jacobian coordinate (≈ 𝟏𝟎 multiplications per PADD)
L159@P12: • Double-point mode:
L160@P12: based on homogeneous coordinate (≈
L161@P12: 𝟓𝑴+𝟐𝑺 +𝟏𝟐𝑴
L162@P12: 𝟐
L163@P12: = 𝟗. 𝟓 multiplications per PADD)
L164@P12: Optimal
L165@P12-13: coordinateOur work  Load-balanced accumulation phase
L166@P13: • Not adjust the edges,
L167@P13: i.e., keep s, e = [
L168@P13: 𝑛
L169@P13: 𝑁
L170@P13: ⋅ 𝑡𝑖𝑑,
L171@P13: 𝑛
L172@P13: 𝑁
L173@P13: ⋅ 𝑡𝑖𝑑 + 1 ),
L174@P13: and pre-allocate non-conflicting point buffers
L175@P13: • Radix sort the array 𝑘෠
L176@P13: 𝑖,𝑗
L177@P13: , 𝑖 by key 𝑘෠
L178@P13: 𝑖,𝑗
L179@P13: .𝑂𝐷𝐷
L180@P13: 𝑎ො𝑖, 𝑝𝑖 denotes the ascending array
L181@P13: • Accumulate parts of the buckets
L182@P13: into non-conflicting static buffers
L183@P13: a little similar
L184@P13: to vector
L185@P13: • Aggregate buffered points into buckets
L186@P13-14: utilize a variation ( ) of the binary searchOur work  Load-balanced accumulation phase
L187@P14: • Design of the point buffers
L188@P14: ? Allocate a buffer of 𝒎𝒊𝒏(𝟐
L189@P14: 𝒄−𝟐
L190@P14: ,
L191@P14: 𝒏
L192@P14: 𝑵
L193@P14: ) points for each thread
L194@P14: (cover all possibilities, but memory is too large & sparse!)
L195@P14: Compact buffers
L196@P14: ⇒ set the starting offset 𝒐𝒇𝒇𝒔𝒆𝒕𝒕𝒊𝒅 for the thread(𝑡𝑖𝑑) as 𝒕𝒊𝒅 + 𝐦𝐢𝐧
L197@P14: 𝒂ෝ𝒊. 𝑶𝑫𝑫 + 𝟏
L198@P14: 𝟐
L199@P14: 𝒊∈[𝒔,𝒆)
L200@P14: 𝑏𝑢𝑓𝑓𝑒𝑟 = …
L201@P14: …
L202@P14: Each thread can have at most one unused point space
L203@P14: Memory size:
L204@P14: 𝝀
L205@P14: 𝒄
L206@P14: ⋅ (𝑵 + 𝟐
L207@P14: 𝒄−𝟐
L208@P14-15: )Our work  Load-balanced accumulation phase
L209@P15: • Cost analysis
L210@P15: ➢ worse case (all scalars are identical):
L211@P15: previous accumulation: requires 𝒏 PADD
L212@P15: our accumulation: requires only (
L213@P15: 𝒏
L214@P15: 𝑵
L215@P15: + 𝑵) PADD
L216@P15: ➢ best case (each bucket corresponds to exactly 𝑛
L217@P15: 𝑁
L218@P15: points):
L219@P15: previous accumulation: requires 𝒏
L220@P15: 𝑵
L221@P15: PADD (fortunately load-balanced)
L222@P15: our accumulation: requires only 𝒏
L223@P15: 𝑵
L224@P15: (𝟏 +
L225@P15: 𝟏
L226@P15: 𝟐
L227@P15: 𝒄−𝟐
L228@P15: ) PADD
L229@P15: e.g. BLS12-381, n = 2
L230@P15: 24
L231@P15: ,
L232@P15: RTX4090(N = 2048)
L233@P15: 𝑛/(
L234@P15: 𝑛
L235@P15: 𝑁
L236@P15: + 𝑁) ≈ 𝟏𝟔𝟑𝟖 × speedup
L237@P15: e.g. 𝑐 = 16,
L238@P15: 1
L239@P15: 1+
L240@P15: 1
L241@P15: 2
L242@P15: 𝑐−2
L243@P15: ≈ 99.994%
L244@P15: If the scalar distribution is uneven,
L245@P15-16: the time complexity is greatly reducedOur work  A layered parallel reduction algorithm
L246@P16: for the bucket aggregation
L247@P16: • The subtask is to compute: 𝑸𝒋 = σ𝒊=𝟎
L248@P16: 𝟐
L249@P16: 𝒄−𝟐−𝟏 𝟐 ⋅ 𝒊 + 𝟏 𝑩𝟐⋅𝒊+𝟏 = 𝟐 ⋅ σ 𝒊 ⋅ 𝑩𝟐⋅𝒊+𝟏 − σ 𝑩𝟐⋅𝒊+𝟏
L250@P16: assign 2
L251@P16: 𝑐−3
L252@P16: threads to each subtask (complete the task in 𝑐 − 2 rounds)
L253@P16-17: Logarithmic complexityConstant-time Patch
L254@P17: • "constant-time" means the running-time is independent from the input.
L255@P17: • In bucket accumulation phase,
L256@P17: aggregating the buffered points is not real constant-time!
L257@P17: Our Patch Rule
L258@P17: to process any 4
L259@P17: consecutive data
L260@P17-18: Each thread requires exactly 2 PADD and 2 OUT (copy) operations.Constant-time Patch
L261@P18: 2 log2(𝑁 + 2
L262@P18: 𝑐−2
L263@P18: ) PADD + 2 log2 𝑁 + 2
L264@P18: 𝑐−2 − 1 OUT
L265@P18-19: … 𝑳𝒂𝒚𝒆𝒓𝒆𝒅Implementation Results
L266@P19: Environment
L267@P19: window size 𝑐 = 16
L268@P19: • For BLS12-377/BLS12-381, based on cached Jacobian, threshold 𝜏 = 6
L269@P19: • For BLS24-315, based on homogeneous, threshold 𝜏 = 15
L270@P19-20: Params SetupImplementation Results
L271@P20: • Compared to the 𝒄𝒖𝒁𝑲 [CHES23]
L272@P20: ◆ ≈ 𝟐 × speedup
L273@P20-21: ◆ Both load-balancedImplementation Results
L274@P21: • The performance improvement brought about by
L275@P21: our load balancing algorithm (on BLS24-315)
L276@P21: ◆ 𝟐𝟖. 𝟐%~𝟖𝟓. 𝟑% improvement
L277@P21: ◆ As the MSM size increases,
L278@P21-22: advantage becomes more pronouncedImplementation Results
L279@P22: • Constant-time evaluation
L280@P22: ◆ Even for our non-constant-time version,
L281@P22: the performance drop in the corner case becomes less noticeable with an increase in 𝑛
L282@P22: ◆ For our constant-time version,
L283@P22-23: the running time in the corner case is almost indistinguishable from the average caseWUHAN UNIVERSITY
L284@P23: Yutian Chen, Cong Peng, Yu Dai, Min Luo and Debiao He
L285@P23: Presenter: Yutian Chen
L286@P23: Load-Balanced Parallel Implementation on GPUs
L287@P23:  for Multi-Scalar Multiplication Algorithm

