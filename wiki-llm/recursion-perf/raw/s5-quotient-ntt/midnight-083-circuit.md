# midnight-proofs 0.8.3 constraint degree
> Source: https://docs.rs/crate/midnight-proofs/0.8.3/source/src/plonk/circuit.rs
> Collected: 2026-10-26
> Published: Unknown

midnight-proofs 0.8.3 - Docs.rs

[Docs.rs](/)

* [midnight-proofs-0.8.3](/crate/midnight-proofs/0.8.3 "Fast PLONK-based zero-knowledge proving system
  ")

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

midnight-proofs 0.8.3
=====================

Fast PLONK-based zero-knowledge proving system

* [Crate](/crate/midnight-proofs/0.8.3)
* [Source](/crate/midnight-proofs/0.8.3/source/src/plonk/circuit.rs)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Hide files
* [..](../)
* [bench](./bench/)
* [lookup](./lookup/)
* [permutation](./permutation/)
* [trash](./trash/)
* [vanishing](./vanishing/)
* [circuit.rs](./circuit.rs)
* [error.rs](./error.rs)
* [evaluation.rs](./evaluation.rs)
* [keygen.rs](./keygen.rs)
* [lookup.rs](./lookup.rs)
* [mod.rs](./mod.rs)
* [permutation.rs](./permutation.rs)
* [prover.rs](./prover.rs)
* [traces.rs](./traces.rs)
* [trash.rs](./trash.rs)
* [vanishing.rs](./vanishing.rs)
* [verifier.rs](./verifier.rs)

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
2148
2149
2150
2151
2152
2153
2154
2155
2156
2157
2158
2159
2160
2161
2162
2163
2164
2165
2166
2167
2168
2169
2170
2171
2172
2173
2174
2175
2176
2177
2178
2179
2180
2181
2182
2183
2184
2185
2186
2187
2188
2189
2190
2191
2192
2193
2194
2195
2196
2197
2198
2199
2200
2201
2202
2203
2204
2205
2206
2207
2208
2209
2210
2211
2212
2213
2214
2215
2216
2217
2218
2219
2220
2221
2222
2223
2224
2225
2226
2227
2228
2229
2230
2231
2232
2233
2234
2235
2236
2237
2238
2239
2240
2241
2242
2243
2244
2245
2246
2247
2248
2249
2250
2251
2252
2253
2254
2255
2256
2257
2258
2259
2260
2261
2262
2263
2264
2265
2266
2267
2268
2269
2270
2271
2272
2273
2274
2275
2276
2277
2278
2279
2280
2281
2282
2283
2284
2285
2286
2287
2288
2289
2290
2291
2292
2293
2294
2295
2296
2297
2298
2299
2300
2301
2302
2303
2304
2305
2306
2307
2308
2309
2310
2311
2312
2313
2314
2315
2316
2317
2318
2319
2320
2321
2322
2323
2324
2325
2326
2327
2328
2329
2330
2331
2332
2333
2334
2335
2336
2337
2338
2339
2340
2341
2342
2343
2344
2345
2346
2347
2348
2349
2350
2351
2352
2353
2354
2355
2356
2357
2358
2359
2360
2361
2362
2363
2364
2365
2366
2367
2368
2369
2370
2371
2372
2373
2374
2375
2376
2377
2378
2379
2380
2381
2382
2383
2384
2385
2386
2387
2388
2389
2390
2391
2392
2393
2394
2395
2396
2397
2398
2399
2400
2401
2402
2403
2404
2405
2406
2407
2408
2409
2410
2411
2412
2413
2414
2415
2416
2417
2418
2419
2420
2421
2422
2423
2424
2425
2426
2427
2428
2429
2430
2431
2432
2433
2434
2435
2436
2437
2438
2439
2440
2441
2442
2443
2444
2445
2446
2447
2448
2449
2450
2451
2452
2453
2454
2455
2456
2457
2458
2459
2460
2461
2462
2463
2464
2465
2466
2467
2468
2469
2470
2471
2472
2473
2474
2475
2476
2477
2478
2479
2480
2481
2482
2483
2484
2485
2486
2487
2488
2489
2490
2491
2492
2493
2494
2495
2496
2497
2498
2499
2500
2501
2502
2503
2504
2505
2506
2507
2508
2509
2510
2511
2512
2513
2514
2515
2516
2517
2518
2519
2520
2521
2522
2523
2524
2525
2526
2527
2528
2529
2530
2531
2532
2533
2534
2535
2536
2537
2538
2539
2540
2541
2542
2543
2544
2545
2546
2547
2548
2549
2550
2551
2552
2553
2554
2555
2556
2557
2558
2559
2560
2561
2562
2563
2564
2565
2566
2567
2568
2569
2570
2571
2572
2573
2574
2575
2576
2577
2578
2579
2580
2581
2582
2583
2584
2585
2586
2587
2588
2589
2590
2591
2592
2593
2594
2595
2596
2597
2598
2599
2600
2601
2602
2603
2604
2605
2606
2607
2608
2609
```

```
use core::{
    cmp::max,
    ops::{Add, Mul},
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Neg, Sub},
};

use ff::Field;
use sealed::SealedPhase;

use super::{lookup, permutation, trash, Error};
use crate::{
    circuit::{layouter::SyncDeps, Layouter, Region, Value},
    dev::metadata,
    plonk::trash::Argument,
    poly::Rotation,
    utils::rational::Rational,
};

/// A column type
pub trait ColumnType:
    'static + Sized + Copy + std::fmt::Debug + PartialEq + Eq + Into<Any>
{
    /// Return expression from cell
    fn query_cell<F: Field>(&self, index: usize, at: Rotation) -> Expression<F>;
}

/// A column with an index and type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Column<C: ColumnType> {
    index: usize,
    column_type: C,
}

impl<C: ColumnType> Column<C> {
    #[cfg(test)]
    pub(crate) fn new(index: usize, column_type: C) -> Self {
        Column { index, column_type }
    }

    /// Index of this column.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Type of this column.
    pub fn column_type(&self) -> &C {
        &self.column_type
    }

    /// Return expression from column at a relative position
    pub fn query_cell<F: Field>(&self, at: Rotation) -> Expression<F> {
        self.column_type.query_cell(self.index, at)
    }

    /// Return expression from column at the current row
    pub fn cur<F: Field>(&self) -> Expression<F> {
        self.query_cell(Rotation::cur())
    }

    /// Return expression from column at the next row
    pub fn next<F: Field>(&self) -> Expression<F> {
        self.query_cell(Rotation::next())
    }

    /// Return expression from column at the previous row
    pub fn prev<F: Field>(&self) -> Expression<F> {
        self.query_cell(Rotation::prev())
    }

    /// Return expression from column at the specified rotation
    pub fn rot<F: Field>(&self, rotation: i32) -> Expression<F> {
        self.query_cell(Rotation(rotation))
    }
}

impl<C: ColumnType> Ord for Column<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // This ordering is consensus-critical! The layouters rely on deterministic
        // column orderings.
        match self.column_type.into().cmp(&other.column_type.into()) {
            // Indices are assigned within column types.
            std::cmp::Ordering::Equal => self.index.cmp(&other.index),
            order => order,
        }
    }
}

impl<C: ColumnType> PartialOrd for Column<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) mod sealed {
    /// Phase of advice column
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Phase(pub(super) u8);

    impl Phase {
        pub fn prev(&self) -> Option<Phase> {
            self.0.checked_sub(1).map(Phase)
        }
    }

    impl SealedPhase for Phase {
        fn to_sealed(self) -> Phase {
            self
        }
    }

    /// Sealed trait to help keep `Phase` private.
    pub trait SealedPhase {
        fn to_sealed(self) -> Phase;
    }
}

/// Phase of advice column
pub trait Phase: SealedPhase {}

impl<P: SealedPhase> Phase for P {}

/// First phase
#[derive(Debug)]
pub struct FirstPhase;

impl SealedPhase for super::FirstPhase {
    fn to_sealed(self) -> sealed::Phase {
        sealed::Phase(0)
    }
}

/// Second phase
#[derive(Debug)]
pub struct SecondPhase;

impl SealedPhase for super::SecondPhase {
    fn to_sealed(self) -> sealed::Phase {
        sealed::Phase(1)
    }
}

/// Third phase
#[derive(Debug)]
pub struct ThirdPhase;

impl SealedPhase for super::ThirdPhase {
    fn to_sealed(self) -> sealed::Phase {
        sealed::Phase(2)
    }
}

/// An advice column
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Advice {
    pub(crate) phase: sealed::Phase,
}

impl Default for Advice {
    fn default() -> Advice {
        Advice {
            phase: FirstPhase.to_sealed(),
        }
    }
}

impl Advice {
    /// Returns `Advice` in given `Phase`
    pub fn new<P: Phase>(phase: P) -> Advice {
        Advice {
            phase: phase.to_sealed(),
        }
    }

    /// Phase of this column
    pub fn phase(&self) -> u8 {
        self.phase.0
    }
}

impl std::fmt::Debug for Advice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Advice");
        // Only show advice's phase if it's not in first phase.
        if self.phase != FirstPhase.to_sealed() {
            debug_struct.field("phase", &self.phase);
        }
        debug_struct.finish()
    }
}

/// A fixed column
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Fixed;

/// An instance column
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Instance;

/// An enum over the Advice, Fixed, Instance structs
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Any {
    /// An Advice variant
    Advice(Advice),
    /// A Fixed variant
    Fixed,
    /// An Instance variant
    Instance,
}

impl Any {
    /// Returns Advice variant in `FirstPhase`
    pub fn advice() -> Any {
        Any::Advice(Advice::default())
    }

    /// Returns Advice variant in given `Phase`
    pub fn advice_in<P: Phase>(phase: P) -> Any {
        Any::Advice(Advice::new(phase))
    }
}

