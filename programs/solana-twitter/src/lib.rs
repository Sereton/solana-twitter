use anchor_lang::prelude::*;

declare_id!("BZvPErdwUEKjFk76hT2Z4FGqAwmf8k2cVRpL9wTHswVN");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, body: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &mut Signer = &mut ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();


        if topic.chars().count() > 50 {
        // Return a error...
            return Err(ErrorCode::TopicTooLong.into());
        }

        if body.chars().count() > 280 {
        // Return a too-long-body error...
            return Err(ErrorCode::BodyTooLong.into());
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.body = body;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer=author,space=Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)] 
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub body: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50*4;
const MAX_TWEET_LENGTH: usize = 280 *4;

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH + MAX_TWEET_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    BodyTooLong,
}
