# Pinned Midnight stdlib 2.3.5 source
> Source: https://docs.rs/crate/midnight-zk-stdlib/2.3.5/source/src/lib.rs
> Collected: 2026-10-26
> Published: Unknown

midnight-zk-stdlib 2.3.5 - Docs.rs

[Docs.rs](/)

* [midnight-zk-stdlib-2.3.5](/crate/midnight-zk-stdlib/2.3.5 "Standard library of circuits and utilities for Midnight zero-knowledge proofs")
* [Platform](#)
  + [x86\_64-unknown-linux-gnu](/midnight-zk-stdlib/2.3.5/midnight_zk_stdlib/)
* [Feature flags](/crate/midnight-zk-stdlib/2.3.5/features "Browse available feature flags of midnight-zk-stdlib-2.3.5")

* [docs.rs](#)
  + [About docs.rs](/about)
  + [Badges](/about/badges)
  + [Builds](/about/builds)
  + [Metadata](/about/metadata)
  + [Shorthand URLs](/about/redirections)
  + [Download](/about/download)
  + [Rustdoc JSON](/about/rustdoc-json)
  + [Build queue](/releases/queue)
  + [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  + [Rust website](https://www.rust-lang.org/)
  + [The Book](https://doc.rust-lang.org/book/)
  + [Standard Library API Reference](https://doc.rust-lang.org/std/)
  + [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  + [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  + [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

midnight-zk-stdlib 2.3.5
========================

Standard library of circuits and utilities for Midnight zero-knowledge proofs

* [Crate](/crate/midnight-zk-stdlib/2.3.5)
* [Source](/crate/midnight-zk-stdlib/2.3.5/source/src/lib.rs)
* [Builds](/crate/midnight-zk-stdlib/2.3.5/builds)
* [Feature flags](/crate/midnight-zk-stdlib/2.3.5/features)

[Documentation](/midnight-zk-stdlib/2.3.5/midnight_zk_stdlib/)

* Hide files
* [..](../)
* [external](./external/)
* [utils](./utils/)
* [lib.rs](./lib.rs)

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112
113
114
115
116
117
118
119
120
121
122
123
124
125
126
127
128
129
130
131
132
133
134
135
136
137
138
139
140
141
142
143
144
145
146
147
148
149
150
151
152
153
154
155
156
157
158
159
160
161
162
163
164
165
166
167
168
169
170
171
172
173
174
175
176
177
178
179
180
181
182
183
184
185
186
187
188
189
190
191
192
193
194
195
196
197
198
199
200
201
202
203
204
205
206
207
208
209
210
211
212
213
214
215
216
217
218
219
220
221
222
223
224
225
226
227
228
229
230
231
232
233
234
235
236
237
238
239
240
241
242
243
244
245
246
247
248
249
250
251
252
253
254
255
256
257
258
259
260
261
262
263
264
265
266
267
268
269
270
271
272
273
274
275
276
277
278
279
280
281
282
283
284
285
286
287
288
289
290
291
292
293
294
295
296
297
298
299
300
301
302
303
304
305
306
307
308
309
310
311
312
313
314
315
316
317
318
319
320
321
322
323
324
325
326
327
328
329
330
331
332
333
334
335
336
337
338
339
340
341
342
343
344
345
346
347
348
349
350
351
352
353
354
355
356
357
358
359
360
361
362
363
364
365
366
367
368
369
370
371
372
373
374
375
376
377
378
379
380
381
382
383
384
385
386
387
388
389
390
391
392
393
394
395
396
397
398
399
400
401
402
403
404
405
406
407
408
409
410
411
412
413
414
415
416
417
418
419
420
421
422
423
424
425
426
427
428
429
430
431
432
433
434
435
436
437
438
439
440
441
442
443
444
445
446
447
448
449
450
451
452
453
454
455
456
457
458
459
460
461
462
463
464
465
466
467
468
469
470
471
472
473
474
475
476
477
478
479
480
481
482
483
484
485
486
487
488
489
490
491
492
493
494
495
496
497
498
499
500
501
502
503
504
505
506
507
508
509
510
511
512
513
514
515
516
517
518
519
520
521
522
523
524
525
526
527
528
529
530
531
532
533
534
535
536
537
538
539
540
541
542
543
544
545
546
547
548
549
550
551
552
553
554
555
556
557
558
559
560
561
562
563
564
565
566
567
568
569
570
571
572
573
574
575
576
577
578
579
580
581
582
583
584
585
586
587
588
589
590
591
592
593
594
595
596
597
598
599
600
601
602
603
604
605
606
607
608
609
610
611
612
613
614
615
616
617
618
619
620
621
622
623
624
625
626
627
628
629
630
631
632
633
634
635
636
637
638
639
640
641
642
643
644
645
646
647
648
649
650
651
652
653
654
655
656
657
658
659
660
661
662
663
664
665
666
667
668
669
670
671
672
673
674
675
676
677
678
679
680
681
682
683
684
685
686
687
688
689
690
691
692
693
694
695
696
697
698
699
700
701
702
703
704
705
706
707
708
709
710
711
712
713
714
715
716
717
718
719
720
721
722
723
724
725
726
727
728
729
730
731
732
733
734
735
736
737
738
739
740
741
742
743
744
745
746
747
748
749
750
751
752
753
754
755
756
757
758
759
760
761
762
763
764
765
766
767
768
769
770
771
772
773
774
775
776
777
778
779
780
781
782
783
784
785
786
787
788
789
790
791
792
793
794
795
796
797
798
799
800
801
802
803
804
805
806
807
808
809
810
811
812
813
814
815
816
817
818
819
820
821
822
823
824
825
826
827
828
829
830
831
832
833
834
835
836
837
838
839
840
841
842
843
844
845
846
847
848
849
850
851
852
853
854
855
856
857
858
859
860
861
862
863
864
865
866
867
868
869
870
871
872
873
874
875
876
877
878
879
880
881
882
883
884
885
886
887
888
889
890
891
892
893
894
895
896
897
898
899
900
901
902
903
904
905
906
907
908
909
910
911
912
913
914
915
916
917
918
919
920
921
922
923
924
925
926
927
928
929
930
931
932
933
934
935
936
937
938
939
940
941
942
943
944
945
946
947
948
949
950
951
952
953
954
955
956
957
958
959
960
961
962
963
964
965
966
967
968
969
970
971
972
973
974
975
976
977
978
979
980
981
982
983
984
985
986
987
988
989
990
991
992
993
994
995
996
997
998
999
1000
1001
1002
1003
1004
1005
1006
1007
1008
1009
1010
1011
1012
1013
1014
1015
1016
1017
1018
1019
1020
1021
1022
1023
1024
1025
1026
1027
1028
1029
1030
1031
1032
1033
1034
1035
1036
1037
1038
1039
1040
1041
1042
1043
1044
1045
1046
1047
1048
1049
1050
1051
1052
1053
1054
1055
1056
1057
1058
1059
1060
1061
1062
1063
1064
1065
1066
1067
1068
1069
1070
1071
1072
1073
1074
1075
1076
1077
1078
1079
1080
1081
1082
1083
1084
1085
1086
1087
1088
1089
1090
1091
1092
1093
1094
1095
1096
1097
1098
1099
1100
1101
1102
1103
1104
1105
1106
1107
1108
1109
1110
1111
1112
1113
1114
1115
1116
1117
1118
1119
1120
1121
1122
1123
1124
1125
1126
1127
1128
1129
1130
1131
1132
1133
1134
1135
1136
1137
1138
1139
1140
1141
1142
1143
1144
1145
1146
1147
1148
1149
1150
1151
1152
1153
1154
1155
1156
1157
1158
1159
1160
1161
1162
1163
1164
1165
1166
1167
1168
1169
1170
1171
1172
1173
1174
1175
1176
1177
1178
1179
1180
1181
1182
1183
1184
1185
1186
1187
1188
1189
1190
1191
1192
1193
1194
1195
1196
1197
1198
1199
1200
1201
1202
1203
1204
1205
1206
1207
1208
1209
1210
1211
1212
1213
1214
1215
1216
1217
1218
1219
1220
1221
1222
1223
1224
1225
1226
1227
1228
1229
1230
1231
1232
1233
1234
1235
1236
1237
1238
1239
1240
1241
1242
1243
1244
1245
1246
1247
1248
1249
1250
1251
1252
1253
1254
1255
1256
1257
1258
1259
1260
1261
1262
1263
1264
1265
1266
1267
1268
1269
1270
1271
1272
1273
1274
1275
1276
1277
1278
1279
1280
1281
1282
1283
1284
1285
1286
1287
1288
1289
1290
1291
1292
1293
1294
1295
1296
1297
1298
1299
1300
1301
1302
1303
1304
1305
1306
1307
1308
1309
1310
1311
1312
1313
1314
1315
1316
1317
1318
1319
1320
1321
1322
1323
1324
1325
1326
1327
1328
1329
1330
1331
1332
1333
1334
1335
1336
1337
1338
1339
1340
1341
1342
1343
1344
1345
1346
1347
1348
1349
1350
1351
1352
1353
1354
1355
1356
1357
1358
1359
1360
1361
1362
1363
1364
1365
1366
1367
1368
1369
1370
1371
1372
1373
1374
1375
1376
1377
1378
1379
1380
1381
1382
1383
1384
1385
1386
1387
1388
1389
1390
1391
1392
1393
1394
1395
1396
1397
1398
1399
1400
1401
1402
1403
1404
1405
1406
1407
1408
1409
1410
1411
1412
1413
1414
1415
1416
1417
1418
1419
1420
1421
1422
1423
1424
1425
1426
1427
1428
1429
1430
1431
1432
1433
1434
1435
1436
1437
1438
1439
1440
1441
1442
1443
1444
1445
1446
1447
1448
1449
1450
1451
1452
1453
1454
1455
1456
1457
1458
1459
1460
1461
1462
1463
1464
1465
1466
1467
1468
1469
1470
1471
1472
1473
1474
1475
1476
1477
1478
1479
1480
1481
1482
1483
1484
1485
1486
1487
1488
1489
1490
1491
1492
1493
1494
1495
1496
1497
1498
1499
1500
1501
1502
1503
1504
1505
1506
1507
1508
1509
1510
1511
1512
1513
1514
1515
1516
1517
1518
1519
1520
1521
1522
1523
1524
1525
1526
1527
1528
1529
1530
1531
1532
1533
1534
1535
1536
1537
1538
1539
1540
1541
1542
1543
1544
1545
1546
1547
1548
1549
1550
1551
1552
1553
1554
1555
1556
1557
1558
1559
1560
1561
1562
1563
1564
1565
1566
1567
1568
1569
1570
1571
1572
1573
1574
1575
1576
1577
1578
1579
1580
1581
1582
1583
1584
1585
1586
1587
1588
1589
1590
1591
1592
1593
1594
1595
1596
1597
1598
1599
1600
1601
1602
1603
1604
1605
1606
1607
1608
1609
1610
1611
1612
1613
1614
1615
1616
1617
1618
1619
1620
1621
1622
1623
1624
1625
1626
1627
1628
1629
1630
1631
1632
1633
1634
1635
1636
1637
1638
1639
1640
1641
1642
1643
1644
1645
1646
1647
1648
1649
1650
1651
1652
1653
1654
1655
1656
1657
1658
1659
1660
1661
1662
1663
1664
1665
1666
1667
1668
1669
1670
1671
1672
1673
1674
1675
1676
1677
1678
1679
1680
1681
1682
1683
1684
1685
1686
1687
1688
1689
1690
1691
1692
1693
1694
1695
1696
1697
1698
1699
1700
1701
1702
1703
1704
1705
1706
1707
1708
1709
1710
1711
1712
1713
1714
1715
1716
1717
1718
1719
1720
1721
1722
1723
1724
1725
1726
1727
1728
1729
1730
1731
1732
1733
1734
1735
1736
1737
1738
1739
1740
1741
1742
1743
1744
1745
1746
1747
1748
1749
1750
1751
1752
1753
1754
1755
1756
1757
1758
1759
1760
1761
1762
1763
1764
1765
1766
1767
1768
1769
1770
1771
1772
1773
1774
1775
1776
1777
1778
1779
1780
1781
1782
1783
1784
1785
1786
1787
1788
1789
1790
1791
1792
1793
1794
1795
1796
1797
1798
1799
1800
1801
1802
1803
1804
1805
1806
1807
1808
1809
1810
1811
1812
1813
1814
1815
1816
1817
1818
1819
1820
1821
1822
1823
1824
1825
1826
1827
1828
1829
1830
1831
1832
1833
1834
1835
1836
1837
1838
1839
1840
1841
1842
1843
1844
1845
1846
1847
1848
1849
1850
1851
1852
1853
1854
1855
1856
1857
1858
1859
1860
1861
1862
1863
1864
1865
1866
1867
1868
1869
1870
1871
1872
1873
1874
1875
1876
1877
1878
1879
1880
1881
1882
1883
1884
1885
1886
1887
1888
1889
1890
1891
1892
1893
1894
1895
1896
1897
1898
1899
1900
1901
1902
1903
1904
1905
1906
1907
1908
1909
1910
1911
1912
1913
1914
1915
1916
1917
1918
1919
1920
1921
1922
1923
1924
1925
1926
1927
1928
1929
1930
1931
1932
1933
1934
1935
1936
1937
1938
1939
1940
1941
1942
1943
1944
1945
1946
1947
1948
1949
1950
1951
1952
1953
1954
1955
1956
1957
1958
1959
1960
1961
1962
1963
1964
1965
1966
1967
1968
1969
1970
1971
1972
1973
1974
1975
1976
1977
1978
1979
1980
1981
1982
1983
1984
1985
1986
1987
1988
1989
1990
1991
1992
1993
1994
1995
1996
1997
1998
1999
2000
2001
2002
2003
2004
2005
2006
2007
2008
2009
2010
2011
2012
2013
2014
2015
2016
2017
2018
2019
2020
2021
2022
2023
2024
2025
2026
2027
2028
2029
2030
2031
2032
2033
2034
2035
2036
2037
2038
2039
2040
2041
2042
2043
2044
2045
2046
2047
2048
2049
2050
2051
2052
2053
2054
2055
2056
2057
2058
2059
2060
2061
2062
2063
2064
2065
2066
2067
2068
2069
2070
2071
2072
2073
2074
2075
2076
2077
2078
2079
2080
2081
2082
2083
2084
2085
2086
2087
2088
2089
2090
2091
2092
2093
2094
2095
2096
2097
2098
2099
2100
2101
2102
2103
2104
2105
2106
2107
2108
2109
2110
2111
2112
2113
2114
2115
2116
2117
2118
2119
2120
2121
2122
2123
2124
2125
2126
2127
2128
2129
2130
2131
2132
2133
2134
2135
2136
2137
2138
2139
2140
2141
2142
2143
2144
2145
2146
2147
```

```
// This file is part of MIDNIGHT-ZK.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![doc = include_str!("../README.md")]
//!
//! ## Implementation Details
//!
//! This library uses a fixed configuration, meaning that regardless of what one
//! uses, it will always consist of the same columns, lookups, permutation
//! enabled columns, or gates. The motivation for this is twofold:
//!
//! * It facilitates recursion (we always aggregate circuits that have the same
//!   verification logic).
//!
//! * We could optimise the verifier, who can store part of the circuit
//!   description in memory and does not need to reproduce it everytime it
//!   receives a new proof.

mod external;
pub mod utils;

use std::{cell::RefCell, cmp::max, convert::TryInto, fmt::Debug, io, rc::Rc};

use bincode::{config::standard, Decode, Encode};
use blake2b::blake2b::{
    blake2b_chip::{Blake2bChip, Blake2bConfig},
    NB_BLAKE2B_ADVICE_COLS,
};
use ff::{Field, PrimeField};
use group::{prime::PrimeCurveAffine, Group};
use keccak_sha3::packed_chip::{PackedChip, PackedConfig, PACKED_ADVICE_COLS, PACKED_FIXED_COLS};
use midnight_circuits::{
    biguint::biguint_gadget::BigUintGadget,
    ecc::{
        foreign::{
            edwards_chip::{
                nb_foreign_edwards_chip_columns, ForeignEdwardsEccChip, ForeignEdwardsEccConfig,
            },
            weierstrass_chip::{
                nb_foreign_ecc_chip_columns, ForeignWeierstrassEccChip, ForeignWeierstrassEccConfig,
            },
        },
        hash_to_curve::HashToCurveGadget,
        native::{EccChip, EccConfig, NB_EDWARDS_COLS},
    },
    field::{
        decomposition::{
            chip::{P2RDecompositionChip, P2RDecompositionConfig},
            pow2range::Pow2RangeChip,
        },
        foreign::{
            nb_field_chip_columns, params::MultiEmulationParams as MEP, FieldChip, FieldChipConfig,
        },
        native::{NB_ARITH_COLS, NB_ARITH_FIXED_COLS},
        NativeChip, NativeConfig, NativeGadget,
    },
    hash::{
        poseidon::{
            constants::RATE, PoseidonChip, PoseidonConfig, VarLenPoseidonGadget,
            NB_POSEIDON_ADVICE_COLS, NB_POSEIDON_FIXED_COLS,
        },
        sha256::{
            Sha256Chip, Sha256Config, VarLenSha256Gadget, NB_SHA256_ADVICE_COLS,
            NB_SHA256_FIXED_COLS,
        },
        sha512::{Sha512Chip, Sha512Config, NB_SHA512_ADVICE_COLS, NB_SHA512_FIXED_COLS},
    },
    instructions::{
        hash::VarHashInstructions, public_input::CommittedInstanceInstructions,
        vector::VectorBounds, *,
    },
    map::map_gadget::MapGadget,
    parsing::{
        self,
        scanner::{ScannerChip, ScannerConfig, NB_SCANNER_ADVICE_COLS, NB_SCANNER_FIXED_COLS},
        Base64Chip, Base64Config, ParserGadget, NB_BASE64_ADVICE_COLS,
    },
    types::{
        AssignedBit, AssignedByte, AssignedNative, AssignedNativePoint, ComposableChip, InnerValue,
        Instantiable,
    },
    vec::{vector_gadget::VectorGadget, AssignedVector, Vectorizable},
    verifier::{BlstrsEmulation, VerifierGadget},
};
use midnight_curves::{
    curve25519::{self as curve25519_mod, Curve25519},
    k256::{self as k256_mod, K256},
    p256::{self as p256_mod, P256},
    Fq, G1Affine, G1Projective,
};
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    dev::cost_model::{circuit_model, CircuitModel},
    plonk::{
        self, keygen_vk_with_k, prepare, Circuit, ConstraintSystem, Error, ProvingKey, VerifyingKey,
    },
    poly::{
        commitment::{Guard, Params},
        kzg::{
            params::{ParamsKZG, ParamsVerifierKZG},
            KZGCommitmentScheme,
        },
    },
    transcript::{CircuitTranscript, Hashable, Sampleable, Transcript, TranscriptHash},
    utils::SerdeFormat,
};
use num_bigint::BigUint;
use rand::{CryptoRng, RngCore};

use crate::{
    external::{blake2b::Blake2bWrapper, keccak_sha3::KeccakSha3Wrapper},
    utils::plonk_api::BlstPLONK,
};

type C = midnight_curves::JubjubExtended;
type F = midnight_curves::Fq;

// Type aliases, for readability.
type NG = NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>;
type Secp256k1BaseChip = FieldChip<F, k256_mod::Fp, MEP, NG>;
type Secp256k1ScalarChip = FieldChip<F, k256_mod::Fq, MEP, NG>;
type Secp256k1Chip = ForeignWeierstrassEccChip<F, K256, MEP, Secp256k1ScalarChip, NG>;
type P256BaseChip = FieldChip<F, p256_mod::Fp, MEP, NG>;
type P256ScalarChip = FieldChip<F, p256_mod::Fq, MEP, NG>;
type P256Chip = ForeignWeierstrassEccChip<F, P256, MEP, P256ScalarChip, NG>;
type Bls12381BaseChip = FieldChip<F, midnight_curves::Fp, MEP, NG>;
type Bls12381Chip = ForeignWeierstrassEccChip<
    F,
    midnight_curves::G1Projective,
    midnight_curves::G1Projective,
    NG,
    NG,
>;
type Curve25519BaseChip = FieldChip<F, curve25519_mod::Fp, MEP, NG>;
type Curve25519ScalarChip = FieldChip<F, curve25519_mod::Scalar, MEP, NG>;
type Curve25519Chip = ForeignEdwardsEccChip<F, Curve25519, MEP, Curve25519ScalarChip, NG>;

const ZKSTD_VERSION: u32 = 2;

/// Byte size of a serialized BLS12-381 G1 commitment (compressed).
const COMMITMENT_BYTE_SIZE: usize = 48;

/// Byte size of a serialized BLS12-381 scalar.
const SCALAR_BYTE_SIZE: usize = 32;

/// Architecture of the standard library. Specifies what chips need to be
/// configured.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Encode, Decode)]
pub struct ZkStdLibArch {
    /// Enable the Jubjub chip?
    pub jubjub: bool,

    /// Enable the Poseidon chip?
    pub poseidon: bool,

    /// Enable the SHA256 chip?
    pub sha2_256: bool,

    /// Enable the SHA512 chip?
    pub sha2_512: bool,

    /// Enable the Keccak chip? (third-party implementation)
    ///
    /// Note: is configured using the same columns and tables as sha3_256,
    /// meaning enabling either of the two, or both, requires the same
    /// configuration resources.
    pub keccak_256: bool,

    /// Enable the Sha3 chip? (third-party implementation)
    ///
    /// Note: is configured using the same columns and tables as keccak_256,
    /// meaning enabling either of the two, or both, requires the same
    /// configuration resources.
    pub sha3_256: bool,

    /// Enable the Blake2b chip? (third-party implementation)
    pub blake2b: bool,

    /// Enable the Secp256k1 chip?
    pub secp256k1: bool,

    /// Enable the P256 chip?
    pub p256: bool,

    /// Enable BLS12-381 chip?
    pub bls12_381: bool,

    /// Enable Curve25519 chip?
    pub curve25519: bool,

    /// Enable base64 chip?
    pub base64: bool,

    /// Enable scanner chip (automaton-based parsing and substring checks)?
    pub automaton: bool,

    /// Number of parallel lookups for range checks.
    pub nr_pow2range_cols: u8,
}

impl Default for ZkStdLibArch {
    fn default() -> Self {
        ZkStdLibArch {
            jubjub: false,
            poseidon: false,
            sha2_256: false,
            sha2_512: false,
            sha3_256: false,
            keccak_256: false,
            blake2b: false,
            secp256k1: false,
            p256: false,
            bls12_381: false,
            curve25519: false,
            base64: false,
            automaton: false,
            nr_pow2range_cols: 1,
        }
    }
}

/// Serialized layout of [`ZkStdLibArch`] as it existed in
/// `midnight-zk-stdlib` v1.0.0 (ZKSTD_VERSION 1). Used only for migration
/// when reading old serialized verifying keys.
#[derive(Decode)]
struct ZkStdLibArchV1 {
    jubjub: bool,
    poseidon: bool,
    sha2_256: bool,
    sha2_512: bool,
    keccak_256: bool,
    sha3_256: bool,
    blake2b: bool,
    secp256k1: bool,
    bls12_381: bool,
    base64: bool,
    automaton: bool,
    nr_pow2range_cols: u8,
}

impl From<ZkStdLibArchV1> for ZkStdLibArch {
    fn from(v1: ZkStdLibArchV1) -> Self {
        ZkStdLibArch {
            jubjub: v1.jubjub,
            poseidon: v1.poseidon,
            sha2_256: v1.sha2_256,
            sha2_512: v1.sha2_512,
            keccak_256: v1.keccak_256,
            sha3_256: v1.sha3_256,
            blake2b: v1.blake2b,
            secp256k1: v1.secp256k1,
            p256: false,
            bls12_381: v1.bls12_381,
            curve25519: false,
            base64: v1.base64,
            automaton: v1.automaton,
            nr_pow2range_cols: v1.nr_pow2range_cols,
        }
    }
}

impl ZkStdLibArch {
    /// Writes the ZKStd architecture to a buffer.
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&ZKSTD_VERSION.to_le_bytes())?;
        bincode::encode_into_std_write(self, writer, standard())
            .map(|_| ())
            .map_err(io::Error::other)
    }

    /// Reads the ZkStd architecture from a buffer.
    pub fn read<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let version = u32::from_le_bytes(version);
        match version {
            1 => bincode::decode_from_std_read::<ZkStdLibArchV1, _, _>(reader, standard())
                .map(ZkStdLibArch::from)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            2 => bincode::decode_from_std_read(reader, standard())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported ZKStd version: {}", version),
            )),
        }
    }

    /// Reads the ZkStdArchitecture from a buffer where a MidnightVK was
    /// serialized. This enables the reader to know the architecture without
    /// the need of deserializing the full verifying key.
    pub fn read_from_serialized_vk<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        // The current serialization of the verifying key places the architecture at
        // the beginning.
        Self::read(reader)
    }
}

#[derive(Debug, Clone)]
/// Configured chips for [ZkStdLib].
pub struct ZkStdLibConfig {
    native_config: NativeConfig,
    core_decomposition_config: P2RDecompositionConfig,
    jubjub_config: Option<EccConfig>,
    sha2_256_config: Option<Sha256Config>,
    sha2_512_config: Option<Sha512Config>,
    poseidon_config: Option<PoseidonConfig<midnight_curves::Fq>>,
    secp256k1_scalar_config: Option<FieldChipConfig>,
    secp256k1_config: Option<ForeignWeierstrassEccConfig<K256>>,
    p256_scalar_config: Option<FieldChipConfig>,
    p256_config: Option<ForeignWeierstrassEccConfig<P256>>,
    bls12_381_config: Option<ForeignWeierstrassEccConfig<midnight_curves::G1Projective>>,
    curve25519_scalar_config: Option<FieldChipConfig>,
    curve25519_config: Option<ForeignEdwardsEccConfig<Curve25519>>,
    base64_config: Option<Base64Config>,
    scanner_config: Option<ScannerConfig>,

    // Configuration of external libraries.
    keccak_sha3_config: Option<PackedConfig>,
    blake2b_config: Option<Blake2bConfig>,
}

/// The `ZkStdLib` exposes all tools that are used in circuit generation.
#[derive(Clone, Debug)]
#[allow(clippy::type_complexity)]
pub struct ZkStdLib {
    // Internal chips and gadgets.
    native_gadget: NG,
    core_decomposition_chip: P2RDecompositionChip<F>,
    jubjub_chip: Option<EccChip<C>>,
    sha2_256_chip: Option<Sha256Chip<F>>,
    varlen_sha2_256_gadget: Option<VarLenSha256Gadget<F>>,
    sha2_512_chip: Option<Sha512Chip<F>>,
    poseidon_gadget: Option<PoseidonChip<F>>,
    varlen_poseidon_gadget: Option<VarLenPoseidonGadget<F>>,
    htc_gadget: Option<HashToCurveGadget<F, C, AssignedNative<F>, PoseidonChip<F>, EccChip<C>>>,
    map_gadget: Option<MapGadget<F, NG, PoseidonChip<F>>>,
    biguint_gadget: BigUintGadget<F, NG>,
    secp256k1_chip: Option<Secp256k1Chip>,
    p256_chip: Option<P256Chip>,
    bls12_381_chip: Option<Bls12381Chip>,
    curve25519_chip: Option<Curve25519Chip>,
    base64_chip: Option<Base64Chip<F>>,
    parser_gadget: ParserGadget<F, NG>,
    scanner_chip: Option<ScannerChip<F>>,
    vector_gadget: VectorGadget<F>,
    verifier_gadget: Option<VerifierGadget<BlstrsEmulation>>,

    // Third-party chips.
    keccak_sha3_chip: Option<KeccakSha3Wrapper<F>>,
    blake2b_chip: Option<Blake2bWrapper<F>>,

    // Flags that indicate if certain chips have been used. This way we can load the tables only
    // when necessary (thus reducing the min_k in some cases).
    // Such a usage flag has to be added and updated correctly for each new chip using tables.
    used_sha2_256: Rc<RefCell<bool>>,
    used_sha2_512: Rc<RefCell<bool>>,
    used_secp256k1: Rc<RefCell<bool>>,
    used_p256: Rc<RefCell<bool>>,
    used_bls12_381: Rc<RefCell<bool>>,
    used_curve25519: Rc<RefCell<bool>>,
    used_base64: Rc<RefCell<bool>>,
    used_scanner: Rc<RefCell<bool>>,
    used_keccak_or_sha3: Rc<RefCell<bool>>,
    used_blake2b: Rc<RefCell<bool>>,
}

impl ZkStdLib {
    /// Creates a new [ZkStdLib] given its config.
    pub fn new(config: &ZkStdLibConfig, max_bit_len: usize) -> Self {
        let native_chip = NativeChip::new(&config.native_config, &());
        let core_decomposition_chip =
            P2RDecompositionChip::new(&config.core_decomposition_config, &max_bit_len);
        let native_gadget = NativeGadget::new(core_decomposition_chip.clone(), native_chip.clone());
        let jubjub_chip = (config.jubjub_config.as_ref())
            .map(|jubjub_config| EccChip::new(jubjub_config, &native_gadget));
        let sha2_256_chip = (config.sha2_256_config.as_ref())
            .map(|sha256_config| Sha256Chip::new(sha256_config, &native_gadget));
        let varlen_sha2_256_gadget = sha2_256_chip.as_ref().map(VarLenSha256Gadget::new);
        let sha2_512_chip = (config.sha2_512_config.as_ref())
            .map(|sha512_config| Sha512Chip::new(sha512_config, &native_gadget));
        let poseidon_gadget = (config.poseidon_config.as_ref())
            .map(|poseidon_config| PoseidonChip::new(poseidon_config, &native_chip));
        let varlen_poseidon_gadget = (poseidon_gadget.as_ref())
            .map(|poseidon| VarLenPoseidonGadget::new(poseidon, &native_gadget));
        let htc_gadget = (jubjub_chip.as_ref())
            .zip(poseidon_gadget.as_ref())
            .map(|(ecc_chip, poseidon_gadget)| HashToCurveGadget::new(poseidon_gadget, ecc_chip));
        let biguint_gadget = BigUintGadget::new(&native_gadget);
        let map_gadget = poseidon_gadget
            .as_ref()
            .map(|poseidon_gadget| MapGadget::new(&native_gadget, poseidon_gadget));
        let secp256k1_scalar_chip = (config.secp256k1_scalar_config.as_ref())
            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
        let secp256k1_chip = (config.secp256k1_config.as_ref())
            .zip(secp256k1_scalar_chip.as_ref())
            .map(|(curve_config, scalar_chip)| {
                ForeignWeierstrassEccChip::new(curve_config, &native_gadget, scalar_chip)
            });
        let p256_scalar_chip = (config.p256_scalar_config.as_ref())
            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
        let p256_chip = (config.p256_config.as_ref()).zip(p256_scalar_chip.as_ref()).map(
            |(curve_config, scalar_chip)| {
                ForeignWeierstrassEccChip::new(curve_config, &native_gadget, scalar_chip)
            },
        );
        let bls12_381_chip = (config.bls12_381_config.as_ref()).map(|curve_config| {
            ForeignWeierstrassEccChip::new(curve_config, &native_gadget, &native_gadget)
        });
        let curve25519_scalar_chip = (config.curve25519_scalar_config.as_ref())
            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
        let curve25519_chip = (config.curve25519_config.as_ref())
            .zip(curve25519_scalar_chip.as_ref())
            .map(|(curve_config, scalar_chip)| {
                ForeignEdwardsEccChip::new(curve_config, &native_gadget, scalar_chip)
            });

        let base64_chip = (config.base64_config.as_ref())
            .map(|base64_config| Base64Chip::new(base64_config, &native_gadget));

        let parser_gadget = ParserGadget::new(&native_gadget);
        let scanner_chip =
            config.scanner_config.as_ref().map(|c| ScannerChip::new(c, &native_gadget));
        let vector_gadget = VectorGadget::new(&native_gadget);

        let verifier_gadget = bls12_381_chip.as_ref().zip(poseidon_gadget.as_ref()).map(
            |(curve_chip, sponge_chip)| {
                VerifierGadget::<BlstrsEmulation>::new(curve_chip, &native_gadget, sponge_chip)
            },
        );

        let keccak_sha3_chip = config
            .keccak_sha3_config
            .as_ref()
            .map(|sha3_config| KeccakSha3Wrapper::new(sha3_config, &native_gadget));
        let blake2b_chip = config
            .blake2b_config
            .as_ref()
            .map(|blake2b_config| Blake2bWrapper::new(blake2b_config, &native_gadget));

        Self {
            native_gadget,
            core_decomposition_chip,
            jubjub_chip,
            sha2_256_chip,
            varlen_sha2_256_gadget,
            sha2_512_chip,
            poseidon_gadget,
            varlen_poseidon_gadget,
            map_gadget,
            htc_gadget,
            biguint_gadget,
            secp256k1_chip,
            p256_chip,
            bls12_381_chip,
            curve25519_chip,
            base64_chip,
            parser_gadget,
            scanner_chip,
            vector_gadget,
            verifier_gadget,
            keccak_sha3_chip,
            blake2b_chip,
            used_sha2_256: Rc::new(RefCell::new(false)),
            used_sha2_512: Rc::new(RefCell::new(false)),
            used_secp256k1: Rc::new(RefCell::new(false)),
            used_p256: Rc::new(RefCell::new(false)),
            used_bls12_381: Rc::new(RefCell::new(false)),
            used_curve25519: Rc::new(RefCell::new(false)),
            used_base64: Rc::new(RefCell::new(false)),
            used_scanner: Rc::new(RefCell::new(false)),
            used_keccak_or_sha3: Rc::new(RefCell::new(false)),
            used_blake2b: Rc::new(RefCell::new(false)),
        }
    }

    /// Configure [ZkStdLib] from scratch.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        (arch, max_bit_len): (ZkStdLibArch, u8),
    ) -> ZkStdLibConfig {
        let nb_advice_cols = [
            NB_ARITH_COLS,
            arch.nr_pow2range_cols as usize,
            arch.jubjub as usize * NB_EDWARDS_COLS,
            arch.poseidon as usize * NB_POSEIDON_ADVICE_COLS,
            arch.sha2_256 as usize * NB_SHA256_ADVICE_COLS,
            arch.sha2_512 as usize * NB_SHA512_ADVICE_COLS,
            arch.secp256k1 as usize
                * max(
                    nb_field_chip_columns::<F, k256_mod::Fq, MEP>(),
                    nb_foreign_ecc_chip_columns::<F, K256, MEP, k256_mod::Fq>(),
                ),
            arch.p256 as usize
                * max(
                    nb_field_chip_columns::<F, p256_mod::Fq, MEP>(),
                    nb_foreign_ecc_chip_columns::<F, P256, MEP, p256_mod::Fq>(),
                ),
            arch.bls12_381 as usize
                * max(
                    nb_field_chip_columns::<F, midnight_curves::Fp, MEP>(),
                    nb_foreign_ecc_chip_columns::<
                        F,
                        midnight_curves::G1Projective,
                        MEP,
                        midnight_curves::Fp,
                    >(),
                ),
            arch.curve25519 as usize
                * max(
                    nb_field_chip_columns::<F, curve25519_mod::Scalar, MEP>(),
                    nb_foreign_edwards_chip_columns::<F, Curve25519, MEP>(),
                ),
            arch.base64 as usize * NB_BASE64_ADVICE_COLS,
            arch.automaton as usize * NB_SCANNER_ADVICE_COLS,
            (arch.keccak_256 || arch.sha3_256) as usize * PACKED_ADVICE_COLS,
            arch.blake2b as usize * NB_BLAKE2B_ADVICE_COLS,
        ]
        .into_iter()
        .max()
        .unwrap_or(0);

        let nb_fixed_cols = [
            NB_ARITH_FIXED_COLS,
            arch.poseidon as usize * NB_POSEIDON_FIXED_COLS,
            arch.sha2_256 as usize * NB_SHA256_FIXED_COLS,
            arch.sha2_512 as usize * NB_SHA512_FIXED_COLS,
            (arch.keccak_256 || arch.sha3_256) as usize * PACKED_FIXED_COLS,
            arch.automaton as usize * NB_SCANNER_FIXED_COLS,
        ]
        .into_iter()
        .max()
        .unwrap_or(0);

        let advice_columns = (0..nb_advice_cols).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let fixed_columns = (0..nb_fixed_cols).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let committed_instance_column = meta.instance_column();
        let instance_column = meta.instance_column();

        let native_config = NativeChip::configure(
            meta,
            &(
                advice_columns[..NB_ARITH_COLS].try_into().unwrap(),
                fixed_columns[..NB_ARITH_FIXED_COLS].try_into().unwrap(),
                [committed_instance_column, instance_column],
            ),
        );

        let nb_parallel_range_checks = arch.nr_pow2range_cols as usize;
        let max_bit_len = max_bit_len as u32;

        let pow2range_config =
            Pow2RangeChip::configure(meta, &advice_columns[1..=arch.nr_pow2range_cols as usize]);

        let core_decomposition_config =
            P2RDecompositionChip::configure(meta, &(native_config.clone(), pow2range_config));

        let jubjub_config = arch.jubjub.then(|| {
            EccChip::<C>::configure(meta, &advice_columns[..NB_EDWARDS_COLS].try_into().unwrap())
        });

        let sha2_256_config = arch.sha2_256.then(|| {
            Sha256Chip::configure(
                meta,
                &(
                    advice_columns[..NB_SHA256_ADVICE_COLS].try_into().unwrap(),
                    fixed_columns[..NB_SHA256_FIXED_COLS].try_into().unwrap(),
                ),
            )
        });

        let sha2_512_config = arch.sha2_512.then(|| {
            Sha512Chip::configure(
                meta,
                &(
                    advice_columns[..NB_SHA512_ADVICE_COLS].try_into().unwrap(),
                    fixed_columns[..NB_SHA512_FIXED_COLS].try_into().unwrap(),
                ),
            )
        });

        let poseidon_config = arch.poseidon.then(|| {
            PoseidonChip::configure(
                meta,
                &(
                    advice_columns[..NB_POSEIDON_ADVICE_COLS].try_into().unwrap(),
                    fixed_columns[..NB_POSEIDON_FIXED_COLS].try_into().unwrap(),
                ),
            )
        });

        let secp256k1_scalar_config = arch.secp256k1.then(|| {
            Secp256k1ScalarChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let secp256k1_config = arch.secp256k1.then(|| {
            let base_config = Secp256k1BaseChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            );
            Secp256k1Chip::configure(
                meta,
                &base_config,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let p256_scalar_config = arch.p256.then(|| {
            P256ScalarChip::configure(meta, &advice_columns, nb_parallel_range_checks, max_bit_len)
        });

        let p256_config = arch.p256.then(|| {
            let base_config = P256BaseChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            );
            P256Chip::configure(
                meta,
                &base_config,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let bls12_381_config = arch.bls12_381.then(|| {
            let base_config = Bls12381BaseChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            );
            Bls12381Chip::configure(
                meta,
                &base_config,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let curve25519_scalar_config = arch.curve25519.then(|| {
            Curve25519ScalarChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let curve25519_config = arch.curve25519.then(|| {
            let base_config = Curve25519BaseChip::configure(
                meta,
                &advice_columns,
                nb_parallel_range_checks,
                max_bit_len,
            );
            Curve25519Chip::configure(
                meta,
                &base_config,
                &advice_columns,
                &fixed_columns,
                nb_parallel_range_checks,
                max_bit_len,
            )
        });

        let base64_config = arch.base64.then(|| {
            Base64Chip::configure(
                meta,
                advice_columns[..NB_BASE64_ADVICE_COLS].try_into().unwrap(),
            )
        });

        let scanner_config = arch.automaton.then(|| {
            ScannerChip::configure(
                meta,
                &(
                    advice_columns[..NB_SCANNER_ADVICE_COLS].try_into().unwrap(),
                    fixed_columns[0],
                    parsing::spec_library(),
                ),
            )
        });

        let constant_column =
            (arch.keccak_256 || arch.sha3_256 || arch.blake2b).then(|| meta.fixed_column());

        let keccak_sha3_config = (arch.keccak_256 || arch.sha3_256).then(|| {
            PackedChip::configure(
                meta,
                constant_column.unwrap(),
                advice_columns[..PACKED_ADVICE_COLS].try_into().unwrap(),
                fixed_columns[..PACKED_FIXED_COLS].try_into().unwrap(),
            )
        });

        let blake2b_config = arch.blake2b.then(|| {
            Blake2bChip::configure(
                meta,
                constant_column.unwrap(),
                advice_columns[0],
                advice_columns[1..NB_BLAKE2B_ADVICE_COLS].try_into().unwrap(),
            )
        });

        ZkStdLibConfig {
            native_config,
            core_decomposition_config,
            jubjub_config,
            sha2_256_config,
            sha2_512_config,
            poseidon_config,
            secp256k1_scalar_config,
            secp256k1_config,
            p256_scalar_config,
            p256_config,
            bls12_381_config,
            curve25519_scalar_config,
            curve25519_config,
            base64_config,
            scanner_config,
            keccak_sha3_config,
            blake2b_config,
        }
    }
}

impl ZkStdLib {
    /// Native EccChip.
    pub fn jubjub(&self) -> &EccChip<C> {
        self.jubjub_chip.as_ref().expect("ZkStdLibArch must enable jubjub")
    }

    /// Gadget for performing in-circuit big-unsigned integer operations.
    pub fn biguint(&self) -> &BigUintGadget<F, NG> {
        &self.biguint_gadget
    }

    /// Gadget for performing map and non-map checks
    pub fn map_gadget(&self) -> &MapGadget<F, NG, PoseidonChip<F>> {
        self.map_gadget
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
    }

    /// Chip for performing in-circuit operations over the Secp256k1 curve, its
    /// scalar field or its base field.
    pub fn secp256k1(&self) -> &Secp256k1Chip {
        *self.used_secp256k1.borrow_mut() = true;
        self.secp256k1_chip
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable secp256k1"))
    }

    /// Chip for performing in-circuit operations over the P256 curve, its
    /// scalar field or its base field.
    pub fn p256(&self) -> &P256Chip {
        *self.used_p256.borrow_mut() = true;
        self.p256_chip
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable p256"))
    }

    /// Chip for performing in-circuit operations over the BLS12-381 curve, its
    /// scalar field or its base field.
    pub fn bls12_381(&self) -> &Bls12381Chip {
        *self.used_bls12_381.borrow_mut() = true;
        self.bls12_381_chip
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable bls12_381"))
    }

    /// Chip for performing in-circuit operations over Curve25519.
    pub fn curve25519(&self) -> &Curve25519Chip {
        *self.used_curve25519.borrow_mut() = true;
        self.curve25519_chip
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable curve25519"))
    }

    /// Chip for performing in-circuit base64 decoding.
    pub fn base64(&self) -> &Base64Chip<F> {
        *self.used_base64.borrow_mut() = true;
        self.base64_chip
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable base64"))
    }

    /// Gadget for column-free parsing helpers (fetch_bytes, date_to_int, etc.).
    /// Always available (no arch flag needed).
    pub fn parser(&self) -> &ParserGadget<F, NG> {
        &self.parser_gadget
    }

    /// Chip for various scanning functions based on lookups. This includes
    /// automaton-based parsing ([`ScannerChip::parse`]) and substring checks
    /// ([`ScannerChip::check_subsequence`], [`ScannerChip::check_bytes`]).
    ///
    /// Returns the scanner chip for automaton-based parsing and substring
    /// checks. The static automaton table is loaded automatically when
    /// `parse` is called with a `Static(..)` variant.
    pub fn scanner(&self) -> &ScannerChip<F> {
        *self.used_scanner.borrow_mut() = true;
        (self.scanner_chip.as_ref()).unwrap_or_else(|| panic!("ZkStdLibArch must enable automaton"))
    }

    /// Chip for performing in-circuit verification of proofs
    /// (generated with Poseidon as the Fiat-Shamir transcript hash).
    pub fn verifier(&self) -> &VerifierGadget<BlstrsEmulation> {
        *self.used_bls12_381.borrow_mut() = true;
        self.verifier_gadget
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable bls12_381 & poseidon"))
    }

    /// Assert that a given assigned bit is true.
    ///
    /// ```
    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
    /// let input: AssignedBit<F> = chip.assign_fixed(layouter, true)?;
    /// chip.assert_true(layouter, &input)?;
    /// # });
    /// ```
    pub fn assert_true(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedBit<F>,
    ) -> Result<(), Error> {
        self.native_gadget.assert_equal_to_fixed(layouter, input, true)
    }

    /// Assert that a given assigned bit is false
    pub fn assert_false(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedBit<F>,
    ) -> Result<(), Error> {
        self.native_gadget.assert_equal_to_fixed(layouter, input, false)
    }

    /// Returns `1` iff `x < y`.
    ///
    /// ```
    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
    /// let condition = chip.lower_than(layouter, &x, &y, 8)?;
    ///
    /// chip.assert_true(layouter, &condition)?;
    /// # });
    /// ```
    ///
    /// # Unsatisfiable Circuit
    ///
    /// If `x` or `y` are not in the range `[0, 2^n)`.
    ///
    /// ```should_panic
    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
    /// let _condition = chip.lower_than(layouter, &x, &y, 7)?;
    /// # });
    /// ```
    ///
    /// Setting `n > (F::NUM_BITS - 1) / 2` will result in a compile-time
    /// error.
    pub fn lower_than(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        y: &AssignedNative<F>,
        n: u32,
    ) -> Result<AssignedBit<F>, Error> {
        let bounded_x = self.native_gadget.bounded_of_element(layouter, n as usize, x)?;
        let bounded_y = self.native_gadget.bounded_of_element(layouter, n as usize, y)?;
        self.native_gadget.lower_than(layouter, &bounded_x, &bounded_y)
    }

    /// Poseidon hash from a slice of native values into a native value.
    ///
    /// ```
    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
    ///
    /// let _hash = chip.poseidon(layouter, &[x, y])?;
    /// # });
    /// ```
    pub fn poseidon(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedNative<F>],
    ) -> Result<AssignedNative<F>, Error> {
        self.poseidon_gadget
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
            .hash(layouter, input)
    }

    /// Poseidon hash over a payload whose element count can vary between
    /// proofs.
    ///
    /// Unlike [`poseidon`](Self::poseidon), which takes a plain slice whose
    /// length is structurally fixed in the circuit (the same for every proof),
    /// this variant takes an [`AssignedVector`]: a specialized structure for
    /// carrying data of varying length, up to a maximum capacity of `M`
    /// elements.
    ///
    /// `M` must be a multiple of 2 (the Poseidon absorption rate).
    pub fn poseidon_varlen<const M: usize>(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedVector<F, AssignedNative<F>, M, RATE>,
    ) -> Result<AssignedNative<F>, Error> {
        assert!(
            M.is_multiple_of(RATE),
            "poseidon_varlen only supports assigned vector whose maxlen M is a multiple of {RATE} (here M = {M})"
        );
        self.varlen_poseidon_gadget
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
            .poseidon_varlen(layouter, input)
    }

    /// Hashes a slice of assigned values into `(x, y)` coordinates which are
    /// guaranteed to be in the curve `C`. For usage, see [HashToCurveGadget].
    pub fn hash_to_curve(
        &self,
        layouter: &mut impl Layouter<F>,
        inputs: &[AssignedNative<F>],
    ) -> Result<AssignedNativePoint<C>, Error> {
        self.htc_gadget
            .as_ref()
            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon and jubjub"))
            .hash_to_curve(layouter, inputs)
    }

    /// SHA2-256.
    /// Takes as input a slice of assigned bytes and returns the assigned
    /// input/output in bytes.
    /// We assume the field uses little endian encoding.
    /// ```
    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
    /// let input = chip.assign_many(
    ///     layouter,
    ///     &[
    ///         Value::known(13),
    ///         Value::known(226),
    ///         Value::known(119),
    ///         Value::known(5),
    ///     ],
    /// )?;
    ///
    /// let _hash = chip.sha2_256(layouter, &input)?;
    /// # });
    /// ```
    pub fn sha2_256(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>], // F -> decompose_bytes -> hash
    ) -> Result<[AssignedByte<F>; 32], Error> {
        *self.used_sha2_256.borrow_mut() = true;
        self.sha2_256_chip
            .as_ref()
            .expect("ZkStdLibArch must enable sha256")
            .hash(layouter, input)
    }

    /// SHA-256 hash over a payload whose byte count can vary between proofs.
    ///
    /// Unlike [`sha2_256`](Self::sha2_256), which takes a plain slice whose
    /// length is structurally fixed in the circuit (the same for every proof),
    /// this variant takes an [`AssignedVector`]: a specialized structure for
    /// carrying data of varying length, up to a maximum capacity of `M` bytes.
    ///
    /// `M` must be a multiple of 64 (the SHA-256 block size).
    pub fn sha2_256_varlen<const M: usize>(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedVector<F, AssignedByte<F>, M, 64>,
    ) -> Result<[AssignedByte<F>; 32], Error> {
        *self.used_sha2_256.borrow_mut() = true;
        assert!(
            M.is_multiple_of(64),
            "sha2_256_varlen only supports assigned vector whose maxlen M is a multiple of 64 (here M = {M})"
        );
        self.varlen_sha2_256_gadget
            .as_ref()
            .expect("ZkStdLibArch must enable sha256")
            .varhash(layouter, input)
    }

    /// SHA2-512 hash.
    pub fn sha2_512(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>], // F -> decompose_bytes -> hash
    ) -> Result<[AssignedByte<F>; 64], Error> {
        *self.used_sha2_512.borrow_mut() = true;
        self.sha2_512_chip
            .as_ref()
            .expect("ZkStdLibArch must enable sha512")
            .hash(layouter, input)
    }

    /// SHA3-256 hash (third-party implementation).
    pub fn sha3_256(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<Fq>; 32], Error> {
        *self.used_keccak_or_sha3.borrow_mut() = true;
        let chip = self
            .keccak_sha3_chip
            .as_ref()
            .expect("ZkStdLibArch must enable sha3 (or keccak)");
        chip.sha3_256_digest(layouter, input)
    }

    /// Keccak-256 hash (third-party implementation).
    pub fn keccak_256(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<Fq>; 32], Error> {
        *self.used_keccak_or_sha3.borrow_mut() = true;
        let chip = self
            .keccak_sha3_chip
            .as_ref()
            .expect("ZkStdLibArch must enable keccak (or sha3)");
        chip.keccak_256_digest(layouter, input)
    }

    /// Blake2b hash with a 256-bit output, unkeyed (third-party
    /// implementation).
    pub fn blake2b_256(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<F>; 32], Error> {
        *self.used_blake2b.borrow_mut() = true;
        let chip = self.blake2b_chip.as_ref().expect("ZkStdLibArch must enable blake2b");
        chip.blake2b_256_digest(layouter, input)
    }

    /// Blake2b hash with a 512-bit output, unkeyed (third-party
    /// implementation).
    pub fn blake2b_512(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<F>; 64], Error> {
        *self.used_blake2b.borrow_mut() = true;
        let chip = self.blake2b_chip.as_ref().expect("ZkStdLibArch must enable blake2b");
        chip.blake2b_512_digest(layouter, input)
    }
}

impl<T> AssignmentInstructions<F, T> for ZkStdLib
where
    T: InnerValue,
    T::Element: Clone,
    NG: AssignmentInstructions<F, T>,
{
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<T::Element>,
    ) -> Result<T, Error> {
        self.native_gadget.assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        constant: T::Element,
    ) -> Result<T, Error> {
        self.native_gadget.assign_fixed(layouter, constant)
    }

    fn assign_many(
        &self,
        layouter: &mut impl Layouter<F>,
        values: &[Value<T::Element>],
    ) -> Result<Vec<T>, Error> {
        self.native_gadget.assign_many(layouter, values)
    }
}

impl<T> PublicInputInstructions<F, T> for ZkStdLib
where
    T: Instantiable<F>,
    T::Element: Clone,
    NG: PublicInputInstructions<F, T>,
{
    fn as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        assigned: &T,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        self.native_gadget.as_public_input(layouter, assigned)
    }

    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        assigned: &T,
    ) -> Result<(), Error> {
        self.native_gadget.constrain_as_public_input(layouter, assigned)
    }

    fn assign_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<<T>::Element>,
    ) -> Result<T, Error> {
        self.native_gadget.assign_as_public_input(layouter, value)
    }
}

impl<T> CommittedInstanceInstructions<F, T> for ZkStdLib
where
    F: PrimeField,
    T: Instantiable<F>,
    NG: CommittedInstanceInstructions<F, T>,
{
    fn constrain_as_committed_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        assigned: &T,
    ) -> Result<(), Error> {
        self.native_gadget.constrain_as_committed_public_input(layouter, assigned)
    }
}

impl<T> AssertionInstructions<F, T> for ZkStdLib
where
    T: InnerValue,
    NG: AssertionInstructions<F, T>,
{
    fn assert_equal(&self, layouter: &mut impl Layouter<F>, x: &T, y: &T) -> Result<(), Error> {
        self.native_gadget.assert_equal(layouter, x, y)
    }

    fn assert_not_equal(&self, layouter: &mut impl Layouter<F>, x: &T, y: &T) -> Result<(), Error> {
        self.native_gadget.assert_not_equal(layouter, x, y)
    }

    fn assert_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        constant: T::Element,
    ) -> Result<(), Error> {
        self.native_gadget.assert_equal_to_fixed(layouter, x, constant)
    }

    fn assert_not_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        constant: T::Element,
    ) -> Result<(), Error> {
        self.native_gadget.assert_not_equal_to_fixed(layouter, x, constant)
    }
}

impl<T> EqualityInstructions<F, T> for ZkStdLib
where
    T: InnerValue,
    NG: EqualityInstructions<F, T>,
{
    fn is_equal(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        y: &T,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.is_equal(layouter, x, y)
    }

    fn is_not_equal(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        y: &T,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.is_not_equal(layouter, x, y)
    }

    fn is_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        constant: T::Element,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.is_equal_to_fixed(layouter, x, constant)
    }

    fn is_not_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &T,
        constant: T::Element,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.is_not_equal_to_fixed(layouter, x, constant)
    }
}