impl std::fmt::Debug for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Any::Advice(advice) => {
                let mut debug_struct = f.debug_struct("Advice");
                // Only show advice's phase if it's not in first phase.
                if advice.phase != FirstPhase.to_sealed() {
                    debug_struct.field("phase", &advice.phase);
                }
                debug_struct.finish()
            }
            Any::Fixed => f.debug_struct("Fixed").finish(),
            Any::Instance => f.debug_struct("Instance").finish(),
        }
    }
}

impl Ord for Any {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // This ordering is consensus-critical! The layouters rely on deterministic
        // column orderings.
        match (self, other) {
            (Any::Instance, Any::Instance) | (Any::Fixed, Any::Fixed) => std::cmp::Ordering::Equal,
            (Any::Advice(lhs), Any::Advice(rhs)) => lhs.phase.cmp(&rhs.phase),
            // Across column types, sort Instance < Advice < Fixed.
            (Any::Instance, Any::Advice(_))
            | (Any::Advice(_), Any::Fixed)
            | (Any::Instance, Any::Fixed) => std::cmp::Ordering::Less,
            (Any::Fixed, Any::Instance)
            | (Any::Fixed, Any::Advice(_))
            | (Any::Advice(_), Any::Instance) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Any {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ColumnType for Advice {
    fn query_cell<F: Field>(&self, index: usize, at: Rotation) -> Expression<F> {
        Expression::Advice(AdviceQuery {
            index: None,
            column_index: index,
            rotation: at,
            phase: self.phase,
        })
    }
}
impl ColumnType for Fixed {
    fn query_cell<F: Field>(&self, index: usize, at: Rotation) -> Expression<F> {
        Expression::Fixed(FixedQuery {
            index: None,
            column_index: index,
            rotation: at,
        })
    }
}
impl ColumnType for Instance {
    fn query_cell<F: Field>(&self, index: usize, at: Rotation) -> Expression<F> {
        Expression::Instance(InstanceQuery {
            index: None,
            column_index: index,
            rotation: at,
        })
    }
}
impl ColumnType for Any {
    fn query_cell<F: Field>(&self, index: usize, at: Rotation) -> Expression<F> {
        match self {
            Any::Advice(Advice { phase }) => Expression::Advice(AdviceQuery {
                index: None,
                column_index: index,
                rotation: at,
                phase: *phase,
            }),
            Any::Fixed => Expression::Fixed(FixedQuery {
                index: None,
                column_index: index,
                rotation: at,
            }),
            Any::Instance => Expression::Instance(InstanceQuery {
                index: None,
                column_index: index,
                rotation: at,
            }),
        }
    }
}

impl From<Advice> for Any {
    fn from(advice: Advice) -> Any {
        Any::Advice(advice)
    }
}

impl From<Fixed> for Any {
    fn from(_: Fixed) -> Any {
        Any::Fixed
    }
}

impl From<Instance> for Any {
    fn from(_: Instance) -> Any {
        Any::Instance
    }
}

impl From<Column<Advice>> for Column<Any> {
    fn from(advice: Column<Advice>) -> Column<Any> {
        Column {
            index: advice.index(),
            column_type: Any::Advice(advice.column_type),
        }
    }
}

impl From<Column<Fixed>> for Column<Any> {
    fn from(advice: Column<Fixed>) -> Column<Any> {
        Column {
            index: advice.index(),
            column_type: Any::Fixed,
        }
    }
}

impl From<Column<Instance>> for Column<Any> {
    fn from(advice: Column<Instance>) -> Column<Any> {
        Column {
            index: advice.index(),
            column_type: Any::Instance,
        }
    }
}

impl TryFrom<Column<Any>> for Column<Advice> {
    type Error = &'static str;

    fn try_from(any: Column<Any>) -> Result<Self, Self::Error> {
        match any.column_type() {
            Any::Advice(advice) => Ok(Column {
                index: any.index(),
                column_type: *advice,
            }),
            _ => Err("Cannot convert into Column<Advice>"),
        }
    }
}

impl TryFrom<Column<Any>> for Column<Fixed> {
    type Error = &'static str;

    fn try_from(any: Column<Any>) -> Result<Self, Self::Error> {
        match any.column_type() {
            Any::Fixed => Ok(Column {
                index: any.index(),
                column_type: Fixed,
            }),
            _ => Err("Cannot convert into Column<Fixed>"),
        }
    }
}

impl TryFrom<Column<Any>> for Column<Instance> {
    type Error = &'static str;

    fn try_from(any: Column<Any>) -> Result<Self, Self::Error> {
        match any.column_type() {
            Any::Instance => Ok(Column {
                index: any.index(),
                column_type: Instance,
            }),
            _ => Err("Cannot convert into Column<Instance>"),
        }
    }
}

/// A selector, representing a fixed boolean value per row of the circuit.
///
/// Selectors can be used to conditionally enable (portions of) gates:
/// ```
/// use midnight_proofs::poly::Rotation;
/// # use midnight_curves::Fq as Scalar;
/// # use midnight_proofs::plonk::{Constraints, ConstraintSystem};
///
/// # let mut meta = ConstraintSystem::<Scalar>::default();
/// let a = meta.advice_column();
/// let b = meta.advice_column();
/// let s = meta.selector();
///
/// meta.create_gate("foo", |meta| {
///     let a = meta.query_advice(a, Rotation::prev());
///     let b = meta.query_advice(b, Rotation::cur());
///
///     // On rows where the selector is enabled, a is constrained to equal b.
///     // On rows where the selector is disabled, a and b can take any value.
///     Constraints::with_selector(s, vec![a - b])
/// });
/// ```
///
/// Selectors are disabled on all rows by default, and must be explicitly
/// enabled on each row when required:
/// ```
/// use ff::Field;
/// use midnight_proofs::{
///     circuit::{Chip, Layouter, Value},
///     plonk::{Advice, Column, Error, Selector},
/// };
/// # use midnight_proofs::plonk::Fixed;
///
/// struct Config {
///     a: Column<Advice>,
///     b: Column<Advice>,
///     s: Selector,
/// }
///
/// fn circuit_logic<F: Field, C: Chip<F>>(
///     chip: C,
///     mut layouter: impl Layouter<F>,
/// ) -> Result<(), Error> {
///     let config = chip.config();
///     # let config: Config = todo!();
///     layouter.assign_region(
///         || "bar",
///         |mut region| {
///             region.assign_advice(|| "a", config.a, 0, || Value::known(F::ONE))?;
///             region.assign_advice(|| "a", config.b, 1, || Value::known(F::ONE))?;
///             config.s.enable(&mut region, 1)
///         },
///     )?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Selector(pub(crate) usize, bool);

impl Selector {
    /// Enable this selector at the given offset within the given region.
    pub fn enable<F: Field>(&self, region: &mut Region<F>, offset: usize) -> Result<(), Error> {
        region.enable_selector(|| "", self, offset)
    }

    /// Is this selector "simple"? Simple selectors can only be multiplied
    /// by expressions that contain no other simple selectors.
    pub fn is_simple(&self) -> bool {
        self.1
    }

    /// Returns index of this selector
    pub fn index(&self) -> usize {
        self.0
    }

    /// Return expression from selector
    pub fn expr<F: Field>(&self) -> Expression<F> {
        Expression::Selector(*self)
    }
}

/// Query of fixed column at a certain relative location
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FixedQuery {
    /// Query index
    pub(crate) index: Option<usize>,
    /// Column index
    pub(crate) column_index: usize,
    /// Rotation of this query
    pub(crate) rotation: Rotation,
}

impl FixedQuery {
    /// Return the query index
    pub fn index(&self) -> Option<usize> {
        self.index
    }
    /// Column index
    pub fn column_index(&self) -> usize {
        self.column_index
    }

    /// Rotation of this query
    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}

/// Query of advice column at a certain relative location
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AdviceQuery {
    /// Query index
    pub index: Option<usize>,
    /// Column index
    pub(crate) column_index: usize,
    /// Rotation of this query
    pub(crate) rotation: Rotation,
    /// Phase of this advice column
    pub(crate) phase: sealed::Phase,
}

impl AdviceQuery {
    /// Column index
    pub fn column_index(&self) -> usize {
        self.column_index
    }

    /// Rotation of this query
    pub fn rotation(&self) -> Rotation {
        self.rotation
    }

    /// Phase of this advice column
    pub fn phase(&self) -> u8 {
        self.phase.0
    }
}

/// Query of instance column at a certain relative location
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstanceQuery {
    /// Query index
    pub index: Option<usize>,
    /// Column index
    pub(crate) column_index: usize,
    /// Rotation of this query
    pub(crate) rotation: Rotation,
}

impl InstanceQuery {
    /// Column index
    pub fn column_index(&self) -> usize {
        self.column_index
    }

    /// Rotation of this query
    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}

/// A fixed column of a lookup table.
///
/// A lookup table can be loaded into this column via
/// [`Layouter::assign_table`]. Columns can currently only contain a single
/// table, but they may be used in multiple lookup arguments via
/// [`ConstraintSystem::lookup`].
///
/// Lookup table columns are always "encumbered" by the lookup arguments they
/// are used in; they cannot simultaneously be used as general fixed columns.
///
/// [`Layouter::assign_table`]: crate::circuit::Layouter::assign_table
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TableColumn {
    /// The fixed column that this table column is stored in.
    ///
    /// # Security
    ///
    /// This inner column MUST NOT be exposed in the public API, or else chip
    /// developers can load lookup tables into their circuits without
    /// default-value-filling the columns, which can cause soundness bugs.
    inner: Column<Fixed>,
}

impl TableColumn {
    /// Returns inner column
    pub fn inner(&self) -> Column<Fixed> {
        self.inner
    }
}

/// A challenge squeezed from transcript after advice columns at the phase have
/// been committed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Challenge {
    index: usize,
    pub(crate) phase: sealed::Phase,
}

impl Challenge {
    /// Index of this challenge.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Phase of this challenge.
    pub fn phase(&self) -> u8 {
        self.phase.0
    }

    /// Return Expression
    pub fn expr<F: Field>(&self) -> Expression<F> {
        Expression::Challenge(*self)
    }
}

/// This trait allows a [`Circuit`] to direct some backend to assign a witness
/// for a constraint system.
pub trait Assignment<F: Field> {
    /// Creates a new region and enters into it.
    ///
    /// Panics if we are currently in a region (if `exit_region` was not
    /// called).
    ///
    /// Not intended for downstream consumption; use [`Layouter::assign_region`]
    /// instead.
    ///
    /// [`Layouter::assign_region`]: crate::circuit::Layouter#method.assign_region
    fn enter_region<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR;

    /// Allows the developer to include an annotation for an specific column
    /// within a `Region`.
    ///
    /// This is usually useful for debugging circuit failures.
    fn annotate_column<A, AR>(&mut self, annotation: A, column: Column<Any>)
    where
        A: FnOnce() -> AR,
        AR: Into<String>;

    /// Exits the current region.
    ///
    /// Panics if we are not currently in a region (if `enter_region` was not
    /// called).
    ///
    /// Not intended for downstream consumption; use [`Layouter::assign_region`]
    /// instead.
    ///
    /// [`Layouter::assign_region`]: crate::circuit::Layouter#method.assign_region
    fn exit_region(&mut self);

    /// Enables a selector at the given row.
    fn enable_selector<A, AR>(
        &mut self,
        annotation: A,
        selector: &Selector,
        row: usize,
    ) -> Result<(), Error>
    where
        A: FnOnce() -> AR,
        AR: Into<String>;

    /// Queries the cell of an instance column at a particular absolute row.
    ///
    /// Returns the cell's value, if known.
    fn query_instance(&self, column: Column<Instance>, row: usize) -> Result<Value<F>, Error>;

    /// Assign an advice column value (witness)
    fn assign_advice<V, VR, A, AR>(
        &mut self,
        annotation: A,
        column: Column<Advice>,
        row: usize,
        to: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>;

    /// Assign a fixed value
    fn assign_fixed<V, VR, A, AR>(
        &mut self,
        annotation: A,
        column: Column<Fixed>,
        row: usize,
        to: V,
    ) -> Result<(), Error>
    where
        V: FnOnce() -> Value<VR>,
        VR: Into<Rational<F>>,
        A: FnOnce() -> AR,
        AR: Into<String>;

    /// Assign two cells to have the same value
    fn copy(
        &mut self,
        left_column: Column<Any>,
        left_row: usize,
        right_column: Column<Any>,
        right_row: usize,
    ) -> Result<(), Error>;

    /// Fills a fixed `column` starting from the given `row` with value `to`.
    fn fill_from_row(
        &mut self,
        column: Column<Fixed>,
        row: usize,
        to: Value<Rational<F>>,
    ) -> Result<(), Error>;

    /// Queries the value of the given challenge.
    ///
    /// Returns `Value::unknown()` if the current synthesis phase is before the
    /// challenge can be queried.
    fn get_challenge(&self, challenge: Challenge) -> Value<F>;

    /// Creates a new (sub)namespace and enters into it.
    ///
    /// Not intended for downstream consumption; use [`Layouter::namespace`]
    /// instead.
    ///
    /// [`Layouter::namespace`]: crate::circuit::Layouter#method.namespace
    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR;

    /// Exits out of the existing namespace.
    ///
    /// Not intended for downstream consumption; use [`Layouter::namespace`]
    /// instead.
    ///
    /// [`Layouter::namespace`]: crate::circuit::Layouter#method.namespace
    fn pop_namespace(&mut self, gadget_name: Option<String>);
}

/// A floor planning strategy for a circuit.
///
/// The floor planner is chip-agnostic and applies its strategy to the circuit
/// it is used within.
pub trait FloorPlanner {
    /// Given the provided `cs`, synthesize the given circuit.
    ///
    /// `constants` is the list of fixed columns that the layouter may use to
    /// assign global constant values. These columns will all have been
    /// equality-enabled.
    ///
    /// Internally, a floor planner will perform the following operations:
    /// - Instantiate a [`Layouter`] for this floor planner.
    /// - Perform any necessary setup or measurement tasks, which may involve
    ///   one or more calls to `Circuit::default().synthesize(config, &mut
    ///   layouter)`.
    /// - Call `circuit.synthesize(config, &mut layouter)` exactly once.
    fn synthesize<F: Field, CS: Assignment<F> + SyncDeps, C: Circuit<F>>(
        cs: &mut CS,
        circuit: &C,
        config: C::Config,
        constants: Vec<Column<Fixed>>,
    ) -> Result<(), Error>;
}

/// This is a trait that circuits provide implementations for so that the
/// backend prover can ask the circuit to synthesize using some given
/// [`ConstraintSystem`] implementation.
pub trait Circuit<F: Field> {
    /// This is a configuration object that stores things like columns.
    type Config: Clone;
    /// The floor planner used for this circuit. This is an associated type of
    /// the `Circuit` trait because its behaviour is circuit-critical.
    type FloorPlanner: FloorPlanner;
    /// Optional circuit configuration parameters. Requires the `circuit-params`
    /// feature.
    #[cfg(feature = "circuit-params")]
    type Params: Default;

    /// Returns a copy of this circuit with no witness values (i.e. all
    /// witnesses set to `None`). For most circuits, this will be equal to
    /// `Self::default()`.
    fn without_witnesses(&self) -> Self;

    /// Returns a reference to the parameters that should be used to configure
    /// the circuit. Requires the `circuit-params` feature.
    #[cfg(feature = "circuit-params")]
    fn params(&self) -> Self::Params {
        Self::Params::default()
    }

    /// The circuit is given an opportunity to describe the exact gate
    /// arrangement, column arrangement, etc.  Takes a runtime parameter.  The
    /// default implementation calls `configure` ignoring the `_params`
    /// argument in order to easily support circuits that don't use
    /// configuration parameters.
    #[cfg(feature = "circuit-params")]
    fn configure_with_params(
        meta: &mut ConstraintSystem<F>,
        _params: Self::Params,
    ) -> Self::Config {
        Self::configure(meta)
    }

    /// The circuit is given an opportunity to describe the exact gate
    /// arrangement, column arrangement, etc.
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config;

    /// Given the provided `cs`, synthesize the circuit. The concrete type of
    /// the caller will be different depending on the context, and they may or
    /// may not expect to have a witness present.
    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error>;
}

/// Low-degree expression representing an identity that must hold over the
/// committed columns.
#[derive(Clone, PartialEq, Eq)]
pub enum Expression<F> {
    /// This is a constant polynomial
    Constant(F),
    /// This is a virtual selector
    Selector(Selector),
    /// This is a fixed column queried at a certain relative location
    Fixed(FixedQuery),
    /// This is an advice (witness) column queried at a certain relative
    /// location
    Advice(AdviceQuery),
    /// This is an instance (external) column queried at a certain relative
    /// location
    Instance(InstanceQuery),
    /// This is a challenge
    Challenge(Challenge),
    /// This is a negated polynomial
    Negated(Box<Expression<F>>),
    /// This is the sum of two polynomials
    Sum(Box<Expression<F>>, Box<Expression<F>>),
    /// This is the product of two polynomials
    Product(Box<Expression<F>>, Box<Expression<F>>),
    /// This is a scaled polynomial
    Scaled(Box<Expression<F>>, F),
}

impl<F: Field> Expression<F> {
    /// Make side effects
    pub fn query_cells(&mut self, cells: &mut VirtualCells<'_, F>) {
        match self {
            Expression::Constant(_) => (),
            Expression::Selector(selector) => {
                if !cells.queried_selectors.contains(selector) {
                    cells.queried_selectors.push(*selector);
                }
            }
            Expression::Fixed(query) => {
                if query.index.is_none() {
                    let col = Column {
                        index: query.column_index,
                        column_type: Fixed,
                    };
                    cells.queried_cells.push((col, query.rotation).into());
                    query.index = Some(cells.meta.query_fixed_index(col, query.rotation));
                }
            }
            Expression::Advice(query) => {
                if query.index.is_none() {
                    let col = Column {
                        index: query.column_index,
                        column_type: Advice { phase: query.phase },
                    };
                    cells.queried_cells.push((col, query.rotation).into());
                    query.index = Some(cells.meta.query_advice_index(col, query.rotation));
                }
            }
            Expression::Instance(query) => {
                if query.index.is_none() {
                    let col = Column {
                        index: query.column_index,
                        column_type: Instance,
                    };
                    cells.queried_cells.push((col, query.rotation).into());
                    query.index = Some(cells.meta.query_instance_index(col, query.rotation));
                }
            }
            Expression::Challenge(_) => (),
            Expression::Negated(a) => a.query_cells(cells),
            Expression::Sum(a, b) => {
                a.query_cells(cells);
                b.query_cells(cells);
            }
            Expression::Product(a, b) => {
                a.query_cells(cells);
                b.query_cells(cells);
            }
            Expression::Scaled(a, _) => a.query_cells(cells),
        };
    }

    /// Evaluate the polynomial using the provided closures to perform the
    /// operations.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<T>(
        &self,
        constant: &impl Fn(F) -> T,
        selector_column: &impl Fn(Selector) -> T,
        fixed_column: &impl Fn(FixedQuery) -> T,
        advice_column: &impl Fn(AdviceQuery) -> T,
        instance_column: &impl Fn(InstanceQuery) -> T,
        challenge: &impl Fn(Challenge) -> T,
        negated: &impl Fn(T) -> T,
        sum: &impl Fn(T, T) -> T,
        product: &impl Fn(T, T) -> T,
        scaled: &impl Fn(T, F) -> T,
    ) -> T {
        match self {
            Expression::Constant(scalar) => constant(*scalar),
            Expression::Selector(selector) => selector_column(*selector),
            Expression::Fixed(query) => fixed_column(*query),
            Expression::Advice(query) => advice_column(*query),
            Expression::Instance(query) => instance_column(*query),
            Expression::Challenge(value) => challenge(*value),
            Expression::Negated(a) => {
                let a = a.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                negated(a)
            }
            Expression::Sum(a, b) => {
                let a = a.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                let b = b.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                sum(a, b)
            }
            Expression::Product(a, b) => {
                let a = a.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                let b = b.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                product(a, b)
            }
            Expression::Scaled(a, f) => {
                let a = a.evaluate(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                );
                scaled(a, *f)
            }
        }
    }

    /// Evaluate the polynomial lazily using the provided closures to perform
    /// the operations.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_lazy<T: PartialEq>(
        &self,
        constant: &impl Fn(F) -> T,
        selector_column: &impl Fn(Selector) -> T,
        fixed_column: &impl Fn(FixedQuery) -> T,
        advice_column: &impl Fn(AdviceQuery) -> T,
        instance_column: &impl Fn(InstanceQuery) -> T,
        challenge: &impl Fn(Challenge) -> T,
        negated: &impl Fn(T) -> T,
        sum: &impl Fn(T, T) -> T,
        product: &impl Fn(T, T) -> T,
        scaled: &impl Fn(T, F) -> T,
        zero: &T,
    ) -> T {
        match self {
            Expression::Constant(scalar) => constant(*scalar),
            Expression::Selector(selector) => selector_column(*selector),
            Expression::Fixed(query) => fixed_column(*query),
            Expression::Advice(query) => advice_column(*query),
            Expression::Instance(query) => instance_column(*query),
            Expression::Challenge(value) => challenge(*value),
            Expression::Negated(a) => {
                let a = a.evaluate_lazy(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                    zero,
                );
                negated(a)
            }
            Expression::Sum(a, b) => {
                let a = a.evaluate_lazy(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                    zero,
                );
                let b = b.evaluate_lazy(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                    zero,
                );
                sum(a, b)
            }
            Expression::Product(a, b) => {
                let (a, b) = if a.complexity() <= b.complexity() {
                    (a, b)
                } else {
                    (b, a)
                };
                let a = a.evaluate_lazy(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                    zero,
                );

                if a == *zero {
                    a
                } else {
                    let b = b.evaluate_lazy(
                        constant,
                        selector_column,
                        fixed_column,
                        advice_column,
                        instance_column,
                        challenge,
                        negated,
                        sum,
                        product,
                        scaled,
                        zero,
                    );
                    product(a, b)
                }
            }
            Expression::Scaled(a, f) => {
                let a = a.evaluate_lazy(
                    constant,
                    selector_column,
                    fixed_column,
                    advice_column,
                    instance_column,
                    challenge,
                    negated,
                    sum,
                    product,
                    scaled,
                    zero,
                );
                scaled(a, *f)
            }
        }
    }

    fn write_identifier<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Expression::Constant(scalar) => write!(writer, "{scalar:?}"),
            Expression::Selector(selector) => write!(writer, "selector[{}]", selector.0),
            Expression::Fixed(query) => {
                write!(
                    writer,
                    "fixed[{}][{}]",
                    query.column_index, query.rotation.0
                )
            }
            Expression::Advice(query) => {
                write!(
                    writer,
                    "advice[{}][{}]",
                    query.column_index, query.rotation.0
                )
            }
            Expression::Instance(query) => {
                write!(
                    writer,
                    "instance[{}][{}]",
                    query.column_index, query.rotation.0
                )
            }
            Expression::Challenge(challenge) => {
                write!(writer, "challenge[{}]", challenge.index())
            }
            Expression::Negated(a) => {
                writer.write_all(b"(-")?;
                a.write_identifier(writer)?;
                writer.write_all(b")")
            }
            Expression::Sum(a, b) => {
                writer.write_all(b"(")?;
                a.write_identifier(writer)?;
                writer.write_all(b"+")?;
                b.write_identifier(writer)?;
                writer.write_all(b")")
            }
            Expression::Product(a, b) => {
                writer.write_all(b"(")?;
                a.write_identifier(writer)?;
                writer.write_all(b"*")?;
                b.write_identifier(writer)?;
                writer.write_all(b")")
            }
            Expression::Scaled(a, f) => {
                a.write_identifier(writer)?;
                write!(writer, "*{f:?}")
            }
        }
    }

    /// Identifier for this expression. Expressions with identical identifiers
    /// do the same calculation (but the expressions don't need to be exactly
    /// equal in how they are composed e.g. `1 + 2` and `2 + 1` can have the
    /// same identifier).
    pub fn identifier(&self) -> String {
        let mut cursor = std::io::Cursor::new(Vec::new());
        self.write_identifier(&mut cursor).unwrap();
        String::from_utf8(cursor.into_inner()).unwrap()
    }

    /// Compute the degree of this polynomial
    pub fn degree(&self) -> usize {
        match self {
            Expression::Constant(_) => 0,
            Expression::Selector(_) => 1,
            Expression::Fixed(_) => 1,
            Expression::Advice(_) => 1,
            Expression::Instance(_) => 1,
            Expression::Challenge(_) => 0,
            Expression::Negated(poly) => poly.degree(),
            Expression::Sum(a, b) => max(a.degree(), b.degree()),
            Expression::Product(a, b) => a.degree() + b.degree(),
            Expression::Scaled(poly, _) => poly.degree(),
        }
    }

    /// Approximate the computational complexity of this expression.
    pub fn complexity(&self) -> usize {
        match self {
            Expression::Constant(_) => 0,
            Expression::Selector(_) => 1,
            Expression::Fixed(_) => 1,
            Expression::Advice(_) => 1,
            Expression::Instance(_) => 1,
            Expression::Challenge(_) => 0,
            Expression::Negated(poly) => poly.complexity() + 5,
            Expression::Sum(a, b) => a.complexity() + b.complexity() + 15,
            Expression::Product(a, b) => a.complexity() + b.complexity() + 30,
            Expression::Scaled(poly, _) => poly.complexity() + 30,
        }
    }

    /// Square this expression.
    pub fn square(self) -> Self {
        self.clone() * self
    }

    /// Returns whether or not this expression contains a simple `Selector`.
    fn contains_simple_selector(&self) -> bool {
        self.evaluate(
            &|_| false,
            &|selector| selector.is_simple(),
            &|_| false,
            &|_| false,
            &|_| false,
            &|_| false,
            &|a| a,
            &|a, b| a || b,
            &|a, b| a || b,
            &|a, _| a,
        )
    }
}

