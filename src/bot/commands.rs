use crate::handle_result_with_traceback;
use crate::parse_discordid;
use crate::put_response;
use crate::utils::{discord, discord::IdTypes};
use chrono::NaiveDate;
use serenity::model::prelude::*;
use serenity::{async_trait, prelude::*};

use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, Delimiter,
};
#[allow(unused_imports)]
use serenity::model::prelude::*;
use serenity::utils::Colour;

use crate::models::Birthday;
use crate::utils;

use crate::database::statements;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready as {}", ready.user.name);
    }
}

#[group]
#[commands(add, list, delete, clearlastdates)]
struct Commands;

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[description = "add: Add a new birthday to the calendar. \n`Usage: ;add @Member day month @Notifyrole Option<carlomode[1/0]>`"]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult<()> {
    if args.len() < 4 {
        put_response!("Wrong number of arguments! \n`Usage: ;add @Member day month @Notifyrole Option<carlomode[1/0]>`", ctx, msg);
    }
    // This (checking if user exists) isn't implemented, fix it.
    let r_userid = parse_discordid!(
        IdTypes::User,
        handle_result_with_traceback!(&args.single::<String>(), ctx, msg, "commands_add_userid"),
        ctx,
        msg
    );
    let userid = if UserId(if let Ok(u64_userid) = r_userid.parse::<u64>() {
        u64_userid
    } else {
        put_response!("User was invalid!", ctx, msg);
    })
    .to_user(ctx)
    .await
    .is_ok()
    {
        r_userid
    } else {
        put_response!(format!("User <@{r_userid}> not found!"), ctx, msg);
    };

    let day: u32 = if let Ok(val) = args.single::<u32>() {
        val
    } else {
        put_response!("Your value wasn't a valid number!", ctx, msg);
    };
    let month: u32 = if let Ok(val) = args.single::<u32>() {
        val
    } else {
        put_response!("Your value wasn't a valid number!", ctx, msg);
    };

    let p_notifyid = parse_discordid!(
        IdTypes::Role,
        handle_result_with_traceback!(&args.single::<String>(), ctx, msg, "commands_add_notifyid"),
        ctx,
        msg
    );

    let notifyid: Option<String> = match p_notifyid.as_str() {
        "everyone" => None,
        role => Some({
            let Ok(parsed_id) = role.parse::<u64>() else {
                put_response!(format!("Your value `{role}` wasn't a valid role!"), ctx, msg);
            };
            
            if RoleId(parsed_id).to_role_cached(ctx).is_some() {
                role.to_string()
            } else {
                put_response!(format!("Your value `{role}` wasn't a valid role!"), ctx, msg);
            }
        }),
    };

    let year: i32 = 0;

    let guildid = msg.guild_id.unwrap().to_string();

    let channelid = msg.channel_id.to_string();

    let date: NaiveDate = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    let allexceptdate: bool = if args.len() == 5 {
        matches!(args.current(), Some("1"))
    } else {
        false
    };

    let bday = Birthday {
        id: utils::gen_id(),
        userid,
        channelid,
        notifyrole: notifyid,
        guildid,
        date,
        lastdate: NaiveDate::from_ymd_opt(0, 1, 1).unwrap(),
        allexceptdate,
    };

    if statements::insert_bday(bday).is_ok() {
        put_response!(
            format!("Succesfully added birthday to db. <@{}>", msg.author.id),
            ctx,
            msg
        );
    } else {
        msg.channel_id
            .say(
                ctx,
                format!(
                    "Couldn't add birthday to db. <@{}>. (Does the birthday already exist?)",
                    msg.author.id
                ),
            )
            .await?;
        list(ctx, msg, Args::new("", &[Delimiter::Single(' ')])).await?;
        return Ok(());
    }
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[description = "list: list all birthdays in server or get entry for a user. \n`Usage: ;list [member]`"]
async fn list(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
        let userid = parse_discordid!(IdTypes::User, &args.single::<String>().unwrap(), ctx, msg);
        let Some(result) = statements::get_bday_with_userid(&userid) else {
            msg.delete(ctx).await.unwrap();
            msg.channel_id
                .say(ctx, format!("Couldn't fix user <@{userid}>"))
                .await
                .unwrap();
            return Ok(());
        };

        put_response!(discord::format_bday(ctx, result).await, ctx, msg);
    } else {
        let results =
            statements::get_bdays_with_guildid(&msg.guild_id.unwrap().as_u64().to_string());
        // format!("Birthdays in this server: ```\n{}```", formatted_results)

        let mut formatted_results: Vec<(String, String)> = Vec::new();
        for i in results {
            formatted_results.append(&mut vec![(
                discord::get_username(ctx, &i.userid, &i.guildid).await,
                discord::format_bday(ctx, i).await,
            )]);
        }

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    let embed = e
                        .title("Birthdays in this server:")
                        .colour(Colour::from_rgb(0, 192, 100));
                    for i in formatted_results {
                        embed.field(i.0, i.1, false);
                    }
                    embed
                })
            })
            .await?;
        msg.delete(ctx).await?;
        Ok(())
    }
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[description = "delete: delete a birthday from db. \nUsage: `;delete [member]`"]
async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 1 {
        put_response!(
            "Wrong number of arguments! \n`Usage: ;delete [member]`",
            ctx,
            msg
        );
    }

    let user = parse_discordid!(
        IdTypes::User,
        handle_result_with_traceback!(&args.single::<String>(), ctx, msg, "commands_delete_userid"),
        ctx,
        msg
    );

    if statements::delete_bday_with_userid_and_guildid(&user, &msg.guild_id.unwrap().to_string())
        .is_ok()
    {
        put_response!(
            format!("Succesfully deleted user <@{user}> from db."),
            ctx,
            msg
        );
    } else {
        put_response!(
            format!("Couldn't delete user <@{user}> from db."),
            ctx,
            msg
        );
    }
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[description = "clearlastdates: clear all last dates from db. \n`Usage: ;clearlastdates`"]
async fn clearlastdates(ctx: &Context, msg: &Message) -> CommandResult<()> {
    if statements::clear_all_lastdates().is_err() {
        put_response!(
            format!("Couldn't clear lastdates! <@{}>", msg.author.id),
            ctx,
            msg
        );
    };

    put_response!(
        format!("Succesfully cleared lastdates! <@{}>", msg.author.id),
        ctx,
        msg
    );
}