impl<T1, T2> ConversionInstructions<F, T1, T2> for ZkStdLib
where
    T1: InnerValue,
    T2: InnerValue,
    NG: ConversionInstructions<F, T1, T2>,
{
    fn convert_value(&self, x: &T1::Element) -> Option<T2::Element> {
        ConversionInstructions::<_, T1, T2>::convert_value(&self.native_gadget, x)
    }

    fn convert(&self, layouter: &mut impl Layouter<F>, x: &T1) -> Result<T2, Error> {
        self.native_gadget.convert(layouter, x)
    }
}

impl CanonicityInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn le_bits_lower_than(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
        bound: BigUint,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.le_bits_lower_than(layouter, bits, bound)
    }

    fn le_bits_geq_than(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
        bound: BigUint,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.le_bits_geq_than(layouter, bits, bound)
    }
}

impl DecompositionInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn assigned_to_le_bits(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bits: Option<usize>,
        enforce_canonical: bool,
    ) -> Result<Vec<AssignedBit<F>>, Error> {
        self.native_gadget.assigned_to_le_bits(layouter, x, nb_bits, enforce_canonical)
    }

    fn assigned_to_le_bytes(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bytes: Option<usize>,
    ) -> Result<Vec<AssignedByte<F>>, Error> {
        self.native_gadget.assigned_to_le_bytes(layouter, x, nb_bytes)
    }

    fn assigned_to_le_chunks(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bits_per_chunk: usize,
        nb_chunks: Option<usize>,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        self.native_gadget
            .assigned_to_le_chunks(layouter, x, nb_bits_per_chunk, nb_chunks)
    }

    fn sgn0(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.sgn0(layouter, x)
    }
}

