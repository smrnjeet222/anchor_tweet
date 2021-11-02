use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("ArCw5w96LA8PDPbWUswKNPhqvAtLE8JHQLXAvjtWAkXy");

#[program]
pub mod anchor_tweet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let tweet_account = &mut ctx.accounts.tweet_account;
        tweet_account.authority = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>, new_post: Vec<u8>) -> ProgramResult {
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8, from bytes {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;

        msg!("Post (len: {}): {:?}", post.len(), post);

        let tweet_account = &mut ctx.accounts.tweet_account;
        tweet_account.latest_post = new_post;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8   //account discriminator
                + 32 //pubkey
                + 566 //post size
        )]
    pub tweet_account: Account<'info, TweetAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut, has_one = authority)]
    pub tweet_account: Account<'info, TweetAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct TweetAccount {
    pub latest_post: Vec<u8>,
    pub authority: Pubkey,
}