impl<F: std::fmt::Debug> std::fmt::Debug for Expression<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(scalar) => f.debug_tuple("Constant").field(scalar).finish(),
            Expression::Selector(selector) => f.debug_tuple("Selector").field(selector).finish(),
            // Skip enum variant and print query struct directly to maintain backwards
            // compatibility.
            Expression::Fixed(query) => {
                let mut debug_struct = f.debug_struct("Fixed");
                match query.index {
                    None => debug_struct.field("query_index", &query.index),
                    Some(idx) => debug_struct.field("query_index", &idx),
                };
                debug_struct
                    .field("column_index", &query.column_index)
                    .field("rotation", &query.rotation)
                    .finish()
            }
            Expression::Advice(query) => {
                let mut debug_struct = f.debug_struct("Advice");
                match query.index {
                    None => debug_struct.field("query_index", &query.index),
                    Some(idx) => debug_struct.field("query_index", &idx),
                };
                debug_struct
                    .field("column_index", &query.column_index)
                    .field("rotation", &query.rotation);
                // Only show advice's phase if it's not in first phase.
                if query.phase != FirstPhase.to_sealed() {
                    debug_struct.field("phase", &query.phase);
                }
                debug_struct.finish()
            }
            Expression::Instance(query) => {
                let mut debug_struct = f.debug_struct("Instance");
                match query.index {
                    None => debug_struct.field("query_index", &query.index),
                    Some(idx) => debug_struct.field("query_index", &idx),
                };
                debug_struct
                    .field("column_index", &query.column_index)
                    .field("rotation", &query.rotation)
                    .finish()
            }
            Expression::Challenge(challenge) => {
                f.debug_tuple("Challenge").field(challenge).finish()
            }
            Expression::Negated(poly) => f.debug_tuple("Negated").field(poly).finish(),
            Expression::Sum(a, b) => f.debug_tuple("Sum").field(a).field(b).finish(),
            Expression::Product(a, b) => f.debug_tuple("Product").field(a).field(b).finish(),
            Expression::Scaled(poly, scalar) => {
                f.debug_tuple("Scaled").field(poly).field(scalar).finish()
            }
        }
    }
}