impl ArithInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn linear_combination(
        &self,
        layouter: &mut impl Layouter<F>,
        terms: &[(F, AssignedNative<F>)],
        constant: F,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.linear_combination(layouter, terms, constant)
    }

    fn mul(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        y: &AssignedNative<F>,
        multiplying_constant: Option<F>,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.mul(layouter, x, y, multiplying_constant)
    }

    fn div(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        y: &AssignedNative<F>,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.div(layouter, x, y)
    }

    fn inv(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.inv(layouter, x)
    }

    fn inv0(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.inv0(layouter, x)
    }
}

impl ZeroInstructions<F, AssignedNative<F>> for ZkStdLib {}

impl<Assigned> ControlFlowInstructions<F, Assigned> for ZkStdLib
where
    Assigned: InnerValue,
    NG: ControlFlowInstructions<F, Assigned>,
{
    fn select(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &Assigned,
        y: &Assigned,
    ) -> Result<Assigned, Error> {
        self.native_gadget.select(layouter, cond, x, y)
    }

    fn cond_swap(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &Assigned,
        y: &Assigned,
    ) -> Result<(Assigned, Assigned), Error> {
        self.native_gadget.cond_swap(layouter, cond, x, y)
    }
}

impl FieldInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn order(&self) -> BigUint {
        self.native_gadget.order()
    }
}

