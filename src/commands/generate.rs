use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn generate(
    ctx: Context<'_>,
    #[description = "How many sentences do you want me to generate? (1 to 10!)"] quantity: Option<u8>
) -> Result<(), Error> {
    match quantity {
        q if q.unwrap() == 0 => ctx.say("Very funny!! Are you playing with me? I won't generate anything then! Hmpf!").await?,
        _ => {
            if let Some(qt) = quantity {

            }
        }
    };

    Ok(())
}