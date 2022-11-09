use bigdecimal::{num_traits::PrimInt, ToPrimitive};
use poise::serenity_prelude::CreateEmbed;
use url::Url;

use crate::{
    consts,
    enums::pnw::AlliancePosition,
    strings::pnw,
    structs::{Alliance, Nation},
    types::Context,
};

pub fn alliance<'a>(
    ctx: &'a Context<'_>,
    alliance: &'a Alliance,
) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a> {
    let user = ctx.author();
    let nations = ctx
        .data()
        .cache
        .find_many_nations(|a| a.alliance_id == alliance.id);
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
        .thumbnail(alliance.flag.as_ref().unwrap_or(&"https://politicsandwar.com/uploads/33e0b12e9a0b4a535fc23bea764bcdd0b9f1f158513.png".to_string()))
        .description(pnw::link(
            "Alliance Page".to_string(),
            format!("https://politicsandwar.com/alliance/id={}", alliance.id),
        ))
        .fields([
            ("Alliance ID", format!("{}", alliance.id), true),
            ("Name", alliance.name.to_string(), true),
            (
                "Achronym",
                alliance
                    .acronym
                    .as_ref()
                    .unwrap_or(&"None".to_string())
                    .to_string(),
                true,
            ),
            ("Colour", alliance.color.to_string(), true),
            ("Rank", "Not Implimented".to_string(), true),
            ("Members", pnw::link( nations.iter().filter(|a| a.alliance_position != AlliancePosition::Applicant).count().to_string(),format!("https://politicsandwar.com/index.php?id=15&keyword={}&cat=alliance&ob=score&od=DESC&maximum=50&minimum=0&search=Go&memberview=true",alliance.name) ),true),
            ("Score", alliance.score.round(2).to_string(),true),
            ("Average Score", (format!("{:.2}",alliance.score.to_f32().unwrap() / (nations.iter().filter(|n| n.alliance_position != AlliancePosition::Applicant && n.vacation_mode_turns>0) .count() as f32))), true),
            ("Applicants", nations.iter().filter(|a| a.alliance_position == AlliancePosition::Applicant).count().to_string(),true),
            ("Leaders", nations.iter().filter(|a| a.alliance_position == AlliancePosition::Leader).count().to_string(),true),
            ("Fourm Link",pnw::link("Click Here".to_string(),alliance.forum_link.as_ref().unwrap_or(&"None".to_string()).to_string()),true), 
            ("Dicord Link",pnw::link("Click Here".to_string(),alliance.discord_link.as_ref().unwrap_or(&"None".to_string()).to_string()),true), 
            ("Vacation Mode",nations.iter().filter(|a| a.vacation_mode_turns > 0 && a.alliance_position != AlliancePosition::Applicant).count().to_string(),true),
            ("Average Cities","Not Implimented".to_string(),true),
            ("Average Infrastructure","Not Implimented".to_string(),true),
            ("Treasures","Not Implimented".to_string(),true),

        ])
    })
}

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
            ("War Policy", nation.war_policy.to_string(), true),
            ("Domestic Policy", nation.domestic_policy.to_string(), true),
            ("Continent", nation.continent.to_string(), true),
            ("Color", nation.color.to_string(), true),
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
                nation.alliance_position.to_string(),
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
