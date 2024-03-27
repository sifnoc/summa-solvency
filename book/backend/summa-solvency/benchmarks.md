# Benchmarks

Benchmarks ran on an AWS m7a.8xlarge with 32 cores and 128GB RAM. The generics parameters are :&#x20;

* `N_CURRENCIES = 1`&#x20;
* `N_BYTES = 8`
* `LEVELS=20` (2^20 users = 1,048,576 users)

**Merkle Sum Tree**

| MST init |
| -------- |
| 24.542 s |

| MST init (sorted) |
| ----------------- |
| 25.754 s          |

**Proof of Inclusion Circuit**

<table><thead><tr><th>VK Gen</th><th width="110">Pk Gen</th><th width="161">Proof Generation</th><th width="175">Proof Verification</th><th>Proof Size (bytes)</th></tr></thead><tbody><tr><td>106.88 ms</td><td>125.87 ms</td><td>403.33 ms</td><td>4.0000 ms</td><td>1632</td></tr></tbody></table>

To reproduce benchmarks like the one above, please refer to [this](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/benches/full\_solvency\_flow.rs)

The benchmark results based on larger userbase (2^28) are in the table below:

| MST init |
| -------- |
| 6279.0 s |

<table><thead><tr><th>VK Gen</th><th width="110">Pk Gen</th><th width="161">Proof Generation</th><th width="175">Proof Verification</th><th>Proof Size (bytes)</th></tr></thead><tbody><tr><td>110.61 ms</td><td>152.24 ms</td><td>460.05 ms</td><td>4.0001 ms</td><td>1632</td></tr></tbody></table>

Notably, the results show that there is no significant time difference in the Proof generation or verification between the two datasets despite the substantial increase in entry size.

Custodians can reduce the time required to build a Merkle sum tree by utilizing [`summa-aggregation`](../summa-aggregation/). For detailed benchmark results, please refer to [here](../summa-aggregation/benchmarks.md).