impl RangeCheckInstructions<F, AssignedNative<F>> for ZkStdLib {
    fn assign_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<F>,
        bound: &BigUint,
    ) -> Result<AssignedNative<F>, Error> {
        self.native_gadget.assign_lower_than_fixed(layouter, value, bound)
    }

    fn assert_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        bound: &BigUint,
    ) -> Result<(), Error> {
        self.native_gadget.assert_lower_than_fixed(layouter, x, bound)
    }
}

impl DivisionInstructions<F, AssignedNative<F>> for ZkStdLib {}

impl BinaryInstructions<F> for ZkStdLib {
    fn and(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.and(layouter, bits)
    }

    fn or(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.or(layouter, bits)
    }

    fn xor(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.xor(layouter, bits)
    }

    fn not(
        &self,
        layouter: &mut impl Layouter<F>,
        bit: &AssignedBit<F>,
    ) -> Result<AssignedBit<F>, Error> {
        self.native_gadget.not(layouter, bit)
    }
}

impl BitwiseInstructions<F, AssignedNative<F>> for ZkStdLib {}

impl<const M: usize, const A: usize, T> VectorInstructions<F, T, M, A> for ZkStdLib
where
    T: Vectorizable,
    T::Element: Copy,
    NG: AssignmentInstructions<F, T> + ControlFlowInstructions<F, T>,
{
    fn trim_beginning(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedVector<F, T, M, A>,
        n_elems: usize,
    ) -> Result<AssignedVector<F, T, M, A>, Error> {
        self.vector_gadget.trim_beginning(layouter, input, n_elems)
    }

    fn padding_flag(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedVector<F, T, M, A>,
    ) -> Result<(Box<[AssignedBit<F>; M]>, VectorBounds<F>), Error> {
        self.vector_gadget.padding_flag(layouter, input)
    }

    fn get_limits(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &AssignedVector<F, T, M, A>,
    ) -> Result<VectorBounds<F>, Error> {
        self.vector_gadget.get_limits(layouter, input)
    }

    fn resize<const L: usize>(
        &self,
        layouter: &mut impl Layouter<F>,
        input: AssignedVector<F, T, M, A>,
    ) -> Result<AssignedVector<F, T, L, A>, Error> {
        self.vector_gadget.resize(layouter, input)
    }

    fn assign_with_filler(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<Vec<<T>::Element>>,
        filler: Option<<T>::Element>,
    ) -> Result<AssignedVector<F, T, M, A>, Error> {
        self.vector_gadget.assign_with_filler(layouter, value, filler)
    }
}

