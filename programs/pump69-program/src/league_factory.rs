use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use switchboard_v2::{AggregatorAccountData, SwitchboardDecimal};

use crate::ErrorCode;

// League Factory Account
#[account]
pub struct LeagueFactory {
    pub authority: Pubkey,          // Factory admin
    pub fee_recipient: Pubkey,      // Platform fee recipient
    pub league_count: u64,          // Total leagues created
    pub platform_fee_bps: u16,      // Platform fee in basis points (default: 300 = 3%)
    pub creator_fee_bps: u16,       // Creator fee in basis points (default: 600 = 6%)
    pub referrer_fee_bps: u16,      // Referrer fee in basis points (default: 100 = 1%)
    pub bump: u8,
}

impl LeagueFactory {
    pub const LEN: usize = 8 +      // discriminator
                          32 +     // authority
                          32 +     // fee_recipient
                          8 +      // league_count
                          2 +      // platform_fee_bps
                          2 +      // creator_fee_bps
                          2 +      // referrer_fee_bps
                          1;       // bump
}

// League Account
#[account]
pub struct League {
    pub factory: Pubkey,            // Reference to factory
    pub creator: Pubkey,            // League creator
    pub league_id: u64,             // Unique league ID
    pub entry_fee: u64,             // Entry fee amount
    pub max_participants: u32,      // Maximum participants
    pub current_participants: u32,  // Current participant count
    pub duration: i64,              // League duration in seconds
    pub start_time: i64,            // League start timestamp
    pub end_time: i64,              // League end timestamp
    pub state: LeagueState,         // Current league state
    pub prize_pool: u64,            // Total prize pool
    pub creator_fee_bps: u16,       // Creator fee for this league
    pub platform_fee_bps: u16,      // Platform fee for this league
    pub referrer_fee_bps: u16,      // Referrer fee for this league
    pub winner: Option<Pubkey>,     // Winner of the league
    pub vrf_request_key: Option<Pubkey>, // Switchboard VRF request
    pub bump: u8,
}

impl League {
    pub const LEN: usize = 8 +      // discriminator
                          32 +     // factory
                          32 +     // creator
                          8 +      // league_id
                          8 +      // entry_fee
                          4 +      // max_participants
                          4 +      // current_participants
                          8 +      // duration
                          8 +      // start_time
                          8 +      // end_time
                          1 +      // state
                          8 +      // prize_pool
                          2 +      // creator_fee_bps
                          2 +      // platform_fee_bps
                          2 +      // referrer_fee_bps
                          1 + 32 + // winner (Option<Pubkey>)
                          1 + 32 + // vrf_request_key (Option<Pubkey>)
                          1;       // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum LeagueState {
    Created,    // League created, accepting participants
    Filled,     // League filled, ready to start
    Active,     // League is active
    Resolved,   // League resolved with winner
    Cancelled,  // League cancelled
}

// Participant Account
#[account]
pub struct Participant {
    pub league: Pubkey,         // Reference to league
    pub user: Pubkey,           // Participant address
    pub entry_time: i64,        // When they joined
    pub referrer: Option<Pubkey>, // First-touch referrer
    pub score: u32,             // Participant score
    pub rewards_claimed: bool,   // Whether rewards have been claimed
    pub bump: u8,
}

impl Participant {
    pub const LEN: usize = 8 +      // discriminator
                          32 +     // league
                          32 +     // user
                          8 +      // entry_time
                          1 + 32 + // referrer (Option<Pubkey>)
                          4 +      // score
                          1 +      // rewards_claimed
                          1;       // bump
}

// Referrer PDA Account (first-touch attribution)
#[account]
pub struct ReferrerAccount {
    pub user: Pubkey,           // User being referred
    pub referrer: Pubkey,       // First-touch referrer
    pub first_referral_time: i64, // When first referral was made
    pub total_referrals: u64,   // Total referrals made
    pub total_rewards: u64,     // Total rewards earned
    pub bump: u8,
}

impl ReferrerAccount {
    pub const LEN: usize = 8 +      // discriminator
                          32 +     // user
                          32 +     // referrer
                          8 +      // first_referral_time
                          8 +      // total_referrals
                          8 +      // total_rewards
                          1;       // bump
}

// Premium Creator Account
#[account]
pub struct PremiumCreator {
    pub creator: Pubkey,        // Creator address
    pub total_leagues: u64,     // Total leagues created
    pub total_volume: u64,      // Total volume generated
    pub tier: CreatorTier,      // Premium tier
    pub reduced_fee_bps: u16,   // Reduced platform fee
    pub bonus_rewards: u64,     // Accumulated bonus rewards
    pub bump: u8,
}

