use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::prelude::*;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Sides"] sides: i8,
) -> Result<(), Error> {
    let mut rng = thread_rng();
    let result: u8 = rng.gen_range(1..sides);
    let response = format!("Your d{} rolled a {}", sides, result);
    ctx.say(response).await?;
    Ok(())
}
