# Univariate Grand Sum

Univariate grand sum circuit serves two main purposes:

* interpolating user entries as polynomials and committing to them;
* performing range check on the user balance values.

The interpolation and commitment come "for free" with the Halo2 framework. The advice columns in Halo2 are interpolated as polynomials over the roots of unity during the proof generation. The KZG commitment to each of the interpolated polynomials is then produced by the framework and recorded inside the proof transcript. This allows the prover to simply fill the advice columns with user entry values without performing any additional actions. An important detail to note here is that the cryptocurrency balance advice columns are unblinded columns, because blinding values would alter the polynomial grand sum (please refer to the [grand sum calculation technique](../cryptographic-primitives/univariate-polynomial-grand-sum.md) to see why).&#x20;

The range check is performed using the range check chip.

## Range Check Chip

Range check chip constrains the input value to be within a range of 8 bytes. In the context of Proof of Reserves, performing the range check is necessary to avoid overflow during the polynomial grand sum calculation. The chip performs a decomposition of the checked value into two-byte words and ensures that each of the word values is found in a lookup table. The lookup table is loaded with all the possible values that a word can take (`0` to `2^16 - 1`). Therefore, the chip is decomposing the 8-byte value into four words.

To prove that the 64-bit balance values would not cause the overflow of the grand sum, let's consider the case at the limit in which we have 2^28 users and all their balances are the maximum possible (namely, $$2^{64} - 1$$):

```
>>> 2**28 * (2**64-1)
4951760157141521099328061440
>>> n = 4951760157141521099328061440
>>> num_bits = n.bit_length()
>>> print(num_bits)
92
```

The resulting grand sum is only 92 bits. Therefore, we can conclude that the range check of 64 bits on the $$2^{28}$$ user balances safely removes the risk of overflow in the grand sum calculation.

## Circuit Configuration

The circuit configuration allocates one advice column for the user ID values and N\_CURRENCIES unblinded advice columns for the user cryptocurrency balances. One fixed column is allocated for the lookup table of the range check chip. This fixed column is shared across all the range check chips. The circuit configures one range check chip per user per cryptocurrency. Each range check chip configuration allocates four advice columns to decompose the balance value into four two-byte words.&#x20;

The circuit does not calculate any kind of public outputs because the desired results of the circuit operation are the KZG commitments to advice polynomials that Halo2 framework writes to the proof transcript during the proof generation.

## Circuit Operation

The circuit synthesis begins with assigning the user entries. The user ID is assigned to the first advice column, and the N\_CURRENCIES user balances are assigned to the corresponding unblinded advice columns.&#x20;

Later, the circuit assigns the values `0...2^16 - 1` to the fixed column used for the range check chips.

Finally, the circuit fills the range check chip advice cells with the decomposed values for the corresponding user cryptocurrency balance.&#x20;

The ZK circuit structure is shown in the figure below.

<figure><picture><source srcset="../.gitbook/assets/Untitled Diagram(6)(2)(1).png" media="(prefers-color-scheme: dark)"><img src="../.gitbook/assets/Untitled Diagram(6)(2)_light.png" alt=""></picture><figcaption><p>Univariate grand sum ZK circuit structure</p></figcaption></figure>
