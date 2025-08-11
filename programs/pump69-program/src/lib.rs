use anchor_lang::prelude::*;

// Program modules
pub mod league_factory;
pub mod league;

// Use the modules
pub use league_factory::*;
pub use league::*;

// Program ID (will be updated after deployment)
declare_id!("11111111111111111111111111111112");

#[program]
pub mod pump69_program {
    use super::*;

    // League Factory Instructions
    pub fn initialize_factory(
        ctx: Context<InitializeFactory>,
        fee_recipient: Pubkey,
    ) -> Result<()> {
        league_factory::initialize_factory(ctx, fee_recipient)
    }

    pub fn create_league(
        ctx: Context<CreateLeague>,
        league_id: u64,
        entry_fee: u64,
        max_participants: u32,
        duration: i64,
        creator_fee_bps: u16,
        platform_fee_bps: u16,
        referrer_fee_bps: u16,
    ) -> Result<()> {
        league_factory::create_league(
            ctx,
            league_id,
            entry_fee,
            max_participants,
            duration,
            creator_fee_bps,
            platform_fee_bps,
            referrer_fee_bps,
        )
    }

    // League Instructions
    pub fn join_league(
        ctx: Context<JoinLeague>,
        referrer: Option<Pubkey>,
    ) -> Result<()> {
        league::join_league(ctx, referrer)
    }

    pub fn fill_league(ctx: Context<FillLeague>) -> Result<()> {
        league::fill_league(ctx)
    }

    pub fn resolve_league(
        ctx: Context<ResolveLeague>,
        winner: Pubkey,
        scores: Vec<(Pubkey, u32)>,
    ) -> Result<()> {
        league::resolve_league(ctx, winner, scores)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        league::claim_rewards(ctx)
    }

    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>) -> Result<()> {
        league::emergency_withdraw(ctx)
    }
}

// Global error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid fee percentages")]
    InvalidFeePercentages,
    #[msg("League is full")]
    LeagueFull,
    #[msg("League not active")]
    LeagueNotActive,
    #[msg("League already resolved")]
    LeagueAlreadyResolved,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid league state")]
    InvalidLeagueState,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("VRF request failed")]
    VrfRequestFailed,
    #[msg("Invalid VRF proof")]
    InvalidVrfProof,
}
