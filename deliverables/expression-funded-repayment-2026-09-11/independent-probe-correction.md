The initial independent probe used unsupported unary ! syntax and reserved metadata field amount.
Corrected the probe to `is_negative(nominal) == false` and source field `transferAmount`, matching the existing grammar and adapter.
The retained first output records this probe construction failure; it is not an adapter behavior failure.
