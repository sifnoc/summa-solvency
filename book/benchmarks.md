# Benchmarks

Benchmarks ran on an AWS m7a.8xlarge with 32 cores and 128GB RAM. The generics parameters are :&#x20;

* `K = 18`
* `N_CURRENCIES = 175`&#x20;
* `N_USERS = 2^18 - 6 (= 262,138 users)`

**zk-SNARK Proof**

| zkSNARK Construction |
| -------------------- |
| 2569.2 s             |

**Commitment Proof**

| Benchmark               | Time      |
| ----------------------- | --------- |
| Grand sum opening       | 624.48 ms |
| Grand sum opening (GWC) | 865.42 ms |
| Grand sum verification  | 3.9942 ms |

**User Inclusion Proof**

|                             | Time      |
| --------------------------- | --------- |
| User inclusion opening      | 623.11 ms |
| User inclusion verification | 3.9688 ms |

To reproduce benchmarks like the one above, please refer to [this](https://github.com/summa-dev/summa-solvency/tree/v2/prover#benchmarks)
