# Solidity API

submitProofOfAddressOwnership

```solidity
function submitProofOfAddressOwnership(struct Summa.AddressOwnershipProof[] _addressOwnershipProofs) public
```

Submit an optimistic proof of multiple address ownership for the Custodian. The proof is subject to an off-chain verification as it's not feasible to verify the signatures of non-EVM chains in an Ethereum smart contract.

**Parameters**

| Name                     | Type                                  | Description                          |
| ------------------------ | ------------------------------------- | ------------------------------------ |
| \_addressOwnershipProofs | struct Summa.AddressOwnershipProof\[] | The list of address ownership proofs |

#### submitCommitment

```solidity
function submitCommitment(bytes snarkProof, bytes grandSumProof, uint256[] totalBalances, uint256 timestamp) public
```

Submit liabilities commitment for the Custodian

**Parameters**

| Name          | Type       | Description                                                                                    |
| ------------- | ---------- | ---------------------------------------------------------------------------------------------- |
| snarkProof    | bytes      | A zk-SNARK proof from Univariate Grand Sum circuit includes KZG commitments to the polynomials |
| grandSumProof | bytes      | A KZG opening proof for the grand total sum of cryptocurrency balances                         |
| totalBalances | uint256\[] | The grand total sum of all user balances included                                              |
| timestamp     | uint256    | The timestamp at which the Custodian took the snapshot of its assets and liabilities           |



#### verifyInclusionProof

```solidity
function verifyInclusionProof(uint256 timestamp, bytes inclusionProof, uint256[] challenges, uint256[] values) public view returns (bool)
```

Verify the proof of user inclusion into the liabilities commitment

**Parameters**

| Name           | Type       | Description                                                                                                 |
| -------------- | ---------- | ----------------------------------------------------------------------------------------------------------- |
| timestamp      | uint256    | The timestamp at which the Custodian took the snapshot of its assets and liabilities to generate this proof |
| inclusionProof | bytes      | A KZG proof for the polynomials containing usernames and balances, associated with a specific user index    |
| challenges     | uint256\[] | An array representing a point in the G2 affine space, corresponding to the user index                       |
| values         | uint256\[] | The user data that contains user ID and balances                                                            |

