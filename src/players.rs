use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementSummary {
    pub fixtures: Vec<Fixture>,
    pub history: Vec<History>,
    #[serde(rename = "history_past")]
    pub history_past: Vec<HistoryPast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    pub id: i64,
    pub code: i64,
    #[serde(rename = "team_h")]
    pub team_h: i64,
    #[serde(rename = "team_h_score")]
    pub team_h_score: Value,
    #[serde(rename = "team_a")]
    pub team_a: i64,
    #[serde(rename = "team_a_score")]
    pub team_a_score: Value,
    pub event: i64,
    pub finished: bool,
    pub minutes: i64,
    #[serde(rename = "provisional_start_time")]
    pub provisional_start_time: bool,
    #[serde(rename = "kickoff_time")]
    pub kickoff_time: String,
    #[serde(rename = "event_name")]
    pub event_name: String,
    #[serde(rename = "is_home")]
    pub is_home: bool,
    pub difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub element: i64,
    pub fixture: i64,
    #[serde(rename = "opponent_team")]
    pub opponent_team: i64,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    #[serde(rename = "was_home")]
    pub was_home: bool,
    #[serde(rename = "kickoff_time")]
    pub kickoff_time: String,
    #[serde(rename = "team_h_score")]
    pub team_h_score: Option<i64>, // not sure why?
    #[serde(rename = "team_a_score")]
    pub team_a_score: Option<i64>, // not sure why
    pub round: i64,
    pub minutes: i64,
    #[serde(rename = "goals_scored")]
    pub goals_scored: i64,
    pub assists: i64,
    #[serde(rename = "clean_sheets")]
    pub clean_sheets: i64,
    #[serde(rename = "goals_conceded")]
    pub goals_conceded: i64,
    #[serde(rename = "own_goals")]
    pub own_goals: i64,
    #[serde(rename = "penalties_saved")]
    pub penalties_saved: i64,
    #[serde(rename = "penalties_missed")]
    pub penalties_missed: i64,
    #[serde(rename = "yellow_cards")]
    pub yellow_cards: i64,
    #[serde(rename = "red_cards")]
    pub red_cards: i64,
    pub saves: i64,
    pub bonus: i64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    #[serde(rename = "ict_index")]
    pub ict_index: String,
    pub starts: i64,
    #[serde(rename = "expected_goals")]
    pub expected_goals: String,
    #[serde(rename = "expected_assists")]
    pub expected_assists: String,
    #[serde(rename = "expected_goal_involvements")]
    pub expected_goal_involvements: String,
    #[serde(rename = "expected_goals_conceded")]
    pub expected_goals_conceded: String,
    pub value: i64,
    #[serde(rename = "transfers_balance")]
    pub transfers_balance: i64,
    pub selected: i64,
    #[serde(rename = "transfers_in")]
    pub transfers_in: i64,
    #[serde(rename = "transfers_out")]
    pub transfers_out: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPast {
    #[serde(rename = "season_name")]
    pub season_name: String,
    #[serde(rename = "element_code")]
    pub element_code: i64,
    #[serde(rename = "start_cost")]
    pub start_cost: i64,
    #[serde(rename = "end_cost")]
    pub end_cost: i64,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    pub minutes: i64,
    #[serde(rename = "goals_scored")]
    pub goals_scored: i64,
    pub assists: i64,
    #[serde(rename = "clean_sheets")]
    pub clean_sheets: i64,
    #[serde(rename = "goals_conceded")]
    pub goals_conceded: i64,
    #[serde(rename = "own_goals")]
    pub own_goals: i64,
    #[serde(rename = "penalties_saved")]
    pub penalties_saved: i64,
    #[serde(rename = "penalties_missed")]
    pub penalties_missed: i64,
    #[serde(rename = "yellow_cards")]
    pub yellow_cards: i64,
    #[serde(rename = "red_cards")]
    pub red_cards: i64,
    pub saves: i64,
    pub bonus: i64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    #[serde(rename = "ict_index")]
    pub ict_index: String,
    pub starts: i64,
    #[serde(rename = "expected_goals")]
    pub expected_goals: String,
    #[serde(rename = "expected_assists")]
    pub expected_assists: String,
    #[serde(rename = "expected_goal_involvements")]
    pub expected_goal_involvements: String,
    #[serde(rename = "expected_goals_conceded")]
    pub expected_goals_conceded: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GWLiveData {
    pub elements: Vec<Element>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: i64,
    pub stats: Stats,
    pub explain: Vec<Explain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub minutes: i64,
    #[serde(rename = "goals_scored")]
    pub goals_scored: i64,
    pub assists: i64,
    #[serde(rename = "clean_sheets")]
    pub clean_sheets: i64,
    #[serde(rename = "goals_conceded")]
    pub goals_conceded: i64,
    #[serde(rename = "own_goals")]
    pub own_goals: i64,
    #[serde(rename = "penalties_saved")]
    pub penalties_saved: i64,
    #[serde(rename = "penalties_missed")]
    pub penalties_missed: i64,
    #[serde(rename = "yellow_cards")]
    pub yellow_cards: i64,
    #[serde(rename = "red_cards")]
    pub red_cards: i64,
    pub saves: i64,
    pub bonus: i64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    #[serde(rename = "ict_index")]
    pub ict_index: String,
    pub starts: i64,
    #[serde(rename = "expected_goals")]
    pub expected_goals: String,
    #[serde(rename = "expected_assists")]
    pub expected_assists: String,
    #[serde(rename = "expected_goal_involvements")]
    pub expected_goal_involvements: String,
    #[serde(rename = "expected_goals_conceded")]
    pub expected_goals_conceded: String,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    #[serde(rename = "in_dreamteam")]
    pub in_dreamteam: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Explain {
    pub fixture: i64,
    pub stats: Vec<Stat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub identifier: String,
    pub points: i64,
    pub value: i64,
}
