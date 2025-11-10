import React from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Program, AnchorProvider, web3 } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

export const StakingPanel: React.FC = () => {
  const { connection } = useConnection();
  const wallet = useWallet();
  const [stakedAmount, setStakedAmount] = React.useState<number>(0);

  const stakeTokens = async (amount: number) => {
    if (!wallet.publicKey) return;
    
    const provider = new AnchorProvider(connection, wallet as any, {});
    const programId = new PublicKey("snowDApp111111111111111111111111111");
    
    // Stake transaction logic
    console.log(`Staking ${amount} SNOW tokens`);
  };

  return (
    <div className="staking-panel">
      <h2>SNOW Staking</h2>
      <div className="staked-amount">
        Staked: {stakedAmount} SNOW
      </div>
      <button onClick={() => stakeTokens(1000)}>
        Stake 1000 SNOW
      </button>
    </div>
  );
};
