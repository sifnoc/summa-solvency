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
function submitCommitment(uint256 mstRoot, uint256[] rootBalances, struct Summa.Cryptocurrency[] cryptocurrencies, uint256 timestamp) public
```

Submit liabilities commitment for the Custodian

**Parameters**

| Name             | Type                           | Description                                                                          |
| ---------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| mstRoot          | uint256                        | Merkle sum tree root of the Custodian's liabilities                                  |
| rootBalances     | uint256\[]                     | The total sums of the liabilities included into the Merkle sum tree                  |
| cryptocurrencies | struct Summa.Cryptocurrency\[] | The cryptocurrencies included into the Merkle sum tree                               |
| timestamp        | uint256                        | The timestamp at which the Custodian took the snapshot of its assets and liabilities |

#### verifyInclusionProof

```solidity
function verifyInclusionProof(bytes proof, uint256[] publicInputs, uint256 timestamp) public view returns (bool)
```

Verify the proof of user inclusion into the liabilities MST

**Parameters**

| Name         | Type       | Description                                                                                                 |
| ------------ | ---------- | ----------------------------------------------------------------------------------------------------------- |
| proof        | bytes      | ZK proof                                                                                                    |
| publicInputs | uint256\[] | proof public inputs                                                                                         |
| timestamp    | uint256    | The timestamp at which the Custodian took the snapshot of its assets and liabilities to generate this proof |

