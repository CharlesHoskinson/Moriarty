"""SDK-free codec specifications. Only public synthetic golden files are read."""

import copy
import hashlib
import json
import os
from pathlib import Path
import unittest

import frtf


FIXTURES = Path(os.environ.get(
    "FRTF_V3_ROOT", "/home/charl/projectfenrir/recursion/rollup-test-format/v3"))
POSITIVE = ["profile-a/minimal/minimal"] + [
    f"profile-{p}/batches/batch-{i:02}" for p in ("a", "b") for i in range(1, 7)]
MODULUS = int("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001", 16)
HEADER = 1060
STEP = 130


def changed(data, offset, replacement):
    out = bytearray(data)
    out[offset:offset + len(replacement)] = replacement
    return bytes(out)


def tiny(profile=1):
    """Independent literal writer: two transactions close headers 1 and 2."""
    u = lambda n, width: n.to_bytes(width, "little")
    field = lambda n: u(n, 32)
    out = bytearray(b"FRTF" + bytes((3, profile, 0, 0)))
    out += b"".join(u(n, 8) for n in (7, 1, 1, 2))
    out += b"".join(field(n) for n in range(24))
    out += field(25) + field(27) + field(28)
    out += u(0, 8) + u(0, 8) + field(0) * 2
    out += bytes(40 + 32) + u(2, 4)
    for i in range(2):
        out += field(30 + i) + u((i + 1) * 65536, 8) + u(i, 8)
        out += field(25 + i) + field(26 + i)
        out += u(0, 4) + u(i + 1, 8) + u(1, 4)
        out += bytes((i, int(profile == 2 and i == 0)))
    if profile == 2:
        out += u(2, 4)
        for height in (1, 2):
            out += u(height, 8) + bytes((height,)) * 128
        out += u(1, 4) + u(0, 4) + bytes((3,)) * 64 + bytes((4,)) * 32
    return bytes(out)


