use serenity::{async_trait, futures::future::BoxFuture, prelude::*};
use std::ops::{DerefMut};
use serenity::model::prelude::*;
use chrono::NaiveDate;
use crate::{check_userid, utils::discord};

use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult
};
use serenity::model::id::{ChannelId, RoleId};
use serenity::model::prelude::Message;
use diesel::prelude::*;

use crate::models::Birthday;
use crate::schema::birthdays;

use crate::DB;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready as {}", ready.user.name);
    }
}

#[group]

struct Commands;


#[command]
#[only_in(guilds)]
#[description = "add: Add a new birthday to the calendar. \n Usage: ;add @Member day month @Notifyrole carlomode[1/0]"]
async fn add(ctx: &Context, msg: &Message)->CommandResult<()> {
    let tokens = msg.content.split(' ').collect::<Vec<&str>>();
    
    let userid = check_userid!(tokens[1].to_string(), ctx, msg);

    let mut notifyid = tokens[4].to_string();
    if notifyid.len() < 20 {
        msg.channel_id.say(ctx, "Your inputed role wasn't valid!").await?;
    }
    notifyid.remove(0);
    notifyid.remove(0);
    notifyid.pop();

    match notifyid.parse::<u64>() {
        Ok(_) => (),
        Err(_) => {
        msg.channel_id.say(ctx, format!("Your value \"{}\" wasn't a valid role!", notifyid)).await?;
        },
    }

    let day: u32 = match tokens[2].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            msg.channel_id.say(ctx, format!("Your value \"{}\" wasn't a valid numer!", tokens[2])).await?;
            return Ok(());
        }
    };
    let month: u32 = match tokens[3].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            msg.channel_id.say(ctx, format!("Your value \"{}\" wasn't a valid numer!", tokens[3])).await?;
            return Ok(());
        }
    };
    let year: i32 = 0;

    let guildid = msg.guild_id.unwrap().to_string();

    let channelid = msg.channel_id.to_string();

    let date: NaiveDate = NaiveDate::from_ymd(year, month, day);

    let allexceptdate: bool = matches!(tokens[5], "1");

    let bday = Birthday {
        userid,
        channelid,
        notifyrole: notifyid,
        guildid,
        date,
        allexceptdate,
    };

    if diesel::insert_into(birthdays::dsl::birthdays)
    .values(bday)
    .execute(DB.lock().unwrap().deref_mut()).is_ok() {
        msg.channel_id.say(ctx, format!("Succesfully added birthday to db. <@{}>", msg.author.id)).await?;
    } else {
        msg.channel_id.say(ctx, format!("Couldn't add birthday to db. <@{}>", msg.author.id)).await?;
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "list: list all birthdays in server or get entry for a user. \n Usage: ;list [member]"]
async fn list(ctx: &Context, msg: &Message)->CommandResult {
    let tokens = msg.content.split(' ').collect::<Vec<&str>>();
    
    if tokens.len() == 2 {
        let userid = check_userid!(tokens[1].to_string(), ctx, msg);
        let result: Birthday = birthdays::dsl::birthdays
        .filter(birthdays::dsl::userid.eq(userid))
        .first(DB.lock().unwrap().deref_mut()).unwrap();

        let role_id = result.notifyrole.parse::<u64>().unwrap();

        let role = RoleId(role_id).to_role_cached(ctx).await.unwrap();
        
        msg.channel_id.say(ctx, format!("In channel: <#{}>, birthdate: {}, carlomode: {}, role to notify `{}`",
        result.channelid, result.date.format("%A the %dth of %B %Y"), result.allexceptdate, role.name)).await?;
    } else if tokens.len() == 1 {
        let results: Vec<Birthday> = birthdays::dsl::birthdays.filter(birthdays::dsl::guildid.eq(msg.guild_id.unwrap().as_u64().to_string()))
        .load::<Birthday>(DB.lock().unwrap().deref_mut()).unwrap();
        
        let mut formatted_results: String = String::new();

        for i in results {
            let role_id = i.notifyrole.parse::<u64>().unwrap();

            let role = RoleId(role_id).to_role_cached(ctx).await.unwrap();
            formatted_results.push_str(format!("In channel: <#{}>, birthdate: {}, carlomode: {}, role to notify `{}`. \n",
            i.channelid, i.date.format("%A the %dth of %B %Y"), i.allexceptdate, role.name).as_str());
        }

        msg.channel_id.say(ctx, format!("Birthdays in this server: ```\n{}```", formatted_results)).await?;
    } else {
        msg.channel_id.say(ctx, "Wrong amount of arguments. Check help.").await?;
    }
    return Ok(());
}