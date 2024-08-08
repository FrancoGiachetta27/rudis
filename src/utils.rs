use crate::bot::{Context, Error};

pub mod embed {
    use super::{Context, Error};
    use std::time::Duration;

    use poise::CreateReply;
    use rand::Rng;
    use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor};
    /// sends a single line embeded message
    pub async fn send_embed_message(ctx: &Context<'_>, embed: CreateEmbed) -> Result<(), Error> {
        let reply_builder = CreateReply::default().embed(embed);

        ctx.send(reply_builder).await?;

        Ok(())
    }

    pub fn create_simple_embed((name, value): (&str, &str)) -> CreateEmbed {
        let (r, g, b) = random_rgb();

        CreateEmbed::new()
            .field(name, value, false)
            .colour(Colour::from_rgb(r, g, b))
    }

    pub fn create_multi_embed(fields: Vec<(String, String, bool)>) -> CreateEmbed {
        let (r, g, b) = random_rgb();

        CreateEmbed::new()
            .fields(fields)
            .colour(Colour::from_rgb(r, g, b))
    }

    pub fn create_track_embed(
        title: String,
        description: &str,
        author: String,
        artist: String,
        url: String,
        thumbnail: String,
        duration: Duration,
    ) -> CreateEmbed {
        let (r, g, b) = random_rgb();
        let duration = duration.as_secs() as f64 / 60.0;

        CreateEmbed::new()
            .title(title)
            .description(description)
            .field("Artist", artist, true)
            .field(
                "⌚ Duration: ",
                format!("{} minutes", duration.round().to_string()),
                true,
            )
            .field("Author", author, true)
            .url(url)
            .thumbnail(thumbnail)
            .colour(Colour::from_rgb(r, g, b))
    }

    pub fn create_embed_error(value: &str) -> CreateEmbed {
        let (r, g, b) = random_rgb();

        CreateEmbed::new()
            .field("❌ ERROR!", value, false)
            .colour(Colour::from_rgb(r, g, b))
    }

    fn random_rgb() -> (u8, u8, u8) {
        (
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
        )
    }
}
