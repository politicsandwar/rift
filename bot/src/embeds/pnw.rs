use poise::serenity_prelude::CreateEmbed;
use url::Url;

use crate::{consts, enums, structs::Nation, traits::Enum, types::Context};

pub fn nation<'a>(
    ctx: &'a Context,
    nation: &'a Nation,
) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a> {
    let user = ctx.author();
    Box::new(move |e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            user.name.clone(),
            if let Some(url) = user.avatar_url() {
                url
            } else {
                user.default_avatar_url()
            },
        ))
        .colour(consts::embed::INFO_EMBED_COLOUR)
        .fields([
            ("Nation ID", format!("{}", nation.id), true),
            ("Name", nation.name.to_string(), true),
            ("Leader", nation.leader.to_string(), true),
            (
                "War Policy",
                enums::pnw::WarPolicy::from_i16(nation.war_policy)
                    .unwrap()
                    .to_string(),
                true,
            ),
            (
                "Domestic Policy",
                enums::pnw::DomesticPolicy::from_i16(nation.domestic_policy)
                    .unwrap()
                    .to_string(),
                true,
            ),
            (
                "Continent",
                enums::pnw::Continent::from_i16(nation.continent)
                    .unwrap()
                    .to_string(),
                true,
            ),
            (
                "Colour",
                enums::pnw::Color::from_i16(nation.color)
                    .unwrap()
                    .to_string(),
                true,
            ),
            (
                "Alliance",
                format!(
                    "[{}]({})",
                    nation.alliance_id,
                    Url::parse(&format!(
                        "{}{}",
                        "https://politicsandwar.com/alliance/id=", nation.alliance_id
                    ))
                    .expect("somtinbroke")
                ),
                true,
            ),
            (
                "Alliance Position",
                enums::pnw::AlliancePosition::from_i16(nation.alliance_position)
                    .unwrap()
                    .to_string(),
                true,
            ),
            ("Cities", format!("{}", nation.num_cities), true),
            ("Score", format!("{}", nation.score), true),
            (
                "Vacation Mode",
                format!("{}", nation.vacation_mode_turns),
                true,
            ),
            (
                "Soldiers",
                format!(
                    "{}/{}",
                    nation.soldiers,
                    nation.num_cities * consts::pnw::MAX_SOLDIERS_PER_CITY
                ),
                true,
            ),
            (
                "Tanks",
                format!(
                    "{}/{}",
                    nation.tanks,
                    nation.num_cities * consts::pnw::MAX_TANKS_PER_CITY
                ),
                true,
            ),
            (
                "Aircraft",
                format!(
                    "{}/{}",
                    nation.aircraft,
                    nation.num_cities * consts::pnw::MAX_AIRCRAFT_PER_CITY
                ),
                true,
            ),
            (
                "Ships",
                format!(
                    "{}/{}",
                    nation.ships,
                    nation.num_cities * consts::pnw::MAX_SHIPS_PER_CITY
                ),
                true,
            ),
            ("Missiles", format!("{}", nation.missiles), true),
            ("Nukes", format!("{}", nation.nukes), true),
            ("Average Infrastructure", "test".to_string(), true),
            ("Average Land", "test".to_string(), true),
        ])
    })
}
