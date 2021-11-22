pub fn check_and_format_userid(mut userid: String) -> Result<String, String> {
    if userid.len() < 20 {
        return Err("Your inputed user wasn't valid!".to_string());
    }
    userid.remove(0);
    userid.remove(0);
    userid.pop();
    if userid.starts_with('!') {
        userid.remove(0);
    }
    match userid.parse::<u64>() {
        Ok(_) => (),
        Err(_) => {
        return Err(format!("Your value \"{}\" wasn't a valid user!", userid));
        },
    }

    Ok(userid)
}

#[macro_export]
macro_rules! check_userid {
    ($x:expr, $ctx:expr, $msg:expr) => {
        match discord::check_and_format_userid($x) {
            Ok(userid) => {userid}
            Err(err) => {
                $msg.channel_id.say($ctx, err).await?;
                return Ok(());
            },

        }
    };
}