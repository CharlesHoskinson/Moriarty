# Optimizing Multi-Scalar Multiplication Over Fixed Bases — fallback source capture
> Source: https://www.preprints.org/manuscript/202604.0045
> Collected: 2026-10-26
> Published: 2026-04-02

Capture note: scrapling returned an Akamai interstitial (fixed-base-2026.md); the following is the public page text obtained through the web tool fallback.

Optimizing Multi-Scalar Multiplication Over Fixed Bases[v1] | Preprints.org (https://www.preprints.org/manuscript/202604.0045)
citeturn4view0 [wordlim: 200] Crawled: today; Content type: text/html; Source: open({"ref_id":"https://www.preprints.org/manuscript/202604.0045","lineno":null}); Total lines: 387
L0:   * 【0†Instructions for Authors】
L1:   * 【1†About】
L2:   * 【2†Help Center】
L3:   * 【3†Blog】
L4:   * 【4†10th Anniversary】
L5:   * 【5†Author Services】
L6: 
L7: [Input: Search]
L8: 
L9: Search
L10: 
L11: Log in
L12: 
L13: [Input][Input]
L14: 
L15: Version 1
L16: 
L17: Submitted:
L18: 
L19: 31 March 2026
L20: 
L21: Posted:
L22: 
L23: 02 April 2026
L24: 
L25: You are already at the latest version
L26: 
L27: Subscription
L28: 
L29: Notify me about updates to this article or when a peer-reviewed version is published.
L30: 
L31: [Input: Your Email]Subscribe
L32: 
L33: Preprint
L34: 
L35: Article
L36: 
L37: This version is not peer-reviewed.
L38: #
L39: 
L40: Optimizing Multi-Scalar Multiplication Over Fixed Bases
L41: 
L42: 【6†Saulius Grigaitis†sciprofiles.com】^{  *}
L43: 
L44: 【6†Saulius Grigaitis†sciprofiles.com】^{  *}
L45: 
L46: Show more
L47: 
L48: [Input][Input]
L49: 
L50: Version 1
L51: 
L52: Submitted:
L53: 
L54: 31 March 2026
L55: 
L56: Posted:
L57: 
L58: 02 April 2026
L59: 
L60: You are already at the latest version
L61: ######
L62: 
L63: Abstract
L64: This work investigates multi-scalar multiplication (MSM) over a fixed base for small input sizes, where classical large-scale optimizations are less effective. We propose a novel variant of the Pippenger-based bucket method that enhance performance by using additional precomputation. In particular, our approach extends the BGMW method by introducing structured precomputations of point combinations, enabling the replacement of multiple point additions with table lookups.
L65: We further generalize this idea through chunk-based precomputation, allowing flexible trade-offs between memory usage and runtime performance. Experimental results demonstrate that the proposed variants significantly outperform the Fixed Window method for small MSM instances, achieving up to 3× speedup under practical memory constraints. These results challenge the common assumption that bucket-based methods are inefficient for small MSMs.
L66: ###### Keywords:
L67: 
L68: multi-scalar multiplication
L69: 
L70: ;
L71: 
L72: elliptic curve cryptography
L73: 
L74: ;
L75: 
L76: pippenger’s algorithm
L77: 
L78: ;
L79: 
L80: bucket method
L81: 
L82: ;
L83: 
L84: precomputation
L85: 
L86: ;
L87: 
L88: fixed base scalar multiplication
L89: 
L90: ######
L91: 
L92: Subject:
L93: 
L94: 【7†Computer Science and Mathematics】  -   【8†Security Systems】
L95: ## 1. Introduction
L96: 
L97: Multi-scalar multiplication (MSM) is a fundamental operation in elliptic curve cryptography and underpins many cryptographic protocols. While MSM has been extensively studied, most recent advances focus on large-scale instances, where Pippenger’s bucket method and its variants dominate in performance.
L98: In contrast, small MSM instances exhibit different characteristics and allow alternative optimization strategies. In this regime, precomputation becomes a powerful tool, as its cost can be amortized over relatively small inputs. This opens the door to algorithms that trade memory for improved runtime efficiency.
L99: However, classical bucket-based methods such as the BGMW approach are often considered suboptimal for small MSMs. In this work, we challenge this assumption by introducing novel precomputation technique that reduce the number of required group operations.
L100: We propose a new variant that extends the BGMW method and demonstrate that, when combined with carefully designed precomputation strategies, bucket-based methods can outperform widely used approaches such as the Fixed Window method in the small MSM regime.
L101: ## 2. Previous Work
L102: 
L103: Multi-scalar multiplication (MSM) over fixed base has been extensively studied over several decades. Recent advances have primarily targeted large-scale MSM instances, typically of size at least ${2}^{20}$, where state-of-the-art algorithms are based on variants of Pippenger’s bucket method [【9†1】].
L104: In contrast, smaller MSM instances admit different optimization strategies, most notably through the use of precomputation. In this regime, the precomputation cost is often proportional to the MSM size, allowing relatively modest overhead. Furthermore, smaller instances enable significantly larger precomputation per point, creating opportunities for more sophisticated algorithms that trade memory for improved runtime performance.
L105: 
L106: This chapter reviews classical MSM algorithms.
L107: ### 2.1. Shamir’s Trick
L108: 
L109: Shamir’s Trick [【10†2】] computes a double-scalar multiplication $S={k}_{0}{P}_{0}+{k}_{1}{P}_{1}$ using a minimal precomputation table consisting of $\mathcal{O}$, ${P}_{0}$, ${P}_{1}$, and ${P}_{0}+{P}_{1}$. Only the latter requires explicit precomputation.
L110: The scalars are processed simultaneously in binary form from the most significant bit to the least significant bit. At each step, the accumulator is doubled, and a precomputed value is conditionally added based on the current bit pair. This method achieves an efficient trade-off between precomputation and runtime cost.
L111: ### 2.2. Straus’s Method
L112: 
L113: Straus’s method [【11†3】] generalizes Shamir’s Trick to n scalars:
L114: 
L115: $$S=\sum _{i=0}^{n-1}{k}_{i}{P}_{i}.$$
L116: 
L117: The algorithm processes all scalars in parallel and relies on a precomputation table containing all possible sums of subsets of $\{{P}_{0},\dots ,{P}_{n-1}\}$. At each bit position, the accumulator is doubled and the subset sum corresponding to active bits is added.
L118: While computationally efficient—requiring $k-1$ doublings and $k-1$ additions for k-bit scalars—the method suffers from exponential precomputation complexity, rendering it practical only for very small n.
L119: ### 2.3. Bos-Coster Method
L120: 
L121: The Bos-Coster method [【12†4】] is a recursive MSM algorithm that iteratively reduces the largest scalars. At each step, the two largest scalars ${k}_{0}>{k}_{1}$ are replaced as
L122: 
L123: $${k}_{0}{P}_{0}+{k}_{1}{P}_{1}\phantom{\rule{0.277778em}{0ex}}\to \phantom{\rule{0.277778em}{0ex}}({k}_{0}-{k}_{1}){P}_{0}+{k}_{1}({P}_{0}+{P}_{1}).$$
L124: Each iteration requires one scalar subtraction and one group addition. The process continues until a single scalar remains. The method effectively reduces scalar sizes while increasing point reuse, making it attractive in settings where additions are significantly cheaper than scalar operations.
L125: ### 2.4. Fixed Window Method
L126: 
L127: The Fixed Window method [【13†5】] accelerates scalar and multi-scalar multiplication through per-point precomputation. For a window size w, each point ${P}_{i}$ is expanded into
L128: 
L129: $$\{{P}_{i},2{P}_{i},\dots ,({2}^{w}-1){P}_{i}\}.$$
L130: Scalars are partitioned into w-bit chunks, and each chunk selects a precomputed multiple. The contributions are combined using appropriate doublings. The method achieves a predictable trade-off between precomputation cost $O\left(n{2}^{w}\right)$ and runtime efficiency.
L131: ### 2.5. Sliding Window Method
L132: 
L133: The Sliding Window method [【13†5】] improves upon the fixed window approach by dynamically selecting windows. Instead of fixed partitions, the algorithm scans the scalar representation and processes a window beginning with a non-zero bit.
L134: 
L135: This reduces the number of additions by skipping zero segments, making the method particularly effective for scalars with low Hamming weight.
L136: ### 2.6. Pippenger’s Bucket Method
L137: 
L138: For large-scale MSM, Pippenger’s bucket method and its variants are widely regarded as the most efficient approaches [【14†6】]. The central idea is to group points according to scalar digits and accumulate them into buckets.
L139: In the windowed variant, scalars are decomposed into base-${2}^{w}$ digits. For each window, points are assigned to ${2}^{w}-1$ buckets based on their digit values. After bucket accumulation, the result is computed using a Horner-like scheme, followed by appropriate doublings between windows.
L140: 
L141: This approach significantly reduces the number of required scalar multiplications by replacing them with structured additions.
L142: ### 2.7. BGMW Method
L143: 
L144: The BGMW method [【15†7】] extends Pippenger’s approach by incorporating structured precomputation. For each point ${P}_{i}$, a table of size h is constructed:
L145: 
L146: $$\{{q}^{j}{P}_{i}\mid i=0,\dots ,n-1,\phantom{\rule{0.277778em}{0ex}}j=0,\dots ,h-1\},$$
L147: 
L148: resulting in $O\left(nh\right)$ precomputed points.
L149: This moderate precomputation cost enables efficient bucket accumulation and improves data locality. However, a key characteristic of BGMW is the existence of an optimal parameter regime: increasing precomputation (e.g., by reducing the window size w) does not necessarily improve performance and even degrade it if w is below the optimal.
L150: This behavior contrasts with fixed-window methods, where larger precomputation typically yields improvements. Although several extensions of BGMW have been proposed [【14†6】], they are not performant for scalar multiplication and small MSMs.
L151: ## 3. Pippenger’s Bucket Method for Small MSMs
L152: 
L153: The bucket method improves performance by first aggregating points within each bucket and subsequently multiplying the accumulated sum by the corresponding bucket index. This strategy is particularly effective for large MSM instances, where the expected number of points per bucket grows with the MSM size n. In contrast, for small MSM instances, bucket occupancy is low, and the resulting performance gains are limited.
L154: The BGMW method addresses this limitation by increasing the number of points per bucket through precomputation for Pippenger method. In particular, points corresponding to the same bucket index across different windows can be aggregated. This is enabled by the BGMW precomputation, which provides, for each point, its representation across all windows.
L155: A straightforward attempt to further increase bucket occupancy would be to decrease the window size w. However, this approach is not effective, as the BGMW method admits an optimal window size. Reducing w below this optimum increases the total number of group operations and consequently degrades overall performance.
L156: The main contribution of this work is the introduction of additional precomputation techniques for the Pippenger method, building upon the precomputations proposed in the BGMW method, which further improve performance for small MSM instances. In particular, the proposed variant outperform existing precomputation-based methods, such as the Fixed Window method, for small MSMs.
L157: ### 3.1. The Proposed Variant
L158: 
L159: The primary goal of the proposed method is to reduce the number of point additions required during bucket accumulation. For small MSM instances and moderate window sizes w: some buckets contain zero or one point, while other buckets contain multiple points. For such buckets with more than one point, point additions can be reduced by replacing explicit additions with lookups into precomputation tables containing sums of point pairs.
L160: More precisely, if a bucket contains two points, their sum can be obtained via a single lookup, provided that the corresponding pair has been precomputed. For buckets containing three points, two additions can be replaced by one lookup and one addition, and so on. In principle, one could precompute all possible point pairs; however, this becomes prohibitively expensive for large n or small w. To address this, we restrict precomputation to chunks of points.
L161: This significantly reduces memory requirements while preserving most of the performance benefits. The resulting trade-off is well suited for scenarios where memory usage is constrained.
L162: The variant proceeds as follows. First, the standard BGMW precomputation of size $nk/w$ is generated. Next, the resulting BGMW precomputation is used to precompute additional values of the form ${P}_{i}+{P}_{j}$ for indices
L163: 
L164: $$i,j\in \bigcup _{d=0}^{{\displaystyle \frac{nk}{tw}}-1}\{dt,dt+1,\dots ,(d+1)t-1\},\phantom{\rule{1.em}{0ex}}i\ne j,$$
L165: 
L166: where w denotes the window size in bits, n is the MSM size, k is the scalar size in bits, and t is the chunk size.
L167: The resulting algorithm follows the standard BGMW procedure, with modifications to the bucket accumulation phase. During accumulation, the algorithm scans each bucket and replaces pairs of points with their corresponding precomputed sums whenever possible. Increasing the chunk size t enlarges the set of available precomputed pairs, thereby reducing the number of explicit point additions.
L168: ## 4. Experiments
L169: 
L170: Authors of MSM algorithms often fork existing implementations, modify them, and publish the results. For example, [【14†6】] forked the `BLST` [【16†8】] library and released a modified version [【17†9】]. While this enables rapid prototyping, such implementations are typically tightly coupled to a specific elliptic curve cryptography (ECC) library, making fair comparisons across different libraries difficult. Moreover, these repositories are often not maintained, limiting reproducibility.
L171: To address these issues, the author initiated the `Rust-KZG` project [【18†10】], a unified framework supporting multiple ECC backends, including `Arkworks` [【19†11】], `BLST` [【16†8】], `Constantine` [【20†12】], `MCL` [【21†13】], and `ZKCrypto` [【22†14】]. It integrates MSM implementations under a common interface, enabling fair and reproducible comparisons. The source code for the methods proposed in this work will be publicly released upon acceptance of the corresponding paper.
L172: `Rust-KZG` is implemented in Rust, providing memory safety and performance comparable to C, and is already used by other researchers [【23†15】].
L173: 
L174: The proposed methods are compared against the Fixed Window method, specifically the optimized `wbits` implementation from `BLST`. Both the proposed variant and `wbits` use batched addition. Experiments are conducted on the widely used `BLS12_381` curve [【24†16】], using the same backend to ensure consistency.
L175: All experiments are performed on a system with an AMD Ryzen 9 5950X CPU, 64 GB of RAM, and Ubuntu 24.04, using the `Criterion` benchmarking library.
L176: ### 4.1. Experiment Results
L177: 
L178: The performance of the Fixed Window method improves as the window size increases, unless further increases no longer reduce the number of scalar windows. However, each increase doubles the size of the precomputation table, leading to poor scalability with respect to memory.
L179: In contrast, the proposed variant inherit from the BGMW method the property that an optimal window size exists. Deviating from this optimal value degrades performance, even if the precomputation size increases. Furthermore, the proposed variant introduce an additional parameter, the chunk size (a multiplier of n), resulting in an optimal combination of window size and chunk size for a given memory budget.
L180: Due to the fundamentally different precomputation strategies, direct comparison with the Fixed Window method is not straightforward, as matching precomputation sizes is often not possible. Therefore, the proposed variant are compared using configurations with equal or smaller precomputation size than the Fixed Window method.
L181: The experimental results for the proposed variant indicate that the proposed variant achieves up to almost 3X performance improvements over the Fixed Window method for small MSM sizes. In particular, the runtime is reduced by 63.05% for MSM of size ${2}^{1}$, 47.32% for size ${2}^{2}$, 32.31% for size ${2}^{3}$, 24.44% for size ${2}^{4}$, and 11.41% for size ${2}^{5}$.
L182: For MSM of size ${2}^{6}$, the proposed variant reaches very similar performance (only -0.77% difference) compared to the Fixed Window method.
L183: It should be noted that these improvements are achieved under specific memory budgets for precomputation, as detailed in the corresponding table. The proposed variant begins to outperform the Fixed Window method only when a sufficient memory budget for precomputation is available. However, the required memory remains relatively modest: 18.43 KB for MSM of size ${2}^{1}$, 48.96 KB for size ${2}^{2}$, 176.26 KB for size ${2}^{3}$, 665.86 KB for size ${2}^{4}$, and 2.59 MB for size ${2}^{5}$.
L184: Furthermore, the proposed variant exhibits both globally optimal parameter settings for each MSM size and locally optimal settings for specific memory budgets. For example, for MSM of size ${2}^{2}$, the globally optimal configuration uses a window size of 4 and a chunk size of 64, resulting in a precomputation table of approximately 3.16 MB. However, if the available memory is limited to 1 MB, the optimal configuration changes to a window size of 4 and a chunk size of 9.
L185: The chunks parameter values are divided by n in the tables below. All reported execution times are measured in milliseconds.
L186: 【25†Image†pub.mdpi-res.com】
L187: 
L188: Table 1. MSM size ${2}^{1}$ Fixed-Window vs. The Proposed Variant.
L189: 
L190: Table 1. MSM size ${2}^{1}$ Fixed-Window vs. The Proposed Variant.
L191: 
L192: 【26†Image: Preprints 205962 i001】
L193: 
L194: 【25†Image†pub.mdpi-res.com】
L195: 
L196: Table 2. MSM size ${2}^{2}$ Fixed-Window vs. The Proposed Variant.
L197: 
L198: Table 2. MSM size ${2}^{2}$ Fixed-Window vs. The Proposed Variant.
L199: 
L200: 【27†Image: Preprints 205962 i002】
L201: 
L202: 【25†Image†pub.mdpi-res.com】
L203: 
L204: Table 3. MSM size ${2}^{3}$ Fixed-Window vs. The Proposed Variant.
L205: Table 3. MSM size ${2}^{3}$ Fixed-Window vs. The Proposed Variant.
L206: 
L207: 【28†Image: Preprints 205962 i003】
L208: 
L209: 【25†Image†pub.mdpi-res.com】
L210: 
L211: Table 4. MSM size ${2}^{4}$ Fixed-Window vs. The Proposed Variant.
L212: 
L213: Table 4. MSM size ${2}^{4}$ Fixed-Window vs. The Proposed Variant.
L214: 
L215: 【29†Image: Preprints 205962 i004】
L216: 
L217: 【25†Image†pub.mdpi-res.com】
L218: 
L219: Table 5. MSM size ${2}^{5}$ Fixed-Window vs. The Proposed Variant.
L220: 
L221: Table 5. MSM size ${2}^{5}$ Fixed-Window vs. The Proposed Variant.
L222: 
L223: 【30†Image: Preprints 205962 i005】
L224: 【25†Image†pub.mdpi-res.com】
L225: 
L226: Table 6. MSM size ${2}^{6}$ Fixed-Window vs. The Proposed Variant.
L227: 
L228: Table 6. MSM size ${2}^{6}$ Fixed-Window vs. The Proposed Variant.
L229: 
L230: 【31†Image: Preprints 205962 i006】
L231: ## 5. Conclusions
L232: 
L233: This work presented a novel variant of the Pippenger-based MSM method that enhance performance for small MSM instances. The proposed approach builds upon the BGMW framework and introduces additional structured precomputation, enabling the replacement of multiple point additions with efficient table lookups.
L234: The experimental results show that the proposed variants consistently outperform the Fixed Window method for small MSM sizes, achieving up to approximately 3× speedup under realistic memory constraints. Importantly, these improvements are obtained without excessive memory requirements, making the approach practical.
L235: The results also demonstrate that bucket-based methods, when augmented with carefully designed precomputation, are highly competitive even in regimes where they are traditionally considered inefficient.
L236: ## 6. Further Research
L237: The author is currently working on several extensions of the proposed variant. First, the method can be generalized by increasing the number of precomputed combinations, including triples, quadruples, and higher-order groupings of points. While this may further improve runtime performance, it introduces additional memory and computational overhead during precomputation.
L238: Nevertheless, this approach yields performance gains for very small MSM instances and even for single-scalar multiplication, while still maintaining moderate memory requirements.
L239: Second, additional strategies for increasing bucket occupancy, such as the use of Booth encoding and related techniques, further improve performance.
L240: 
L241: Third, alternative chunk layouts increase performance. For example, overlapping chunks could enable more group operations to be replaced by lookup operations.
L242: 
L243: Finally, hybrid approaches that combine multiple MSM techniques, including representations based on wNAF, may provide further performance improvements.
L244: ## References
L245:   1. Pippenger, N. On the Evaluation of Powers and Related Problems. SIAM Journal on Computing 1976, 9, 230–250. [【32†Google Scholar†scholar.google.com】] [【33†CrossRef†doi.org】]
L246:   2. Möller, B. Algorithms for multi-exponentiation. In Proceedings of the International workshop on selected areas in cryptography; Springer, 2001; pp. 165–180. [【34†Google Scholar†scholar.google.com】]
L247:   3. Straus, E.G. Addition chains of vectors (problem 5125). American Mathematical Monthly 1964, 70, 16. [【35†Google Scholar†scholar.google.com】]
L248:   4. Bos, J.; Coster, M. Addition chain heuristics. In Proceedings of the Conference on the Theory and Application of Cryptology; Springer, 1989; pp. 400–407. [【36†Google Scholar†scholar.google.com】]
L249:   5. Hankerson, D.; Menezes, A.; Vanstone, S. Guide to Elliptic Curve Cryptography; Springer, 2004. [【37†Google Scholar†scholar.google.com】]
L250:   6. Luo, G.; Fu, S.; Gong, G. Speeding up multi-scalar multiplication over fixed points towards efficient zksnarks. IACR Transactions on Cryptographic Hardware and Embedded Systems 2023, 358–380. [【38†Google Scholar†scholar.google.com】] [【39†CrossRef†doi.org】]
L251:   7. Brickell, E.F.; Gordon, D.M.; McCurley, K.S.; Wilson, D.B. Fast exponentiation with precomputation. In Proceedings of the Workshop on the Theory and Application of of Cryptographic Techniques; Springer, 1992; pp. 200–207. [【40†Google Scholar†scholar.google.com】]
L252:   8. Supranational. blst: Multilingual BLS12-381 signature library. https://github.com/supranational/blst, 2020–2026. Accessed: 2026-03-06.
L253:   9. LuoGuiwen. MSM_blst: Multi Scalar Multiplication over the BLS12-381 curve utilizing blst. https://github.com/LuoGuiwen/MSM_blst, 2023–2026. Accessed: 2026-03-06.
L254:   10. Grandine. rust-kzg: A Multi-Backend KZG and MSM Framework. https://github.com/grandinetech/rust-kzg, 2020–2025. Accessed 2026. (accessed on 2026).
L255:   11. arkworks contributors. arkworks-rs: Rust ecosystem for zkSNARK programming. https://github.com/arkworks-rs, 2022–2026. Accessed: 2026-03-06.
L256:   12. mratsim. Constantine: High-performance cryptography stack for verifiable computation and blockchain protocols. https://github.com/mratsim/constantine, 2024–2026. Accessed: 2026-03-06.
L257:   13. Shigeo Mitsunari (herumi). MCL: A portable and fast pairing-based cryptography library. https://github.com/herumi/mcl, 2015–2026. Accessed: 2026-03-06.
L258:   14. zkcrypto contributors. zkcrypto: Rust ecosystem for zero-knowledge cryptography. https://github.com/zkcrypto, 2018–2026. Accessed: 2026-03-06.
L259:   15. Dziembowski, S.; Faust, S.; Kędzior, P.; Mielniczuk, M.; Mohanty, S.K.; Pietrzak, K. Beholder Signatures. In Cryptology ePrint Archive; 2025. [【41†Google Scholar†scholar.google.com】]
L260:   16. Boneh, D.; et al. BLS Signatures, 2023. IETF CFRG draft.
L261:  | Disclaimer/Publisher’s Note: The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content.
L262: © 2026 by the author. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the 【42†Creative Commons Attribution (CC BY) license†creativecommons.org】.
L263: 
L264: Copyright: This open access article is published under a 【42†Creative Commons CC BY 4.0 license†creativecommons.org】, which permit the free download, distribution, and reuse, provided that the author and preprint are cited in any reuse.
L265: 
L266: Downloads
L267: 
L268: 164
L269: 
L270: Views
L271: 
L272: 182
L273: 
L274: Comments
L275: 
L276: 0
L277: 
L278: Subscription
L279: Notify me about updates to this article or when a peer-reviewed version is published.
L280: 
L281: [Input: Your Email]Subscribe
L282: 
L283: Recommended Preprints
L284: 
L285: 【43†Overlays for 4x4 Matrix Multiplication: Peeling, Value-Aware Collapse, and Bit-Sliced Zero-Multiply Paths†doi.org】
L286: 
L287: Michael Rey
L288: 
L289: ,
L290: 
L291: 2025
L292: 
L293: 【44†Multiplication-Free Matrix Multiplication: Bitplane Semantics and Semantic Filtering†doi.org】
L294: 
L295: Michael Rey
L296: 
L297: ,
L298: 
L299: 2025
L300: 
L301: 【45†Matrix Multiplication: Optimality Under Resource Constraints†doi.org】
L302: 
L303: Michael Rey
L304: 
L305: ,
L306: 
L307: 2025
L308: Recommended Articles
L309: 
L310: 【46†Optimizing the Performance of the Sparse Matrix–Vector Multiplication Kernel in FPGA Guided by the Roofline Model†doi.org】
L311: 
L312: Federico Favaro
L313: 
L314: et al.
L315: 
L316: Micromachines,
L317: 
L318: 2023
L319: 
L320: 【47†Algorithms and Data Structures for Sparse Polynomial Arithmetic†doi.org】
L321: 
L322: Mohammadali Asadi
L323: 
L324: et al.
L325: 
L326: Mathematics,
L327: 
L328: 2019
L329: 
L330: 【48†AGP-GEMM: Adaptive Grouping and Partitioning Framework for Accelerating Small and Irregular Matrices on CPUs†doi.org】
L331: 
L332: Hongzhe Zhou
L333: 
L334: et al.
L335: 
L336: Computers,
L337: 
L338: 2026
L339: Preprints.org is a free preprint server supported by MDPI in Basel, Switzerland.
L340: 
L341: 【49†Image: weChat logo】
L342: 
L343: Contact Us RSS
L344: ##### MDPI Initiatives
L345: 
L346: Expand MDPI Initiatives
L347: 
L348:   * 【50†SciProfiles†sciprofiles.com】
L349:   * 【51†Sciforum†sciforum.net】
L350:   * 【52†Encyclopedia†encyclopedia.pub】
L351:   * 【53†MDPI Books†www.mdpi.com】
L352:   * 【54†Scilit†www.scilit.com】
L353:   * 【55†Proceedings†www.mdpi.com】
L354:   * 【56†JAMS†jams.pub】
L355: 
L356: ##### Important Links
L357: 
L358: Expand Important Links
L359: 
L360:   * 【57†Activities】
L361:   * 【58†Advisory Board】
L362:   * 【59†Collections】
L363:   * 【60†How It Works】
L364:   * 【61†Preprints Friendly Journals】
L365:   * 【62†Reading List】
L366:   * 【63†News】
L367:   * 【64†Statistics】
L368: ##### Subscribe
L369: 
L370: Expand Subscribe
L371: 
L372: Choose an area of interest and we will send you notifications of new preprints at your preferred frequency.
L373: 
L374: Subscribe
L375: 
L376: © 2026 【65†MDPI†www.mdpi.com】 (Basel, Switzerland) unless otherwise stated
L377: 
L378: 【66†Accessibility†www.mdpi.com】
L379: 
L380: Disclaimer
L381: 
L382: 【67†Terms of Use】
L383: 
L384: 【68†Privacy Policy】
L385: 
L386: [Button: Privacy Settings]