impl<F: Field + From<u64>> From<u64> for Expression<F> {
    fn from(value: u64) -> Self {
        Expression::Constant(F::from(value))
    }
}

impl<F: Field> Neg for Expression<F> {
    type Output = Expression<F>;
    fn neg(self) -> Self::Output {
        Expression::Negated(Box::new(self))
    }
}

impl<F: Field> Neg for &Expression<F> {
    type Output = Expression<F>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

impl<F: Field> Add for Expression<F> {
    type Output = Expression<F>;
    fn add(self, rhs: Expression<F>) -> Expression<F> {
        if self.contains_simple_selector() || rhs.contains_simple_selector() {
            panic!("attempted to use a simple selector in an addition");
        }
        if self == Expression::Constant(F::ZERO) {
            return rhs;
        }
        if rhs == Expression::Constant(F::ZERO) {
            return self;
        }
        Expression::Sum(Box::new(self), Box::new(rhs))
    }
}

impl<'a, F: Field> Add for &'a Expression<F> {
    type Output = Expression<F>;
    fn add(self, rhs: &'a Expression<F>) -> Expression<F> {
        self.clone() + rhs.clone()
    }
}

impl<F: Field> Add<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;
    fn add(self, rhs: Expression<F>) -> Expression<F> {
        self.clone() + rhs
    }
}

