use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gopulse {
    use super::*;

    pub fn post_v0(ctx: Context<PostContent>, content_link: String, 
                amount: u64, validator_threshold: i64) -> Result<()> {

        let content: &mut Account<Content> = &mut ctx.accounts.content;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        if content_link.chars().count() < 1 {
            return Err(ErrorCode::ContentRequired.into())  
        }

        if validator_threshold % 2 == 0 {
            return Err(ErrorCode::ThresholdEven.into())
        }

        if validator_threshold < 7 {
            return Err(ErrorCode::ThresholdTooSmall.into())
        }

        content.author = *author.key;
        content.timestamp = clock.unix_timestamp;
        content.content_link = content_link;
        content.amount = amount;
        content.validator_threshold = validator_threshold;
        content.total_pool = 0;
        content.long_pool = 0;
        content.short_pool = 0;
        content.validator_count = 0;
        content.validator_threshold_reached = false;
        content.long_win = false;
        content.short_win = false;

        //transfer SOL to vault
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.author.to_account_info(),
                to: ctx.accounts.vault_keypair.clone(),
            });
        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn validate_v0(ctx: Context<ValidateContent>, amount: u64, position: String) -> Result<()> {

        let validate: &mut Account<Validate> = &mut ctx.accounts.validate;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        validate.author = *author.key;
        validate.timestamp = clock.unix_timestamp;
        validate.amount = amount;
        validate.position = position;
        validate.key = ctx.accounts.key.key();

        let content: &mut Account<Content> = &mut ctx.accounts.key;
        content.validator_count += 1;
        validate.count = content.validator_count;
        content.total_pool += amount;

        if validate.position == "long" {
            content.long_pool += amount;
        }

        if validate.position == "short" {
            content.short_pool += amount;
        }

        if content.validator_threshold_reached == true {
            return Err(ErrorCode::ThresholdReached.into())
        }

        if validate.count >= content.validator_threshold {
            content.validator_threshold_reached = true;
            if content.short_pool > content.long_pool {
                content.short_win = true;
            }
            else if content.long_pool > content.short_pool {
                content.long_win = true;
            }
        }
     
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.author.to_account_info(),
                to: ctx.accounts.vault_keypair.clone(),
            });
        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn collect_v0(ctx: Context<Collect>, program: Pubkey) -> Result<()> {

        let validate: &mut Account<Validate> = &mut ctx.accounts.validate;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        let content: &mut Account<Content> = &mut ctx.accounts.key;
        let vault = &mut ctx.accounts.vault_keypair.key;

        if content.long_win == true && validate.position == "long" {
            let percentage = validate.amount / content.long_pool;
            let dispersement = content.short_pool * percentage;
            let final_dispersement = dispersement * 0.9 as u64;          
        }

        **ctx.accounts.vault_keypair.try_borrow_mut_lamports()? -= 7000000;
        **ctx.accounts.author.try_borrow_mut_lamports()? += 7000000;

        // let (vault_pubkey, vault_bump_seed) = Pubkey::find_program_address(
        //     &[b"vault", content.key().as_ref()],
        //     &program
        // );

        // invoke_signed(
        //     &system_instruction::transfer(&vault_pubkey, author.key, 7000000),
        //     &[
        //         ctx.accounts.vault_keypair.to_account_info(),
        //         ctx.accounts.author.to_account_info(),
        //         ctx.accounts.system_program.to_account_info()
        //     ],
        //     &[&[
        //         b"vault".as_ref(),
        //         content.key().as_ref(),
        //         &[vault_bump_seed],
        //     ]],
        // );

        Ok(())
    }

}

#[derive(Accounts)]
pub struct PostContent<'info> {
    #[account(init, payer = author, space = Content::LEN, seeds = [b"content", author.key().as_ref()], bump)]
    pub content: Account<'info, Content>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = Content::LEN, seeds = [b"vault", content.key().as_ref()], bump)]
    /// CHECK: This is not dangerous because we just pay to this account
    pub vault_keypair: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateContent<'info> {
    #[account(init, payer = author, space = Validate::LEN, seeds = [b"validate", author.key().as_ref()], bump)]
    pub validate: Account<'info, Validate>,
    #[account(mut)]
    pub author: Signer<'info>, 
    #[account(mut)]
    pub key: Account<'info, Content>,
    #[account(mut)]
    /// CHECK:
    poster_key: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we just pay to this account
    pub vault_keypair: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Collect<'info> {
    #[account(mut)]
    pub validate: Account<'info, Validate>,
    #[account(mut)]
    pub author: Signer<'info>, 
    #[account(mut)]
    pub key: Account<'info, Content>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we just pay to this account
    pub vault_keypair: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Content {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content_link: String,
    pub amount: u64,
    pub total_pool: u64,
    pub short_pool: u64,
    pub long_pool: u64,
    pub short_win: bool,
    pub long_win: bool,
    pub validator_threshold: i64,
    pub validator_count: i64,
    pub validator_threshold_reached: bool,
}

#[account]
pub struct Validate {
    pub author: Pubkey,
    pub key: Pubkey,
    pub timestamp: i64,
    pub amount: u64,
    pub count: i64,
    pub position: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.
const REVIEW_LENGTH: usize = 32;

impl Content {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH
        + REVIEW_LENGTH; // Content.
}

impl Validate {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // TweetKey.
        + PUBLIC_KEY_LENGTH // TweetKey.
        + TIMESTAMP_LENGTH // Timestamp.
        + PUBLIC_KEY_LENGTH; // Verifier.
}

#[error_code]
pub enum ErrorCode {
    #[msg("Content link Required")]
    ContentRequired,
    #[msg("Validator Threshold must be odd")]
    ThresholdEven,
    #[msg("Validator Threshold must be 51 or greater")]
    ThresholdTooSmall,
    #[msg("Validator Threshold has been reached")]
    ThresholdReached,
    #[msg("No Dispersement Available")]
    NoDispersement,
}