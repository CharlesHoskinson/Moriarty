# Midnight curves 0.3.1 G1 implementations
> Source: https://docs.rs/crate/midnight-curves/0.3.1/source/src/bls12_381/g1.rs
> Collected: 2026-10-26
> Published: Unknown

midnight-curves 0.3.1 - Docs.rs

[Docs.rs](/)

* [midnight-curves-0.3.1](/crate/midnight-curves/0.3.1 "Implementation of BLS12 381 and Jubjub curves.")
* [Platform](#)
  + [x86\_64-unknown-linux-gnu](/midnight-curves/0.3.1/midnight_curves/)
* [Feature flags](/crate/midnight-curves/0.3.1/features "Browse available feature flags of midnight-curves-0.3.1")

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

midnight-curves 0.3.1
=====================

Implementation of BLS12 381 and Jubjub curves.

* [Crate](/crate/midnight-curves/0.3.1)
* [Source](/crate/midnight-curves/0.3.1/source/src/bls12_381/g1.rs)
* [Builds](/crate/midnight-curves/0.3.1/builds)
* [Feature flags](/crate/midnight-curves/0.3.1/features)

[Documentation](/midnight-curves/0.3.1/midnight_curves/)

* Hide files
* [..](../)
* [bls\_pairing.rs](./bls_pairing.rs)
* [fp.rs](./fp.rs)
* [fp12.rs](./fp12.rs)
* [fp2.rs](./fp2.rs)
* [fp6.rs](./fp6.rs)
* [fq.rs](./fq.rs)
* [g1.rs](./g1.rs)
* [g2.rs](./g2.rs)
* [gt.rs](./gt.rs)
* [mod.rs](./mod.rs)

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
```

```
//! An implementation of the $\mathbb{G}_1$ group of BLS12-381.
#![allow(unused_variables)]

use core::{
    borrow::Borrow,
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use std::{convert::TryInto, io::Read};

use blst::*;
use ff::Field;
use group::{
    prime::{PrimeCurve, PrimeCurveAffine, PrimeGroup},
    Curve, Group, GroupEncoding, UncompressedEncoding, WnafGroup,
};
use pairing::{Engine, PairingCurveAffine};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use super::{
    fp::{Fp, ZETA_BASE},
    Bls12, Fq, G2Affine, Gt,
};
use crate::{
    curve::{Coordinates, CurveAffine, CurveExt},
    serde_traits::SerdeObject,
};

/// This is an element of $\mathbb{G}_1$ represented in the affine coordinate
/// space. It is ideal to keep elements in this representation to reduce memory
/// usage and improve performance through the use of mixed curve model
/// arithmetic.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct G1Affine(pub(crate) blst_p1_affine);

const COMPRESSED_SIZE: usize = 48;
const UNCOMPRESSED_SIZE: usize = 96;

pub const A: Fp = Fp::ZERO;
pub const B: Fp = Fp(blst_fp {
    // 0x04 in Montgomery form.
    l: [
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x09d645513d83de7e,
    ],
});

impl fmt::Debug for G1Affine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let is_ident: bool = self.is_identity().into();
        f.debug_struct("G1Affine")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("infinity", &is_ident)
            .finish()
    }
}

impl fmt::Display for G1Affine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_identity().into() {
            write!(f, "G1Affine(Infinity)")
        } else {
            write!(f, "G1Affine(x={}, y={})", self.x(), self.y())
        }
    }
}

impl core::hash::Hash for G1Affine {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.x.l.hash::<H>(state);
        self.0.y.l.hash::<H>(state)
    }
}

impl Default for G1Affine {
    fn default() -> G1Affine {
        G1Affine::identity()
    }
}

impl From<&G1Projective> for G1Affine {
    fn from(p: &G1Projective) -> G1Affine {
        let mut out = blst_p1_affine::default();

        unsafe { blst_p1_to_affine(&mut out, &p.0) };

        G1Affine(out)
    }
}

impl From<G1Projective> for G1Affine {
    fn from(p: G1Projective) -> G1Affine {
        G1Affine::from(&p)
    }
}

impl AsRef<blst_p1_affine> for G1Affine {
    fn as_ref(&self) -> &blst_p1_affine {
        &self.0
    }
}

impl AsMut<blst_p1_affine> for G1Affine {
    fn as_mut(&mut self) -> &mut blst_p1_affine {
        &mut self.0
    }
}

impl Eq for G1Affine {}
impl PartialEq for G1Affine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { blst_p1_affine_is_equal(&self.0, &other.0) }
    }
}

impl Neg for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn neg(self) -> G1Projective {
        -*self
    }
}

impl Neg for G1Projective {
    type Output = G1Projective;

    #[inline]
    fn neg(mut self) -> G1Projective {
        unsafe { blst_p1_cneg(&mut self.0, true) };
        self
    }
}

impl Neg for &G1Affine {
    type Output = G1Affine;

    #[inline]
    fn neg(self) -> G1Affine {
        -*self
    }
}

impl Neg for G1Affine {
    type Output = G1Affine;

    #[inline]
    fn neg(mut self) -> G1Affine {
        // Missing for affine in blst
        if (!self.is_identity()).into() {
            unsafe {
                blst_fp_cneg(&mut self.0.y, &self.0.y, true);
            }
        }
        self
    }
}

impl Add<&G1Projective> for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn add(self, rhs: &G1Projective) -> G1Projective {
        let mut out = blst_p1::default();
        unsafe { blst_p1_add_or_double(&mut out, &self.0, &rhs.0) };
        G1Projective(out)
    }
}

impl Add<&G1Projective> for &G1Affine {
    type Output = G1Projective;

    #[inline]
    fn add(self, rhs: &G1Projective) -> G1Projective {
        rhs.add_mixed(self)
    }
}

impl Add<&G1Affine> for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn add(self, rhs: &G1Affine) -> G1Projective {
        self.add_mixed(rhs)
    }
}

impl Sub<&G1Projective> for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn sub(self, rhs: &G1Projective) -> G1Projective {
        self + (-rhs)
    }
}

impl Sub<&G1Projective> for &G1Affine {
    type Output = G1Projective;

