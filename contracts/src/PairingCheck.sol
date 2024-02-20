pragma solidity ^0.8.0;


contract PairingCheck {
    // Do call function

    function testPairingWithInputs(uint256 lhs_x, uint256 lhs_y, uint256 rhs_x, uint256 rhs_y, uint256 neg_g2_x1, uint256 neg_g2_x2, uint256 neg_g2_y1, uint256 neg_g2_y2) public view returns (bool) {
      uint256 ret;
      assembly {
        // For result of pairing check starting input from 0x20
        mstore(0x20, lhs_x)
        mstore(0x40, lhs_y)
        mstore(0x60, 0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // G2_X_1
        mstore(0x80, 0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // G2_X_2
        mstore(0xa0, 0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // G2_Y_1
        mstore(0xc0, 0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // G2_Y_2
        mstore(0xe0, rhs_x)
        mstore(0x100, rhs_y)
        mstore(0x120, neg_g2_x1)
        mstore(0x140, neg_g2_x2)
        mstore(0x160, neg_g2_y1)
        mstore(0x180, neg_g2_y2)
        ret := staticcall(gas(), 0x08, 0x20, 0x180, 0x00, 0x20)
        return(0, 0x20)
        // revert(0, 0x40)
      }
    }
    
    function testPairingWithInputsBasedVerifyingKey(uint256 lhs_x, uint256 lhs_y, uint256 rhs_x, uint256 rhs_y) public view returns (bool) {
      uint256 ret;
      assembly {
        // For result of pairing check starting input from 0x20
        mstore(0x20,  lhs_x)
        mstore(0x40,  lhs_y)
        mstore(0x60,  0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // G2_X_1
        mstore(0x80,  0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // G2_X_2
        mstore(0xa0,  0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // G2_Y_1
        mstore(0xc0,  0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // G2_Y_2
        mstore(0xe0,  rhs_x)
        mstore(0x100, rhs_y)
        // Thses points comes from verifying key
        mstore(0x120, 0x26186a2d65ee4d2f9c9a5b91f86597d35f192cd120caf7e935d8443d1938e23d) // NEG_G2_X_1
        mstore(0x140, 0x30441fd1b5d3370482c42152a8899027716989a6996c2535bc9f7fee8aaef79e) // NEG_G2_X_2
        mstore(0x160, 0x16f363f103c80d7bbc8ad3c6867e0822bbc6000be91a4689755c7df40221c145) // NEG_G2_Y_1
        mstore(0x180, 0x2b1cbb3e521edf5a622d82762a44a5e63f1e50b332d71154a4a7958d6011deff) // NEG_G2_Y_2
        ret := staticcall(gas(), 0x08, 0x20, 0x180, 0x00, 0x20)
        return(0, 0x20)
        // revert(0, 0x40)
      }
    }
    
    function testPairingGrandsum() public view returns (bool) {
      uint256 ret;
      assembly {
        // This data is from the second column of total balances in the grand sum
        mstore(0x20,  0x285eb4eb14df106c55a25b5fc3e1bcd814dd19f72258148a056bc85a19e3e4ea)
        mstore(0x40,  0x224c5e14bf3231c7c720da97d80bc067540c93ad0d401cb3951dd27d1759e9ec)
        mstore(0x60,  0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // G2_X_1
        mstore(0x80,  0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // G2_X_2
        mstore(0xa0,  0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // G2_Y_1
        mstore(0xc0,  0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // G2_Y_2
        mstore(0xe0,  0x2f33a8fd3f5f2b7175e613a865da34001aa4e92ec6f746ab2d58506987a8885f)
        mstore(0x100, 0x277e3056c790cfbe4d40fddf9954da9eaf0694270a7b4987c6cd9148623df72b)
        mstore(0x120, 0x26186a2d65ee4d2f9c9a5b91f86597d35f192cd120caf7e935d8443d1938e23d) // NEG_G2_X_1
        mstore(0x140, 0x30441fd1b5d3370482c42152a8899027716989a6996c2535bc9f7fee8aaef79e) // NEG_G2_X_2
        // this points from while generating the proof
        mstore(0x160, 0x1970ea81dd6992adfbc571effb03503adbbb6a857f578403c6c40e22d65b3c02)
        mstore(0x180, 0x054793348f12c0cf5622c340573cb277586319de359ab9389778f689786b1e48)
        // mstore(0x160, 0x16f363f103c80d7bbc8ad3c6867e0822bbc6000be91a4689755c7df40221c145) // NEG_G2_Y_1
        // mstore(0x180, 0x2b1cbb3e521edf5a622d82762a44a5e63f1e50b332d71154a4a7958d6011deff) // NEG_G2_Y_2
        ret := staticcall(gas(), 0x08, 0x20, 0x180, 0x00, 0x20)
        return(0, 0x20)
      }
    }

    function testPairingUsername() public view returns (bytes memory) {
      uint256 ret;
      assembly {
        // This data is from the first column, which is username polynomial in the inclusion proof.
        mstore(0x20,  0x27e05775da20e129777c1eec20cdae6aedeb7751ebea38b188bef9e1949214e7)
        mstore(0x40,  0x19f372e6b01d22a59829f23e0bf222e53d28833ff5771a3e4026dd43e61e6e54)
        mstore(0x60,  0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // G2_X_1
        mstore(0x80,  0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // G2_X_2
        mstore(0xa0,  0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // G2_Y_1
        mstore(0xc0,  0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // G2_Y_2
        mstore(0xe0,  0x0dc23eaf438eb057ab1787895b6e559956f14a24de408c717af7f8e11f73a19f)
        mstore(0x100, 0x29bd5e658897d9baae1e7419c26557c9a08474fd0bb612ac9a34fb207cf952fa)
        mstore(0x120, 0x0f79a0045992596e3278606b5317aaf4f6bb65071219b1c89d542509fe6dddd3) // NEG_G2_X_1
        mstore(0x140, 0x2299faaf0e21893e99005dc9165fba869b5aa88bcac5af4395071fd569686fde) // NEG_G2_X_2
        // these points  from verifying key
        mstore(0x160, 0x16f363f103c80d7bbc8ad3c6867e0822bbc6000be91a4689755c7df40221c145) // NEG_G2_Y_1
        mstore(0x180, 0x2b1cbb3e521edf5a622d82762a44a5e63f1e50b332d71154a4a7958d6011deff) // NEG_G2_Y_2
        
        // these points comes from while generating the proof
        // mstore(0x160, 0x0be3817a3dae5325fd73daaa4c0a1ae80e7968fb0f732be97c9aaff6b6252d6d) // NEG_G2_Y_1
        // mstore(0x180, 0x048ceab5d8b15475806a2040918bd414214e86be6527a99a93b7db036a3221d9) // NEG_G2_Y_2
        ret := staticcall(gas(), 0x08, 0x20, 0x180, 0x00, 0x20)
        revert(0, 0x20)
      }
    }

    function testPairingInclusion() public view returns (bytes memory) {
        assembly {
          function ec_pairing(success, lhs_x, lhs_y, rhs_x, rhs_y) -> ret {
                mstore(0x00, lhs_x)
                mstore(0x20, lhs_y)
                mstore(0x40,  0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2) // G2_X_1
                mstore(0x60,  0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed) // G2_X_2
                mstore(0x80,  0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b) // G2_Y_1
                mstore(0xa0,  0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa) // G2_Y_2
                mstore(0xc0, rhs_x)
                mstore(0xe0, rhs_y)
                mstore(0x100, 0x233ce63d80e3ea8a781291bd72ebca76de04e21f2b08ca15386d82c91245c50a) // NEG_G2_X_1
                mstore(0x120, 0x22992c3f314d6eae9ce7b3289df7b62b37bf7c166689c431d164af9243cb21f4) // NEG_G2_X_2
                mstore(0x140, 0x19f2f14868cd0e5889e0cd71f0e39e79e019d1222b1cb3eddd3289e27ef12345) // NEG_G2_Y_1
                mstore(0x160, 0x108420b9b3fc57b5519f7c0ebfbedbefee5edd15d7f7d041c3f1b70ea84f7e98) // NEG_G2_Y_2
                ret := and(success, staticcall(gas(), 0x08, 0x00, 0x180, 0x00, 0x20))
                ret := and(ret, mload(0x00))
            }

          let success := true
          
          success := ec_pairing(
                success,
                0x1a05446eddbfe8094fbbd87fe761546f4709a69973a9454ccbd0a4cef60f2ad,
                0x3002c80e5981ea653bd0d054faac17d628f82ae69c26828ddff5838685ab2670,
                0x2d536e6e7be13957f2eb82d0ced26c44b76e3dee0b896af61f57575935a34687,
                0x1fb31cc9f50e9d5084bb0d2856c98aa13718b511415ecad0e00bbf66c8f6d900
          )


          if iszero(success) {
            mstore(0, 0xfff)
            revert(0, 0x20)
          } 
          
          success := ec_pairing(
                success,
                0x16c75d0c6fda53ee4d4ae3db974ccbceab90243e1424bb2291ad06dff2e06546,
                0x19ed72f6dcc1b33b4097aaccdf16fdd9e30c34498c659480e65810d1a6e460a1,
                0x19607eef17080b10765d361f4270861ce82b5dc6c258048d3b843f81be897c35,
                0x0ea61094c716cdd97300517fa9e0b84b9730f18c72cb6f14057b3ee339b9ff0d
          )
          

          if iszero(success) {
            mstore(0, 0xf)
            revert(0, 0x20)
          }
        }
    }
}