/// Circuit structure which is used to create any circuit that can be compiled
/// into keys using the ZK standard library.
#[derive(Clone, Debug)]
pub struct MidnightCircuit<'a, R: Relation> {
    relation: &'a R,
    k: u32,
    instance: Value<R::Instance>,
    witness: Value<R::Witness>,
    nb_public_inputs: Rc<RefCell<Option<usize>>>,
    circuit_error: Rc<RefCell<Option<R::Error>>>,
}

impl<'a, R: Relation> MidnightCircuit<'a, R> {
    /// A MidnightCircuit with unknown instance-witness for the given relation.
    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
    /// If `k` is `None`, the optimal value is computed automatically.
    pub fn from_relation(relation: &'a R, k: Option<u32>) -> Self {
        MidnightCircuit::new(relation, Value::unknown(), Value::unknown(), k)
    }

    /// Creates a new MidnightCircuit for the given relation.
    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
    /// If `k` is `None`, the optimal value is computed automatically.
    pub fn new(
        relation: &'a R,
        instance: Value<R::Instance>,
        witness: Value<R::Witness>,
        k: Option<u32>,
    ) -> Self {
        let k = k.unwrap_or_else(|| optimal_k(relation));
        MidnightCircuit {
            relation,
            k,
            instance,
            witness,
            nb_public_inputs: Rc::new(RefCell::new(None)),
            circuit_error: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the log2 of the circuit size.
    pub fn k(&self) -> u32 {
        self.k
    }

    /// Takes the circuit error stashed during synthesis, if any.
    pub fn take_error(&self) -> Option<R::Error> {
        self.circuit_error.take()
    }
}

/// A verifier key of a Midnight circuit.
#[derive(Clone, Debug)]
pub struct MidnightVK {
    architecture: ZkStdLibArch,
    k: u8,
    nb_public_inputs: usize,
    vk: VerifyingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>>,
}

impl MidnightVK {
    /// Writes a verifying key to a buffer.
    ///
    /// Depending on the `format`:
    /// - `Processed`: Takes less space, but more time to read.
    /// - `RawBytes`: Takes more space, but faster to read.
    ///
    /// Using `RawBytesUnchecked` will have the same effect as `RawBytes`,
    /// but it is not recommended.
    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
        self.architecture.write(writer)?;

        writer.write_all(&[self.k])?;

        writer.write_all(&(self.nb_public_inputs as u32).to_le_bytes())?;

        self.vk.write(writer, format)
    }

    /// Reads a verification key from a buffer.
    ///
    /// The `format` must match the one that was used when writing the key.
    /// If the key was written with `RawBytes`, it can be read with `RawBytes`
    /// or `RawBytesUnchecked` (which is faster).
    ///
    /// # WARNING
    /// Use `RawBytesUnchecked` only if you trust the party who wrote the key.
    pub fn read<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self> {
        let architecture = ZkStdLibArch::read(reader)?;

        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        let k = byte[0];

        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        let nb_public_inputs = u32::from_le_bytes(bytes) as usize;

        let mut cs = ConstraintSystem::default();
        let _config = ZkStdLib::configure(&mut cs, (architecture, k - 1));

        let vk = VerifyingKey::read_from_cs::<R>(reader, format, cs)?;

        Ok(MidnightVK {
            architecture,
            k,
            nb_public_inputs,
            vk,
        })
    }

    /// The size of the domain associated to this verifying key.
    pub fn k(&self) -> u8 {
        self.k
    }

    /// The underlying midnight-proofs verifying key.
    pub fn vk(
        &self,
    ) -> &VerifyingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>> {
        &self.vk
    }
}

/// A proving key of a Midnight circuit.
#[derive(Clone, Debug)]
pub struct MidnightPK<R: Relation> {
    k: u8,
    relation: R,
    pk: ProvingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>>,
}

impl<Rel: Relation> MidnightPK<Rel> {
    /// Writes a proving key to a buffer.
    ///
    /// Depending on the `format`:
    /// - `Processed`: Takes less space, but more time to read.
    /// - `RawBytes`: Takes more space, but faster to read.
    ///
    /// Using `RawBytesUnchecked` will have the same effect as `RawBytes`,
    /// but it is not recommended.
    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
        writer.write_all(&[self.k])?;