impl PremiumCreator {
    pub const LEN: usize = 8 +      // discriminator
                          32 +     // creator
                          8 +      // total_leagues
                          8 +      // total_volume
                          1 +      // tier
                          2 +      // reduced_fee_bps
                          8 +      // bonus_rewards
                          1;       // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CreatorTier {
    Bronze,  // Basic tier
    Silver,  // Mid tier
    Gold,    // Premium tier
    Diamond, // Elite tier
}

// Initialize Factory Instruction
#[derive(Accounts)]
pub struct InitializeFactory<'info> {
    #[account(
        init,
        payer = authority,
        space = LeagueFactory::LEN,
        seeds = [b"factory"],
        bump
    )]
    pub factory: Account<'info, LeagueFactory>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Create League Instruction
#[derive(Accounts)]
#[instruction(league_id: u64)]
pub struct CreateLeague<'info> {
    #[account(
        seeds = [b"factory"],
        bump = factory.bump
    )]
    pub factory: Account<'info, LeagueFactory>,
    
    #[account(
        init,
        payer = creator,
        space = League::LEN,
        seeds = [b"league", factory.key().as_ref(), league_id.to_le_bytes().as_ref()],
        bump
    )]
    pub league: Account<'info, League>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// League Factory implementation
pub fn initialize_factory(
    ctx: Context<InitializeFactory>,
    fee_recipient: Pubkey,
) -> Result<()> {
    let factory = &mut ctx.accounts.factory;
    factory.authority = *ctx.accounts.authority.key;
    factory.fee_recipient = fee_recipient;
    factory.league_count = 0;
    factory.platform_fee_bps = 300;  // 3%
    factory.creator_fee_bps = 600;   // 6%
    factory.referrer_fee_bps = 100;  // 1%
    factory.bump = ctx.bumps.factory;
    
    msg!("League Factory initialized with authority: {}", factory.authority);
    
    // Emit event
    emit!(FactoryInitialized {
        authority: factory.authority,
        fee_recipient: factory.fee_recipient,
    });
    
    Ok(())
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
    let factory = &mut ctx.accounts.factory;
    let league = &mut ctx.accounts.league;
    let clock = Clock::get()?;
    
    // Validate fee percentages (total should not exceed 100% and follow 60/30/10 split)
    let total_fees = creator_fee_bps + platform_fee_bps + referrer_fee_bps;
    require!(total_fees <= 1000, ErrorCode::InvalidFeePercentages); // Max 10% total fees
    
    // Enforce 60/30/10 fee split
    require!(
        creator_fee_bps == 600 && platform_fee_bps == 300 && referrer_fee_bps == 100,
        ErrorCode::InvalidFeePercentages
    );
    
    require!(max_participants >= 2, ErrorCode::InvalidLeagueState);
    require!(duration > 0, ErrorCode::InvalidLeagueState);
    require!(entry_fee > 0, ErrorCode::InvalidLeagueState);
    
    // Initialize league
    league.factory = factory.key();
    league.creator = *ctx.accounts.creator.key;
    league.league_id = league_id;
    league.entry_fee = entry_fee;
    league.max_participants = max_participants;
    league.current_participants = 0;
    league.duration = duration;
    league.start_time = 0; // Will be set when league is filled
    league.end_time = 0;
    league.state = LeagueState::Created;
    league.prize_pool = 0;
    league.creator_fee_bps = creator_fee_bps;
    league.platform_fee_bps = platform_fee_bps;
    league.referrer_fee_bps = referrer_fee_bps;
    league.winner = None;
    league.vrf_request_key = None;
    league.bump = ctx.bumps.league;
    
    // Update factory stats
    factory.league_count += 1;
    
    msg!("League {} created by {} with entry fee {}", 
         league_id, league.creator, entry_fee);
    
    // Emit event
    emit!(LeagueCreated {
        league_id,
        creator: league.creator,
        entry_fee,
        max_participants,
        duration,
    });
    
    Ok(())
}

// Events
#[event]
pub struct FactoryInitialized {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
}

#[event]
pub struct LeagueCreated {
    pub league_id: u64,
    pub creator: Pubkey,
    pub entry_fee: u64,
    pub max_participants: u32,
    pub duration: i64,
}

#[event]
pub struct LeagueFilled {
    pub league_id: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct ParticipantJoined {
    pub league_id: u64,
    pub participant: Pubkey,
    pub referrer: Option<Pubkey>,
    pub entry_time: i64,
}

#[event]
pub struct LeagueResolved {
    pub league_id: u64,
    pub winner: Pubkey,
    pub prize_pool: u64,
    pub resolution_time: i64,
}

#[event]
pub struct RewardsClaimed {
    pub league_id: u64,
    pub participant: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ReferralReward {
    pub referrer: Pubkey,
    pub referred_user: Pubkey,
    pub league_id: u64,
    pub reward_amount: u64,
}

#[event]
pub struct PremiumCreatorUpdated {
    pub creator: Pubkey,
    pub tier: CreatorTier,
    pub reduced_fee_bps: u16,
    pub bonus_rewards: u64,
}
