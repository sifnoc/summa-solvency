# Benchmarks

The following table presents the benchmark results for Summa Aggregation, specifically focused on building the `AggregationMerkleSumTree` as part of the V1 improvements to Summa Solvency.

The benchmarks were conducted using an AWS `m7a.8xlarge` main machine with 32 vCores and 128GB RAM, while the workers operated on AWS `c7a.4xlarge` instances with 16 vCores and 32GB RAM.

The number of "Chunks" indicates how the 2^20 entries (equivalent to 1,048,576 users) were divided into smaller segments for processing.

<table data-full-width="true"><thead><tr><th width="186" align="center">-</th><th align="right">Time for 32 Chunks</th><th align="right">Time for 64 Chunks</th><th align="right">Time for 128 Chunks</th></tr></thead><tbody><tr><td align="center">4 Workers</td><td align="right">47.166 s</td><td align="right">47.357 s</td><td align="right">46.612 s</td></tr><tr><td align="center">8 Workers</td><td align="right">24.262 s</td><td align="right">23.619 s</td><td align="right">23.636 s</td></tr><tr><td align="center">16 Workers</td><td align="right">12.378 s</td><td align="right">12.377 s</td><td align="right">12.372 s</td></tr></tbody></table>

The benchmark results based on larger entries (2^28) are in the table below:

<table data-full-width="true"><thead><tr><th width="186" align="center">-</th><th align="right">Time for 8192 Chunks</th><th align="right">Time for 16384 Chunks</th></tr></thead><tbody><tr><td align="center">20 Workers</td><td align="right">2498 s</td><td align="right">2469 s</td></tr></tbody></table>

These results indicate that, in most cases, increasing the number of chunks and workers leads to improved performance. This benchmark suggests that custodians can enhance performance by deploying larger machines with additional workers.