impl<'a, F: Field> Add<&'a Expression<F>> for Expression<F> {
    type Output = Expression<F>;
    fn add(self, rhs: &'a Expression<F>) -> Expression<F> {
        self + rhs.clone()
    }
}

impl<F: Field> Sub for Expression<F> {
    type Output = Expression<F>;
    fn sub(self, rhs: Expression<F>) -> Expression<F> {
        if self.contains_simple_selector() || rhs.contains_simple_selector() {
            panic!("attempted to use a simple selector in a subtraction");
        }
        if self == Expression::Constant(F::ZERO) {
            return -rhs;
        }
        if rhs == Expression::Constant(F::ZERO) {
            return self;
        }
        Expression::Sum(Box::new(self), Box::new(-rhs))
    }
}

impl<'a, F: Field> Sub for &'a Expression<F> {
    type Output = Expression<F>;
    fn sub(self, rhs: &'a Expression<F>) -> Expression<F> {
        self.clone() - rhs.clone()
    }
}

impl<F: Field> Sub<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;
    fn sub(self, rhs: Expression<F>) -> Expression<F> {
        self.clone() - rhs
    }
}

impl<'a, F: Field> Sub<&'a Expression<F>> for Expression<F> {
    type Output = Expression<F>;
    fn sub(self, rhs: &'a Expression<F>) -> Expression<F> {
        self - rhs.clone()
    }
}

impl<F: Field> Mul for Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: Expression<F>) -> Expression<F> {
        if self.contains_simple_selector() && rhs.contains_simple_selector() {
            panic!("attempted to multiply two expressions containing simple selectors");
        }
        if self == Expression::Constant(F::ZERO) || rhs == Expression::Constant(F::ZERO) {
            return Expression::Constant(F::ZERO);
        }
        if self == Expression::Constant(F::ONE) {
            return rhs;
        }
        if rhs == Expression::Constant(F::ONE) {
            return self;
        }
        Expression::Product(Box::new(self), Box::new(rhs))
    }
}

impl<'a, F: Field> Mul for &'a Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: &'a Expression<F>) -> Expression<F> {
        self.clone() * rhs.clone()
    }
}

impl<F: Field> Mul<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: Expression<F>) -> Expression<F> {
        self.clone() * rhs
    }
}

impl<'a, F: Field> Mul<&'a Expression<F>> for Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: &'a Expression<F>) -> Expression<F> {
        self * rhs.clone()
    }
}

impl<F: Field> Mul<F> for Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: F) -> Expression<F> {
        Expression::Scaled(Box::new(self), rhs)
    }
}

impl<F: Field> Mul<F> for &Expression<F> {
    type Output = Expression<F>;
    fn mul(self, rhs: F) -> Expression<F> {
        self.clone() * rhs
    }
}

impl<F: Field> Sum<Self> for Expression<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc + x).unwrap_or(Expression::Constant(F::ZERO))
    }
}

impl<F: Field> Product<Self> for Expression<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc * x).unwrap_or(Expression::Constant(F::ONE))
    }
}

/// A "virtual cell" is a PLONK cell that has been queried at a particular
/// relative offset within a custom gate.
#[derive(Clone, Debug)]
pub struct VirtualCell {
    pub(crate) column: Column<Any>,
    pub(crate) rotation: Rotation,
}

impl<Col: Into<Column<Any>>> From<(Col, Rotation)> for VirtualCell {
    fn from((column, rotation): (Col, Rotation)) -> Self {
        VirtualCell {
            column: column.into(),
            rotation,
        }
    }
}

/// An individual polynomial constraint.
///
/// These are returned by the closures passed to
/// `ConstraintSystem::create_gate`.
#[derive(Debug)]
pub struct Constraint<F: Field> {
    name: String,
    poly: Expression<F>,
}

impl<F: Field> From<Expression<F>> for Constraint<F> {
    fn from(poly: Expression<F>) -> Self {
        Constraint {
            name: "".to_string(),
            poly,
        }
    }
}

impl<F: Field, S: AsRef<str>> From<(S, Expression<F>)> for Constraint<F> {
    fn from((name, poly): (S, Expression<F>)) -> Self {
        Constraint {
            name: name.as_ref().to_string(),
            poly,
        }
    }
}

impl<F: Field> From<Expression<F>> for Vec<Constraint<F>> {
    fn from(poly: Expression<F>) -> Self {
        vec![Constraint {
            name: "".to_string(),
            poly,
        }]
    }
}

#[derive(Debug)]
enum SelectorType {
    Multiplicative(Selector),
    Additive(Selector),
    None,
}

/// A set of polynomial constraints with a common selector.
///
/// ```
/// use ff::Field;
/// use midnight_curves::Fq as Scalar;
/// use midnight_proofs::{
///     plonk::{Constraints, Expression},
///     poly::Rotation,
/// };
/// # use midnight_proofs::plonk::ConstraintSystem;
///
/// # let mut meta = ConstraintSystem::<Scalar>::default();
/// let a = meta.advice_column();
/// let b = meta.advice_column();
/// let c = meta.advice_column();
/// let s_ternary = meta.selector();
///
/// meta.create_gate("foo", |meta| {
///     let next = meta.query_advice(a, Rotation::next());
///     let a = meta.query_advice(a, Rotation::cur());
///     let b = meta.query_advice(b, Rotation::cur());
///     let c = meta.query_advice(c, Rotation::cur());
///
///     let one_minus_a = Expression::Constant(Scalar::ONE) - a.clone();
///
///     Constraints::with_selector(
///         s_ternary,
///         vec![
///             ("a is boolean", a.clone() * one_minus_a.clone()),
///             ("next == a ? b : c", next - (a * b + one_minus_a * c)),
///         ],
///     )
/// });
/// ```
#[derive(Debug)]
pub struct Constraints<F: Field> {
    selector: SelectorType,
    constraints: Vec<Constraint<F>>,
}

impl<F: Field> Constraints<F> {
    /// Constructs a set of constraints that are controlled by the given
    /// selector.
    ///
    /// Each constraint `c` in `iterator` will be converted into the constraint
    /// `selector * c`.
    pub fn with_selector<I: Into<Constraint<F>>>(selector: Selector, constraints: Vec<I>) -> Self {
        Constraints {
            selector: SelectorType::Multiplicative(selector),
            constraints: constraints.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Constructs a set of constraints that are controlled by the given
    /// selector. The selector is involved additively (via a trash::argument)
    /// and thus, it does not increase the constraints degree.
    ///
    /// The enforced constraint will have the form
    /// `(sum_i r^i constraints[i]) - (1 - selector) * trash`, where `r` is
    /// a Fiat-Shamir challenge and `trash` is an unrestricted column.
    pub fn with_additive_selector<I: Into<Constraint<F>>>(
        selector: Selector,
        constraints: Vec<I>,
    ) -> Self {
        Constraints {
            selector: SelectorType::Additive(selector),
            constraints: constraints.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Constructs a set of constraints that do not have a selector and
    /// therefore are always enabled.
    pub fn without_selector<I: Into<Constraint<F>>>(constraints: Vec<I>) -> Self {
        Constraints {
            selector: SelectorType::None,
            constraints: constraints.into_iter().map(|c| c.into()).collect(),
        }
    }
}

/// Gate
#[derive(Clone, Debug)]
pub struct Gate<F: Field> {
    name: String,
    constraint_names: Vec<String>,
    polys: Vec<Expression<F>>,
    /// We track queried selectors separately from other cells, so that we can
    /// use them to trigger debug checks on gates.
    queried_selectors: Vec<Selector>,
    queried_cells: Vec<VirtualCell>,
}

impl<F: Field> Gate<F> {
    /// Returns the gate name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the name of the constraint at index `constraint_index`.
    pub fn constraint_name(&self, constraint_index: usize) -> &str {
        self.constraint_names[constraint_index].as_str()
    }

    /// Returns constraints of this gate
    pub fn polynomials(&self) -> &[Expression<F>] {
        &self.polys
    }

    pub(crate) fn queried_selectors(&self) -> &[Selector] {
        &self.queried_selectors
    }

    pub(crate) fn queried_cells(&self) -> &[VirtualCell] {
        &self.queried_cells
    }
}

/// This is a description of the circuit environment, such as the gate, column
/// and permutation arrangements.
#[derive(Debug, Clone)]
pub struct ConstraintSystem<F: Field> {
    pub(crate) num_fixed_columns: usize,
    pub(crate) num_advice_columns: usize,
    pub(crate) num_instance_columns: usize,
    pub(crate) num_selectors: usize,
    pub(crate) num_challenges: usize,

    /// Contains the index of each advice column that is left unblinded.
    pub(crate) unblinded_advice_columns: Vec<usize>,

    /// Contains the phase for each advice column. Should have same length as
    /// num_advice_columns.
    pub(crate) advice_column_phase: Vec<sealed::Phase>,
    /// Contains the phase for each challenge. Should have same length as
    /// num_challenges.
    pub(crate) challenge_phase: Vec<sealed::Phase>,

    pub(crate) gates: Vec<Gate<F>>,
    pub(crate) advice_queries: Vec<(Column<Advice>, Rotation)>,
    // Contains an integer for each advice column
    // identifying how many distinct queries it has
    // so far; should be same length as num_advice_columns.
    num_advice_queries: Vec<usize>,
    pub(crate) instance_queries: Vec<(Column<Instance>, Rotation)>,
    pub(crate) fixed_queries: Vec<(Column<Fixed>, Rotation)>,

    // Permutation argument for performing equality constraints
    pub(crate) permutation: permutation::Argument,

    // Vector of lookup arguments, where each corresponds to a sequence of
    // input expressions and a sequence of table expressions involved in the lookup.
    pub(crate) lookups: Vec<lookup::Argument<F>>,

    // Vector of trash arguments. Each contains a selector and a sequence of expressions
    // that will all be enforced to be zero when the selector is enabled. This is done
    // via an argument that involves the selector additively (instead of multiplicatively)
    // with the help of an extra so-called trash column.
    pub(crate) trashcans: Vec<trash::Argument<F>>,

    // List of indexes of Fixed columns which are associated to a circuit-general Column tied to
    // their annotation.
    pub(crate) general_column_annotations: HashMap<metadata::Column, String>,

    // Vector of fixed columns, which can be used to store constant values
    // that are copied into advice columns.
    pub(crate) constants: Vec<Column<Fixed>>,

    pub(crate) minimum_degree: Option<usize>,
}

/// Represents the minimal parameters that determine a `ConstraintSystem`.
#[allow(dead_code)]
pub struct PinnedConstraintSystem<'a, F: Field> {
    num_fixed_columns: &'a usize,
    num_advice_columns: &'a usize,
    num_instance_columns: &'a usize,
    num_selectors: &'a usize,
    num_challenges: &'a usize,
    advice_column_phase: &'a Vec<sealed::Phase>,
    challenge_phase: &'a Vec<sealed::Phase>,
    gates: PinnedGates<'a, F>,
    advice_queries: &'a Vec<(Column<Advice>, Rotation)>,
    instance_queries: &'a Vec<(Column<Instance>, Rotation)>,
    fixed_queries: &'a Vec<(Column<Fixed>, Rotation)>,
    permutation: &'a permutation::Argument,
    lookups: &'a Vec<lookup::Argument<F>>,
    trashcans: &'a Vec<trash::Argument<F>>,
    constants: &'a Vec<Column<Fixed>>,
    minimum_degree: &'a Option<usize>,
}

impl<F: Field> std::fmt::Debug for PinnedConstraintSystem<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self {
            num_fixed_columns,
            num_advice_columns,
            num_instance_columns,
            num_selectors,
            num_challenges,
            advice_column_phase,
            challenge_phase,
            gates,
            advice_queries,
            instance_queries,
            fixed_queries,
            permutation,
            lookups,
            trashcans,
            constants,
            minimum_degree,
        } = &self;
        let mut debug_struct = f.debug_struct("PinnedConstraintSystem");
        debug_struct
            .field("num_fixed_columns", num_fixed_columns)
            .field("num_advice_columns", num_advice_columns)
            .field("num_instance_columns", num_instance_columns)
            .field("num_selectors", num_selectors);
        // Only show multi-phase related fields if it's used.
        if *num_challenges > &0 {
            debug_struct
                .field("num_challenges", num_challenges)
                .field("advice_column_phase", advice_column_phase)
                .field("challenge_phase", challenge_phase);
        }
        debug_struct
            .field("gates", &gates)
            .field("advice_queries", advice_queries)
            .field("instance_queries", instance_queries)
            .field("fixed_queries", fixed_queries)
            .field("permutation", permutation)
            .field("lookups", lookups)
            .field("trashcans", trashcans);
        debug_struct
            .field("constants", constants)
            .field("minimum_degree", minimum_degree);
        debug_struct.finish()
    }
}

struct PinnedGates<'a, F: Field>(&'a Vec<Gate<F>>);

