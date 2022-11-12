use async_trait::async_trait;
use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

use crate::{
    enums::pnw::{AlliancePosition, Color, Continent, DomesticPolicy, WarPolicy},
    errors::NotFoundError,
    structs::resources::Resources,
    traits::Convert,
    types::{Context, Error},
};

#[derive(Clone, Debug, Model)]
#[table = "nations"]
#[cache_name = "nation"]
#[subscriptions = "Nation"]
pub struct Nation {
    pub id: i32,
    pub alliance_id: i32,
    pub alliance_position: AlliancePosition,
    #[field("nation_name")]
    pub name: String,
    #[field("leader_name")]
    pub leader: String,
    pub continent: Continent,
    pub war_policy: WarPolicy,
    pub domestic_policy: DomesticPolicy,
    pub color: Color,
    pub num_cities: i32,
    pub score: BigDecimal,
    pub flag: String,
    pub vacation_mode_turns: i32,
    pub beige_turns: i32,
    pub espionage_available: bool,
    #[field_custom("time::OffsetDateTime::UNIX_EPOCH")]
    #[field_no_update]
    pub last_active: OffsetDateTime,
    pub date: OffsetDateTime,
    pub soldiers: i32,
    pub tanks: i32,
    pub aircraft: i32,
    pub ships: i32,
    pub missiles: i32,
    pub nukes: i32,
    #[field("discord")]
    pub discord_username: Option<String>,
    #[field_custom("None")]
    #[field_no_update]
    pub discord_id: Option<String>,
    pub turns_since_last_city: i32,
    pub turns_since_last_project: i32,
    pub projects: i32,
    pub wars_won: i32,
    pub wars_lost: i32,
    pub tax_id: i32,
    pub alliance_seniority: i32,
    #[no_type_check]
    #[field_custom("None")]
    pub estimated_resources: Option<Resources>,
}

#[async_trait]
impl Convert for Nation {
    async fn convert_option(ctx: &Context<'_>, val: Option<String>) -> Result<Self, Error> {
        if let Some(val) = val {
            let res = val.parse::<i32>();
            if res.is_err() {
                return Err(NotFoundError::Nation(Some(val)).into());
            }
            let res = res.unwrap();
            let nation = ctx.data().cache.get_nation(&res);
            if let Some(nation) = nation {
                Ok(nation)
            } else {
                Err(NotFoundError::Nation(Some(val)).into())
            }
        } else {
            let user_id = ctx.author().id.to_string();
            let nation = ctx.data().cache.find_exactly_one_nation(|n| {
                if let Some(discord_id) = &n.discord_id {
                    discord_id == &user_id
                } else {
                    false
                }
            });
            if let Some(nation) = nation {
                Ok(nation)
            } else {
                Err(NotFoundError::Nation(None).into())
            }
        }
    }
}
