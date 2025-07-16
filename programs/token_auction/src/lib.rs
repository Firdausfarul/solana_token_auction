use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("ChsHAR6RQRYdDhtG1ohzWfyEQ1mVBwt2gLdrbL6v6iJq");

#[program]
pub mod token_auction {
    use super::*;

    // Creates a new auction.
    pub fn create_auction(ctx: Context<CreateAuction>, end_time: i64, min_bid: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.authority = *ctx.accounts.authority.key;
        auction.end_time = end_time;
        auction.min_bid = min_bid;
        auction.highest_bid = 0;
        auction.highest_bidder = Pubkey::default();
        auction.is_active = true;
        auction.bump = ctx.bumps.auction;

        // Transfer the token to be auctioned into the vault.
        let cpi_accounts = Transfer {
            from: ctx.accounts.authority_token_account.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }

    // Places a bid on an auction.
    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get()?;

        require!(auction.is_active, AuctionError::AuctionInactive);
        require!(clock.unix_timestamp < auction.end_time, AuctionError::AuctionEnded);
        require!(amount > auction.highest_bid, AuctionError::BidTooLow);
        require!(amount >= auction.min_bid, AuctionError::BidTooLow);

        // In a real-world scenario, you would use a PDA to hold the bids.
        // For simplicity, we are transferring the SOL directly to the auction authority.
        // This is NOT recommended for production code.
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.bidder.key(),
            &ctx.accounts.authority.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.bidder.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
        )?;

        // Refund the previous highest bidder.
        if auction.highest_bid > 0 {
            let ix_refund = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.authority.key(),
                &ctx.accounts.previous_bidder.key(),
                auction.highest_bid,
            );
            anchor_lang::solana_program::program::invoke(
                &ix_refund,
                &[
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.previous_bidder.to_account_info(),
                ],
            )?;
        }

        auction.highest_bid = amount;
        auction.highest_bidder = *ctx.accounts.bidder.key;

        Ok(())
    }

    // Ends the auction and settles the funds.
    pub fn end_auction(ctx: Context<EndAuction>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get()?;

        require!(auction.is_active, AuctionError::AuctionInactive);
        require!(clock.unix_timestamp >= auction.end_time, AuctionError::AuctionNotEnded);

        auction.is_active = false;

        // Transfer the token from the vault to the winner.
        let seeds = &[
            b"auction".as_ref(),
            auction.authority.as_ref(),
            &[auction.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.winner_token_account.to_account_info(),
            authority: auction.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 32 + 1 + 1,
        seeds = [b"auction".as_ref(), authority.key().as_ref()],
        bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        seeds = [b"token_vault".as_ref(), auction.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = auction,
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut, seeds = [b"auction".as_ref(), auction.authority.as_ref()], bump = auction.bump)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    /// CHECK: The auction authority. In a real app, use a PDA vault.
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: The previous highest bidder.
    pub previous_bidder: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndAuction<'info> {
    #[account(mut, seeds = [b"auction".as_ref(), auction.authority.as_ref()], bump = auction.bump, has_one = authority)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"token_vault".as_ref(), auction.key().as_ref()], bump)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Auction {
    pub authority: Pubkey,
    pub end_time: i64,
    pub min_bid: u64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}

#[error_code]
pub enum AuctionError {
    #[msg("Auction has ended.")]
    AuctionEnded,
    #[msg("Auction is not active.")]
    AuctionInactive,
    #[msg("Bid amount is too low.")]
    BidTooLow,
    #[msg("Auction has not ended yet.")]
    AuctionNotEnded,
}