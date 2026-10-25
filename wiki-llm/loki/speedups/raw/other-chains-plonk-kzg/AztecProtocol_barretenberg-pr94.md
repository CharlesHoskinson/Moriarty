# Commit to Wires Using Lagrange SRS
https://github.com/AztecProtocol/barretenberg/pull/94
merged: 2023-02-12T01:42:51Z


# **Copied from PR https://github.com/AztecProtocol/barretenberg/pull/39**
   - **Original author: @suyash67**
   - This copy uses branches with git history restored

---

Committing to the wire polynomials with Lagrange srs is expected to improve the proof construction time. This is because the wire values are very small in magnitude as compared to the field size, and the resulting MSM while computing the commitments to the wire polynomials is faster.

This is going to be very useful if/when we start using the new lookup argument for doing permutation check ([paper](https://eprint.iacr.org/2022/1530.pdf)).
---

# Comments from original PR

## **Copied from comment (https://github.com/AztecProtocol/barretenberg/pull/39#issuecomment-1375487455) by author @suyash67**

Note: we need to merge #36 before this one.