class FrtfTests(unittest.TestCase):
    def view(self, relative="profile-a/minimal/minimal"):
        return json.loads((FIXTURES / (relative + ".json")).read_text())

    def test_tagged_hash_has_u64_le_utf8_byte_length_and_ordered_parts(self):
        self.assertEqual(frtf.th("é", b"ab", b"c"), hashlib.sha256(
            (2).to_bytes(8, "little") + "é".encode() + b"abc").digest())
        self.assertNotEqual(frtf.th("x", b"a", b"b"), frtf.th("x", b"b", b"a"))

    def test_all_thirteen_public_views_reproduce_authoritative_bytes(self):
        self.assertEqual(len(POSITIVE), 13)
        for relative in POSITIVE:
            with self.subTest(relative=relative):
                view = self.view(relative)
                data = (FIXTURES / (relative + ".frtf")).read_bytes()
                self.assertEqual(frtf.encode_view(view), data)
                result = frtf.validate(data)
                self.assertEqual(result["envelope_digest"], view["envelope_digest"])
                self.assertEqual(result["profile"], 2 if "profile-b/" in relative else 1)
                self.assertEqual(result["pre_digest"], view["pre"])
                self.assertEqual(result["post_digest"], view["post"])
                self.assertEqual(len(result["steps"]), len(view["steps"]))

    def test_state_field_order_and_integer_little_endian(self):
        view = self.view()
        data = frtf.encode_view(view)
        expected = (bytes.fromhex(view["state_pre"]["cm_root"])
                    + view["state_pre"]["cm_count"].to_bytes(32, "little")
                    + bytes.fromhex(view["state_pre"]["nf_root"]))
        self.assertEqual(data[40:136], expected)
        self.assertEqual(frtf.decode(data)["state_pre"][1],
                         view["state_pre"]["cm_count"].to_bytes(32, "little").hex())

    def test_profile_b_renamed_slot_fields_and_covenant_memberships(self):
        view = self.view("profile-b/batches/batch-01")
        normalized = frtf.validate(frtf.encode_view(view))
        self.assertEqual(normalized["body"]["slots"][0]["fefx_digest"],
                         view["slots"][0]["fefx_draft_digest"])
        self.assertEqual(normalized["body"]["slots"][0]["fhdr4_digest"],
                         view["slots"][0]["fhdr4_draft_digest"])
        self.assertEqual(normalized["body"]["sign_fold_root"], view["sign_fold_root_draft"])
        self.assertEqual([i for i, s in enumerate(normalized["steps"]) if s["covenant"]],
                         [t["step"] for t in view["tierc"]])

    def test_independent_minimal_a_and_b_envelopes(self):
        for profile in (1, 2):
            with self.subTest(profile=profile):
                result = frtf.validate(tiny(profile))
                self.assertEqual(result["seed"], 7)
                self.assertEqual(result["state_post"][11], (23).to_bytes(32, "little").hex())
                self.assertEqual(result["steps"][1]["kind"], 1)

    def test_hash_free_scope_does_not_recompute_state_or_commitment(self):
        data = changed(tiny(), 40, (99).to_bytes(32, "little"))
        data = changed(data, 872, (100).to_bytes(32, "little"))
        self.assertEqual(frtf.validate(data)["in_root"], (100).to_bytes(32, "little").hex())

    def test_header_magic_version_profile_workload_flags_reject(self):
        for offset, replacement in ((0, b"NOPE"), (4, b"\x02"), (5, b"\x03"),
                                    (6, b"\x02"), (7, b"\x01")):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(), offset, replacement))

    def test_truncation_and_trailing_bytes_reject(self):
        for data in (b"", tiny()[:7], tiny()[:-1], tiny(2)[:-1], tiny() + b"\x00"):
            with self.subTest(length=len(data)), self.assertRaises(frtf.CodecError):
                frtf.validate(data)

    def test_field_modulus_is_exclusive_but_modulus_minus_one_is_canonical(self):
        for offset in (40, 808, 872, 920, HEADER):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.decode(changed(tiny(), offset, MODULUS.to_bytes(32, "little")))
        self.assertEqual(frtf.validate(changed(tiny(), 40,
                         (MODULUS - 1).to_bytes(32, "little")))["state_pre"][0],
                         (MODULUS - 1).to_bytes(32, "little").hex())

    def test_batch_and_deposit_ranges_reject(self):
        for offset, number in ((16, 0), (24, 0), (32, 0), (32, 1 << 47), (904, 1)):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(), offset, number.to_bytes(8, "little")))

    def test_bounded_counts_and_full_step_count_reject_before_iteration(self):
        for count in (0, 1, 65, (1 << 32) - 1):
            with self.subTest(count=count), self.assertRaises(frtf.CodecError):
                frtf.decode(changed(tiny(), 1056, count.to_bytes(4, "little")))
        with self.assertRaises(frtf.CodecError):
            frtf.decode(changed(tiny(), 6, b"\x01"))
        with self.assertRaises(frtf.CodecError):
            frtf.decode(changed(tiny(2), HEADER + 2 * STEP, (4097).to_bytes(4, "little")))

    def test_invalid_kind_or_covenant_codes_reject(self):
        for offset, value in ((HEADER + 128, 6), (HEADER + 129, 2)):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.decode(changed(tiny(), offset, bytes((value,))))

    def test_step_chain_endpoint_and_internal_breaks_reject(self):
        for offset in (808, 840, HEADER + 48, HEADER + 80, HEADER + STEP + 80):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(), offset, (88).to_bytes(32, "little")))

    def test_positions_and_transaction_index_order_reject(self):
        for offset, value in ((HEADER + 32, 0), (HEADER + STEP + 32, 65536),
                              (HEADER + STEP + 40, 0), (HEADER + 40, (1 << 64) - 1)):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(), offset, value.to_bytes(8, "little")))

    def test_header_close_gaps_overlap_later_precloses_and_stray_from_reject(self):
        for offset, width, value in ((HEADER + 116, 8, 2), (HEADER + 124, 4, 0),
                                     (HEADER + 124, 4, 2), (HEADER + STEP + 116, 8, 1),
                                     (HEADER + STEP + 112, 4, 1)):
            with self.subTest(offset=offset, value=value), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(), offset, value.to_bytes(width, "little")))

    def test_profile_a_covenant_rejects(self):
        with self.assertRaises(frtf.CodecError):
            frtf.validate(changed(tiny(), HEADER + 129, b"\x01"))

    def test_profile_b_slot_coverage_and_order_reject(self):
        body = HEADER + 2 * STEP
        for offset, value in ((body + 4, 2), (body + 4 + 136, 1)):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(2), offset, value.to_bytes(8, "little")))

    def test_profile_b_tierc_bounds_association_and_digests_reject(self):
        count = HEADER + 2 * STEP + 4 + 2 * 136
        for offset, replacement in ((count, (3).to_bytes(4, "little")),
                                    (count + 4, (1).to_bytes(4, "little")),
                                    (count + 4, (2).to_bytes(4, "little")),
                                    (count + 8 + 32, bytes((9,)) * 32)):
            with self.subTest(offset=offset), self.assertRaises(frtf.CodecError):
                frtf.validate(changed(tiny(2), offset, replacement))

    def test_view_claimed_envelope_digest_is_required_and_binding(self):
        for mutation in (None, "00" * 32):
            view = self.view()
            if mutation is None:
                del view["envelope_digest"]
            else:
                view["envelope_digest"] = mutation
            with self.subTest(mutation=mutation), self.assertRaises(frtf.CodecError):
                frtf.encode_view(view)

    def test_view_scalar_types_missing_fields_and_widths_are_codec_errors(self):
        for key, value in (("seed", "0x10000000000000000"), ("batch_no", True),
                           ("fee_sum", -1), ("dep_to", 1 << 64), ("steps", None),
                           ("pre", "ff" * 32), ("prev_envelope_digest", "00"),
                           ("profile", "C"), ("format_version", 2)):
            view = self.view()
            view[key] = value
            with self.subTest(key=key), self.assertRaises(frtf.CodecError):
                frtf.encode_view(view)
        view = self.view()
        del view["state_pre"]["cm_root"]
        with self.assertRaises(frtf.CodecError):
            frtf.encode_view(view)

    def test_view_metadata_is_not_followed_as_a_path(self):
        view = self.view()
        original = frtf.encode_view(view)
        view["frtf"] = "/does/not/exist/private.frtf"
        view["steps"][0]["sherlock_input"] = {"tx": "/does/not/exist"}
        self.assertEqual(frtf.encode_view(view), original)

    def test_view_invalid_kind_and_tierc_duplicate_are_rejected(self):
        view = self.view()
        view["steps"][0]["kind"] = "invented"
        with self.assertRaises(frtf.CodecError):
            frtf.encode_view(view)
        view = self.view("profile-b/batches/batch-01")
        view["tierc"].append(copy.deepcopy(view["tierc"][0]))
        with self.assertRaises(frtf.CodecError):
            frtf.encode_view(view)


if __name__ == "__main__":
    unittest.main()
