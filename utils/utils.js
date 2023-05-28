import * as anchor from '@project-serum/anchor'
import { WalletNotConnectedError } from '@solana/wallet-adapter-base'
import { STABLE_POOL_IDL, STABLE_POOL_PROGRAM_ID } from './const'                 // Address & ABI

export function getProgramInstance(connection, wallet) {
    
    if(!wallet.publicKey) throw new WalletNotConnectedError;                      // If wallet does not have public key, it means wallet 

    const provider = new anchor.Provider(                                         // Provider (Connection + Wallet)
        connection, 
        wallet,
        anchor.Provider.defaultOptions()
    ) 

    const idl = STABLE_POOL_IDL;                           // ABI

    const programId = STABLE_POOL_PROGRAM_ID;              // Address


    // Here we are creating instance of our Smart Contract.
    const program = new anchor.Program(idl,programId,provider);

    return program;
}