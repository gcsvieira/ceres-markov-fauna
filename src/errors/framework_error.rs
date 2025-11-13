use std::backtrace::Backtrace;
use log::error as log_error;
use std::error as std_error;
use crate::Data;

type Error = Box<dyn std_error::Error + Send + Sync>;
type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

pub(crate) async fn on_error(error: FrameworkError<'_>) {
    match error {
        FrameworkError::MissingUserPermissions { ctx, ..} => {
            if let Err(why) = ctx.say("You don't have the permissions for that!").await {
                log_error!("Error sending message: {:?}", why);
            }
        }
        _ => {
            log_error!("Error handling for this is not implemented yet! Reason: {}\n \
            {}", error, Backtrace::capture())
        }
    }
}