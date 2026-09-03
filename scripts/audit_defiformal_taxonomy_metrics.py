#!/usr/bin/env python3
"""Reproduce DeFiFormal label-cohesion and nearest-neighbour measurements."""

from __future__ import annotations

import csv
import json
import math
import os
import subprocess
from collections import Counter
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
DEFIFORMAL_ROOT = Path(os.environ.get("DEFIFORMAL_ROOT", "/home/charl/defiformal"))
LANES = DEFIFORMAL_ROOT / "corpus50" / "lanes"

CATEGORY_IDS = {
    "Spot DEX / AMM": "D01",
    "Lending": "D02",
    "CDP / collateral-backed stablecoins": "D03",
    "Liquid staking & restaking": "D04",
    "Perpetuals / derivatives": "D05",
    "Yield / vaults / aggregators": "D06",
    "Bridges / cross-domain": "D07",
    "Intents / aggregation / order flow": "D08",
    "RWA / tokenised treasuries & private credit": "D09",
    "Options / structured products": "D10",
    "Reserve-backed / fiat stablecoin issuers": "D11",
    "Prediction markets & other (uncategorized large protocols)": "D12",
}


def jaccard(left: frozenset[str], right: frozenset[str]) -> float:
    union = left | right
    return len(left & right) / len(union) if union else 1.0


def mean(values: list[float]) -> float:
    return sum(values) / len(values)


def adjusted_rand_index(actual: list[str], predicted: list[int]) -> float:
    """Compute the Hubert-Arabie adjusted Rand index without third-party code."""

    actual_groups = sorted(set(actual))
    predicted_groups = sorted(set(predicted))
    contingency = {
        (left, right): sum(
            1 for a_value, p_value in zip(actual, predicted, strict=True)
            if a_value == left and p_value == right
        )
        for left in actual_groups
        for right in predicted_groups
    }
    choose_two = lambda value: value * (value - 1) // 2
    joint = sum(choose_two(value) for value in contingency.values())
    actual_pairs = sum(choose_two(actual.count(group)) for group in actual_groups)
    predicted_pairs = sum(choose_two(predicted.count(group)) for group in predicted_groups)
    total_pairs = choose_two(len(actual))
    expected = actual_pairs * predicted_pairs / total_pairs
    maximum = (actual_pairs + predicted_pairs) / 2
    return (joint - expected) / (maximum - expected) if maximum != expected else 1.0


def agglomerative_partitions(
    distances: list[list[float]], method: str, requested: set[int]
) -> dict[int, list[int]]:
    """Reproduce SciPy-style average, complete, and Ward linkage deterministically."""

    count = len(distances)
    clusters: dict[int, list[int]] = {index: [index] for index in range(count)}
    active = set(clusters)
    pair_distances = {
        (left, right): distances[left][right]
        for left in range(count)
        for right in range(left + 1, count)
    }
    partitions: dict[int, list[int]] = {}
    next_cluster = count

    def key(left: int, right: int) -> tuple[int, int]:
        return (left, right) if left < right else (right, left)

    while len(active) > min(requested):
        left, right = min(
            (key(left, right) for left in active for right in active if left < right),
            key=lambda pair: (pair_distances[pair], pair[0], pair[1]),
        )
        left_size = len(clusters[left])
        right_size = len(clusters[right])
        merge_distance = pair_distances[key(left, right)]
        others = sorted(active - {left, right})
        new_distances: dict[tuple[int, int], float] = {}
        for other in others:
            left_distance = pair_distances[key(left, other)]
            right_distance = pair_distances[key(right, other)]
            if method == "average":
                distance = (
                    left_size * left_distance + right_size * right_distance
                ) / (left_size + right_size)
            elif method == "complete":
                distance = max(left_distance, right_distance)
            elif method == "ward":
                other_size = len(clusters[other])
                total_size = left_size + right_size + other_size
                squared = (
                    (other_size + left_size) / total_size * left_distance**2
                    + (other_size + right_size) / total_size * right_distance**2
                    - other_size / total_size * merge_distance**2
                )
                distance = math.sqrt(max(0.0, squared))
            else:
                raise ValueError(f"Unsupported linkage method: {method}")
            new_distances[key(next_cluster, other)] = distance

        clusters[next_cluster] = clusters[left] + clusters[right]
        active.remove(left)
        active.remove(right)
        active.add(next_cluster)
        pair_distances.update(new_distances)
        next_cluster += 1

        if len(active) in requested:
            labels = [-1] * count
            for label, cluster_id in enumerate(sorted(active)):
                for member in clusters[cluster_id]:
                    labels[member] = label
            partitions[len(active)] = labels

    return partitions


