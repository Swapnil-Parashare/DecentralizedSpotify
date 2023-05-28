// use :- Its just like 'import' in javascript.


use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self,Token};                                   // SPL :- Solana Program Library.
use std::mem::size_of;

// This is your program's public key and it will update
// automatically when you build the project.

declare_id!("FmxYHCu54HP2fVBr86gCUPX1Y1AyThXLrY5Q2f1kj7yi");                       // This is the address where Smart Contract lives on Blockchain. 


#[program]                                                       
mod decentralized_audio {                                              // This is the name of our smart contract
    
    
    
    use super::*;


    /*
        Below are the 2 important functions of our smart conctract.
        For each function we are providing "Context" as parametre.
        This "Context" is nothing but like a Class whose objects are created later.
    */
    

    // Function 1 : For Accepting Payment. (Subscription Fees.)
    pub fn accept_payment(ctx: Context<PayerContext>) -> ProgramResult                                        // Here 'PayerContext' is used as Context. 
    {                                                                                                         // Syntax for parameter ==> parametereName : parametreDataType
                                                                                                              // 'ctx' is the parametre & 'Context<PayerContext>' is the data type.        
        let payer_wallet = &mut ctx.accounts.payer_wallet;
        payer_wallet.wallet = ctx.accounts.authority.key();

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.receiver.key(),
            100000000,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        )
    }
}
/*
    pub struct ---> It is used for creating "Context".
    Context is just like a class.
    
    We have created 4 Contexts  & each Context have different tags :-
    
        Context                            Tag
        
    1. PayerContext                   #[derive(Accounts)]   <--------------|
    2. PayerAccount                   #[account]            ---------------|  It is used in above context
    
    3. CreateMusic                    #[derive(Accounts)]   <--------------|
    4. MusicAccount                   #[account]            ---------------|  It is used in above context
    
    So #[account] is used in #[derive(Accounts)]
*/


#[derive(Accounts)]
pub struct PayerContext<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)  
    
    #[account(
        init, 
        seeds = [b"payer".as_ref() , authority.key().as_ref()],                   // It creates a Unique hash based on what you put into the array.
        bump,                                                                     // If unique hash created by 'seeds' already exists then 'bump' will increment it to make sure it is 100% unique.
        payer = authority, 
        space = size_of::<PayerAccount>() + 8 
    )]
    
    
    pub payer_wallet: Account<'info, PayerAccount>,
    
    #[account(mut)]
    pub receiver: AccountInfo<'info>,                 // The one who recieves the money.

    #[account(mut)]
    pub authority: Signer<'info>,                      // 'Authority' is the 'Signer' who paid transaction fee.
    

    pub system_program: UncheckedAccount<'info>,

    //Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info , Token>,
        
    // Clock to save time
    pub clock : Sysvar<'info , Clock>,
       
}



// Entity 1 :- User (Payer)
#[account]                              // This tag inherets all the information of the current account on the smart contract.                            
pub struct PayerAccount {
    pub wallet : Pubkey,                // User only has single property i.e  "Public Address"
    
}


/****************************************************************************************************************************

Here on Payment i.e Subscription Fee functionality is developed.
Contract is build & deployed successfully. (As you can see the 'declareId' has changed)

Links to track deployed smart contract.

Solscan         : https://solscan.io/tx/26DN772RkeNNEB31iL2ijgScwGBhsezTERNtZsDPXHDMXFwDiTNmzB52jqLRHjk1TEAotHRxX2FUTeFs9dnhpM1y?cluster=devnet
Solana Explorer : https://explorer.solana.com/tx/26DN772RkeNNEB31iL2ijgScwGBhsezTERNtZsDPXHDMXFwDiTNmzB52jqLRHjk1TEAotHRxX2FUTeFs9dnhpM1y?cluster=devnet




*****************************************************************************************************************************/