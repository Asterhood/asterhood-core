use anchor_lang::prelude::*;

declare_id!("AsterHood1111111111111111111111111111111111");

#[program]
pub mod asterhood_core_program {
    use super::*;

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        emit!(CoreEvent { message: "pong".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping {}

#[event]
pub struct CoreEvent {
    pub message: String,
}
