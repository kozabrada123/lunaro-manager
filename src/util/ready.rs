use poise::serenity_prelude::Member;

use crate::{
    env::Environment,
    types::{error::Error, poise::PoiseContext},
};

/// Add the ready role to a member.
pub async fn add(member: &mut Member, context: PoiseContext<'_>) -> Result<(), Error> {
    member
        .add_role(context, Environment::load()?.ready_role_id)
        .await?;

    log::debug!("Added {} to ready role", member.user.tag());

    Ok(())
}

/// Remove the ready role from a member.
pub async fn remove(member: &mut Member, context: PoiseContext<'_>) -> Result<(), Error> {
    member
        .remove_role(context, Environment::load()?.ready_role_id)
        .await?;

    log::debug!("Removed {} from ready role", member.user.tag());

    Ok(())
}