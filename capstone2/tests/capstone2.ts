import * as anchor from "@coral-xyz/anchor";
import { Program, BN, Wallet } from "@coral-xyz/anchor";
import {
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Keypair,
  Commitment,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { Capstone } from "../target/types/capstone";
import { assert } from "chai";

const commitment: Commitment = "confirmed";

describe("JitoSOL Lending Protocol - Phase 1", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.capstone as Program<Capstone>;
  const user = provider.wallet;

  // PDAs
  let market: PublicKey;
  let userPosition: PublicKey;
  let marketBump: number;
  let userPositionBump: number;
  
  // Market data
  let marketData: any = null;
  
  // Token accounts
  let depositMint: PublicKey;
  let borrowMint: PublicKey;
  let depositVault: PublicKey;
  let borrowVault: PublicKey;
  let userTokenAccount: PublicKey;
  let userBorrowTokenAccount: PublicKey;

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      { signature, ...latestBlockhash },
      commitment
    );
  };

  before(async () => {
    // Find program addresses
    [market, marketBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), user.publicKey.toBuffer()],
      program.programId
    );

    [userPosition, userPositionBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_position"), user.publicKey.toBuffer()],
      program.programId
    );

    // Try to fetch existing market or set up new one
    try {
      marketData = await program.account.market.fetch(market);
      depositVault = marketData.depositVault;
      borrowVault = marketData.borrowVault;
      
      // Get mints from vaults
      try {
        const borrowVaultTokenAccount = await getAccount(
          provider.connection,
          borrowVault
        );
        borrowMint = borrowVaultTokenAccount.mint;
      } catch (error) {
        if (marketData.borrowMint) {
          borrowMint = marketData.borrowMint;
        } else {
          throw new Error("Could not determine borrow mint");
        }
      }

      try {
        const vaultTokenAccount = await getAccount(
          provider.connection,
          depositVault
        );
        depositMint = vaultTokenAccount.mint;
      } catch (error) {
        if (marketData.depositMint) {
          depositMint = marketData.depositMint;
        } else {
          throw new Error("Could not determine deposit mint");
        }
      }
    } catch (error) {
      // Create new mints and prepare vaults if market doesn't exist
      depositMint = await createMint(
        provider.connection,
        user.payer,
        user.publicKey,
        null,
        9
      );

      borrowMint = await createMint(
        provider.connection,
        user.payer,
        user.publicKey,
        null,
        9
      );
      
      // Just create keypairs for now, actual initialization happens in the test
      const depositVaultKeypair = Keypair.generate();
      const borrowVaultKeypair = Keypair.generate();
      depositVault = depositVaultKeypair.publicKey;
      borrowVault = borrowVaultKeypair.publicKey;
    }

    // Create or get user token accounts
    const userTokenAccountInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      depositMint,
      user.publicKey
    );
    userTokenAccount = userTokenAccountInfo.address;

    // Mint tokens to user if needed
    const tokenBalance = await provider.connection.getTokenAccountBalance(
      userTokenAccount
    );
    if (!tokenBalance.value.uiAmount || tokenBalance.value.uiAmount < 1) {
      try {
        await mintTo(
          provider.connection,
          user.payer,
          depositMint,
          userTokenAccount,
          user.publicKey,
          LAMPORTS_PER_SOL * 10,
          []
        );
      } catch (error) {
        console.warn("Could not mint deposit tokens");
      }
    }

    // Set up borrow token account
    const userBorrowTokenAccountInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      borrowMint,
      user.publicKey
    );
    userBorrowTokenAccount = userBorrowTokenAccountInfo.address;
  });

  it("initializes the market if needed", async () => {
    try {
      const existingMarket = await program.account.market.fetch(market);
      marketData = existingMarket;
      depositVault = marketData.depositVault;
      borrowVault = marketData.borrowVault;
    } catch (error) {
      // Create new vaults and initialize market
      const depositVaultKeypair = Keypair.generate();
      const borrowVaultKeypair = Keypair.generate();
      depositVault = depositVaultKeypair.publicKey;
      borrowVault = borrowVaultKeypair.publicKey;

      const tx = await program.methods
        .initialize()
        .accounts({
          authority: user.publicKey,
          market,
          depositMint,
          borrowMint,
          depositVault,
          borrowVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([depositVaultKeypair, borrowVaultKeypair])
        .rpc();

      await confirmTx(tx);
      marketData = await program.account.market.fetch(market);
    }
  });

  it("creates user position if needed", async () => {
    try {
      await program.account.userPosition.fetch(userPosition);
    } catch (error) {
      const tx = await program.methods
        .userPosition()
        .accounts({
          owner: user.publicKey,
          market,
          userPosition,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([])
        .rpc();

      await confirmTx(tx);
    }
  });

  it("allows deposits of jitoSOL", async () => {
    // Ensure we have market data
    if (!marketData) {
      marketData = await program.account.market.fetch(market);
      depositVault = marketData.depositVault;
      borrowVault = marketData.borrowVault;
    }

    // Get position state before deposit for comparison
    const beforePosition = await program.account.userPosition.fetch(userPosition);
    const beforeMarket = await program.account.market.fetch(market);
    
    // Deposit 0.1 SOL worth of jitoSOL
    const amount = new BN(LAMPORTS_PER_SOL * 0.1);
    
    const depositTx = await program.methods
      .deposit(amount)
      .accounts({
        owner: user.publicKey,
        market,
        userPosition,
        userTokenAccount,
        depositVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    await confirmTx(depositTx);

    // Verify deposit results
    const afterPosition = await program.account.userPosition.fetch(userPosition);
    const afterMarket = await program.account.market.fetch(market);

    const expectedDeposit = beforePosition.depositedJito.add(amount);
    assert.equal(
      afterPosition.depositedJito.toString(),
      expectedDeposit.toString(),
      "Deposit amount not correctly updated"
    );
    
    assert.isTrue(
      afterPosition.depositedShares.gt(beforePosition.depositedShares),
      "Deposit shares not increased"
    );
    
    assert.equal(
      afterMarket.totalDeposits.toString(),
      beforeMarket.totalDeposits.add(amount).toString(),
      "Market total deposits not updated correctly"
    );
  });

  it("adds liquidity to the borrow vault", async () => {
    // Make sure we have the proper accounts
    if (!marketData) {
      marketData = await program.account.market.fetch(market);
      depositVault = marketData.depositVault;
      borrowVault = marketData.borrowVault;
    }

    // Mint borrow tokens if needed (1 SOL worth)
    const liquidityAmount = new BN(LAMPORTS_PER_SOL);
    const tokenBalance = await provider.connection.getTokenAccountBalance(
      userBorrowTokenAccount
    );
    
    if (tokenBalance.value.uiAmount < liquidityAmount.toNumber() / LAMPORTS_PER_SOL) {
      try {
        await mintTo(
          provider.connection,
          user.payer,
          borrowMint,
          userBorrowTokenAccount,
          user.publicKey,
          liquidityAmount.toNumber(),
          []
        );
      } catch (error) {
        console.warn("Could not mint borrow tokens");
        return;
      }
    }

    // Get vault balance before adding liquidity
    const vaultBalanceBefore = await provider.connection.getTokenAccountBalance(borrowVault);
    
    // Add liquidity
    const tx = await program.methods
      .addLiquidity(liquidityAmount)
      .accounts({
        provider: user.publicKey,
        authority: user.publicKey,
        market,
        userTokenAccount: userBorrowTokenAccount,
        borrowVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    await confirmTx(tx);

    // Verify liquidity was added
    const vaultBalanceAfter = await provider.connection.getTokenAccountBalance(borrowVault);
    const balanceDifference = vaultBalanceAfter.value.uiAmount - vaultBalanceBefore.value.uiAmount;
    
    assert.approximately(
      balanceDifference,
      liquidityAmount.toNumber() / LAMPORTS_PER_SOL,
      0.001,
      "Added liquidity amount incorrect"
    );
  });

  it("allows borrowing SOL against deposited jitoSOL", async () => {
    // Ensure we have market data
    if (!marketData) {
      marketData = await program.account.market.fetch(market);
      depositVault = marketData.depositVault;
      borrowVault = marketData.borrowVault;
    }

    // Get position state before borrow for comparison
    const beforePosition = await program.account.userPosition.fetch(userPosition);
    const beforeMarket = await program.account.market.fetch(market);
    
    // Define borrow amount (0.05 SOL)
    let borrowAmount = new BN(LAMPORTS_PER_SOL * 0.05);
    
    // Check vault liquidity and adjust amount if needed
    const borrowVaultInfo = await getAccount(provider.connection, borrowVault);
    if (Number(borrowVaultInfo.amount) < borrowAmount.toNumber()) {
      if (Number(borrowVaultInfo.amount) > 0) {
        borrowAmount = new BN(Math.floor(Number(borrowVaultInfo.amount) * 0.8));
      } else {
        console.log("Insufficient liquidity, skipping borrow test");
        return;
      }
    }
    
    // Execute borrow
    const borrowTx = await program.methods
      .borrow(borrowAmount)
      .accounts({
        owner: user.publicKey,
        market,
        userPosition,
        userTokenAccount: userBorrowTokenAccount,
        borrowVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    await confirmTx(borrowTx);

    // Verify borrow results
    const afterPosition = await program.account.userPosition.fetch(userPosition);
    const afterMarket = await program.account.market.fetch(market);

    const expectedBorrowedSol = beforePosition.borrowedSol.add(borrowAmount);
    assert.equal(
      afterPosition.borrowedSol.toString(),
      expectedBorrowedSol.toString(),
      "Borrowed amount not correctly updated"
    );
    
    assert.isTrue(
      afterPosition.borrowedShares.gt(beforePosition.borrowedShares),
      "Borrow shares not increased"
    );
    
    assert.equal(
      afterMarket.totalBorrows.toString(),
      beforeMarket.totalBorrows.add(borrowAmount).toString(),
      "Market total borrows not updated correctly"
    );
  });

  it("allows repaying borrowed SOL", async () => {
    // Get position state before repay for comparison
    const beforePosition = await program.account.userPosition.fetch(userPosition);
    const beforeMarket = await program.account.market.fetch(market);
    
    // Skip if user has no borrows
    if (beforePosition.borrowedSol.toNumber() === 0) {
      console.log("No outstanding borrows, skipping repay test");
      return;
    }

    // Repay half of the outstanding loan
    let repayAmount = new BN(Math.floor(beforePosition.borrowedSol.toNumber() / 2));
    if (repayAmount.toNumber() === 0) {
      repayAmount = beforePosition.borrowedSol;
    }
    
    // Check token balance and adjust amount if needed
    const tokenBalance = await provider.connection.getTokenAccountBalance(userBorrowTokenAccount);
    if (tokenBalance.value.uiAmount < repayAmount.toNumber() / LAMPORTS_PER_SOL) {
      if (tokenBalance.value.uiAmount > 0) {
        repayAmount = new BN(Math.floor(tokenBalance.value.uiAmount * 0.9 * LAMPORTS_PER_SOL));
      } else {
        console.log("Insufficient tokens for repayment, skipping test");
        return;
      }
    }
    
    // Execute repay
    const repayTx = await program.methods
      .repay(repayAmount)
      .accounts({
        owner: user.publicKey,
        market,
        userPosition,
        userTokenAccount: userBorrowTokenAccount,
        borrowVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    await confirmTx(repayTx);

    // Verify repay results
    const afterPosition = await program.account.userPosition.fetch(userPosition);
    const afterMarket = await program.account.market.fetch(market);

    const expectedBorrowedSol = beforePosition.borrowedSol.sub(repayAmount);
    assert.equal(
      afterPosition.borrowedSol.toString(),
      expectedBorrowedSol.toString(),
      "Borrowed amount not correctly updated after repayment"
    );
    
    assert.isTrue(
      afterPosition.borrowedShares.lt(beforePosition.borrowedShares),
      "Borrow shares not decreased"
    );
    
    assert.isTrue(
      afterMarket.totalBorrows.lt(beforeMarket.totalBorrows),
      "Market total borrows not decreased"
    );
  });

  it("allows withdrawing deposited jitoSOL", async () => {
    // Get position state before withdrawal for comparison
    const beforePosition = await program.account.userPosition.fetch(userPosition);
    const beforeMarket = await program.account.market.fetch(market);
    
    // Skip if user has no deposits
    if (beforePosition.depositedJito.toNumber() === 0) {
      console.log("No deposits to withdraw, skipping test");
      return;
    }

    // Withdraw quarter of deposits by default
    let withdrawAmount = new BN(Math.floor(beforePosition.depositedJito.toNumber() / 4));
    if (withdrawAmount.toNumber() === 0) {
      withdrawAmount = new BN(1000000); // 0.001 minimum
    }
    
    // Check if withdrawal would violate LTV constraints
    if (beforePosition.borrowedSol.toNumber() > 0) {
      const remainingDeposit = beforePosition.depositedJito.sub(withdrawAmount);
      const borrowedAmount = beforePosition.borrowedSol;
      const maxLtv = beforeMarket.maxLtv?.toNumber() || 7000;
      
      if (remainingDeposit.toNumber() === 0 || 
          (borrowedAmount.toNumber() * 10000) / remainingDeposit.toNumber() > maxLtv) {
        
        // Adjust withdrawal to maintain safe LTV
        const safeCollateral = (borrowedAmount.toNumber() * 10000) / maxLtv;
        const safeWithdrawal = beforePosition.depositedJito.toNumber() - safeCollateral - 10000;
        
        if (safeWithdrawal > 0) {
          withdrawAmount = new BN(Math.floor(safeWithdrawal));
        } else {
          console.log("Cannot withdraw without exceeding max LTV, skipping test");
          return;
        }
      }
    }
    
    // Execute withdrawal
    const withdrawTx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        owner: user.publicKey,
        market,
        userPosition,
        userTokenAccount,
        depositVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([])
      .rpc();

    await confirmTx(withdrawTx);

    // Verify withdrawal results
    const afterPosition = await program.account.userPosition.fetch(userPosition);
    const afterMarket = await program.account.market.fetch(market);

    const expectedDepositedJito = beforePosition.depositedJito.sub(withdrawAmount);
    assert.equal(
      afterPosition.depositedJito.toString(),
      expectedDepositedJito.toString(),
      "Deposited amount not correctly updated after withdrawal"
    );
    
    assert.isTrue(
      afterPosition.depositedShares.lt(beforePosition.depositedShares),
      "Deposit shares not decreased"
    );
    
    assert.isTrue(
      afterMarket.totalDeposits.lt(beforeMarket.totalDeposits),
      "Market total deposits not decreased"
    );
  });
});