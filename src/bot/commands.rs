use serenity::{async_trait, prelude::*};
use std::ops::DerefMut;
use serenity::model::prelude::*;
use chrono::NaiveDate;
use crate::utils::{discord::IdTypes, discord};
use crate::parse_discordid;
use crate::handle_discord;
use crate::put_response;

use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult
};
#[allow(unused_imports)]
use serenity::model::prelude::*;
use diesel::prelude::*;

use crate::models::Birthday;
use crate::schema::birthdays;

use crate::DB;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready as {}", ready.user.name);
    }
}

#[group]
#[commands(add, list, delete)]
struct Commands;


#[command]
#[only_in(guilds)]
#[description = "add: Add a new birthday to the calendar. \n`Usage: ;add @Member day month @Notifyrole Option<carlomode[1/0]>`"]
async fn add(ctx: &Context, msg: &Message, mut args: Args)->CommandResult<()> {
    if args.len() < 4 {
        put_response!("Wrong number of arguments! \n`Usage: ;add @Member day month @Notifyrole Option<carlomode[1/0]>`", ctx, msg);
    }
    let r_userid = parse_discordid!(IdTypes::User, handle_discord!(args.single::<String>(), ctx, msg, "commands_add_userid"), ctx, msg);
    let userid = match UserId(match r_userid.parse::<u64>() {
        Ok(x) => x,
        Err(_) => {
            put_response!("User was invalid!", ctx, msg);
        },
    }).to_user_cached(ctx).await {
        Some(_) => r_userid,
        None => {
            put_response!(format!("User <@{}> not found!", r_userid), ctx, msg);
        }
    };

    let day: u32 = match args.single::<u32>() {
        Ok(val) => val,
        Err(_) => {
            put_response!("Your value wasn't a valid number!", ctx, msg);
        }
    };
    let month: u32 = match args.single::<u32>() {
        Ok(val) => val,
        Err(_) => {
            put_response!("Your value wasn't a valid number!", ctx, msg);
        }
    };

    let p_notifyid = parse_discordid!(IdTypes::Role, handle_discord!(args.single::<String>(), ctx, msg, "commands_add_notifyid"), ctx, msg);

    let notifyid: Option<String> = match p_notifyid.as_str() {
        "everyone" => {
            None
        }
        x => Some({
            match RoleId(match x.parse::<u64>() {
                Ok(y) => y,
                Err(_) => {
                    put_response!(format!("Your value `{}` wasn't a valid role!", x), ctx, msg);
                },
            }).to_role_cached(ctx).await {
                Some(_) => x.to_string(),
                None => {
                    put_response!(format!("Your value `{}` wasn't a valid role!", x), ctx, msg);
                }
            }
        })
    }; 

    let year: i32 = 0;

    let guildid = msg.guild_id.unwrap().to_string();

    let channelid = msg.channel_id.to_string();

    let date: NaiveDate = NaiveDate::from_ymd(year, month, day);

    let allexceptdate: bool;
    if args.len() == 5 {
        allexceptdate = matches!(args.current(), Some("1"));
    } else {
        allexceptdate = false;
    }

    let bday = Birthday {
        userid,
        channelid,
        notifyrole: notifyid,
        guildid,
        date,
        lastdate: NaiveDate::from_ymd(0, 1, 1),
        allexceptdate,
    };

    if diesel::insert_into(birthdays::dsl::birthdays)
    .values(bday)
    .execute(DB.lock().unwrap().deref_mut()).is_ok() {
        put_response!(format!("Succesfully added birthday to db. <@{}>", msg.author.id), ctx, msg);
    } else {
        put_response!(format!("Couldn't add birthday to db. <@{}>", msg.author.id), ctx, msg);
    }
}

#[command]
#[only_in(guilds)]
#[description = "list: list all birthdays in server or get entry for a user. \n`Usage: ;list [member]`"]
async fn list(ctx: &Context, msg: &Message, mut args: Args)->CommandResult {
    if args.len() == 1 {
        let userid = parse_discordid!(IdTypes::User, args.single::<String>().unwrap(), ctx, msg);
        let result: Birthday = birthdays::dsl::birthdays
        .filter(birthdays::dsl::userid.eq(userid))
        .first(DB.lock().unwrap().deref_mut()).unwrap();
       
        put_response!(discord::format_bday(ctx, result).await, ctx, msg);
    } else {
        let results: Vec<Birthday> = birthdays::dsl::birthdays.filter(birthdays::dsl::guildid.eq(msg.guild_id.unwrap().as_u64().to_string()))
        .load::<Birthday>(DB.lock().unwrap().deref_mut()).unwrap();
        
        let mut formatted_results: String = String::new();

        for i in results {
            formatted_results.push_str(discord::format_bday(ctx, i).await.as_str());
        }

        put_response!(format!("Birthdays in this server: ```\n{}```", formatted_results), ctx, msg);
    }
}

#[command]
#[only_in(guilds)]
#[description = "delete: delete a birthday from db. \nUsage: `;delete [member]`"]
async fn delete(ctx: &Context, msg: &Message, mut args: Args)->CommandResult {
    if args.len() != 1 {
        put_response!("Wrong number of arguments! \n`Usage: ;delete [member]`", ctx, msg);
    }

    let user = parse_discordid!(IdTypes::User, handle_discord!(args.single::<String>(), ctx, msg, "commands_delete_userid"), ctx, msg);
    let user_format = user.clone();

    if diesel::delete(birthdays::dsl::birthdays)
        .filter(birthdays::dsl::userid.eq(user))
        .execute(DB.lock().unwrap().deref_mut()).is_ok() {
            put_response!(format!("Succesfully deleted user <@{}> from db.", user_format), ctx, msg);
    } else {
            put_response!(format!("Couldn't delete user <@{}> from db.", user_format), ctx, msg);
    }
}