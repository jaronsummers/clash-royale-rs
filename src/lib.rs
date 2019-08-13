extern crate reqwest;

use serde::{Deserialize};

pub struct ClashClient {
    pub token: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub tag: String,
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub clan_chest_points: u32
}

#[derive(Deserialize)]
pub struct ClanMembers {
    pub items: Vec<ClanMember>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLogEntry {
    pub standings: Vec<ClanWarStanding>,
    pub season_id: u32,
    pub participants: Vec<ClanWarParticipant>,
    pub created_date: String
}

#[derive(Deserialize)]
pub struct ClanWarLog {
    pub items: Vec<ClanWarLogEntry>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarStanding {
    pub trophy_change: i32,
    pub clan: ClanWarClan
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarParticipant {
    pub tag: String,
    pub name: String,
    pub cards_earned: u32,
    pub battles_played: u32,
    pub wins: u32,
    pub collection_day_battles_played: u32,
    pub number_of_battles: u32
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarClan {
    pub tag: String,
    pub clan_score: u32,
    pub crowns: u32,
    pub badge_id: u32,
    pub name: String,
    pub participants: u32,
    pub battles_played: u32,
    pub wins: u32
}

impl ClashClient {
    pub fn get_clan_warlog(&mut self, clan_id: &str) -> Result<ClanWarLog, reqwest::Error> {
        let formatted_url = format!("https://api.clashroyale.com/v1/clans/{}/warlog", clan_id);
        let res = self.get_request(&formatted_url)?;
        let parsed: ClanWarLog = serde_json::from_value(res).unwrap();
        Ok(parsed)
    }

    pub fn get_clan_members(&mut self, clan_id: &str) -> Result<ClanMembers, reqwest::Error> {
        let formatted_url = format!("https://api.clashroyale.com/v1/clans/{}/members", clan_id);
        let res = self.get_request(&formatted_url)?;
        let parsed: ClanMembers = serde_json::from_value(res).unwrap();
        Ok(parsed)
    }

    fn get_request(&mut self, formatted_url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res: serde_json::Value = client
            .get(formatted_url)
            .header("Accept","application/json")
            .header("authorization", format!("Bearer {}", self.token))
            .send()?
            .json()?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basically_nothing() {
        let client = ClashClient { token: "fake_token".to_owned() };

        assert!("fake_token" == client.token)

    }
}