    #[inline]
    fn sub(self, rhs: &G1Projective) -> G1Projective {
        self + (-rhs)
    }
}

impl Sub<&G1Affine> for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn sub(self, rhs: &G1Affine) -> G1Projective {
        self + (-rhs)
    }
}

impl AddAssign<&G1Projective> for G1Projective {
    #[inline]
    fn add_assign(&mut self, rhs: &G1Projective) {
        unsafe { blst_p1_add_or_double(&mut self.0, &self.0, &rhs.0) };
    }
}

impl SubAssign<&G1Projective> for G1Projective {
    #[inline]
    fn sub_assign(&mut self, rhs: &G1Projective) {
        *self += &-rhs;
    }
}

impl AddAssign<&G1Affine> for G1Projective {
    #[inline]
    fn add_assign(&mut self, rhs: &G1Affine) {
        unsafe { blst_p1_add_or_double_affine(&mut self.0, &self.0, &rhs.0) };
    }
}

impl SubAssign<&G1Affine> for G1Projective {
    #[inline]
    fn sub_assign(&mut self, rhs: &G1Affine) {
        *self += &-rhs;
    }
}

impl Mul<&Fq> for &G1Projective {
    type Output = G1Projective;

    fn mul(self, scalar: &Fq) -> Self::Output {
        self.multiply(scalar)
    }
}

impl Mul<&Fq> for &G1Affine {
    type Output = G1Projective;

    fn mul(self, scalar: &Fq) -> Self::Output {
        G1Projective::from(self).multiply(scalar)
    }
}

impl MulAssign<&Fq> for G1Projective {
    #[inline]
    fn mul_assign(&mut self, rhs: &Fq) {
        *self = *self * rhs;
    }
}

impl MulAssign<&Fq> for G1Affine {
    #[inline]
    fn mul_assign(&mut self, rhs: &Fq) {
        *self = (*self * rhs).into();
    }
}

impl_add_sub!(G1Projective);
impl_add_sub!(G1Projective, G1Affine);
impl_add_sub!(G1Affine, G1Projective, G1Projective);

impl_add_sub_assign!(G1Projective);
impl_add_sub_assign!(G1Projective, G1Affine);

impl_mul!(G1Projective, Fq);
impl_mul!(G1Affine, Fq, G1Projective);

impl_mul_assign!(G1Projective, Fq);
impl_mul_assign!(G1Affine, Fq);

impl<T> Sum<T> for G1Projective
where
    T: Borrow<G1Projective>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::identity(), |acc, item| acc + item.borrow())
    }
}

impl ConditionallySelectable for G1Affine {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G1Affine(blst_p1_affine {
            x: Fp::conditional_select(&a.x(), &b.x(), choice).0,
            y: Fp::conditional_select(&a.y(), &b.y(), choice).0,
        })
    }
}

impl ConditionallySelectable for G1Projective {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G1Projective(blst_p1 {
            x: Fp::conditional_select(&a.x(), &b.x(), choice).0,
            y: Fp::conditional_select(&a.y(), &b.y(), choice).0,
            z: Fp::conditional_select(&a.z(), &b.z(), choice).0,
        })
    }
}

// Internal serializations methods.
impl G1Affine {
    /// Serializes this element into compressed form.
    fn to_compressed(&self) -> [u8; COMPRESSED_SIZE] {
        let mut out = [0u8; COMPRESSED_SIZE];

        unsafe {
            blst_p1_affine_compress(out.as_mut_ptr(), &self.0);
        }

        out
    }

    /// Serializes this element into uncompressed form.
    fn to_uncompressed(&self) -> [u8; UNCOMPRESSED_SIZE] {
        let mut out = [0u8; UNCOMPRESSED_SIZE];
        unsafe {
            blst_p1_affine_serialize(out.as_mut_ptr(), &self.0);
        }

        out
    }

    /// Attempts to deserialize an uncompressed element.
    fn from_uncompressed(bytes: &[u8; UNCOMPRESSED_SIZE]) -> CtOption<Self> {
        G1Affine::from_uncompressed_unchecked(bytes)
            .and_then(|p| CtOption::new(p, p.is_torsion_free()))
    }

    /// Attempts to deserialize an uncompressed element, not checking if the
    /// element is on the curve and not checking if it is in the correct
    /// subgroup.
    ///
    /// **This is dangerous to call unless you trust the bytes you are reading;
    /// otherwise, API invariants may be broken.** Please consider using
    /// `from_uncompressed()` instead.
    fn from_uncompressed_unchecked(bytes: &[u8; UNCOMPRESSED_SIZE]) -> CtOption<Self> {
        let mut raw = blst_p1_affine::default();
        let success =
            unsafe { blst_p1_deserialize(&mut raw, bytes.as_ptr()) == BLST_ERROR::BLST_SUCCESS };
        CtOption::new(G1Affine(raw), Choice::from(success as u8))
    }

    /// Attempts to deserialize a compressed element.
    fn from_compressed(bytes: &[u8; COMPRESSED_SIZE]) -> CtOption<Self> {
        G1Affine::from_compressed_unchecked(bytes)
            .and_then(|p| CtOption::new(p, p.is_torsion_free()))
    }

    /// Attempts to deserialize an uncompressed element, not checking if the
    /// element is in the correct subgroup.
    ///
    /// **This is dangerous to call unless you trust the bytes you are reading;
    /// otherwise, API invariants may be broken.** Please consider using
    /// `from_compressed()` instead.
    fn from_compressed_unchecked(bytes: &[u8; COMPRESSED_SIZE]) -> CtOption<Self> {
        let mut raw = blst_p1_affine::default();
        let success =
            unsafe { blst_p1_uncompress(&mut raw, bytes.as_ptr()) == BLST_ERROR::BLST_SUCCESS };
        CtOption::new(G1Affine(raw), Choice::from(success as u8))
    }

    pub const fn uncompressed_size() -> usize {
        UNCOMPRESSED_SIZE
    }

    pub const fn compressed_size() -> usize {
        COMPRESSED_SIZE
    }
}