        Rel::write_relation(&self.relation, writer)?;

        self.pk.write(writer, format)
    }

    /// Reads a proving key from a buffer.
    ///
    /// The `format` must match the one that was used when writing the key.
    /// If the key was written with `RawBytes`, it can be read with `RawBytes`
    /// or `RawBytesUnchecked` (which is faster).
    ///
    /// # WARNING
    /// Use `RawBytesUnchecked` only if you trust the party who wrote the key.
    pub fn read<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self> {
        let mut byte = [0u8; 1];

        reader.read_exact(&mut byte)?;
        let k = byte[0];

        let relation = Rel::read_relation(reader)?;

        let pk = ProvingKey::read::<R, MidnightCircuit<Rel>>(
            reader,
            format,
            MidnightCircuit::new(
                &relation,
                Value::unknown(),
                Value::unknown(),
                Some(k as u32),
            )
            .params(),
        )?;

        Ok(MidnightPK { k, relation, pk })
    }

    /// The size of the domain associated to this proving key.
    pub fn k(&self) -> u8 {
        self.k
    }

    /// The underlying midnight-proofs proving key.
    pub fn pk(
        &self,
    ) -> &ProvingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>> {
        &self.pk
    }
}

/// Helper trait, used to abstract the circuit developer from Halo2's
/// boilerplate.
///
/// `Relation` has a default implementation for loading only the tables
/// needed for the requested chips. The developer needs to implement the
/// function [Relation::circuit], which essentially contains the
/// statement of the proof we are creating.
///
/// # Important note
///
/// The API provided here guarantees that the number of public inputs
/// used during verification matches the number of public inputs (as raw
/// scalars) declared in [Relation::circuit] through the
/// [PublicInputInstructions] interface. Proof verification will fail if
/// this requirement is not met.
///
/// # Example
///
/// ```
/// # use midnight_circuits::{
/// #     instructions::{AssignmentInstructions, PublicInputInstructions},
/// #     types::{AssignedByte, Instantiable},
/// # };
/// # use midnight_zk_stdlib::{utils::plonk_api::filecoin_srs, Relation, ZkStdLib, ZkStdLibArch};
/// # use midnight_proofs::{
/// #     circuit::{Layouter, Value},
/// #     plonk::Error,
/// # };
/// # use rand::{rngs::OsRng, Rng, SeedableRng};
/// # use rand_chacha::ChaCha8Rng;
/// # use sha2::Digest;
/// #
/// type F = midnight_curves::Fq;
///
/// #[derive(Clone, Default)]
/// struct ShaPreImageCircuit;
///
/// impl Relation for ShaPreImageCircuit {
///     // When defining a circuit, one must clearly state the instance and the witness
///     // of the underlying NP-relation.
///     type Instance = [u8; 32];
///     type Witness = [u8; 24]; // 192 = 24 * 8
///     type Error = Error;
///
///     // We must specify how the instance is converted into raw field elements to
///     // be process by the prover/verifier. The order here must be consistent with
///     // the order in which public inputs are constrained/assigned in [circuit].
///     fn format_instance(instance: &Self::Instance) -> Result<Vec<F>, Error> {
///         Ok(instance.iter().flat_map(AssignedByte::<F>::as_public_input).collect())
///     }
///
///     // Define the logic of the NP-relation being proved.
///     fn circuit(
///         &self,
///         std_lib: &ZkStdLib,
///         layouter: &mut impl Layouter<F>,
///         _instance: Value<Self::Instance>,
///         witness: Value<Self::Witness>,
///     ) -> Result<(), Error> {
///         let assigned_input = std_lib.assign_many(layouter, &witness.transpose_array())?;
///         let output = std_lib.sha2_256(layouter, &assigned_input)?;
///         output.iter().try_for_each(|b| std_lib.constrain_as_public_input(layouter, b))
///     }
///
///     fn used_chips(&self) -> ZkStdLibArch {
///         ZkStdLibArch {
///             sha2_256: true,
///             ..ZkStdLibArch::default()
///         }
///     }
///
///     fn write_relation<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
///         Ok(())
///     }
///
///     fn read_relation<R: std::io::Read>(_reader: &mut R) -> std::io::Result<Self> {
///         Ok(ShaPreImageCircuit)
///     }
/// }
///
/// const K: u32 = 13;
/// let mut srs = filecoin_srs(K);
///
/// let relation = ShaPreImageCircuit;
///
/// let vk = midnight_zk_stdlib::setup_vk(&srs, &relation);
/// let pk = midnight_zk_stdlib::setup_pk(&relation, &vk);
///
/// let mut rng = ChaCha8Rng::from_entropy();
/// let witness: [u8; 24] = core::array::from_fn(|_| rng.gen());
/// let instance = sha2::Sha256::digest(witness).into();
///
/// let proof = midnight_zk_stdlib::prove::<ShaPreImageCircuit, blake2b_simd::State>(
///     &srs, &pk, &relation, &instance, witness, OsRng,
/// )
/// .expect("Proof generation should not fail");
///
/// assert!(
///     midnight_zk_stdlib::verify::<ShaPreImageCircuit, blake2b_simd::State>(
///         &srs.verifier_params(),
///         &vk,
///         &instance,
///         None,
///         &proof
///     )
///     .is_ok()
/// )
/// ```
pub trait Relation: Clone {
    /// The instance of the NP-relation described by this circuit.
    type Instance: Clone;

