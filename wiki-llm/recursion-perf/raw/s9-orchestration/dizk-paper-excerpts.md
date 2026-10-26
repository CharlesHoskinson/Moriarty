# DIZK paper: protocol and experimental setup excerpts
> Source: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf
> Collected: 2026-10-26
> Published: Unknown

SOURCE: Paper PDF fetched by helper using Scrapling; local PDF conversion unavailable. Source text below extracted through public web PDF reader, retaining page/line coordinates. Excerpts, not the complete paper. Proceedings dated August 15–17, 2018.

L295@P3: 2.3 The zkSNARK protocol of Groth
L296@P3: Our system provides a distributed implementation of
L297@P3: a zkSNARK protocol due to Groth [42]. We selected
L298@P3: Groth’s protocol because it is, to our knowledge, the
L1148@P10: 8 Implementation
L1149@P10: We implemented the distributed zkSNARK in ≈ 10K lines
L1150@P10: of Java code over Apache Spark [2], a popular cluster computing framework. All data representations are designed
L1151@P10: to fit within the Spark computation model. For example,
L1152@P10: we represent an R1CS instance φ = (k,N,M,a,b, c) via
L1153@P10: three RDDs, one for each of the three matrices a,b, c, and
L1154@P10: each record in an RDD is a tuple (j,(i, v)) where v is the
L1155@P10: (i, j)-th entry of the matrix. (Recall from §2.2 that a,b, c
L1156@P10: are coefficient matrices that determine all constraints of
L1157@P10: the instance.) Since DIZK deals with large instances, we
L1158@P10: carefully adjust the RDD partition size such that each
L1159@P10: partition fits on an executor’s heap space.
L1160@P10: 9 Experimental setup
L1161@P10: We evaluated DIZK on Amazon EC2 using r3.large instances (2 vCPUs, 15 GiB of memory) and r3.8xlarge
L1162@P10: instances (32 vCPUs, 244 GiB of memory). For singlemachine experiments, we used one r3.large instance.
L1163@P10: For distributed experiments, we used a cluster of ten
L1164@P10-11: 684 27th USENIX Security Symposium USENIX Associationr3.8xlarge instances for up to 128 executors, and a cluster
L1165@P11: of twenty r3.8xlarge for 256 executors.
L1166@P11: We instantiate the zkSNARK via a 256-bit Barreto–
L1167@P11: Naehrig curve [8], a standard choice in prior zkSNARK
L1168@P11: implementations. This means that G1 and G2 are elliptic
L1169@P11: curve groups of a prime order p of 256 bits, and the scalar
L1170@P11: field F has this same size.
L1171@P11: An important technicality is that we cannot rely on
L1172@P11: curves used in prior zkSNARK works, because they do
L1173@P11: not support the large instance sizes in this work, as we
L1174@P11: now explain. To allow for efficient implementations of the
L1175@P11: setup and the prover one needs a curve in which the group
L1176@P11: order p is such that p−1 is divisible by 2
L1177@P11: a
L1178@P11: , where 2
L1179@P11: a
L1180@P11: is
L1181@P11: larger than the maximum instance size to be supported
L1182@P11: [11]. As the instance sizes that we support are in the
L1183@P11: billions (at least 230), we need, say, a ≥ 40.
L1184@P11: We thus generated (by modifying the sampling
L1185@P11: algorithm in [8]) a 256-bit Barreto–Naehrig curve with
L1186@P11: a = 50, which suffices for our purposes. The curve is
L1187@P11: E/Fq : y
L1188@P11: 2 = x3 +13 with q = 178558083348049028502
L1189@P11: 609238317702557737797405798625193380108245358
L1190@P11: 56509878273, and its order is p = 17855808334804902
L1191@P11: 850260923831770255773646114952324966112694569
L1192@P11: 107431857586177.
L1193@P11: 10 Evaluation of the distributed zkSNARK
L1194@P11: We evaluated our distributed zkSNARK and show that:
L1195@P11: 1. We support instances of more than a billion gates, a
L1196@P11: significant improvement over serial implementations,
L1197@P11: which exceed memory bounds at 10-20 million gates.
L1198@P11: 2. Fixing a number of executors on the cluster and letting
L1199@P11: the instance size increase (from several millions to
L1200@P11: over a billion), the running time of the setup and prover
L1201@P11: increases close to linearly as expected, demonstrating
L1202@P11: scalability over this range of instance sizes.
L1203@P11: 3. Fixing an input size and increasing the number of
L1204@P11: executors, the running time of the setup and prover
L1205@P11: decreases close to linearly as expected, demonstrating
L1206@P11: parallelization over this range of executors.
L1207@P11: In the next few sub-sections we support these findings.
L1208@P11: 10.1 Evaluation of the setup and prover
L1209@P11: We evaluate our distributed implementation of the zkSNARK setup and prover. Below we use ‘instance size’ to
L1210@P11: denote the number of constraints M in a R1CS instance.2
L1211@P11: First, we measure the largest instance size (as a power
L1212@P11: of 2) that is supported by:
L1213@P11: 2The number of variables N also affects performance, but it is usually
L1214@P11: close to M and so our discussions only mention M with the understanding that N ≈ M in our experiments. The number of inputs k in an R1CS
L1215@P11: instance is bounded by the number of variables N, and either way does
L1216@P11: not affect the setup’s and prover’s performance by much; moreover, k
L1217@P11: is much, much smaller than N in typical applications and so we do not
L1218@P11: focus on it.
L1219@P11: • the serial implementation of Groth’s protocol [59], a
L1220@P11: state-of-the-art zkSNARK library; and
L1221@P11: • our distributed implementation of the same protocol.
L1222@P11: (Also, we plot the same for the serial implementation
L1223@P11: of PGHR [55]’s protocol in libsnark, a common zkSNARK choice.)
L1224@P11: Data from our experiments, reported in Fig. 4, shows
L1225@P11: that using more executors allows us to support larger
L1226@P11: instance sizes, in particular supporting billions of constraints with sufficiently many executors. Instances of
L1227@P11: this size are much larger than what was previously possible via serial techniques.
L1228@P11: Next, we measure the running time of the setup and the
L1229@P11: prover on an increasing number of constraints and with
L1230@P11: an increasing number of executors. Data from our experiments, reported in Fig. 5, shows that (a) for a given number of executors, running times increase nearly linearly
L1231@P11: as expected, demonstrating scalability over a wide range
L1232@P11: of instance sizes; (b) for a given instance size, running
