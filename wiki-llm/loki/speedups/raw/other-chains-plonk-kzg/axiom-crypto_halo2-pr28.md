# [chore] use rayon work-stealing to improve `evaluate_h`
https://github.com/axiom-crypto/halo2/pull/28
merged: 2023-11-23T16:47:52Z

I have no idea how rayon work-stealing works, but by adding some `par_iter`s to the multiple FFTs, it has improved the performance substantially on machines with large numbers of cores.

We have also discovered that Scroll's `parallel` FFT is better than the `recursive` FFT on `c7a` machines (8xl up to 48xl). So we use `cfg(target_arch = "x86_64")` to select which one to use now.
