use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::instruction::create_metadata_accounts_v3;

declare_id!("4MfNNwfzUD4zEQ5tsiYetXspFGVcMsUzE8uAFfzTQXYz");

#[program]
pub mod calculator {
    use super::*;

    pub fn calculate(ctx: Context<Calc>,operation: Functions,a:i32,b:i32) -> Result<()> {
        require!(a>-2000 && a<2000 && b>-2000 && b<2000, MyError::DataInvalid);
        match operation {
            Functions::Add => {
                ctx.accounts.user.result = a + b;
            },
            Functions::Sub =>{
                ctx.accounts.user.result = a - b;
            },
            Functions::Mul => {
                ctx.accounts.user.result = a * b;
            },
            Functions::Div => {
                ctx.accounts.user.result = a / b;
            }
        }

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
        let signer = [&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ),
            100000000,
        )?;

        msg!("calculation success and minted 10 tokens");
        Ok(())
    }

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()>{
        msg!("pda initialized for calculator");
        
        Ok(())
    }

    pub fn init_mint(ctx: Context<InitMint>, metadata: InitTokenParams) -> Result<()>{
        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        invoke_signed(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint.key(),
                metadata.name,
                metadata.symbol,
                metadata.uri,
                None,
                0,
                true,
                true,
                None,
                None,
                None,
            ),
            account_info.as_slice(),
            &signer,
        )?;

        msg!("Token mint created successfully.");
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + 32 + 32,
        seeds = ["calc".as_bytes(),signer.key().as_ref()],
        bump
    )]
    pub pda_account : Account<'info,Answer>,
    #[account(mut)]
    pub signer : Signer<'info>,
    pub system_program : Program<'info,System>
}

#[derive(Accounts)]
pub struct Calc <'info> {
    #[account(
        mut,
        seeds = ["calc".as_bytes(),signer.key().as_ref()],
        bump
    )
    ]
    pub user : Account<'info,Answer>,
    #[account(mut)]
    pub signer : Signer<'info>,


    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub destination: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(
    params: InitTokenParams
)]
pub struct InitMint<'info>{
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum  Functions {
    Add,
    Sub,
    Mul,
    Div
}

#[account]
#[derive(Default)]
pub struct Answer{
    pub result : i32
}

#[error_code]
pub enum MyError {
    #[msg("inputs accpeted only between -2000 and 2000")]
    DataInvalid,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}