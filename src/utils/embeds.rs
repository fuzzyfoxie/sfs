use chrono::Utc;
use serenity::builder::CreateMessage;

/**
 * Send a success embed.
 */
pub fn success_embed<'a>(
    m: &'a mut CreateMessage,
    title: &str,
    desc: &str,
) -> &'a mut CreateMessage<'a> {
    m.embed(|e| {
        e.color((0xaa, 0xff, 0xaa))
            .title(title.to_owned())
            .description(desc.to_owned())
            .timestamp(Utc::now().to_rfc3339())
    })
}

/**
 * Send an error embed.
 */
pub fn error_embed(title: &str, desc: &str) {}