impl G1Affine {
    /// Returns true if this point is free of an $h$-torsion component, and so
    /// it exists within the $q$-order subgroup $\mathbb{G}_1$. This should
    /// always return true unless an "unchecked" API was used.
    pub fn is_torsion_free(&self) -> Choice {
        unsafe { Choice::from(blst_p1_affine_in_g1(&self.0) as u8) }
    }

    /// Returns the x coordinate.
    pub fn x(&self) -> Fp {
        Fp(self.0.x)
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> Fp {
        Fp(self.0.y)
    }

    // Internal wrapper for `blst_p1_affine`.
    fn from_raw_unchecked(x: Fp, y: Fp, _infinity: bool) -> Self {
        // FIXME: what about infinity?
        let raw = blst_p1_affine { x: x.0, y: y.0 };
        G1Affine(raw)
    }
}

impl SerdeObject for G1Affine {
    fn from_raw_bytes_unchecked(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), UNCOMPRESSED_SIZE);
        let input: [u8; UNCOMPRESSED_SIZE] = bytes.try_into().unwrap();
        Self::from_uncompressed_unchecked(&input).unwrap()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        debug_assert_eq!(bytes.len(), UNCOMPRESSED_SIZE);
        let input: [u8; UNCOMPRESSED_SIZE] = bytes.try_into().unwrap();
        Self::from_uncompressed(&input).into()
    }

    fn to_raw_bytes(&self) -> Vec<u8> {
        self.to_uncompressed().into()
    }

    fn read_raw_unchecked<R: Read>(reader: &mut R) -> Self {
        let mut buf = [0u8; UNCOMPRESSED_SIZE];
        reader.read_exact(&mut buf).expect("Could not read from buffer.");
        Self::from_uncompressed_unchecked(&buf)
            .expect("from_uncompressed_unchecked should return a point.")
    }

    fn read_raw<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; UNCOMPRESSED_SIZE];
        reader.read_exact(&mut buf)?;
        let res = Self::from_uncompressed(&buf);
        if res.is_some().into() {
            Ok(res.unwrap())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid point. (Either not on curve, or not in subgroup.",
            ))
        }
    }

    fn write_raw<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.to_uncompressed())
    }
}

/// This is an element of $\mathbb{G}_1$ represented in the projective
/// coordinate space.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct G1Projective(pub(crate) blst_p1);

impl fmt::Debug for G1Projective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("G1Projective")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

impl fmt::Display for G1Projective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", G1Affine::from(self))
    }
}

impl AsRef<blst_p1> for G1Projective {
    fn as_ref(&self) -> &blst_p1 {
        &self.0
    }
}

impl AsMut<blst_p1> for G1Projective {
    fn as_mut(&mut self) -> &mut blst_p1 {
        &mut self.0
    }
}

impl From<&G1Affine> for G1Projective {
    fn from(p: &G1Affine) -> G1Projective {
        let mut out = blst_p1::default();

        unsafe { blst_p1_from_affine(&mut out, &p.0) };

        G1Projective(out)
    }
}

impl From<G1Affine> for G1Projective {
    fn from(p: G1Affine) -> G1Projective {
        G1Projective::from(&p)
    }
}

impl Eq for G1Projective {}
impl PartialEq for G1Projective {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let self_is_zero: bool = self.is_identity().into();
        let other_is_zero: bool = other.is_identity().into();
        (self_is_zero && other_is_zero)
            || (!self_is_zero && !other_is_zero && unsafe { blst_p1_is_equal(&self.0, &other.0) })
    }
}

// Internal serializations methods.
impl G1Projective {
    /// Serializes this element into compressed form.
    fn to_compressed(&self) -> [u8; COMPRESSED_SIZE] {
        let mut out = [0u8; COMPRESSED_SIZE];

        unsafe {
            blst_p1_compress(out.as_mut_ptr(), &self.0);
        }
        out
    }

    /// Attempts to deserialize a compressed element.
    fn from_compressed(bytes: &[u8; COMPRESSED_SIZE]) -> CtOption<Self> {
        G1Affine::from_compressed(bytes).map(Into::into)
    }

    /// Attempts to deserialize an uncompressed element, not checking if the
    /// element is in the correct subgroup.
    ///
    /// **This is dangerous to call unless you trust the bytes you are reading;
    /// otherwise, API invariants may be broken.** Please consider using
    /// `from_compressed()` instead.
    fn from_compressed_unchecked(bytes: &[u8; COMPRESSED_SIZE]) -> CtOption<Self> {
        G1Affine::from_compressed_unchecked(bytes).map(Into::into)
    }
}

impl G1Projective {
    /// Adds this point to another point in the affine model.
    fn add_mixed(&self, rhs: &G1Affine) -> G1Projective {
        let mut out = blst_p1::default();

        unsafe { blst_p1_add_or_double_affine(&mut out, &self.0, &rhs.0) };

        G1Projective(out)
    }

    /// Returns true if this point is on the curve. This should always return
    /// true unless an "unchecked" API was used.
    pub fn is_on_curve(&self) -> Choice {
        let is_on_curve = unsafe { Choice::from(blst_p1_on_curve(&self.0) as u8) };
        is_on_curve | self.is_identity()
    }

    fn multiply(&self, scalar: &Fq) -> G1Projective {
        let mut out = blst_p1::default();

        // Scalar is 255 bits wide.
        const NBITS: usize = 255;

        unsafe { blst_p1_mult(&mut out, &self.0, scalar.to_bytes_le().as_ptr(), NBITS) };

        G1Projective(out)
    }

    fn from_raw_unchecked(x: Fp, y: Fp, z: Fp) -> Self {
        let raw = blst_p1 {
            x: x.0,
            y: y.0,
            z: z.0,
        };

        G1Projective(raw)
    }

