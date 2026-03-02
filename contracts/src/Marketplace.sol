// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./Verifier.sol";

/// @title zkVM Proof Marketplace
/// @notice A decentralized marketplace for purchasing verifiable STARK computations
contract ZkMarketplace {
    Verifier public zkVmVerifier;
    
    struct Bounty {
        address requester;
        bytes32 programHash;
        uint256 reward;
        bool completed;
        address prover;
    }
    
    mapping(uint256 => Bounty) public bounties;
    uint256 public nextBountyId;
    
    event BountyCreated(uint256 indexed bountyId, bytes32 indexed programHash, uint256 reward);
    event ProofSubmitted(uint256 indexed bountyId, address indexed prover);
    
    constructor(address _verifier) {
        zkVmVerifier = Verifier(_verifier);
    }
    
    /// @notice Request an external prover network to compute a zkVM trace
    function createBounty(bytes32 _programHash) external payable {
        require(msg.value > 0, "Bounty reward must be > 0");
        
        uint256 id = nextBountyId++;
        bounties[id] = Bounty({
            requester: msg.sender,
            programHash: _programHash,
            reward: msg.value,
            completed: false,
            prover: address(0)
        });
        
        emit BountyCreated(id, _programHash, msg.value);
    }
    
    /// @notice Submit a STARK proof solving the requested bounty
    function submitProof(uint256 _bountyId, bytes memory _proofData) external {
        Bounty storage bounty = bounties[_bountyId];
        require(!bounty.completed, "Bounty already fulfilled");
        
        // Ensure STARK proof is valid
        bool isValid = zkVmVerifier.verifyProof(bounty.programHash, _proofData);
        require(isValid, "Invalid zkVM Proof");
        
        bounty.completed = true;
        bounty.prover = msg.sender;
        
        // Payout the miner/prover
        payable(msg.sender).transfer(bounty.reward);
        
        emit ProofSubmitted(_bountyId, msg.sender);
    }
}
