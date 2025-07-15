use serde::Deserialize;
use crate::models::player::Skill;

#[derive(Debug, Deserialize)]
struct ResponseStats {
    pub skills: Vec<ResponseSkill>,
    // pub activities: Vec<ResponseActivity>,
}

#[derive(Debug, Deserialize)]
struct ResponseSkill {
    // pub id: u32,
    pub name: String,
    // pub rank: i64,
    pub level: u32,
    pub xp: u64,
}

#[derive(Debug, Deserialize)]
struct ResponseActivity {
    // pub id: u32,
    // pub name: String,
    // pub rank: i64,
    // pub score: i64,
}

// Use the OSRS API to get player stats
pub async fn get_player_stats(player_name: &str) -> Result<Vec<Skill>, Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player={}",
        player_name);
    
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .send()
        .await?
        .text()
        .await
        .expect("Failed to get response text");

    let parsed: ResponseStats = serde_json::from_str(&response).unwrap();
    // let player = Player {
    //     name: player_name.to_string(),
    //     skills: parsed.skills.iter().map(|s| {
    //         Skill {
    //             name: s.name.clone(),
    //             level: s.level,
    //             xp: s.xp,
    //         }
    //     }).collect(),
    // };
    let stats = parsed.skills.iter().map(|s| {
        Skill {
            name: s.name.clone(),
            level: s.level,
            xp: s.xp,
        }
    }).collect();
    Ok(stats)
}