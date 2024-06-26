# Merkle Sum Tree Inclusion

`MstInclusion` [Circuit](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/src/circuits/merkle\_sum\_tree.rs) proves the inclusion of the user's cryptocurrency balances into the Merkle Sum Tree built by the Custodian to snapshot its liabilities. The [inclusion proof](../introduction/protocol-flow.md#3.-inclusion-proof) is generated by the custodian and verified by the user.&#x20;

In essence, the circuit constrains the classic Merkle proof and the liability sums calculation.

### Chips

All chips impose the necessary constraints corresponding to a single component of the overall circuit. The detailed constraint descriptions are omitted for brevity.

#### Poseidon Chip

[`PoseidonChip`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/src/chips/poseidon/hash.rs#L40) performs Poseidon hashing of the leaf and nodes of the Merkle sum tree (including the root) and outputs the result. The main circuit is using the chip with two different configurations:

* The configuration for entry hashing takes the username and N user cryptocurrency balances as inputs.
* The configuration for node hashing is used for all the middle nodes and the root. It takes, in order, N sums of cryptocurrency balances of the two child nodes and the hashes of each child node as its inputs.

#### Merkle Sum Tree Chip

[`MerkleSumTreeChip`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/src/chips/poseidon/hash.rs#L40) enforces the following contraints:

* Checks that path index is a boolean value.
* Swaps the sibling hashes along the Merkle path according to the path index for each level.
* Calculates the sum of sibling cryptocurrency balances to be used as balances for the next level node.

#### Range Check Chip

[`RangeCheckChip`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/src/chips/range/range\_check.rs#L52)enforces that a certain value exists in a range of `N_BYTES`. In the context of Proof of Solvency, performing range check is necessary to avoid the overflow during the cryptocurrency balances summation.&#x20;

The chip performs a decomposition of the checked value into bytes and ensures that each of the byte values is found in a lookup table. The lookup table is loaded with all the possible values that one byte can take (`0` to `2^8 - 1`).

### Inputs

The [`MstInclusionCircuit`](https://github.com/summa-dev/summa-solvency/blob/master/zk\_prover/src/circuits/merkle\_sum\_tree.rs#L31) takes the following inputs:

* `entry` - username and cryptocurrency balances of the user whose inclusion is being verified;
* `path_indices` - the boolean Merkle path indices indicating whether the current level node is on the left or right side;
* `sibling_leaf_node_hash_preimage` - The preimage of the hash that corresponds to the sibling leaf (part of the Merkle Proof);
* `sibling_middle_node_hash_preimages` - The preimages of the hashes that correspond to the sibling non-leaf nodes (part of the Merkle Proof).
* `root` - The root of the Merkle Sum Tree. This is composed of the root hash and the root cryptocurrency balance sums of the Merkle Sum Tree&#x20;

### Public Inputs/Outputs

The circuit exposes to the public:

* The leaf hash of the user whose inclusion is being verified.
* The root hash of the Merkle sum tree &#x20;
* The root cryptocurrency balance sums

### Circuit Operation

The circuit begins with the hashing of the username and user cryptocurrency balance values in the `poseidon_entry_chip` These values are a private witness. The resulting hash is then exposed as a public instance value.

The circuit then proceeds to load the range check lookup table with all possible one-byte values for the `range_check_chip` .

Further on, the circuit computes the Merkle Root starting from the Merkle Proof and the leaf node inside a loop that goes through all the levels of the Merkle Sum Tree. The loop also performs cryptocurrency balances summation for all the levels.

For the leaf level, the circuit first performs the hashing of `sibling_leaf_node_hash_preimage` to get the sibling leaf node. The circuit also performs the range checks on the user cryptocurrency balances `entry.balances[]`and the sibling leaf cryptocurrency balances using the `range_check_chip`.

For the middle and root levels, the circuit first performs the hashing of `sibling_leaf_node_hash_preimages[level]` to get the sibling middle node. The circuit also performs the range checks on the sibling node cryptocurrency balances using the `range_check_chip`.

For each level, the two nodes (current and sibling) are assigned to `merkle_sum_tree_chip` together with the path index associated to that level. The chip:

* Performs the swap, if necessary, according to the boolean value in the path index
* Calculates the cryptocurrency balance sums for parent node &#x20;
* Calculates the hash of the parent node

After exiting the loop, the circuit exposes the root hash and the resulting root cryptocurrency balance sums as public outputs.

The ZK circuit structure is shown in the figure below.

<figure><picture><source srcset="../.gitbook/assets/Untitled Diagram(6)(2).png" media="(prefers-color-scheme: dark)"><img src="../.gitbook/assets/Untitled Diagram(6)(1).png" alt=""></picture><figcaption><p>Merkle Sum Tree Inclusion ZK Circuit Structure</p></figcaption></figure>

### ZK Merkle Proof Vs Classic Merkle Proof

Merkle tree has long been known as a convenient data structure to prove the inclusion of an element into a dataset (see [Merkle Sum Tree](../cryptographic-primitives/merkle-sum-tree.md) for the reference). However, in case of a Merkle sum tree, a simple Merkle proof would expose the sensitive information about the sibling user cryptocurrency balances necessary to verify the correct summation. Even when depersonalized, the sibling user balance values could be analyzed and cross-checked with Custodian's incoming and outgoing blockchain transactions, potentially revealing the sibling's on-chain identity. The ZK Merkle proof solves this problem by hiding the sensitive user data as the circuit's private inputs.

{% hint style="warning" %}
The proof of inclusion only performs the range check on the entry balances, the sibling leaf node balances and the sibling middle node balances but not on the summation. There's still a risk of overflow during the summation. This needs to be checked during the verification as described [here](../smart-contract/summa.sol/#verifier-contract-validity) in step 4.&#x20;
{% endhint %}
