// https://fantasy.premierleague.com/api/entry/4324/
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

//
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub id: i64,
    #[serde(rename = "joined_time")]
    pub joined_time: String,
    #[serde(rename = "started_event")]
    pub started_event: i64,
    #[serde(rename = "favourite_team")]
    pub favourite_team: i64,
    #[serde(rename = "player_first_name")]
    pub player_first_name: String,
    #[serde(rename = "player_last_name")]
    pub player_last_name: String,
    #[serde(rename = "player_region_id")]
    pub player_region_id: i64,
    #[serde(rename = "player_region_name")]
    pub player_region_name: String,
    #[serde(rename = "player_region_iso_code_short")]
    pub player_region_iso_code_short: String,
    #[serde(rename = "player_region_iso_code_long")]
    pub player_region_iso_code_long: String,
    #[serde(rename = "years_active")]
    pub years_active: i64,
    #[serde(rename = "summary_overall_points")]
    pub summary_overall_points: i64,
    #[serde(rename = "summary_overall_rank")]
    pub summary_overall_rank: i64,
    #[serde(rename = "summary_event_points")]
    pub summary_event_points: i64,
    #[serde(rename = "summary_event_rank")]
    pub summary_event_rank: i64,
    #[serde(rename = "current_event")]
    pub current_event: i64,
    pub leagues: Leagues,
    pub name: String,
    #[serde(rename = "name_change_blocked")]
    pub name_change_blocked: bool,
    #[serde(rename = "entered_events")]
    pub entered_events: Vec<i64>,
    pub kit: String,
    #[serde(rename = "last_deadline_bank")]
    pub last_deadline_bank: i64,
    #[serde(rename = "last_deadline_value")]
    pub last_deadline_value: i64,
    #[serde(rename = "last_deadline_total_transfers")]
    pub last_deadline_total_transfers: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leagues {
    pub classic: Vec<Classic>,
    pub h2h: Vec<Value>,
    pub cup: Cup,
    #[serde(rename = "cup_matches")]
    pub cup_matches: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Classic {
    pub id: i64,
    pub name: String,
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    pub created: String,
    pub closed: bool,
    pub rank: Value,
    #[serde(rename = "max_entries")]
    pub max_entries: Value,
    #[serde(rename = "league_type")]
    pub league_type: String,
    pub scoring: String,
    #[serde(rename = "admin_entry")]
    pub admin_entry: Option<i64>,
    #[serde(rename = "start_event")]
    pub start_event: i64,
    #[serde(rename = "entry_can_leave")]
    pub entry_can_leave: bool,
    #[serde(rename = "entry_can_admin")]
    pub entry_can_admin: bool,
    #[serde(rename = "entry_can_invite")]
    pub entry_can_invite: bool,
    #[serde(rename = "has_cup")]
    pub has_cup: bool,
    #[serde(rename = "cup_league")]
    pub cup_league: Value,
    #[serde(rename = "cup_qualified")]
    pub cup_qualified: Value,
    #[serde(rename = "rank_count")]
    pub rank_count: Option<i64>,
    #[serde(rename = "entry_percentile_rank")]
    pub entry_percentile_rank: Option<i64>,
    #[serde(rename = "active_phases")]
    pub active_phases: Vec<ActivePhase>,
    #[serde(rename = "entry_rank")]
    pub entry_rank: i64,
    #[serde(rename = "entry_last_rank")]
    pub entry_last_rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePhase {
    pub phase: i64,
    pub rank: i64,
    #[serde(rename = "last_rank")]
    pub last_rank: i64,
    #[serde(rename = "rank_sort")]
    pub rank_sort: i64,
    pub total: i64,
    #[serde(rename = "league_id")]
    pub league_id: i64,
    #[serde(rename = "rank_count")]
    pub rank_count: Option<i64>,
    #[serde(rename = "entry_percentile_rank")]
    pub entry_percentile_rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cup {
    pub matches: Vec<Value>,
    pub status: Status,
    #[serde(rename = "cup_league")]
    pub cup_league: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "qualification_event")]
    pub qualification_event: Value,
    #[serde(rename = "qualification_numbers")]
    pub qualification_numbers: Value,
    #[serde(rename = "qualification_rank")]
    pub qualification_rank: Value,
    #[serde(rename = "qualification_state")]
    pub qualification_state: Value,
}