    /// The witness of the NP-relation described by this circuit.
    type Witness: Clone;

    /// The error type returned by [Self::circuit] and [Self::format_instance].
    type Error: From<plonk::Error> + Debug;

    /// Produces a vector of field elements in PLONK format representing the
    /// given [Self::Instance].
    fn format_instance(instance: &Self::Instance) -> Result<Vec<F>, Self::Error>;

    /// Produces a vector of field elements in PLONK format representing the
    /// data inside the committed instance.
    fn format_committed_instances(_witness: &Self::Witness) -> Vec<F> {
        vec![]
    }

    /// Defines the circuit's logic.
    fn circuit(
        &self,
        std_lib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        instance: Value<Self::Instance>,
        witness: Value<Self::Witness>,
    ) -> Result<(), Self::Error>;

    /// Specifies what chips are enabled in the standard library. A chip needs
    /// to be enabled if it is used in [Self::circuit], but it can also be
    /// enabled even if it is not used (possibly to share the same architecture
    /// with other circuits).
    ///
    /// The blanket implementation enables none of them.
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch::default()
    }

    /// Writes a relation to a buffer.
    fn write_relation<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;

    /// Reads a relation from a buffer.
    fn read_relation<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl<R: Relation> Circuit<F> for MidnightCircuit<'_, R> {
    type Config = ZkStdLibConfig;

    // FIXME: this could be parametrised by MidnightCircuit.
    type FloorPlanner = SimpleFloorPlanner;

    type Params = (ZkStdLibArch, u8);

    fn without_witnesses(&self) -> Self {
        unreachable!()
    }

    fn params(&self) -> Self::Params {
        (self.relation.used_chips(), (self.k - 1) as u8)
    }

    fn configure_with_params(
        meta: &mut ConstraintSystem<F>,
        params: (ZkStdLibArch, u8),
    ) -> Self::Config {
        ZkStdLib::configure(meta, params)
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        ZkStdLib::configure(meta, (ZkStdLibArch::default(), 8))
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let max_bit_len = (self.k - 1) as usize;
        let zk_std_lib = ZkStdLib::new(&config, max_bit_len);

        self.relation
            .circuit(
                &zk_std_lib,
                &mut layouter.namespace(|| "Running logic circuit"),
                self.instance.clone(),
                self.witness.clone(),
            )
            .map_err(|e| {
                let error_msg = format!("Relation::circuit error: {e:?}");
                *self.circuit_error.borrow_mut() = Some(e);
                Error::Synthesis(error_msg)
            })?;

        // After the circuit function has been called, we can update the expected
        // number of raw public inputs in [Self] (via a RefCell). This number will
        // be stored in the MidnightVK so that we can make sure it matches the number of
        // public inputs provided during verification.
        *self.nb_public_inputs.borrow_mut() =
            Some(zk_std_lib.native_gadget.native_chip.nb_public_inputs());

        // We load the tables at the end, once we have figured out what chips/gadgets
        // were actually used.
        zk_std_lib.core_decomposition_chip.load(&mut layouter)?;

        if let Some(sha256_chip) = zk_std_lib.sha2_256_chip {
            if *zk_std_lib.used_sha2_256.borrow() {
                sha256_chip.load(&mut layouter)?;
            }
        }

        if let Some(sha512_chip) = zk_std_lib.sha2_512_chip {
            if *zk_std_lib.used_sha2_512.borrow() {
                sha512_chip.load(&mut layouter)?;
            }
        }

        if let Some(b64_chip) = zk_std_lib.base64_chip {
            if *zk_std_lib.used_base64.borrow() {
                b64_chip.load(&mut layouter)?;
            }
        }

        if let Some(scanner_chip) = zk_std_lib.scanner_chip {
            if *zk_std_lib.used_scanner.borrow() {
                scanner_chip.load(&mut layouter)?;
            }
        }

        if let Some(keccak_sha3_chip) = zk_std_lib.keccak_sha3_chip {
            if *zk_std_lib.used_keccak_or_sha3.borrow() {
                keccak_sha3_chip.load(&mut layouter)?;
            }
        }

        if let Some(blake2b_chip) = zk_std_lib.blake2b_chip {
            if *zk_std_lib.used_blake2b.borrow() {
                blake2b_chip.load(&mut layouter)?;
            }
        }

        Ok(())
    }
}