    /// Returns the x coordinate.
    pub fn x(&self) -> Fp {
        Fp(self.0.x)
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> Fp {
        Fp(self.0.y)
    }

    /// Returns the z coordinate.
    pub fn z(&self) -> Fp {
        Fp(self.0.z)
    }

    /// Hash to curve algorithm.
    pub fn hash_to_curve(msg: &[u8], dst: &[u8], aug: &[u8]) -> Self {
        let mut res = Self::identity();
        unsafe {
            blst_hash_to_g1(
                &mut res.0,
                msg.as_ptr(),
                msg.len(),
                dst.as_ptr(),
                dst.len(),
                aug.as_ptr(),
                aug.len(),
            );
        }
        res
    }

    /// Perform a multi-exponentiation, aka "multi-scalar-multiplication" (MSM)
    /// using `blst`'s implementation of Pippenger's algorithm.
    /// Note: `scalars` is cloned in this method.
    pub fn multi_exp(points: &[Self], scalars: &[Fq]) -> Self {
        let n = if points.len() < scalars.len() {
            points.len()
        } else {
            scalars.len()
        };
        let points =
            unsafe { std::slice::from_raw_parts(points.as_ptr() as *const blst_p1, points.len()) };

        let points = p1_affines::from(points);

        let mut scalar_bytes: Vec<u8> = Vec::with_capacity(n * 32);
        for a in scalars.iter().map(|s| s.to_bytes_le()) {
            scalar_bytes.extend_from_slice(&a);
        }

        let res = points.mult(scalar_bytes.as_slice(), 255);

        G1Projective(res)
    }
}

impl Group for G1Projective {
    type Scalar = Fq;

    fn random(mut rng: impl RngCore) -> Self {
        let mut out = blst_p1::default();
        let mut msg = [0u8; 64];
        rng.fill_bytes(&mut msg);
        const DST: [u8; 16] = [0; 16];
        const AUG: [u8; 16] = [0; 16];

        unsafe {
            blst_encode_to_g1(
                &mut out,
                msg.as_ptr(),
                msg.len(),
                DST.as_ptr(),
                DST.len(),
                AUG.as_ptr(),
                AUG.len(),
            )
        };

        G1Projective(out)
    }

    fn identity() -> Self {
        G1Projective(blst_p1::default())
    }

    fn generator() -> Self {
        G1Projective(unsafe { *blst_p1_generator() })
    }

    fn is_identity(&self) -> Choice {
        unsafe { Choice::from(blst_p1_is_inf(&self.0) as u8) }
    }

    fn double(&self) -> Self {
        let mut double = blst_p1::default();
        unsafe { blst_p1_double(&mut double, &self.0) };
        G1Projective(double)
    }
}

impl WnafGroup for G1Projective {
    fn recommended_wnaf_for_num_scalars(num_scalars: usize) -> usize {
        const RECOMMENDATIONS: [usize; 12] =
            [1, 3, 7, 20, 43, 120, 273, 563, 1630, 3128, 7933, 62569];

        let mut ret = 4;
        for r in &RECOMMENDATIONS {
            if num_scalars > *r {
                ret += 1;
            } else {
                break;
            }
        }

        ret
    }
}

impl PrimeGroup for G1Projective {}

impl Curve for G1Projective {
    type AffineRepr = G1Affine;

    /// Converts a batch of projective elements into affine elements. This
    /// function will panic if `p.len() != q.len()`.
    fn batch_normalize(p: &[Self], q: &mut [Self::AffineRepr]) {
        assert_eq!(p.len(), q.len());
        let points = unsafe { std::slice::from_raw_parts(p.as_ptr() as *const blst_p1, p.len()) };

        p1_affines::from(points)
            .as_slice()
            .iter()
            .zip(q.iter_mut())
            .for_each(|(val, res)| *res = G1Affine(*val));
    }

    fn to_affine(&self) -> Self::AffineRepr {
        self.into()
    }
}

impl PrimeCurve for G1Projective {
    type Affine = G1Affine;
}

impl PrimeCurveAffine for G1Affine {
    type Scalar = Fq;
    type Curve = G1Projective;

    fn identity() -> Self {
        G1Affine(blst_p1_affine::default())
    }

    fn generator() -> Self {
        G1Affine(unsafe { *blst_p1_affine_generator() })
    }

    fn is_identity(&self) -> Choice {
        unsafe { Choice::from(blst_p1_affine_is_inf(&self.0) as u8) }
    }

    fn to_curve(&self) -> Self::Curve {
        self.into()
    }
}

impl GroupEncoding for G1Projective {
    type Repr = G1Compressed;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_compressed(&bytes.0)
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_compressed_unchecked(&bytes.0)
    }

    fn to_bytes(&self) -> Self::Repr {
        G1Compressed(self.to_compressed())
    }
}

impl GroupEncoding for G1Affine {
    type Repr = G1Compressed;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_compressed(&bytes.0)
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_compressed_unchecked(&bytes.0)
    }

    fn to_bytes(&self) -> Self::Repr {
        G1Compressed(self.to_compressed())
    }
}

impl UncompressedEncoding for G1Affine {
    type Uncompressed = G1Uncompressed;

    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_uncompressed(&bytes.0)
    }

    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_uncompressed_unchecked(&bytes.0)
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        G1Uncompressed(self.to_uncompressed())
    }
}

// UncompressedEncoding is not implemented for projective coordinates
// impl UncompressedEncoding for G1Projective{}

#[derive(Copy, Clone)]
#[repr(transparent)]
// Wrapper for [u8; UNCOMPRESSED_SIZE].
// This is needed to satisfy the [`Default`] bound on the Uncompressed type
// for [`UncompressedEncoding`].
pub struct G1Uncompressed([u8; UNCOMPRESSED_SIZE]);

encoded_point_delegations!(G1Uncompressed);

impl Default for G1Uncompressed {
    fn default() -> Self {
        G1Uncompressed([0u8; UNCOMPRESSED_SIZE])
    }
}

impl fmt::Debug for G1Uncompressed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0[..].fmt(formatter)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct G1Compressed([u8; COMPRESSED_SIZE]);

encoded_point_delegations!(G1Compressed);

impl Default for G1Compressed {
    fn default() -> Self {
        G1Compressed([0u8; COMPRESSED_SIZE])
    }
}

impl fmt::Debug for G1Compressed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0[..].fmt(formatter)
    }
}

