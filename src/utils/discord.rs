/*
TODO:
* Implement Embed for formatting.
*/

use crate::models::Birthday;
use serenity::client::Context;
use serenity::model::prelude::*;
use std::cmp::PartialEq;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum IdTypes {
    User,
    Role,
    Channel,
}

#[allow(clippy::needless_pass_by_value)]
pub fn check_and_format_discordid(id_type: IdTypes, r_id: &str) -> Result<String, String> {
    let min_id_length = match id_type {
        IdTypes::User | IdTypes::Channel => 20,
        IdTypes::Role => 21,
    };
    if id_type == IdTypes::Role && r_id == "@everyone" {
        return Ok("everyone".to_string());
    }
    if r_id.len() < min_id_length {
        return Err("Your inputed user wasn't valid!".to_string());
    }
    let id_firstchar = match id_type {
        IdTypes::User | IdTypes::Channel => 2,
        IdTypes::Role => 3,
    };
    let mut id = r_id[id_firstchar..r_id.len() - 1].to_string();
    if id_type == IdTypes::User && id.starts_with('!') {
        id.remove(0);
    }
    match id.parse::<u64>() {
        Ok(_) => (),
        Err(_) => {
            return Err(format!(
                "Your value \"{}\" wasn't a valid {}!",
                r_id,
                match id_type {
                    IdTypes::User => "user",
                    IdTypes::Role => "role",
                    IdTypes::Channel => "channel",
                }
            ));
        }
    }

    Ok(id)
}

pub async fn format_bday(ctx: &Context, bday: Birthday) -> String {
    let role = match bday.notifyrole {
        Some(x) => {
            RoleId(x.parse::<u64>().unwrap())
                .to_role_cached(ctx)
                .unwrap()
                .name
        }
        None => "everyone".to_string(),
    };

    let channel = ChannelId(bday.channelid.parse::<u64>().unwrap())
        .name(ctx)
        .await
        .unwrap();
    let date = crate::utils::date_as_year_today(bday.date);
    let day_of_month = date.format("%d").to_string().parse::<i32>().unwrap();
    let date_day_end: &str = if (11..=13).contains(&day_of_month) {
        "th"
    } else {
        match day_of_month % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!(
        "In channel: {}, Birthdate: {}, Role to notify: {}, Carlomode: {} \n",
        channel,
        date.format(format!("%d{} of %B", date_day_end).as_str()),
        role,
        bday.allexceptdate
    )
}

pub async fn get_username(ctx: &Context, id: &str, guild_id: &str) -> String {
    let user = UserId(id.parse::<u64>().unwrap())
        .to_user(ctx)
        .await
        .unwrap();
    match user.nick_in(ctx, guild_id.parse::<u64>().unwrap()).await {
        Some(x) => x,
        None => user.name,
    }
}

#[macro_export]
macro_rules! parse_discordid {
    ($id:expr, $x:expr, $ctx:expr, $msg:expr) => {
        match discord::check_and_format_discordid($id, $x) {
            Ok(id) => id,
            Err(err) => {
                $msg.channel_id
                    .say($ctx, format!("Error: `{}`, id: `{}`", err, $x))
                    .await?;
                return Ok(());
            }
        }
    };
}

#[macro_export]
macro_rules! handle_result_with_traceback {
    ($x:expr, $ctx:expr, $msg:expr, $traceback:expr) => {
        match $x {
            Ok(data) => data,
            Err(err) => {
                $msg.channel_id
                    .say($ctx, format!("{} \nDebug Traceback: `{}`", err, $traceback))
                    .await?;
                return Ok(());
            }
        }
    };
}

#[macro_export]
macro_rules! put_response {
    ($x:expr, $ctx:expr, $msg:expr) => {
        $msg.channel_id.say($ctx, $x).await?;
        $msg.delete($ctx).await?;
        return Ok(());
    };
}