/// Generates a verifying key for a `MidnightCircuit<R>` circuit.
///
/// The log2 of the circuit size (`k`) is derived from the SRS parameters.
/// For optimal performance, downsize the SRS to the circuit's optimal `k`
/// beforehand (see [optimal_k]). Otherwise, the circuit will use the full
/// size of the SRS, which may be unnecessarily large.
pub fn setup_vk<R: Relation>(
    params: &ParamsKZG<midnight_curves::Bls12>,
    relation: &R,
) -> MidnightVK {
    let k = params.max_k();
    let circuit = MidnightCircuit::from_relation(relation, Some(k));
    let vk = keygen_vk_with_k(params, &circuit, k).expect("keygen_vk should not fail");

    // During the call to [setup_vk] the circuit RefCell on public inputs has been
    // mutated with the correct value. The following [unwrap] is safe here.
    let nb_public_inputs = circuit.nb_public_inputs.clone().borrow().unwrap();

    MidnightVK {
        architecture: relation.used_chips(),
        k: circuit.k as u8,
        nb_public_inputs,
        vk,
    }
}

/// Generates a proving key for a `MidnightCircuit<R>` circuit.
pub fn setup_pk<R: Relation>(relation: &R, vk: &MidnightVK) -> MidnightPK<R> {
    let circuit = MidnightCircuit::new(
        relation,
        Value::unknown(),
        Value::unknown(),
        Some(vk.k() as u32),
    );
    let pk = BlstPLONK::<MidnightCircuit<R>>::setup_pk(&circuit, &vk.vk);
    MidnightPK {
        k: vk.k(),
        relation: relation.clone(),
        pk,
    }
}

/// Produces a proof of relation `R` for the given instance (using the given
/// proving key and witness).
pub fn prove<R: Relation, H: TranscriptHash>(
    params: &ParamsKZG<midnight_curves::Bls12>,
    pk: &MidnightPK<R>,
    relation: &R,
    instance: &R::Instance,
    witness: R::Witness,
    rng: impl RngCore + CryptoRng,
) -> Result<Vec<u8>, R::Error>
where
    G1Projective: Hashable<H>,
    F: Hashable<H> + Sampleable<H>,
{
    let pi = R::format_instance(instance)?;
    let com_inst = R::format_committed_instances(&witness);
    let circuit = MidnightCircuit::new(
        relation,
        Value::known(instance.clone()),
        Value::known(witness),
        Some(pk.k as u32),
    );
    BlstPLONK::<MidnightCircuit<R>>::prove::<H>(
        params,
        &pk.pk,
        &circuit,
        1,
        &[com_inst.as_slice(), &pi],
        rng,
    )
    .map_err(|e| circuit.take_error().unwrap_or_else(|| e.into()))
}

/// Verifies the given proof of relation `R` with respect to the given instance.
/// Returns `Ok(())` if the proof is valid.
pub fn verify<R: Relation, H: TranscriptHash>(
    params_verifier: &ParamsVerifierKZG<midnight_curves::Bls12>,
    vk: &MidnightVK,
    instance: &R::Instance,
    committed_instance: Option<G1Affine>,
    proof: &[u8],
) -> Result<(), R::Error>
where
    G1Projective: Hashable<H>,
    F: Hashable<H> + Sampleable<H>,
{
    let pi = R::format_instance(instance)?;
    let committed_pi = committed_instance.unwrap_or(G1Affine::identity());
    if pi.len() != vk.nb_public_inputs {
        return Err(Error::InvalidInstances.into());
    }
    Ok(BlstPLONK::<MidnightCircuit<R>>::verify::<H>(
        params_verifier,
        &vk.vk,
        &[committed_pi],
        &[&pi],
        proof,
    )?)
}

/// Verifies a batch of proofs with respect to their corresponding vk.
/// This method does not need to know the `Relation` the proofs are associated
/// to and, indeed, it can verify proofs from different `Relation`s.
/// For that, this function does not take `instance`s, but public inputs
/// in raw format (`Vec<F>`).
///
/// Returns `Ok(())` if all proofs are valid.
pub fn batch_verify<H: TranscriptHash + Send + Sync>(
    params_verifier: &ParamsVerifierKZG<midnight_curves::Bls12>,
    vks: &[MidnightVK],
    pis: &[Vec<F>],
    proofs: &[Vec<u8>],
) -> Result<(), Error>
where
    G1Projective: Hashable<H>,
    F: Hashable<H> + Sampleable<H>,
{
    use rayon::prelude::*;

    // TODO: For the moment, committed instances are not supported.
    let n = vks.len();
    if pis.len() != n || proofs.len() != n {
        // TODO: have richer types in halo2
        return Err(Error::InvalidInstances);
    }

    let prepared: Vec<(_, F)> = vks
        .par_iter()
        .zip(pis.par_iter())
        .zip(proofs.par_iter())
        .map(|((vk, pi), proof)| {
            if pi.len() != vk.nb_public_inputs {
                return Err(Error::InvalidInstances);
            }

            let mut transcript = CircuitTranscript::init_from_bytes(proof);
            let dual_msm = prepare::<
                midnight_curves::Fq,
                KZGCommitmentScheme<midnight_curves::Bls12>,
                CircuitTranscript<H>,
            >(
                &vk.vk,
                &[&[midnight_curves::G1Projective::identity()]],
                // TODO: We could batch here proofs with the same vk.
                &[&[pi]],
                &mut transcript,
            )?;
            let summary: F = transcript.squeeze_challenge();
            transcript.assert_empty().map_err(|_| Error::Opening)?;
            Ok((dual_msm, summary))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let mut r_transcript = CircuitTranscript::init();
    let mut guards = Vec::with_capacity(n);
    for (guard, summary) in prepared {
        r_transcript.common(&summary)?;
        guards.push(guard);
    }
    let r: F = r_transcript.squeeze_challenge();

    let n_guards = guards.len();
    let powers: Vec<F> =
        std::iter::successors(Some(F::ONE), |p| Some(*p * r)).take(n_guards).collect();
    guards.par_iter_mut().enumerate().for_each(|(i, guard)| guard.scale(powers[i]));

    // Phase 4: add scaled guards sequentially.
    let Some(mut acc_guard) = guards.pop() else {
        return Ok(());
    };
    for guard in guards {
        acc_guard.add_msm(guard);
    }
    // TODO: Have richer error types
    acc_guard.verify(params_verifier).map_err(|_| Error::Opening)
}

/// Cost model of the given relation for the given `k`.
/// `k` is the log2 of the circuit size. If `None`, the optimal value is
/// computed automatically.
pub fn cost_model<R: Relation>(relation: &R, k: Option<u32>) -> CircuitModel {
    let circuit = MidnightCircuit::from_relation(relation, k);
    circuit_model::<_, COMMITMENT_BYTE_SIZE, SCALAR_BYTE_SIZE>(&circuit)
}

/// Finds the optimal `k` (log2 of the circuit size) for the given relation.
/// Tries different values of `k` (9..=25) and picks the smallest one where
/// the circuit fits. The pow2range table uses `max_bit_len = k - 1`.
pub fn optimal_k<R: Relation>(relation: &R) -> u32 {
    let mut best_k = u32::MAX;

    for k in 9..=25 {
        let model = cost_model(relation, Some(k));

        if model.k < best_k {
            best_k = model.k;
        }

        // Stop when the pow2range table (2^k rows) becomes the bottleneck.
        if model.rows < (1 << k) {
            break;
        }
    }

    best_k
}
```