impl PairingCurveAffine for G1Affine {
    type Pair = G2Affine;
    type PairingResult = Gt;

    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
        <Bls12 as Engine>::pairing(self, other)
    }
}

impl Add for G1Affine {
    type Output = <Self as PrimeCurveAffine>::Curve;

    fn add(self, rhs: Self) -> Self::Output {
        G1Projective::from(self) + rhs
    }
}

impl Sub for G1Affine {
    type Output = <Self as PrimeCurveAffine>::Curve;

    fn sub(self, rhs: Self) -> Self::Output {
        G1Projective::from(self) - rhs
    }
}

impl ConstantTimeEq for G1Affine {
    fn ct_eq(&self, other: &Self) -> Choice {
        let z1 = self.is_identity();
        let z2 = other.is_identity();

        (z1 & z2) | ((!z1) & (!z2) & (self.x().ct_eq(&other.x())) & (self.y().ct_eq(&other.y())))
    }
}

impl Default for G1Projective {
    fn default() -> Self {
        G1Projective::identity()
    }
}

impl ConstantTimeEq for G1Projective {
    fn ct_eq(&self, other: &Self) -> Choice {
        // Is (x, y, z) equal to (x', y, z') when converted to affine?
        // => (x/z , y/z) equal to (x'/z' , y'/z')
        // => (xz' == x'z) & (yz' == y'z)

        let x1 = self.x() * other.z();
        let y1 = self.y() * other.z();

        let x2 = other.x() * self.z();
        let y2 = other.y() * self.z();

        let self_is_zero = self.is_identity();
        let other_is_zero = other.is_identity();

        (self_is_zero & other_is_zero) // Both point at infinity
            | ((!self_is_zero) & (!other_is_zero) & x1.ct_eq(&x2) & y1.ct_eq(&y2))
        // Neither point at infinity, coordinates are the same
    }
}

impl CurveExt for G1Projective {
    type ScalarExt = Fq;
    type Base = Fp;
    type AffineExt = G1Affine;
    const CURVE_ID: &'static str = "";

    fn endo(&self) -> Self {
        G1Projective::from_raw_unchecked(self.x() * ZETA_BASE, self.y(), self.z())
    }

    fn jacobian_coordinates(&self) -> (Self::Base, Self::Base, Self::Base) {
        // Homogeneous to Jacobian
        let x = self.x() * self.z();
        let y = self.y() * self.z().square();
        (x, y, self.z())
    }

    fn hash_to_curve<'a>(domain_prefix: &'a str) -> Box<dyn Fn(&[u8]) -> Self + 'a> {
        Box::new(move |message| {
            Self::hash_to_curve(
                message,
                domain_prefix.as_ref(),
                b"BLS12381G1_XMD:SHA-256_SSWU_RO_",
            )
        })
    }

    fn is_on_curve(&self) -> Choice {
        self.is_on_curve()
    }

    fn a() -> Self::Base {
        A
    }

    fn b() -> Self::Base {
        B
    }

    fn new_jacobian(x: Self::Base, y: Self::Base, z: Self::Base) -> CtOption<Self> {
        // Jacobian to homogeneous
        let z_inv = z.invert().unwrap_or(Fp::ZERO);
        let p_x = x * z_inv;
        let p_y = y * z_inv.square();
        let p = G1Projective::from_raw_unchecked(
            p_x,
            Fp::conditional_select(&p_y, &Fp::ONE, z.is_zero()),
            z,
        );
        CtOption::new(p, p.is_on_curve())
    }
}

impl CurveAffine for G1Affine {
    type ScalarExt = Fq;
    type Base = Fp;
    type CurveExt = G1Projective;

    fn coordinates(&self) -> CtOption<Coordinates<Self>> {
        Coordinates::from_xy(self.x(), self.y())
    }

    fn from_xy(x: Self::Base, y: Self::Base) -> CtOption<Self> {
        let p = Self::from_raw_unchecked(x, y, false);
        CtOption::new(p, p.is_torsion_free())
    }

    fn is_on_curve(&self) -> Choice {
        unsafe { Choice::from(blst_p1_affine_on_curve(&self.0) as u8) }
    }

    fn a() -> Self::Base {
        A
    }