def scipy_nn_chain_partitions(
    distances: list[list[float]], method: str, requested: set[int]
) -> dict[int, list[int]]:
    """Match SciPy's nearest-neighbour-chain tie behavior for reducible linkage."""

    count = len(distances)
    matrix = [row[:] for row in distances]
    sizes = [1] * count
    members: list[set[int]] = [{index} for index in range(count)]
    chain: list[int] = []
    merges: list[tuple[float, int, set[int]]] = []

    for operation in range(count - 1):
        if not chain:
            chain.append(next(index for index, size in enumerate(sizes) if size > 0))
        while True:
            left = chain[-1]
            if len(chain) > 1:
                right = chain[-2]
                minimum = matrix[left][right]
            else:
                right = -1
                minimum = math.inf
            for candidate in range(count):
                if candidate == left or sizes[candidate] == 0:
                    continue
                candidate_distance = matrix[left][candidate]
                if candidate_distance < minimum:
                    minimum = candidate_distance
                    right = candidate
            if len(chain) > 1 and right == chain[-2]:
                break
            chain.append(right)

        first = chain[-2]
        second = chain[-1]
        del chain[-2:]
        if first > second:
            first, second = second, first
        first_size = sizes[first]
        second_size = sizes[second]
        union = members[first] | members[second]
        merges.append((minimum, operation, union))
        for other in range(count):
            if sizes[other] == 0 or other in {first, second}:
                continue
            first_distance = matrix[first][other]
            second_distance = matrix[second][other]
            if method == "average":
                updated = (
                    first_size * first_distance + second_size * second_distance
                ) / (first_size + second_size)
            elif method == "complete":
                updated = max(first_distance, second_distance)
            elif method == "ward":
                total = first_size + second_size + sizes[other]
                squared = (
                    (sizes[other] + first_size) / total * first_distance**2
                    + (sizes[other] + second_size) / total * second_distance**2
                    - sizes[other] / total * minimum**2
                )
                updated = math.sqrt(max(0.0, squared))
            else:
                raise ValueError(f"Unsupported linkage method: {method}")
            matrix[second][other] = updated
            matrix[other][second] = updated
        sizes[first] = 0
        sizes[second] = first_size + second_size
        members[first] = set()
        members[second] = union

    ordered_merges = sorted(merges, key=lambda merge: (merge[0], merge[1]))
    partitions: dict[int, list[int]] = {}
    groups = [{index} for index in range(count)]
    for merge_number, (_, _, union) in enumerate(ordered_merges, start=1):
        affected = [group for group in groups if group & union]
        groups = [group for group in groups if not group & union]
        groups.append(set().union(*affected))
        cluster_count = count - merge_number
        if cluster_count in requested:
            labels = [-1] * count
            for label, group in enumerate(sorted(groups, key=lambda value: min(value))):
                for member in group:
                    labels[member] = label
            partitions[cluster_count] = labels
    return partitions


