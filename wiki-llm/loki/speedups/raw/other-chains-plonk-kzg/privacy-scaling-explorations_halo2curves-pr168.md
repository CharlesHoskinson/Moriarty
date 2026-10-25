# feat: skip zeroes in msm
https://github.com/privacy-ethereum/halo2curves/pull/168
merged: 2024-07-23T07:55:06Z

This change only affects the `multiexp_serial` MSM implementation (and not cyclone).

In the multiexp serial MSM algorithm, skip the windows that pick the most significant bits whenever those bits are zeroes in all coefficients.  This is done by finding the max number of bits used by the coefficients.

Extend the msm benchmark to use coefficients of different bit sizes.

Closes https://github.com/privacy-scaling-explorations/halo2curves/issues/150
Supersede https://github.com/privacy-scaling-explorations/halo2curves/pull/152 (bench comparison of this PR approach and previous PR approach: https://github.com/privacy-scaling-explorations/halo2curves/pull/152#issuecomment-2147282783

<details>
<summary>Bench results compared to main branch</summary>

```
msm/singlecore/1b_3     time:   [10.012 µs 10.024 µs 10.044 µs]
                        change: [-95.371% -95.364% -95.357%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
msm/singlecore/1b_8     time:   [90.110 µs 90.146 µs 90.210 µs]
                        change: [-87.330% -87.308% -87.282%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/1b_10    time:   [286.23 µs 286.32 µs 286.46 µs]
                        change: [-78.960% -78.898% -78.862%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/1b_12    time:   [993.64 µs 994.66 µs 996.23 µs]
                        change: [-76.420% -76.393% -76.364%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
msm/singlecore/1b_14    time:   [3.7995 ms 3.8009 ms 3.8031 ms]
                        change: [-62.782% -62.703% -62.644%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/singlecore/1b_16    time:   [15.173 ms 15.181 ms 15.188 ms]
                        change: [-58.847% -58.732% -58.633%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low severe
  1 (10.00%) high severe
msm/multicore/1b_3      time:   [10.034 µs 10.071 µs 10.105 µs]
                        change: [-95.314% -95.306% -95.294%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
msm/multicore/1b_8      time:   [79.733 µs 81.972 µs 83.462 µs]
                        change: [-89.001% -88.788% -88.566%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/1b_10     time:   [135.11 µs 141.52 µs 147.90 µs]
                        change: [-88.487% -88.008% -87.476%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/1b_12     time:   [266.22 µs 271.97 µs 283.62 µs]
                        change: [-84.967% -84.356% -83.652%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/multicore/1b_14     time:   [866.47 µs 876.08 µs 886.38 µs]
                        change: [-80.875% -80.508% -80.115%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/1b_16     time:   [2.6069 ms 2.6250 ms 2.6467 ms]
                        change: [-72.765% -72.466% -72.162%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/1b_18     time:   [9.7711 ms 9.8096 ms 9.8395 ms]
                        change: [-58.567% -58.325% -58.092%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/1b_20     time:   [43.866 ms 44.037 ms 44.176 ms]
                        change: [-51.829% -51.566% -51.281%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking msm/multicore/1b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 9.6s or enable flat sampling.
msm/multicore/1b_22     time:   [173.90 ms 176.90 ms 181.52 ms]
                        change: [-48.954% -47.756% -46.665%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/8b_3     time:   [17.881 µs 17.936 µs 17.989 µs]
                        change: [-91.989% -91.953% -91.923%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/8b_8     time:   [230.56 µs 230.64 µs 230.81 µs]
                        change: [-72.799% -72.663% -72.580%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/8b_10    time:   [807.02 µs 807.62 µs 808.09 µs]
                        change: [-56.930% -56.633% -56.381%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 10 measurements (30.00%)
  2 (20.00%) low severe
  1 (10.00%) high severe
msm/singlecore/8b_12    time:   [1.8198 ms 1.8201 ms 1.8206 ms]
                        change: [-64.437% -64.411% -64.388%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/8b_14    time:   [7.0174 ms 7.0293 ms 7.0470 ms]
                        change: [-47.729% -47.663% -47.581%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/8b_16    time:   [27.663 ms 27.818 ms 27.997 ms]
                        change: [-44.181% -43.967% -43.742%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  2 (20.00%) high mild
msm/multicore/8b_3      time:   [17.554 µs 17.563 µs 17.575 µs]
                        change: [-92.056% -92.050% -92.041%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/multicore/8b_8      time:   [135.86 µs 137.37 µs 138.84 µs]
                        change: [-82.320% -82.079% -81.820%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/8b_10     time:   [227.13 µs 229.35 µs 231.97 µs]
                        change: [-81.499% -81.313% -81.119%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/8b_12     time:   [678.44 µs 681.93 µs 684.93 µs]
                        change: [-68.471% -68.179% -67.842%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/8b_14     time:   [1.9380 ms 1.9480 ms 1.9591 ms]
                        change: [-66.005% -65.392% -64.831%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/8b_16     time:   [4.6823 ms 4.8554 ms 5.0683 ms]
                        change: [-60.605% -59.316% -57.938%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/8b_18     time:   [17.889 ms 18.010 ms 18.136 ms]
                        change: [-45.332% -44.494% -43.779%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/8b_20     time:   [76.471 ms 76.735 ms 76.984 ms]
                        change: [-38.274% -37.808% -37.304%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/8b_22     time:   [302.46 ms 304.55 ms 306.73 ms]
                        change: [-35.234% -34.677% -34.106%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/16b_3    time:   [35.698 µs 35.792 µs 35.884 µs]
                        change: [-84.679% -84.655% -84.637%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low severe
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/singlecore/16b_8    time:   [356.66 µs 356.77 µs 356.95 µs]
                        change: [-63.419% -63.392% -63.358%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/16b_10   time:   [1.2813 ms 1.2820 ms 1.2829 ms]
                        change: [-45.238% -44.838% -44.603%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/singlecore/16b_12   time:   [3.5520 ms 3.5529 ms 3.5536 ms]
                        change: [-46.779% -46.735% -46.699%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/16b_14   time:   [13.838 ms 13.847 ms 13.851 ms]
                        change: [-30.963% -30.733% -30.498%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/16b_16   time:   [54.186 ms 54.192 ms 54.202 ms]
                        change: [-27.621% -27.508% -27.398%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/16b_3     time:   [35.785 µs 35.793 µs 35.805 µs]
                        change: [-84.480% -84.450% -84.428%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/16b_8     time:   [211.43 µs 213.88 µs 218.27 µs]
                        change: [-74.079% -73.648% -73.196%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/16b_10    time:   [394.77 µs 397.74 µs 402.61 µs]
                        change: [-71.395% -71.036% -70.665%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/16b_12    time:   [973.53 µs 978.69 µs 983.80 µs]
                        change: [-59.262% -58.833% -58.426%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/16b_14    time:   [3.1331 ms 3.1508 ms 3.1718 ms]
                        change: [-53.080% -52.680% -52.247%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/16b_16    time:   [9.1825 ms 9.2543 ms 9.3108 ms]
                        change: [-43.377% -42.387% -41.640%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/16b_18    time:   [35.601 ms 35.902 ms 36.223 ms]
                        change: [-27.931% -26.937% -25.789%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking msm/multicore/16b_20: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 8.1s or enable flat sampling.
msm/multicore/16b_20    time:   [145.99 ms 148.54 ms 151.42 ms]
                        change: [-23.488% -22.704% -21.720%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking msm/multicore/16b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 5.7s.
msm/multicore/16b_22    time:   [568.13 ms 575.13 ms 585.35 ms]
                        change: [-21.721% -20.429% -18.565%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/32b_3    time:   [62.876 µs 62.888 µs 62.903 µs]
                        change: [-74.912% -74.901% -74.890%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/32b_8    time:   [706.16 µs 706.27 µs 706.41 µs]
                        change: [-43.968% -43.929% -43.888%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/32b_10   time:   [2.2017 ms 2.2027 ms 2.2038 ms]
                        change: [-30.497% -30.464% -30.427%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/singlecore/32b_12   time:   [7.1927 ms 7.1940 ms 7.1952 ms]
                        change: [-28.142% -27.917% -27.768%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/32b_14   time:   [26.635 ms 26.679 ms 26.715 ms]
                        change: [-17.452% -17.387% -17.297%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  2 (20.00%) high severe
msm/singlecore/32b_16   time:   [82.872 ms 82.890 ms 82.923 ms]
                        change: [-18.644% -18.563% -18.492%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/32b_3     time:   [63.263 µs 63.277 µs 63.291 µs]
                        change: [-74.257% -74.251% -74.245%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/32b_8     time:   [346.68 µs 348.13 µs 350.47 µs]
                        change: [-61.162% -60.834% -60.459%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/32b_10    time:   [688.81 µs 721.32 µs 745.97 µs]
                        change: [-56.990% -55.902% -54.660%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/multicore/32b_12    time:   [1.8502 ms 1.8630 ms 1.8741 ms]
                        change: [-41.796% -40.653% -38.870%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/32b_14    time:   [5.5269 ms 5.5735 ms 5.6319 ms]
                        change: [-37.459% -36.900% -36.290%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/32b_16    time:   [18.182 ms 18.380 ms 18.541 ms]
                        change: [-28.349% -26.572% -24.965%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/32b_18    time:   [69.378 ms 71.895 ms 75.300 ms]
                        change: [-15.264% -13.632% -10.999%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/32b_20    time:   [218.00 ms 219.07 ms 220.15 ms]
                        change: [-16.775% -16.052% -15.354%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking msm/multicore/32b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 8.6s.
msm/multicore/32b_22    time:   [859.64 ms 862.66 ms 865.53 ms]
                        change: [-14.781% -14.304% -13.852%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/64b_3    time:   [124.90 µs 124.93 µs 125.01 µs]
                        change: [-56.005% -55.378% -54.898%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
msm/singlecore/64b_8    time:   [1.3228 ms 1.3240 ms 1.3254 ms]
                        change: [-27.249% -27.130% -27.007%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/64b_10   time:   [4.3958 ms 4.3972 ms 4.3997 ms]
                        change: [-15.937% -15.889% -15.837%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/singlecore/64b_12   time:   [14.023 ms 14.068 ms 14.129 ms]
                        change: [-14.445% -14.316% -14.122%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/64b_14   time:   [47.741 ms 47.748 ms 47.758 ms]
                        change: [-9.2742% -9.2265% -9.1710%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
Benchmarking msm/singlecore/64b_16: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 9.2s or enable flat sampling.
msm/singlecore/64b_16   time:   [166.53 ms 166.62 ms 166.71 ms]
                        change: [-8.3968% -8.2808% -8.1468%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/multicore/64b_3     time:   [123.41 µs 123.44 µs 123.46 µs]
                        change: [-56.405% -55.831% -55.505%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/64b_8     time:   [623.48 µs 627.28 µs 631.03 µs]
                        change: [-41.704% -40.732% -40.017%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/64b_10    time:   [1.2650 ms 1.2723 ms 1.2778 ms]
                        change: [-37.988% -37.469% -36.931%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/64b_12    time:   [3.3797 ms 3.3966 ms 3.4125 ms]
                        change: [-24.781% -24.167% -23.513%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/64b_14    time:   [10.331 ms 10.655 ms 10.977 ms]
                        change: [-20.505% -19.114% -17.407%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/64b_16    time:   [35.492 ms 35.987 ms 36.875 ms]
                        change: [-12.733% -11.353% -9.7589%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking msm/multicore/64b_18: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 6.8s or enable flat sampling.
msm/multicore/64b_18    time:   [123.97 ms 125.02 ms 125.88 ms]
                        change: [-8.8785% -7.7276% -6.5866%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/64b_20    time:   [428.31 ms 432.77 ms 439.22 ms]
                        change: [-8.8539% -7.3306% -5.5788%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking msm/multicore/64b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 14.2s.
msm/multicore/64b_22    time:   [1.4172 s 1.4196 s 1.4222 s]
                        change: [-8.4287% -8.1887% -7.9147%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high mild
msm/singlecore/128b_3   time:   [246.03 µs 246.18 µs 246.61 µs]
                        change: [-28.221% -27.826% -27.207%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/singlecore/128b_8   time:   [2.6412 ms 2.6414 ms 2.6418 ms]
                        change: [-9.9171% -9.7277% -9.5658%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild
msm/singlecore/128b_10  time:   [8.4985 ms 8.5008 ms 8.5047 ms]
                        change: [-5.5796% -5.4646% -5.3931%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/singlecore/128b_12  time:   [26.842 ms 26.847 ms 26.851 ms]
                        change: [-5.6692% -5.4948% -5.3039%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/128b_14  time:   [89.035 ms 89.067 ms 89.118 ms]
                        change: [-3.5993% -3.5594% -3.5189%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild
msm/singlecore/128b_16  time:   [305.95 ms 306.05 ms 306.16 ms]
                        change: [-3.1480% -3.0954% -3.0433%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/128b_3    time:   [243.04 µs 243.07 µs 243.10 µs]
                        change: [-28.677% -28.649% -28.621%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/128b_8    time:   [1.1023 ms 1.1131 ms 1.1239 ms]
                        change: [-21.096% -20.371% -19.447%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  1 (10.00%) high mild
  1 (10.00%) high severe
msm/multicore/128b_10   time:   [2.3728 ms 2.3883 ms 2.3997 ms]
                        change: [-21.188% -18.048% -15.990%] (p = 0.00 < 0.05)
                        Performance has improved.
msm/multicore/128b_12   time:   [6.7139 ms 6.7872 ms 6.8616 ms]
                        change: [-9.7549% -8.7415% -7.7093%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/128b_14   time:   [19.784 ms 20.179 ms 20.639 ms]
                        change: [-10.940% -8.3656% -6.0509%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/128b_16   time:   [67.736 ms 69.220 ms 71.019 ms]
                        change: [-5.0699% -3.9340% -2.2813%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/128b_18   time:   [230.07 ms 231.95 ms 234.08 ms]
                        change: [-5.1782% -3.4534% -1.9762%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking msm/multicore/128b_20: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 7.9s.
msm/multicore/128b_20   time:   [781.62 ms 785.63 ms 790.08 ms]
                        change: [-4.2369% -3.2128% -2.2512%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking msm/multicore/128b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 28.4s.
msm/multicore/128b_22   time:   [2.8127 s 2.8195 s 2.8291 s]
                        change: [-5.3810% -4.5090% -3.6074%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high severe
msm/singlecore/256b_3   time:   [498.90 µs 499.92 µs 501.59 µs]
                        change: [+4.8043% +5.0942% +5.3680%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/256b_8   time:   [5.1020 ms 5.1537 ms 5.1865 ms]
                        change: [-0.1416% +0.4217% +1.0208%] (p = 0.25 > 0.05)
                        No change in performance detected.
msm/singlecore/256b_10  time:   [16.279 ms 16.288 ms 16.304 ms]
                        change: [-0.0880% +0.0576% +0.1954%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/singlecore/256b_12  time:   [52.048 ms 52.064 ms 52.080 ms]
                        change: [+1.7559% +1.8255% +1.9051%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking msm/singlecore/256b_14: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 9.9s or enable flat sampling.
msm/singlecore/256b_14  time:   [179.74 ms 179.88 ms 180.13 ms]
                        change: [+0.3310% +0.6592% +0.9781%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild
Benchmarking msm/singlecore/256b_16: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 6.1s.
msm/singlecore/256b_16  time:   [600.58 ms 602.25 ms 604.29 ms]
                        change: [-0.3017% -0.0214% +0.2939%] (p = 0.91 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/256b_3    time:   [495.93 µs 495.98 µs 496.06 µs]
                        change: [+5.8797% +5.9373% +5.9862%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/256b_8    time:   [2.0879 ms 2.1025 ms 2.1171 ms]
                        change: [-2.4482% -0.0578% +2.3404%] (p = 0.97 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
msm/multicore/256b_10   time:   [4.5919 ms 4.6253 ms 4.6507 ms]
                        change: [+0.0769% +0.9858% +1.8958%] (p = 0.05 < 0.05)
                        Change within noise threshold.
msm/multicore/256b_12   time:   [12.947 ms 13.011 ms 13.084 ms]
                        change: [-1.3435% -0.4832% +0.4385%] (p = 0.33 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
msm/multicore/256b_14   time:   [38.288 ms 38.513 ms 38.750 ms]
                        change: [-1.3095% +0.0486% +1.2736%] (p = 0.95 > 0.05)
                        No change in performance detected.
Benchmarking msm/multicore/256b_16: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 7.3s or enable flat sampling.
msm/multicore/256b_16   time:   [131.07 ms 131.53 ms 132.09 ms]
                        change: [-1.1853% -0.3731% +0.4579%] (p = 0.41 > 0.05)
                        No change in performance detected.
msm/multicore/256b_18   time:   [462.89 ms 466.86 ms 471.77 ms]
                        change: [-0.5471% +0.8462% +2.3218%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low mild
  2 (20.00%) high severe
Benchmarking msm/multicore/256b_20: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 15.5s.
msm/multicore/256b_20   time:   [1.5478 s 1.5497 s 1.5517 s]
                        change: [-0.1209% +0.0626% +0.2466%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low severe
  2 (20.00%) high severe
Benchmarking msm/multicore/256b_22: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 56.0s.
msm/multicore/256b_22   time:   [5.5956 s 5.6042 s 5.6171 s]
                        change: [-0.4943% +0.0524% +0.4883%] (p = 0.87 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
```

</details>

From the benchmarks only 3 cases have regressed:
```
msm/singlecore/256b_3   time:   [498.90 µs 499.92 µs 501.59 µs]
                        change: [+4.8043% +5.0942% +5.3680%] (p = 0.00 < 0.05)
                        Performance has regressed.
msm/singlecore/256b_12  time:   [52.048 ms 52.064 ms 52.080 ms]
                        change: [+1.7559% +1.8255% +1.9051%] (p = 0.00 < 0.05)
                        Performance has regressed.                        
msm/multicore/256b_3    time:   [495.93 µs 495.98 µs 496.06 µs]
                        change: [+5.8797% +5.9373% +5.9862%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

For `msm/multicore/256b_3` and `msm/singlecore/256b_3` these regressions seem acceptable because these cases take less than 500 µs.
The `msm/singlecore/256b_12` case is just `+1.8255%` so it's not too bad, compared to the significant gains we obtain for bit sizes <= 128.  Also I think this result is due to noise.  In all the cases as k grows the improvement decreases.  So I would expect the change in `msm/singlecore/256b_14` to be worse than `msm/singlecore/256b_12`  (and the same with `msm/singlecore/256b_16`) but that's not the case.