    fn b() -> Self::Base {
        B
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::eq_op)]

    use ff::Field;
    use rand_core::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use super::*;

    #[test]
    fn curve_tests() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        {
            let z = G1Projective::identity();
            assert_eq!(z.is_identity().unwrap_u8(), 1);
        }

        // Negation edge case with zero.
        {
            let mut z = G1Projective::identity();
            z = z.neg();
            assert_eq!(z.is_identity().unwrap_u8(), 1);
        }

        // Doubling edge case with zero.
        {
            let mut z = G1Projective::identity();
            z = z.double();
            assert_eq!(z.is_identity().unwrap_u8(), 1);
        }

        // Addition edge cases with zero
        {
            let mut r = G1Projective::random(&mut rng);
            let rcopy = r;
            r += &G1Projective::identity();
            assert_eq!(r, rcopy);
            r += &G1Affine::identity();
            assert_eq!(r, rcopy);

            let mut z = G1Projective::identity();
            z += &G1Projective::identity();
            assert_eq!(z.is_identity().unwrap_u8(), 1);
            z += &G1Affine::identity();
            assert_eq!(z.is_identity().unwrap_u8(), 1);

            let mut z2 = z;
            z2 += &r;

            z += &G1Affine::from(r);

            assert_eq!(z, z2);
            assert_eq!(z, r);
        }

        // Transformations
        {
            let a = G1Projective::random(&mut rng);
            let b: G1Projective = G1Affine::from(a).into();
            let c = G1Projective::from(G1Affine::from(G1Projective::from(G1Affine::from(a))));

            assert_eq!(a, b);
            assert_eq!(b, c);
        }
    }

    #[test]
    fn test_is_on_curve() {
        assert_eq!(G1Projective::identity().is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Projective::generator().is_on_curve().unwrap_u8(), 1);

        assert_eq!(G1Affine::identity().is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Affine::generator().is_on_curve().unwrap_u8(), 1);

        let z = Fp::from_mont_unchecked([
            0xba7afa1f9a6fe250,
            0xfa0f5b595eafe731,
            0x3bdc477694c306e7,
            0x2149be4b3949fa24,
            0x64aa6e0649b2078c,
            0x12b108ac33643c3e,
        ]);

        let gen = G1Affine::generator();
        let z2 = z.square();
        let mut test = G1Projective::from_raw_unchecked(gen.x() * z2, gen.y() * (z2 * z), z);

        assert_eq!(test.is_on_curve().unwrap_u8(), 1);

        test.0.x = z.0;
        assert_eq!(test.is_on_curve().unwrap_u8(), 0);
    }

    #[test]
    fn test_affine_point_equality() {
        let a = G1Affine::generator();
        let b = G1Affine::identity();

        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_ne!(a, b);
        assert_ne!(b, a);
    }

    #[test]
    fn test_projective_point_equality() {
        let a = G1Projective::generator();
        let b = G1Projective::identity();

        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_ne!(a, b);
        assert_ne!(b, a);

        let z = Fp::from_mont_unchecked([
            0xba7afa1f9a6fe250,
            0xfa0f5b595eafe731,
            0x3bdc477694c306e7,
            0x2149be4b3949fa24,
            0x64aa6e0649b2078c,
            0x12b108ac33643c3e,
        ]);

        let z2 = z.square();
        let mut c = G1Projective::from_raw_unchecked(a.x() * z2, a.y() * (z2 * z), z);
        assert_eq!(c.is_on_curve().unwrap_u8(), 1);

        assert_eq!(a, c);
        assert_eq!(c, a);
        assert_ne!(b, c);
        assert_ne!(c, b);

        c.0.y = (-c.y()).0;
        assert_eq!(c.is_on_curve().unwrap_u8(), 1);

        assert_ne!(a, c);
        assert_ne!(b, c);
        assert_ne!(c, a);
        assert_ne!(c, b);

        c.0.y = (-c.y()).0;
        c.0.x = z.0;
        assert_eq!(c.is_on_curve().unwrap_u8(), 0);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn test_projective_to_affine() {
        let a = G1Projective::generator();
        let b = G1Projective::identity();

        assert_eq!(G1Affine::from(a).is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Affine::from(a).is_identity().unwrap_u8(), 0);
        assert_eq!(G1Affine::from(b).is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Affine::from(b).is_identity().unwrap_u8(), 1);

        let z = Fp::from_mont_unchecked([
            0xba7afa1f9a6fe250,
            0xfa0f5b595eafe731,
            0x3bdc477694c306e7,
            0x2149be4b3949fa24,
            0x64aa6e0649b2078c,
            0x12b108ac33643c3e,
        ]);

        let z2 = z.square();
        let c = G1Projective::from_raw_unchecked(a.x() * z2, a.y() * (z2 * z), z);

        assert_eq!(G1Affine::from(c), G1Affine::generator());
    }

    #[test]
    fn test_affine_to_projective() {
        let a = G1Affine::generator();
        let b = G1Affine::identity();

        assert_eq!(G1Projective::from(a).is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Projective::from(a).is_identity().unwrap_u8(), 0);
        assert_eq!(G1Projective::from(b).is_on_curve().unwrap_u8(), 1);
        assert_eq!(G1Projective::from(b).is_identity().unwrap_u8(), 1);
    }

    #[test]
    fn test_doubling() {
        {
            let tmp = G1Projective::identity().double();
            assert_eq!(tmp.is_identity().unwrap_u8(), 1);
            assert_eq!(tmp.is_on_curve().unwrap_u8(), 1);
        }
        {
            let tmp = G1Projective::generator().double();
            assert_eq!(tmp.is_identity().unwrap_u8(), 0);
            assert_eq!(tmp.is_on_curve().unwrap_u8(), 1);

            assert_eq!(
                G1Affine::from(tmp),
                G1Affine::from_raw_unchecked(
                    Fp::from_mont_unchecked([
                        0x53e978ce58a9ba3c,
                        0x3ea0583c4f3d65f9,
                        0x4d20bb47f0012960,
                        0xa54c664ae5b2b5d9,
                        0x26b552a39d7eb21f,
                        0x8895d26e68785
                    ]),
                    Fp::from_mont_unchecked([
                        0x70110b3298293940,
                        0xda33c5393f1f6afc,
                        0xb86edfd16a5aa785,
                        0xaec6d1c9e7b1c895,
                        0x25cfc2b522d11720,
                        0x6361c83f8d09b15
                    ]),
                    false
                )
            );
        }
    }

    #[test]
    fn test_projective_addition() {
        {
            let a = G1Projective::identity();
            let b = G1Projective::identity();
            let c = a + b;
            assert_eq!(c.is_identity().unwrap_u8(), 1);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
        }
        {
            let a = G1Projective::identity();
            let mut b = G1Projective::generator();
            {
                let z = Fp::from_mont_unchecked([
                    0xba7afa1f9a6fe250,
                    0xfa0f5b595eafe731,
                    0x3bdc477694c306e7,
                    0x2149be4b3949fa24,
                    0x64aa6e0649b2078c,
                    0x12b108ac33643c3e,
                ]);

                let z2 = z.square();
                b = G1Projective::from_raw_unchecked(b.x() * (z2), b.y() * (z2 * z), z);
            }
            let c = a + b;
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, G1Projective::generator());
        }
        {
            let a = G1Projective::identity();
            let mut b = G1Projective::generator();
            {
                let z = Fp::from_mont_unchecked([
                    0xba7afa1f9a6fe250,
                    0xfa0f5b595eafe731,
                    0x3bdc477694c306e7,
                    0x2149be4b3949fa24,
                    0x64aa6e0649b2078c,
                    0x12b108ac33643c3e,
                ]);

                let z2 = z.square();
                b = G1Projective::from_raw_unchecked(b.x() * (z2), b.y() * (z2 * z), z);
            }
            let c = b + a;
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, G1Projective::generator());
        }
        {
            let a = G1Projective::generator().double().double(); // 4P
            let b = G1Projective::generator().double(); // 2P
            let c = a + b;

            let mut d = G1Projective::generator();
            for _ in 0..5 {
                d += G1Projective::generator();
            }
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(d.is_identity().unwrap_u8(), 0);
            assert_eq!(d.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, d);
        }

        // Degenerate case
        {
            let mut beta = Fp::from_mont_unchecked([
                0xcd03c9e48671f071,
                0x5dab22461fcda5d2,
                0x587042afd3851b95,
                0x8eb60ebe01bacb9e,
                0x3f97d6e83d050d2,
                0x18f0206554638741,
            ]);
            beta = beta.square();
            let a = G1Projective::generator().double().double();
            let b = G1Projective::from_raw_unchecked(a.x() * beta, -a.y(), a.z());
            assert_eq!(a.is_on_curve().unwrap_u8(), 1);
            assert_eq!(b.is_on_curve().unwrap_u8(), 1);

            let c = a + b;
            assert_eq!(
                G1Affine::from(c),
                G1Affine::from(G1Projective::from_raw_unchecked(
                    Fp::from_mont_unchecked([
                        0x29e1e987ef68f2d0,
                        0xc5f3ec531db03233,
                        0xacd6c4b6ca19730f,
                        0x18ad9e827bc2bab7,
                        0x46e3b2c5785cc7a9,
                        0x7e571d42d22ddd6
                    ]),
                    Fp::from_mont_unchecked([
                        0x94d117a7e5a539e7,
                        0x8e17ef673d4b5d22,
                        0x9d746aaf508a33ea,
                        0x8c6d883d2516c9a2,
                        0xbc3b8d5fb0447f7,
                        0x7bfa4c7210f4f44
                    ]),
                    Fp::ONE,
                ))
            );
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
        }
    }

    #[test]
    fn test_mixed_addition() {
        {
            let a = G1Affine::identity();
            let b = G1Projective::identity();
            let c = a + b;
            assert_eq!(c.is_identity().unwrap_u8(), 1);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
        }
        {
            let a = G1Affine::identity();
            let mut b = G1Projective::generator();
            {
                let z = Fp::from_mont_unchecked([
                    0xba7afa1f9a6fe250,
                    0xfa0f5b595eafe731,
                    0x3bdc477694c306e7,
                    0x2149be4b3949fa24,
                    0x64aa6e0649b2078c,
                    0x12b108ac33643c3e,
                ]);

                let z2 = z.square();
                b = G1Projective::from_raw_unchecked(b.x() * (z2), b.y() * (z2 * z), z);
            }
            let c = a + b;
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, G1Projective::generator());
        }
        {
            let a = G1Affine::identity();
            let mut b = G1Projective::generator();
            {
                let z = Fp::from_mont_unchecked([
                    0xba7afa1f9a6fe250,
                    0xfa0f5b595eafe731,
                    0x3bdc477694c306e7,
                    0x2149be4b3949fa24,
                    0x64aa6e0649b2078c,
                    0x12b108ac33643c3e,
                ]);

                let z2 = z.square();
                b = G1Projective::from_raw_unchecked(b.x() * (z2), b.y() * (z2 * z), z);
            }
            let c = b + a;
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, G1Projective::generator());
        }
        {
            let a = G1Projective::generator().double().double(); // 4P
            let b = G1Projective::generator().double(); // 2P
            let c = a + b;

            let mut d = G1Projective::generator();
            for _ in 0..5 {
                d += G1Affine::generator();
            }
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
            assert_eq!(d.is_identity().unwrap_u8(), 0);
            assert_eq!(d.is_on_curve().unwrap_u8(), 1);
            assert_eq!(c, d);
        }

        // Degenerate case
        {
            let mut beta = Fp::from_mont_unchecked([
                0xcd03c9e48671f071,
                0x5dab22461fcda5d2,
                0x587042afd3851b95,
                0x8eb60ebe01bacb9e,
                0x3f97d6e83d050d2,
                0x18f0206554638741,
            ]);
            beta = beta.square();
            let a = G1Projective::generator().double().double();
            let b = G1Projective::from_raw_unchecked(a.x() * beta, -a.y(), a.z());
            let a = G1Affine::from(a);
            assert_eq!(a.is_on_curve().unwrap_u8(), 1);
            assert_eq!(b.is_on_curve().unwrap_u8(), 1);

            let c = a + b;
            assert_eq!(
                G1Affine::from(c),
                G1Affine::from(G1Projective::from_raw_unchecked(
                    Fp::from_mont_unchecked([
                        0x29e1e987ef68f2d0,
                        0xc5f3ec531db03233,
                        0xacd6c4b6ca19730f,
                        0x18ad9e827bc2bab7,
                        0x46e3b2c5785cc7a9,
                        0x7e571d42d22ddd6
                    ]),
                    Fp::from_mont_unchecked([
                        0x94d117a7e5a539e7,
                        0x8e17ef673d4b5d22,
                        0x9d746aaf508a33ea,
                        0x8c6d883d2516c9a2,
                        0xbc3b8d5fb0447f7,
                        0x7bfa4c7210f4f44
                    ]),
                    Fp::ONE
                ))
            );
            assert_eq!(c.is_identity().unwrap_u8(), 0);
            assert_eq!(c.is_on_curve().unwrap_u8(), 1);
        }
    }

    #[test]
    fn test_projective_negation_and_subtraction() {
        let a = G1Projective::generator().double();
        assert_eq!(a + (-a), G1Projective::identity());
        assert_eq!(a + (-a), a - a);
    }

    #[test]
    fn test_affine_projective_negation_and_subtraction() {
        let a = G1Affine::generator();
        assert_eq!(G1Projective::from(a) + (-a), G1Projective::identity());
        assert_eq!(G1Projective::from(a) + (-a), G1Projective::from(a) - a);
    }

    #[test]
    fn test_projective_scalar_multiplication() {
        let g = G1Projective::generator();
        let a = Fq(blst::blst_fr {
            l: [
                0x2b568297a56da71c,
                0xd8c39ecb0ef375d1,
                0x435c38da67bfbf96,
                0x8088a05026b659b2,
            ],
        });
        let b = Fq(blst::blst_fr {
            l: [
                0x785fdd9b26ef8b85,
                0xc997f25837695c18,
                0x4c8dbc39e7b756c1,
                0x70d9b6cc6d87df20,
            ],
        });
        let c = a * b;

        assert_eq!((g * a) * b, g * c);
    }

    #[test]
    fn test_affine_scalar_multiplication() {
        let g = G1Affine::generator();
        let a = Fq(blst::blst_fr {
            l: [
                0x2b568297a56da71c,
                0xd8c39ecb0ef375d1,
                0x435c38da67bfbf96,
                0x8088a05026b659b2,
            ],
        });
        let b = Fq(blst::blst_fr {
            l: [
                0x785fdd9b26ef8b85,
                0xc997f25837695c18,
                0x4c8dbc39e7b756c1,
                0x70d9b6cc6d87df20,
            ],
        });
        let c = a * b;

        assert_eq!(G1Affine::from(g * a) * b, g * c);
    }

    #[test]
    fn g1_curve_tests() {
        use group::tests::curve_tests;
        curve_tests::<G1Projective>();
    }

    #[test]
    fn test_g1_is_identity() {
        assert_eq!(G1Projective::identity().is_identity().unwrap_u8(), 1);
        assert_eq!(G1Projective::generator().is_identity().unwrap_u8(), 0);
        assert_eq!(G1Affine::identity().is_identity().unwrap_u8(), 1);
        assert_eq!(G1Affine::generator().is_identity().unwrap_u8(), 0);
    }

    #[test]
    fn test_g1_serialization_roundtrip() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        for _ in 0..100 {
            // Affine
            let el: G1Affine = G1Projective::random(&mut rng).into();
            let c = el.to_compressed();
            assert_eq!(G1Affine::from_compressed(&c).unwrap(), el);
            assert_eq!(G1Affine::from_compressed_unchecked(&c).unwrap(), el);

            let u = el.to_uncompressed();
            assert_eq!(G1Affine::from_uncompressed(&u).unwrap(), el);
            assert_eq!(G1Affine::from_uncompressed_unchecked(&u).unwrap(), el);

            let c = el.to_bytes();
            assert_eq!(G1Affine::from_bytes(&c).unwrap(), el);
            assert_eq!(G1Affine::from_bytes_unchecked(&c).unwrap(), el);

            let c = el.to_raw_bytes();
            assert_eq!(G1Affine::from_raw_bytes(&c).unwrap(), el);
            assert_eq!(G1Affine::from_raw_bytes_unchecked(&c), el);

            // Projective
            let el = G1Projective::random(&mut rng);
            let c = el.to_compressed();
            assert_eq!(G1Projective::from_compressed(&c).unwrap(), el);
            assert_eq!(G1Projective::from_compressed_unchecked(&c).unwrap(), el);

            let c = el.to_bytes();
            assert_eq!(G1Projective::from_bytes(&c).unwrap(), el);
            assert_eq!(G1Projective::from_bytes_unchecked(&c).unwrap(), el);
        }
    }

    /// An on-curve G1 point that is NOT in the prime-order subgroup.
    ///
    /// G1 is `y² = x³ + 4` with a large cofactor, so the first on-curve point
    /// found by scanning `x` is, with overwhelming probability, outside the
    /// prime-order subgroup. The explicit `is_torsion_free` guard makes the
    /// helper correct even in the astronomically unlikely case it lands in it.
    fn g1_non_torsion_point() -> G1Affine {
        let b = Fp::from(4u64);
        let mut x = Fp::ONE;
        loop {
            let rhs = x.square() * x + b;
            if let Some(y) = Option::<Fp>::from(rhs.sqrt()) {
                let p = G1Affine::from_raw_unchecked(x, y, false);
                if !bool::from(p.is_torsion_free()) {
                    return p;
                }
            }
            x += Fp::ONE;
        }
    }

    // Deserialization of an on-curve point that is outside the prime-order
    // subgroup must be rejected by every checked path, while the `_unchecked`
    // variants accept it.
    #[test]
    fn test_g1_deserialize_rejects_non_subgroup() {
        let p = g1_non_torsion_point();
        // Precondition: on the curve, but not in the prime-order subgroup.
        assert_eq!(p.is_on_curve().unwrap_u8(), 1);
        assert_eq!(p.is_torsion_free().unwrap_u8(), 0);

        // Uncompressed: checked rejects, unchecked accepts.
        let u = p.to_uncompressed();
        assert!(bool::from(G1Affine::from_uncompressed(&u).is_none()));
        assert_eq!(G1Affine::from_uncompressed_unchecked(&u).unwrap(), p);

        // Compressed: checked rejects, unchecked accepts.
        let c = p.to_compressed();
        assert!(bool::from(G1Affine::from_compressed(&c).is_none()));
        assert_eq!(G1Affine::from_compressed_unchecked(&c).unwrap(), p);

        // SerdeObject raw-bytes paths route through `from_uncompressed`, so they
        // must reject too; the `_unchecked` counterpart accepts.
        let raw = p.to_raw_bytes();
        assert!(G1Affine::from_raw_bytes(&raw).is_none());
        assert_eq!(G1Affine::from_raw_bytes_unchecked(&raw), p);
        assert!(G1Affine::read_raw(&mut raw.as_slice()).is_err());

        // The `UncompressedEncoding` forwarder is the same checked path.
        let enc = <G1Affine as UncompressedEncoding>::to_uncompressed(&p);
        assert!(bool::from(
            <G1Affine as UncompressedEncoding>::from_uncompressed(&enc).is_none()
        ));
    }

    #[test]
    fn test_multi_exp() {
        const SIZE: usize = 10;
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        let points: Vec<G1Projective> = (0..SIZE).map(|_| G1Projective::random(&mut rng)).collect();
        let scalars: Vec<Fq> = (0..SIZE).map(|_| Fq::random(&mut rng)).collect();

        let mut naive = points[0] * scalars[0];
        for i in 1..SIZE {
            naive += points[i] * scalars[i];
        }

        let pippenger = G1Projective::multi_exp(points.as_slice(), scalars.as_slice());

        assert_eq!(naive, pippenger);
    }
}
```