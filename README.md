# Project Overview: My Awesome Journey Through the Q2 Turbine Builder Cohort! 

This project is a showcase of my learning journey through the Solana Turbine Builder Cohort! It highlights the skills and knowledge I've picked up along the way. It's made up of several submodules, each diving into different parts of blockchain and Solana development. Check out the overview below to see how each submodule connects to the Turbine Builder Cohort curriculum. 

## Submodules

### Solana-Starter @ 5b3956b
This submodule documents my journey through building Web3 applications on Solana! It showcases various implementations from basic wallet interactions to more complex operations. It demonstrates my understanding of:

- Creating wallets and managing keypairs like a pro 
- Sending transactions and processing instructions on Solana
- Working with SOL transfers and airdrops 
- Understanding Solana's account model and program architecture

### NFT-Staking @ f8e2622
This submodule implements a staking program on Solana! It demonstrates my knowledge of:

- Staking mechanisms and rewards distribution
- Secure handling of staked assets
- Implementing time-based logic in smart contracts
- Managing NFT collections and staking pools
- Working with PDAs (Program Derived Addresses) for secure data management

### Marketplace @ baba504
This submodule provides a framework for creating a decentralized marketplace on Solana! It showcases my ability to:

- Handle listing and purchasing of assets
- Implement secure payment processing
- Manage user interactions in a decentralized setting
- Create escrow systems for trustless transactions
- Validate and verify NFT authenticity before purchases

### Vault-Anchor @ b41ca51
This is the Solana Vault Program, built with Anchor! It lets users:

- Start a vault
- Put in funds
- Take out funds
- Close the vault

This proves I can:
- Build secure smart contracts using Anchor
- Implement key vault features on Solana
- Manage token custody safely and efficiently

### Escrow @ 76e0a8c
This submodule is the Solana Escrow Program, using Anchor! It lets users:

- Create and manage escrow transactions
- Make, take, and refund escrows

This shows I can:
- Build secure smart contracts with Anchor
- Implement complex logic on the Solana blockchain
- Use escrow principles in a decentralized world

### amm @ 48088c0
This submodule implements an Automated Market Maker (AMM) on Solana! It demonstrates my understanding of:

- Decentralized exchange mechanisms
- Liquidity pool management
- Token swapping logic
- Constant product formula implementation
- Fee calculation and distribution

## Joule-Capstone-Project @ 33061c8
This submodule is the grand finale! It puts everything together and shows how I can build a specialized lending protocol on Solana! It proves I can:

- Create a lending protocol optimized for redeemable assets like staked SOL derivatives (jitoSOL)
- Implement capital-efficient borrowing with LTVs up to 70% while maintaining safety
- Design a "liquidator of last resort" mechanism that automatically unstakes collateral at 73% LTV
- Create a delayed weighted average interest rate model to prevent manipulation
- Build a protocol that creates two effective tranches:
  - Higher-yield positions for borrowers executing carry trades
  - Lower-yield but highly secure positions for depositors

Joule is different from general lending platforms because it focuses exclusively on redeemable assets that can be converted back to their underlying token. This specialized approach allows for:

1. Higher loan-to-value ratios without compromising safety
2. Elimination of bad debt through automatic unstaking mechanisms 
3. Improved capital efficiency for traders and yield for depositors

For all the juicy details, check out each submodule's repository! This project is proof of how much I've grown and learned about Solana development during the Turbine Builder Cohort.

## Future Development Plans

I'm excited to continue working on Joule by implementing:
- The complete "liquidator of last resort" mechanism
- The finalized delayed weighted average interest rate model
- Support for additional staked derivatives beyond jitoSOL

## Tech Stack

- **Language**: Rust 
- **Framework**: Anchor 
- **Testing**: Mocha & Chai
- **Client**: TypeScript
- **Deployment**: Solana Devnet

## Acknowledgments

Huge thanks to the Solana Foundation, the Turbine program mentors, and my fellow cohort participants for their amazing support throughout this learning journey!
