// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./interfaces/IBN256G2.sol";

contract InclusionVerifier {
    // Calldata positions for proofs
    uint256 internal constant       PROOF_LEN_CPTR = 0xc4;
    uint256 internal constant           PROOF_CPTR = 0xe4;

    // Memory positions for the verifying key.
    // The memory location starts at 0x200 due to the maximum operation on the ec_pairing function being 0x180.
    uint256 internal constant                VK_MPTR = 0x200;
    uint256 internal constant         VK_DIGEST_MPTR = 0x200;
    uint256 internal constant                 K_MPTR = 0x220;
    uint256 internal constant             N_INV_MPTR = 0x240;
    uint256 internal constant             OMEGA_MPTR = 0x260;
    uint256 internal constant         OMEGA_INV_MPTR = 0x280;
    uint256 internal constant    OMEGA_INV_TO_L_MPTR = 0x2a0;
    uint256 internal constant     NUM_INSTANCES_MPTR = 0x2c0;
    uint256 internal constant             LHS_X_MPTR = 0x2e0;
    uint256 internal constant             LHS_Y_MPTR = 0x300;
    uint256 internal constant     NUM_ACC_LIMBS_MPTR = 0x320;
    uint256 internal constant NUM_ACC_LIMB_BITS_MPTR = 0x340;
    uint256 internal constant              G1_X_MPTR = 0x360;
    uint256 internal constant              G1_Y_MPTR = 0x380;
    uint256 internal constant            G2_X_1_MPTR = 0x3a0;
    uint256 internal constant            G2_X_2_MPTR = 0x3c0;
    uint256 internal constant            G2_Y_1_MPTR = 0x3e0;
    uint256 internal constant            G2_Y_2_MPTR = 0x400;
    uint256 internal constant      NEG_S_G2_X_1_MPTR = 0x420;
    uint256 internal constant      NEG_S_G2_X_2_MPTR = 0x440;
    uint256 internal constant      NEG_S_G2_Y_1_MPTR = 0x460;
    uint256 internal constant      NEG_S_G2_Y_2_MPTR = 0x480;

    // Declare state variable for BN256G2Interface
    BN256G2Interface private BN256G2; 

    // Constructor to store the bn256g2 address
    constructor(address bn256g2) {
       BN256G2 = BN256G2Interface(bn256g2);
    }

    struct G2Point {
        uint256 x1;
        uint256 x2;
        uint256 y1;
        uint256 y2;
    }

    function g2_to_c(address vk, uint256 scalar) private view returns (G2Point memory) {
        uint256 g2_x1;
        uint256 g2_x2;
        uint256 g2_y1;
        uint256 g2_y2;

        // Load values from memory to variables
        assembly {
            extcodecopy(vk, G2_X_1_MPTR, 0x1a0, 0x20)
            extcodecopy(vk, G2_X_2_MPTR, 0x1c0, 0x20)
            extcodecopy(vk, G2_Y_1_MPTR, 0x1e0, 0x20)
            extcodecopy(vk, G2_Y_2_MPTR, 0x200, 0x20)
            g2_x1 := mload(G2_X_1_MPTR)
            g2_x2 := mload(G2_X_2_MPTR)
            g2_y1 := mload(G2_Y_1_MPTR)
            g2_y2 := mload(G2_Y_2_MPTR)
        }

        (uint256 g2_to_my_x1, uint256 g2_to_my_x2, uint256 g2_to_my_y1, uint256 g2_to_my_y2) = BN256G2.ECTwistMul(scalar, g2_x2, g2_x1, g2_y2, g2_y1);
        return G2Point(g2_to_my_x1, g2_to_my_x2, g2_to_my_y1, g2_to_my_y2);
    }

    function g2_tau_g2c(address vk, G2Point memory point) private view returns (G2Point memory) {
        uint256 neg_s_g2_x1;
        uint256 neg_s_g2_x2;
        uint256 neg_s_g2_y1;
        uint256 neg_s_g2_y2;

        // Load values from memory to variables
        assembly {
            extcodecopy(vk, NEG_S_G2_X_1_MPTR, 0x220, 0x20)
            extcodecopy(vk, NEG_S_G2_X_2_MPTR, 0x240, 0x20)
            extcodecopy(vk, NEG_S_G2_Y_1_MPTR, 0x260, 0x20)
            extcodecopy(vk, NEG_S_G2_Y_2_MPTR, 0x280, 0x20)
            neg_s_g2_x1 := mload(NEG_S_G2_X_1_MPTR)
            neg_s_g2_x2 := mload(NEG_S_G2_X_2_MPTR)
            neg_s_g2_y1 := mload(NEG_S_G2_Y_1_MPTR)
            neg_s_g2_y2 := mload(NEG_S_G2_Y_2_MPTR)
        }
        (uint256 c_s_g2_x1, uint256 c_s_g2_x2, uint256 c_s_g2_y1, uint256 c_s_g2_y2) = 
            BN256G2.ECTwistAdd(point.x1, point.x2, point.y1, point.y2, neg_s_g2_x2, neg_s_g2_x1, neg_s_g2_y2, neg_s_g2_y1);
        
        assembly {
            mstore(NEG_S_G2_X_1_MPTR, c_s_g2_x1)
            mstore(NEG_S_G2_X_2_MPTR, c_s_g2_x2)
            mstore(NEG_S_G2_Y_1_MPTR, c_s_g2_y1)
            mstore(NEG_S_G2_Y_2_MPTR, c_s_g2_y2)
        }
        return G2Point(c_s_g2_x1, c_s_g2_x2, c_s_g2_y1, c_s_g2_y2);
    }

    function verifyProof(
        address vk,
        bytes memory challenge,
        bytes calldata proofs,
        uint256[] calldata values
    ) public view returns (bytes memory) {
        // Initialize NEG_S_G2 points
        uint256 challengeScalar = abi.decode(challenge, (uint256));

        G2Point memory itermediate_g2 = g2_to_c(vk, challengeScalar);
        g2_tau_g2c(vk, itermediate_g2);

        assembly {
            // Check EC point (x, y) is on the curve.
            // the point is on affine plane, and then return success.
            function check_ec_point(success, proof_cptr, q) -> ret {
                let x := calldataload(proof_cptr)
                let y := calldataload(add(proof_cptr, 0x20))
                ret := and(success, lt(x, q))
                ret := and(ret, lt(y, q))
                ret := and(ret, eq(mulmod(y, y, q), addmod(mulmod(x, mulmod(x, x, q), q), 3, q)))
            }

            // Add (x, y) into point at (0x00, 0x20).
            // Return updated (success).
            function ec_add_acc(success, x, y) -> ret {
                mstore(0x40, x)
                mstore(0x60, y)
                ret := and(success, staticcall(gas(), 0x06, 0x00, 0x80, 0x00, 0x40))
            }

            // Scale point at (0x00, 0x20) by scalar.
            function ec_mul_acc(success, scalar) -> ret {
                mstore(0x40, scalar)
                ret := and(success, staticcall(gas(), 0x07, 0x00, 0x60, 0x00, 0x40))
            }

            // Add (x, y) into point at (0x80, 0xa0).
            // Return updated (success).
            function ec_add_tmp(success, x, y) -> ret {
                mstore(0xc0, x)
                mstore(0xe0, y)
                ret := and(success, staticcall(gas(), 0x06, 0x80, 0x80, 0x80, 0x40))
            }

            // Scale point at (0x80, 0xa0) by scalar.
            // Return updated (success).
            function ec_mul_tmp(success, scalar) -> ret {
                mstore(0xc0, scalar)
                ret := and(success, staticcall(gas(), 0x07, 0x80, 0x60, 0x80, 0x40))
            }

            // Perform pairing check.
            function ec_pairing(success, lhs_x, lhs_y, rhs_x, rhs_y) -> ret {
                mstore(0x00, lhs_x)
                mstore(0x20, lhs_y)
                mstore(0x40, mload(G2_X_1_MPTR))
                mstore(0x60, mload(G2_X_2_MPTR))
                mstore(0x80, mload(G2_Y_1_MPTR))
                mstore(0xa0, mload(G2_Y_2_MPTR))
                mstore(0xc0, rhs_x)
                mstore(0xe0, rhs_y)
                // mstore(0x100, mload(NEG_S_G2_X_1_MPTR))
                // mstore(0x120, mload(NEG_S_G2_X_2_MPTR))
                // mstore(0x140, mload(NEG_S_G2_Y_1_MPTR))
                // mstore(0x160, mload(NEG_S_G2_Y_2_MPTR))

                // This is for Index 3 user.
                mstore(0x100, 0x233ce63d80e3ea8a781291bd72ebca76de04e21f2b08ca15386d82c91245c50a)
                mstore(0x120, 0x22992c3f314d6eae9ce7b3289df7b62b37bf7c166689c431d164af9243cb21f4)
                mstore(0x140, 0x19f2f14868cd0e5889e0cd71f0e39e79e019d1222b1cb3eddd3289e27ef12345)
                mstore(0x160, 0x108420b9b3fc57b5519f7c0ebfbedbefee5edd15d7f7d041c3f1b70ea84f7e98)
                ret := and(success, staticcall(gas(), 0x08, 0x00, 0x180, 0x00, 0x20))
                ret := and(ret, mload(0x00))
            }

            // Modulus
            let q := 21888242871839275222246405745257275088696311157297823662689037894645226208583 // BN254 base field
            let r := 21888242871839275222246405745257275088548364400416034343698204186575808495617 // BN254 scalar field

            // Initialize success as true
            let success := true

            // Copy variables from the verifying key until G2 Generator point
            // `NEG_S_G2` point is already calculated and placed on the memory.
            extcodecopy(vk, VK_MPTR, 0x00, 0x0220)

            // The proof length should be divisible by `0x80` bytes, equivalent to four words.
            // The proof is structured as follows: 
            // 2W * n: Commitment points in the SNARK proof.
            // 2W * n: Points in the opening proof.
            // 1W    : Length of evaluation values. 
            // 1W * n: Evaluation values.
            // where W is referred to as a Word, which is 32 bytes.
            // and `n` denotes the number of commitments as well as the number of evaluation values.
            let proof_length := calldataload(PROOF_LEN_CPTR)

            // Ensure the proof length is divisible by `0x80`, accommodating the structured data layout.
            success := and(success, eq(0, mod(proof_length, 0x80)))
            if iszero(success) {
                mstore(0, "Invalid proof length")
                revert(0, 0x20)
            }

            // Load the length of evaluation values, positioned after the proof data.
            let evaluation_values_length_pos := add(add(PROOF_LEN_CPTR, proof_length), 0x20)
            let evaluation_values_length := calldataload(evaluation_values_length_pos)
            
            // The proof length should match 4 times the length of the evaluation values.
            success := and(success, eq(4, div(proof_length, mul(evaluation_values_length, 0x20))))
            if iszero(success) {
                mstore(0, "Number of evaluation mismatch")
                revert(0, 0x20)
            }

            for { let i := 0 } lt(i, evaluation_values_length) { i := add(i, 1) } {
                let shift_pos := mul(i, 0x20)
                let double_shift_pos := mul(i, 0x40) // for next point

                let value := calldataload(add(evaluation_values_length_pos, add(shift_pos, 0x20)))
                let minus_z := sub(r, value)
                // // MINUS_Z is correct
                // mstore(0x00, minus_z)
                // revert(0x0, 0x40)

                // Assign values on memory for multiplication
                mstore(0x80, mload(G1_X_MPTR))
                mstore(0xa0, mload(G1_Y_MPTR))
                success := and(success, ec_mul_tmp(success, minus_z))
                if iszero(success) {
                    mstore(0, "Failed to multiply G1 by minus_z")
                    revert(0, 0x20)
                }

                // Performaing like `c_g_to_minus_z = c + g_to_minus_z` in `verify_kzg_proof` function that is located in `amortized_kzg.rs`. 
                // 
                // `c` is equivalent to `commitment` as input on the `open_grand_sums` function.
                // the values of 'g_to_minus_z` is already located at 0x80 and 0xa0 in the previous step 
                let commitment_proof_pos := add(add(PROOF_CPTR, div(proof_length, 2)), double_shift_pos)
                success := check_ec_point(success, commitment_proof_pos, q)
                if iszero(success) {
                    mstore(0, "Commitment point is not EC point")
                    mstore(0x20, shift_pos)
                    mstore(0x40, commitment_proof_pos)
                    revert(0, 0x60)
                }

                // TODO: check it this is correct
                let lhs_x := calldataload(commitment_proof_pos)            // C_X
                let lhs_y := calldataload(add(commitment_proof_pos, 0x20)) // C_Y
                success := ec_add_tmp(success, lhs_x, lhs_y)
                if iszero(success) {
                    mstore(0, "Failed to add C and g_to_minus_z")
                    revert(0, 0x20)
                }

                // // This is also correct
                // // This is `c_g_to_minus_z` in the `verify_kzg_proof` function.
                // mstore(0, mload(0x80))
                // mstore(0x20, mload(0xa0))
                // revert(0, 0x40)
                
                mstore(LHS_X_MPTR, mload(0x80))
                mstore(LHS_Y_MPTR, mload(0xa0))

                // Checking from calldata
                let proof_pos := add(PROOF_CPTR, shift_pos)
                success := check_ec_point(success, PROOF_CPTR, q)
                if iszero(success) {
                    mstore(0, "Opening point is not EC point")
                    revert(0, 0x20)
                }
                let rhs_x := calldataload(proof_pos) // PI_X
                let rhs_y := calldataload(add(proof_pos, 0x20)) // PI_Y

                success := and(success, ec_pairing(success, mload(LHS_X_MPTR), mload(LHS_Y_MPTR), rhs_x, rhs_y))
                if iszero(success) {
                    mstore(0, success)
                    revert(0, 0x220)
                }
            }

            // Return 1 as result if everything succeeds
            mstore(0x00, 1)
            return(0x00, 0x20)
        }
    }

}