impl<F: Field> std::fmt::Debug for PinnedGates<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_list()
            .entries(self.0.iter().flat_map(|gate| gate.polynomials().iter()))
            .finish()
    }
}

impl<F: Field> Default for ConstraintSystem<F> {
    fn default() -> ConstraintSystem<F> {
        ConstraintSystem {
            num_fixed_columns: 0,
            num_advice_columns: 0,
            num_instance_columns: 0,
            num_selectors: 0,
            num_challenges: 0,
            unblinded_advice_columns: Vec::new(),
            advice_column_phase: Vec::new(),
            challenge_phase: Vec::new(),
            gates: vec![],
            fixed_queries: Vec::new(),
            advice_queries: Vec::new(),
            num_advice_queries: Vec::new(),
            instance_queries: Vec::new(),
            permutation: permutation::Argument::new(),
            lookups: Vec::new(),
            trashcans: Vec::new(),
            general_column_annotations: HashMap::new(),
            constants: vec![],
            minimum_degree: None,
        }
    }
}

impl<F: Field> ConstraintSystem<F> {
    /// Obtain a pinned version of this constraint system; a structure with the
    /// minimal parameters needed to determine the rest of the constraint
    /// system.
    pub fn pinned(&self) -> PinnedConstraintSystem<'_, F> {
        PinnedConstraintSystem {
            num_fixed_columns: &self.num_fixed_columns,
            num_advice_columns: &self.num_advice_columns,
            num_instance_columns: &self.num_instance_columns,
            num_selectors: &self.num_selectors,
            num_challenges: &self.num_challenges,
            advice_column_phase: &self.advice_column_phase,
            challenge_phase: &self.challenge_phase,
            gates: PinnedGates(&self.gates),
            fixed_queries: &self.fixed_queries,
            advice_queries: &self.advice_queries,
            instance_queries: &self.instance_queries,
            permutation: &self.permutation,
            lookups: &self.lookups,
            trashcans: &self.trashcans,
            constants: &self.constants,
            minimum_degree: &self.minimum_degree,
        }
    }

    /// Enables this fixed column to be used for global constant assignments.
    ///
    /// # Side-effects
    ///
    /// The column will be equality-enabled.
    pub fn enable_constant(&mut self, column: Column<Fixed>) {
        if !self.constants.contains(&column) {
            self.constants.push(column);
            self.enable_equality(column);
        }
    }

    /// Enable the ability to enforce equality over cells in this column
    pub fn enable_equality<C: Into<Column<Any>>>(&mut self, column: C) {
        let column = column.into();
        self.query_any_index(column, Rotation::cur());
        self.permutation.add_column(column);
    }

    /// Add a lookup argument for some input expressions and table columns.
    ///
    /// `table_map` returns a map between input expressions and the table
    /// columns they need to match.
    pub fn lookup<S: AsRef<str>>(
        &mut self,
        name: S,
        table_map: impl FnOnce(&mut VirtualCells<'_, F>) -> Vec<(Expression<F>, TableColumn)>,
    ) -> usize {
        let mut cells = VirtualCells::new(self);
        let table_map = table_map(&mut cells)
            .into_iter()
            .map(|(mut input, table)| {
                if input.contains_simple_selector() {
                    panic!("expression containing simple selector supplied to lookup argument");
                }
                let mut table = cells.query_fixed(table.inner(), Rotation::cur());
                input.query_cells(&mut cells);
                table.query_cells(&mut cells);
                (input, table)
            })
            .collect();
        let index = self.lookups.len();

        self.lookups.push(lookup::Argument::new(name.as_ref(), table_map));

        index
    }

    /// Add a lookup argument for some input expressions and table expressions.
    ///
    /// `table_map` returns a map between input expressions and the table
    /// expressions they need to match.
    pub fn lookup_any<S: AsRef<str>>(
        &mut self,
        name: S,
        table_map: impl FnOnce(&mut VirtualCells<'_, F>) -> Vec<(Expression<F>, Expression<F>)>,
    ) -> usize {
        let mut cells = VirtualCells::new(self);
        let table_map = table_map(&mut cells)
            .into_iter()
            .map(|(mut input, mut table)| {
                if input.contains_simple_selector() {
                    panic!("expression containing simple selector supplied to lookup argument");
                }
                if table.contains_simple_selector() {
                    panic!("expression containing simple selector supplied to lookup argument");
                }
                input.query_cells(&mut cells);
                table.query_cells(&mut cells);
                (input, table)
            })
            .collect();
        let index = self.lookups.len();

        self.lookups.push(lookup::Argument::new(name.as_ref(), table_map));

        index
    }

    fn query_fixed_index(&mut self, column: Column<Fixed>, at: Rotation) -> usize {
        // Return existing query, if it exists
        for (index, fixed_query) in self.fixed_queries.iter().enumerate() {
            if fixed_query == &(column, at) {
                return index;
            }
        }

        // Make a new query
        let index = self.fixed_queries.len();
        self.fixed_queries.push((column, at));

        index
    }

    pub(crate) fn query_advice_index(&mut self, column: Column<Advice>, at: Rotation) -> usize {
        // Return existing query, if it exists
        for (index, advice_query) in self.advice_queries.iter().enumerate() {
            if advice_query == &(column, at) {
                return index;
            }
        }

        // Make a new query
        let index = self.advice_queries.len();
        self.advice_queries.push((column, at));
        self.num_advice_queries[column.index] += 1;

        index
    }

    fn query_instance_index(&mut self, column: Column<Instance>, at: Rotation) -> usize {
        // Return existing query, if it exists
        for (index, instance_query) in self.instance_queries.iter().enumerate() {
            if instance_query == &(column, at) {
                return index;
            }
        }

        // Make a new query
        let index = self.instance_queries.len();
        self.instance_queries.push((column, at));

        index
    }

    fn query_any_index(&mut self, column: Column<Any>, at: Rotation) -> usize {
        match column.column_type() {
            Any::Advice(_) => {
                self.query_advice_index(Column::<Advice>::try_from(column).unwrap(), at)
            }
            Any::Fixed => self.query_fixed_index(Column::<Fixed>::try_from(column).unwrap(), at),
            Any::Instance => {
                self.query_instance_index(Column::<Instance>::try_from(column).unwrap(), at)
            }
        }
    }

    pub(crate) fn get_advice_query_index(&self, column: Column<Advice>, at: Rotation) -> usize {
        for (index, advice_query) in self.advice_queries.iter().enumerate() {
            if advice_query == &(column, at) {
                return index;
            }
        }

        panic!("get_advice_query_index called for non-existent query");
    }

    pub(crate) fn get_fixed_query_index(&self, column: Column<Fixed>, at: Rotation) -> usize {
        for (index, fixed_query) in self.fixed_queries.iter().enumerate() {
            if fixed_query == &(column, at) {
                return index;
            }
        }

        panic!("get_fixed_query_index called for non-existent query");
    }

    pub(crate) fn get_instance_query_index(&self, column: Column<Instance>, at: Rotation) -> usize {
        for (index, instance_query) in self.instance_queries.iter().enumerate() {
            if instance_query == &(column, at) {
                return index;
            }
        }

        panic!("get_instance_query_index called for non-existent query");
    }

    pub(crate) fn get_any_query_index(&self, column: Column<Any>, at: Rotation) -> usize {
        match column.column_type() {
            Any::Advice(_) => {
                self.get_advice_query_index(Column::<Advice>::try_from(column).unwrap(), at)
            }
            Any::Fixed => {
                self.get_fixed_query_index(Column::<Fixed>::try_from(column).unwrap(), at)
            }
            Any::Instance => {
                self.get_instance_query_index(Column::<Instance>::try_from(column).unwrap(), at)
            }
        }
    }

    /// Sets the minimum degree required by the circuit, which can be set to a
    /// larger amount than actually needed. This can be used, for example, to
    /// force the permutation argument to involve more columns in the same set.
    pub fn set_minimum_degree(&mut self, degree: usize) {
        self.minimum_degree = Some(degree);
    }

    /// Creates a new gate.
    ///
    /// # Panics
    ///
    /// If `constraints` returns an empty iterator.
    /// (Gates are required to contain polynomial constraints.)
    pub fn create_gate(
        &mut self,
        name: &'static str,
        constraints: impl FnOnce(&mut VirtualCells<'_, F>) -> Constraints<F>,
    ) {
        let mut cells = VirtualCells::new(self);
        let constraints = constraints(&mut cells);

        assert!(
            !constraints.constraints.is_empty(),
            "Gates must contain at least one constraint."
        );

        let (constraint_names, polys): (_, Vec<_>) = cells
            .apply_selector_to_constraints(constraints)
            .into_iter()
            .map(|mut c: Constraint<F>| {
                c.poly.query_cells(&mut cells);
                (c.name, c.poly)
            })
            .unzip();

        let queried_selectors = cells.queried_selectors;
        let queried_cells = cells.queried_cells;

        self.gates.push(Gate {
            name: name.into(),
            constraint_names,
            polys,
            queried_selectors,
            queried_cells,
        });
    }

    /// Does not combine selectors and directly replaces them everywhere with
    /// fixed columns.
    pub fn directly_convert_selectors_to_fixed(
        mut self,
        selectors: Vec<Vec<bool>>,
    ) -> (Self, Vec<Vec<F>>) {
        // The number of provided selector assignments must be the number we
        // counted for this constraint system.
        assert_eq!(selectors.len(), self.num_selectors);

        let (polys, selector_replacements): (Vec<_>, Vec<_>) = selectors
            .into_iter()
            .map(|selector| {
                let poly =
                    selector.iter().map(|b| if *b { F::ONE } else { F::ZERO }).collect::<Vec<_>>();
                let column = self.fixed_column();
                let rotation = Rotation::cur();
                let expr = Expression::Fixed(FixedQuery {
                    index: Some(self.query_fixed_index(column, rotation)),
                    column_index: column.index,
                    rotation,
                });
                (poly, expr)
            })
            .unzip();

        self.replace_selectors_with_fixed(&selector_replacements);
        self.num_selectors = 0;

        (self, polys)
    }

    fn replace_selectors_with_fixed(&mut self, selector_replacements: &[Expression<F>]) {
        fn replace_selectors<F: Field>(
            expr: &mut Expression<F>,
            selector_replacements: &[Expression<F>],
            must_be_nonsimple: bool,
        ) {
            *expr = expr.evaluate(
                &|constant| Expression::Constant(constant),
                &|selector| {
                    if must_be_nonsimple {
                        // Simple selectors are prohibited from appearing in
                        // expressions in the lookup or trash arguments.
                        assert!(!selector.is_simple());
                    }

                    selector_replacements[selector.0].clone()
                },
                &|query| Expression::Fixed(query),
                &|query| Expression::Advice(query),
                &|query| Expression::Instance(query),
                &|challenge| Expression::Challenge(challenge),
                &|a| -a,
                &|a, b| a + b,
                &|a, b| a * b,
                &|a, f| a * f,
            );
        }

        // Substitute selectors for the real fixed columns in all gates.
        for expr in self.gates.iter_mut().flat_map(|gate| gate.polys.iter_mut()) {
            replace_selectors(expr, selector_replacements, false);
        }

        // Substitute non-simple selectors for the real fixed columns in all
        // lookup expressions.
        for expr in self.lookups.iter_mut().flat_map(|lookup| {
            lookup.input_expressions.iter_mut().chain(lookup.table_expressions.iter_mut())
        }) {
            replace_selectors(expr, selector_replacements, true);
        }

        // Substitute selectors in all trash arguments.
        for trash in self.trashcans.iter_mut() {
            replace_selectors(&mut trash.selector, selector_replacements, true);
            for expr in trash.constraint_expressions.iter_mut() {
                replace_selectors(expr, selector_replacements, true);
            }
        }
    }

    /// Allocate a new (simple) selector. Simple selectors cannot be added to
    /// expressions nor multiplied by other expressions containing simple
    /// selectors. Also, simple selectors may not appear in lookup argument
    /// inputs.
    pub fn selector(&mut self) -> Selector {
        let index = self.num_selectors;
        self.num_selectors += 1;
        Selector(index, true)
    }

    /// Allocate a new complex selector that can appear anywhere
    /// within expressions.
    pub fn complex_selector(&mut self) -> Selector {
        let index = self.num_selectors;
        self.num_selectors += 1;
        Selector(index, false)
    }

    /// Allocates a new fixed column that can be used in a lookup table.
    pub fn lookup_table_column(&mut self) -> TableColumn {
        TableColumn {
            inner: self.fixed_column(),
        }
    }

    /// Annotate a Lookup column.
    pub fn annotate_lookup_column<A, AR>(&mut self, column: TableColumn, annotation: A)
    where
        A: Fn() -> AR,
        AR: Into<String>,
    {
        // We don't care if the table has already an annotation. If it's the case we
        // keep the new one.
        self.general_column_annotations.insert(
            metadata::Column::from((Any::Fixed, column.inner().index)),
            annotation().into(),
        );
    }

    /// Annotate an Instance column.
    pub fn annotate_lookup_any_column<A, AR, T>(&mut self, column: T, annotation: A)
    where
        A: Fn() -> AR,
        AR: Into<String>,
        T: Into<Column<Any>>,
    {
        let col_any = column.into();
        // We don't care if the table has already an annotation. If it's the case we
        // keep the new one.
        self.general_column_annotations.insert(
            metadata::Column::from((col_any.column_type, col_any.index)),
            annotation().into(),
        );
    }

    /// Allocate a new fixed column
    pub fn fixed_column(&mut self) -> Column<Fixed> {
        let tmp = Column {
            index: self.num_fixed_columns,
            column_type: Fixed,
        };
        self.num_fixed_columns += 1;
        tmp
    }

    /// Allocate a new unblinded advice column at `FirstPhase`
    pub fn unblinded_advice_column(&mut self) -> Column<Advice> {
        self.unblinded_advice_column_in(FirstPhase)
    }

    /// Allocate a new advice column at `FirstPhase`
    pub fn advice_column(&mut self) -> Column<Advice> {
        self.advice_column_in(FirstPhase)
    }

    /// Allocate a new unblinded advice column in given phase. This allows for
    /// the generation of deterministic commitments to advice columns
    /// which can be used to split large circuits into smaller ones, whose
    /// proofs can then be "joined" together by their common witness
    /// commitments.
    pub fn unblinded_advice_column_in<P: Phase>(&mut self, phase: P) -> Column<Advice> {
        let phase = phase.to_sealed();
        if let Some(previous_phase) = phase.prev() {
            self.assert_phase_exists(
                previous_phase,
                format!("Column<Advice> in later phase {phase:?}").as_str(),
            );
        }

        let tmp = Column {
            index: self.num_advice_columns,
            column_type: Advice { phase },
        };
        self.unblinded_advice_columns.push(tmp.index);
        self.num_advice_columns += 1;
        self.num_advice_queries.push(0);
        self.advice_column_phase.push(phase);
        tmp
    }

    /// Allocate a new advice column in given phase
    ///
    /// # Panics
    ///
    /// If the previous phase does not have advice columns allocated.
    pub fn advice_column_in<P: Phase>(&mut self, phase: P) -> Column<Advice> {
        let phase = phase.to_sealed();
        if let Some(previous_phase) = phase.prev() {
            self.assert_phase_exists(
                previous_phase,
                format!("Column<Advice> in later phase {phase:?}").as_str(),
            );
        }

        let tmp = Column {
            index: self.num_advice_columns,
            column_type: Advice { phase },
        };
        self.num_advice_columns += 1;
        self.num_advice_queries.push(0);
        self.advice_column_phase.push(phase);
        tmp
    }

    /// Allocate a new instance column
    pub fn instance_column(&mut self) -> Column<Instance> {
        let tmp = Column {
            index: self.num_instance_columns,
            column_type: Instance,
        };
        self.num_instance_columns += 1;
        tmp
    }

    /// Requests a challenge that is usable after the given phase.
    ///
    /// # Panics
    ///
    /// If the given phase does not have advice column allocated.
    pub fn challenge_usable_after<P: Phase>(&mut self, phase: P) -> Challenge {
        let phase = phase.to_sealed();
        self.assert_phase_exists(
            phase,
            format!("Challenge usable after phase {phase:?}").as_str(),
        );

        let tmp = Challenge {
            index: self.num_challenges,
            phase,
        };
        self.num_challenges += 1;
        self.challenge_phase.push(phase);
        tmp
    }

    /// Helper funciotn to assert phase exists, to make sure phase-aware
    /// resources are allocated in order, and to avoid any phase to be
    /// skipped accidentally to cause unexpected issue in the future.
    fn assert_phase_exists(&self, phase: sealed::Phase, resource: &str) {
        self.advice_column_phase
            .iter()
            .find(|advice_column_phase| **advice_column_phase == phase)
            .unwrap_or_else(|| {
                panic!(
                    "No Column<Advice> is used in phase {phase:?} while allocating a new {resource:?}"
                )
            });
    }

    /// Return an iterator over the phases of the constraint system.
    pub fn phases(&self) -> impl Iterator<Item = sealed::Phase> {
        let max_phase =
            self.advice_column_phase.iter().max().map(|phase| phase.0).unwrap_or_default();
        (0..=max_phase).map(sealed::Phase)
    }

    /// Compute the degree of the constraint system (the maximum degree of all
    /// constraints).
    pub fn degree(&self) -> usize {
        [
            Some(self.permutation.required_degree()),
            self.lookups.iter().map(|l| l.required_degree()).max(),
            self.trashcans.iter().map(|l| l.required_degree()).max(),
            self.gates
                .iter()
                .flat_map(|gate| gate.polynomials().iter().map(|poly| poly.degree()))
                .max(),
            self.minimum_degree,
        ]
        .iter()
        .filter_map(|&d| d)
        .max()
        .unwrap_or(1)
    }

    /// Compute the number of blinding factors necessary to perfectly blind
    /// each of the prover's witness polynomials.
    pub fn blinding_factors(&self) -> usize {
        // All of the prover's advice columns are evaluated at no more than
        let factors = *self.num_advice_queries.iter().max().unwrap_or(&1);
        // distinct points during gate checks.

        // - The permutation argument witness polynomials are evaluated at most 3 times.
        // - Each lookup argument has independent witness polynomials, and they are
        //   evaluated at most 2 times.
        let factors = std::cmp::max(3, factors);

        // Each trash argument gives away 1 evaluation of a linear combination of some
        // witness polynomials (the evaluation of the trash column).
        let factors = factors + self.trashcans.len();

        // Each polynomial is evaluated at most an additional time during
        // multiopen (at x_3 to produce q_evals):
        let factors = factors + 1;

        // h(x) is derived by the other evaluations so it does not reveal
        // anything; in fact it does not even appear in the proof.

        // h(x_3) is also not revealed; the verifier only learns a single
        // evaluation of a polynomial in x_1 which has h(x_3) and another random
        // polynomial evaluated at x_3 as coefficients -- this random polynomial
        // is "random_poly" in the vanishing argument.

        // Add an additional blinding factor as a slight defense against
        // off-by-one errors.
        let factors = factors + 1;
        if factors > i32::MAX as usize {
            panic!("Number of blinding factors overflowed max expected value");
        }
        factors
    }

    /// Returns the minimum necessary rows that need to exist in order to
    /// account for e.g. blinding factors.
    pub fn minimum_rows(&self) -> usize {
        self.blinding_factors() // m blinding factors
            + 1 // for l_{-(m + 1)} (l_last)
            + 1 // for l_0 (just for extra breathing room for the permutation
                // argument, to essentially force a separation in the
                // permutation polynomial between the roles of l_last, l_0
                // and the interstitial values.)
            + 1 // for at least one row
    }

    /// Returns number of fixed columns
    pub fn num_fixed_columns(&self) -> usize {
        self.num_fixed_columns
    }

    /// Returns number of advice columns
    pub fn num_advice_columns(&self) -> usize {
        self.num_advice_columns
    }

    /// Returns number of instance columns
    pub fn num_instance_columns(&self) -> usize {
        self.num_instance_columns
    }

    /// Returns number of selectors
    pub fn num_selectors(&self) -> usize {
        self.num_selectors
    }

    /// Returns number of challenges
    pub fn num_challenges(&self) -> usize {
        self.num_challenges
    }

    /// Returns phase of advice columns
    pub fn advice_column_phase(&self) -> Vec<u8> {
        self.advice_column_phase.iter().map(|phase| phase.0).collect()
    }

    /// Returns phase of challenges
    pub fn challenge_phase(&self) -> Vec<u8> {
        self.challenge_phase.iter().map(|phase| phase.0).collect()
    }

    /// Returns gates
    pub fn gates(&self) -> &Vec<Gate<F>> {
        &self.gates
    }

    /// Returns general column annotations
    pub fn general_column_annotations(&self) -> &HashMap<metadata::Column, String> {
        &self.general_column_annotations
    }

    /// Returns advice queries
    pub fn advice_queries(&self) -> &Vec<(Column<Advice>, Rotation)> {
        &self.advice_queries
    }

    /// Returns instance queries
    pub fn instance_queries(&self) -> &Vec<(Column<Instance>, Rotation)> {
        &self.instance_queries
    }

    /// Returns fixed queries
    pub fn fixed_queries(&self) -> &Vec<(Column<Fixed>, Rotation)> {
        &self.fixed_queries
    }

    /// Returns permutation argument
    pub fn permutation(&self) -> &permutation::Argument {
        &self.permutation
    }

    /// Returns lookup arguments
    pub fn lookups(&self) -> &Vec<lookup::Argument<F>> {
        &self.lookups
    }

    /// Returns trash arguments
    pub fn trashcans(&self) -> &Vec<trash::Argument<F>> {
        &self.trashcans
    }

    /// Returns constants
    pub fn constants(&self) -> &Vec<Column<Fixed>> {
        &self.constants
    }
}

/// Exposes the "virtual cells" that can be queried while creating a custom gate
/// or lookup table.
#[derive(Debug)]
pub struct VirtualCells<'a, F: Field> {
    meta: &'a mut ConstraintSystem<F>,
    queried_selectors: Vec<Selector>,
    queried_cells: Vec<VirtualCell>,
}

impl<'a, F: Field> VirtualCells<'a, F> {
    fn new(meta: &'a mut ConstraintSystem<F>) -> Self {
        VirtualCells {
            meta,
            queried_selectors: vec![],
            queried_cells: vec![],
        }
    }

    /// Query a selector at the current position.
    pub fn query_selector(&mut self, selector: Selector) -> Expression<F> {
        self.queried_selectors.push(selector);
        Expression::Selector(selector)
    }

    /// Query a fixed column at a relative position
    pub fn query_fixed(&mut self, column: Column<Fixed>, at: Rotation) -> Expression<F> {
        self.queried_cells.push((column, at).into());
        Expression::Fixed(FixedQuery {
            index: Some(self.meta.query_fixed_index(column, at)),
            column_index: column.index,
            rotation: at,
        })
    }

    /// Query an advice column at a relative position
    pub fn query_advice(&mut self, column: Column<Advice>, at: Rotation) -> Expression<F> {
        self.queried_cells.push((column, at).into());
        Expression::Advice(AdviceQuery {
            index: Some(self.meta.query_advice_index(column, at)),
            column_index: column.index,
            rotation: at,
            phase: column.column_type().phase,
        })
    }

    /// Query an instance column at a relative position
    pub fn query_instance(&mut self, column: Column<Instance>, at: Rotation) -> Expression<F> {
        self.queried_cells.push((column, at).into());
        Expression::Instance(InstanceQuery {
            index: Some(self.meta.query_instance_index(column, at)),
            column_index: column.index,
            rotation: at,
        })
    }

    /// Query an Any column at a relative position
    pub fn query_any<C: Into<Column<Any>>>(&mut self, column: C, at: Rotation) -> Expression<F> {
        let column = column.into();
        match column.column_type() {
            Any::Advice(_) => self.query_advice(Column::<Advice>::try_from(column).unwrap(), at),
            Any::Fixed => self.query_fixed(Column::<Fixed>::try_from(column).unwrap(), at),
            Any::Instance => self.query_instance(Column::<Instance>::try_from(column).unwrap(), at),
        }
    }

    /// Query a challenge
    pub fn query_challenge(&mut self, challenge: Challenge) -> Expression<F> {
        Expression::Challenge(challenge)
    }

    fn apply_selector_to_constraints(&mut self, c: Constraints<F>) -> Vec<Constraint<F>> {
        match c.selector {
            SelectorType::Multiplicative(s) => {
                let q = self.query_selector(s);
                c.constraints
                    .into_iter()
                    .map(|constraint| Constraint {
                        name: constraint.name,
                        poly: q.clone() * constraint.poly,
                    })
                    .collect()
            }
            SelectorType::Additive(s) => {
                let q = self.query_selector(s);
                let names: Vec<_> = c.constraints.iter().map(|c| c.name.clone()).collect();
                let polys: Vec<_> = c.constraints.into_iter().map(|c| c.poly).collect();
                (self.meta.trashcans).push(Argument::new(names.join("&"), q, polys));
                vec![]
            }
            SelectorType::None => c.constraints,
        }
    }
}

#[cfg(test)]
mod tests {
    use midnight_curves::Fq as Scalar;

    use super::Expression;

    #[test]
    fn iter_sum() {
        let exprs: Vec<Expression<Scalar>> = vec![
            Expression::from(1),
            Expression::from(2),
            Expression::from(3),
        ];
        let happened: Expression<Scalar> = exprs.into_iter().sum();
        let expected: Expression<Scalar> = Expression::Sum(
            Box::new(Expression::Sum(
                Box::new(Expression::from(1)),
                Box::new(Expression::from(2)),
            )),
            Box::new(Expression::from(3)),
        );

        assert_eq!(happened, expected);
    }

    #[test]
    fn iter_product() {
        let exprs: Vec<Expression<Scalar>> = vec![
            Expression::from(2),
            Expression::from(3),
            Expression::from(6),
        ];
        let happened: Expression<Scalar> = exprs.into_iter().product();
        let expected: Expression<Scalar> = Expression::Product(
            Box::new(Expression::Product(
                Box::new(Expression::from(2)),
                Box::new(Expression::from(3)),
            )),
            Box::new(Expression::from(6)),
        );

        assert_eq!(happened, expected);
    }
}
```