def main() -> None:
    rows: list[dict[str, object]] = []
    for lane_path in sorted(LANES.glob("*.json")):
        lane = json.loads(lane_path.read_text(encoding="utf-8"))
        for category in lane["categories"]:
            category_id = CATEGORY_IDS[category["category"]]
            for protocol in category["protocols"]:
                rows.append(
                    {
                        "name": protocol["name"],
                        "category": category_id,
                        "elements": frozenset(protocol["elements"]),
                    }
                )

    categories = sorted({str(row["category"]) for row in rows})
    one_correct = 0
    three_correct = 0
    one_by_category = {category: Counter(total=0, correct=0) for category in categories}
    predictions: list[dict[str, object]] = []
    for index, row in enumerate(rows):
        neighbours = sorted(
            (
                (jaccard(row["elements"], candidate["elements"]), candidate_index, candidate)
                for candidate_index, candidate in enumerate(rows)
                if candidate_index != index
            ),
            key=lambda item: (-item[0], item[1]),
        )
        one_prediction = str(neighbours[0][2]["category"])
        top_three = neighbours[:3]
        votes = Counter(str(item[2]["category"]) for item in top_three)
        maximum_votes = max(votes.values())
        tied_categories = {category for category, count in votes.items() if count == maximum_votes}
        three_prediction = next(
            str(item[2]["category"])
            for item in top_three
            if str(item[2]["category"]) in tied_categories
        )
        actual = str(row["category"])
        one_match = one_prediction == actual
        three_match = three_prediction == actual
        one_correct += one_match
        three_correct += three_match
        one_by_category[actual]["total"] += 1
        one_by_category[actual]["correct"] += one_match
        predictions.append(
            {
                "protocol": row["name"],
                "actual": actual,
                "one_nn": one_prediction,
                "three_nn": three_prediction,
                "nearest_protocol": neighbours[0][2]["name"],
                "nearest_similarity": round(neighbours[0][0], 6),
            }
        )

    matrix: dict[str, dict[str, float]] = {}
    for left_category in categories:
        left_rows = [row for row in rows if row["category"] == left_category]
        matrix[left_category] = {}
        for right_category in categories:
            right_rows = [row for row in rows if row["category"] == right_category]
            if left_category == right_category:
                comparisons = [
                    jaccard(left_rows[i]["elements"], left_rows[j]["elements"])
                    for i in range(len(left_rows))
                    for j in range(i + 1, len(left_rows))
                ]
            else:
                comparisons = [
                    jaccard(left["elements"], right["elements"])
                    for left in left_rows
                    for right in right_rows
                ]
            matrix[left_category][right_category] = round(mean(comparisons), 2)

    protocol_distances = [
        [1.0 - jaccard(left["elements"], right["elements"]) for right in rows]
        for left in rows
    ]
    actual_labels = [str(row["category"]) for row in rows]
    hierarchical_ari: dict[str, dict[str, float]] = {}
    hierarchical_ari_raw: dict[str, dict[str, float]] = {}
    hierarchical_partitions: dict[str, dict[str, list[list[str]]]] = {}
    for method in ("average", "complete", "ward"):
        partitions = scipy_nn_chain_partitions(protocol_distances, method, {7, 12})
        hierarchical_ari_raw[method] = {
            str(size): round(adjusted_rand_index(actual_labels, partitions[size]), 6)
            for size in (7, 12)
        }
        hierarchical_ari[method] = {
            str(size): round(hierarchical_ari_raw[method][str(size)], 2)
            for size in (7, 12)
        }
        hierarchical_partitions[method] = {
            str(size): [
                [str(rows[index]["name"]) for index, label in enumerate(partitions[size]) if label == cluster]
                for cluster in sorted(set(partitions[size]))
            ]
            for size in (7, 12)
        }

    commit = subprocess.check_output(
        ["git", "-C", str(DEFIFORMAL_ROOT), "rev-parse", "HEAD"], text=True
    ).strip()
    result = {
        "schema_version": 1,
        "source_repository": "https://github.com/CharlesHoskinson/defiformal.git",
        "source_commit": commit,
        "source_dirty": bool(
            subprocess.check_output(
                ["git", "-C", str(DEFIFORMAL_ROOT), "status", "--porcelain"], text=True
            ).strip()
        ),
        "protocols": len(rows),
        "categories": len(categories),
        "method": {
            "distance": "1 - Jaccard over the protocol element set",
            "one_nn_tie_break": "first protocol in sorted lane-file and corpus order",
            "three_nn_vote_tie_break": "category of the nearest tied neighbour",
            "within_category_matrix": "mean over unique unordered protocol pairs",
            "cross_category_matrix": "mean over the Cartesian product",
        },
        "one_nn": {
            "correct": one_correct,
            "total": len(rows),
            "accuracy": round(one_correct / len(rows), 6),
        },
        "three_nn": {
            "correct": three_correct,
            "total": len(rows),
            "accuracy": round(three_correct / len(rows), 6),
        },
        "one_nn_by_category": {
            category: dict(one_by_category[category]) for category in categories
        },
        "mean_jaccard": matrix,
        "hierarchical_ari": hierarchical_ari,
        "hierarchical_ari_raw": hierarchical_ari_raw,
        "hierarchical_partitions": hierarchical_partitions,
        "qualification": (
            "These measurements reproduce element-set coherence, not a causal or economic "
            "validation of the labels. Results depend on the documented deterministic tie rule."
        ),
    }

    evidence_path = (
        MORIARTY_ROOT / "evidence" / "defiformal-taxonomy-metrics-2026-09-02.json"
    )
    evidence_path.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    predictions_path = (
        MORIARTY_ROOT / "evidence" / "defiformal-taxonomy-nn-predictions-2026-09-02.csv"
    )
    with predictions_path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(predictions[0]))
        writer.writeheader()
        writer.writerows(predictions)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
