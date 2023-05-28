// use :- Its just like 'import' in javascript.


use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self,Token};                                   // SPL :- Solana Program Library.
use std::mem::size_of;

// This is your program's public key and it will update
// automatically when you build the project.

declare_id!("CNdiV6oXSAcJYGJrzJay7ftFay3nNfVnGeAnskccUaA2");          // This is the address where Smart Contract lives on Blockchain. 

const TEXT_LENGTH :usize=255;
const MUSIC_URL_LENGTH:usize = 255;


/*                   Smart Contract
    #[program] :- This tag is used for writing smart contract.
    mod :- This is the keyword used for writing smart contract.
*/

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

    // Function 2 : For Creating Music. (Upolad Music Functionality.)    
    pub fn create_music( ctx: Context<CreateMusic>, title: String, music_url: String,) -> ProgramResult          // Here "CreateMusic" is used as a Context.
    {
        let music = &mut ctx.accounts.music;

        music.authority = ctx.accounts.authority.key();
        music.title =  title;
        music.music_url = music_url;

        Ok(())
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
    pub receiver: AccountInfo<'info>,                   // The one who recieves the money.

    #[account(mut)]
    pub authority: Signer<'info>,                       // 'Authority' is the 'Signer' who paid transaction fee.
    

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

// CreateMusic Context

#[derive(Accounts)]
pub struct CreateMusic<'info>{
    #[account(
        init,
        seeds = [b"music".as_ref(),randomkey.key().as_ref()],
        bump,
        payer=authority,
        space = size_of::<MusicAccount>() + TEXT_LENGTH + MUSIC_URL_LENGTH + 8
    )]


    pub music : Account<'info , MusicAccount>,

    #[account(mut)]
    pub randomkey : AccountInfo<'info>,

    //Authority (this is the signer who paid transaction fee)\
    #[account(mut)]
    pub authority: Signer<'info>,


    pub system_program: UncheckedAccount<'info>,

    //Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info , Token>,
        
    // Clock to save time
    pub clock : Sysvar<'info , Clock>,

    
}


// Entity 2 :- Music
#[account]
pub struct MusicAccount{         // Our Music has 3 properties  
    
    pub authority: Pubkey,       // 1. Authority :- The one who uploads the music. (i.e our user)

    pub title: String,           // 2. Title :- The title of our music 
    
    pub music_url: String,       // 3. URL   :- This is the link where actual audio file is stored. (In our case it is IPFS)
}