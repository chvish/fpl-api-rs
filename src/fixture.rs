use serde_derive::Deserialize;
use serde_derive::Serialize;

// Fixtures
//
pub type Fixtures = Vec<Fixture>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    pub code: i64,
    pub event: i64,
    pub finished: bool,
    #[serde(rename = "finished_provisional")]
    pub finished_provisional: bool,
    pub id: i64,
    #[serde(rename = "kickoff_time")]
    pub kickoff_time: String,
    pub minutes: i64,
    #[serde(rename = "provisional_start_time")]
    pub provisional_start_time: bool,
    pub started: bool,
    #[serde(rename = "team_a")]
    pub team_a: i64,
    #[serde(rename = "team_a_score")]
    pub team_a_score: Option<i64>,
    #[serde(rename = "team_h")]
    pub team_h: i64,
    #[serde(rename = "team_h_score")]
    pub team_h_score: Option<i64>,
    pub stats: Vec<Stat>,
    #[serde(rename = "team_h_difficulty")]
    pub team_h_difficulty: i64,
    #[serde(rename = "team_a_difficulty")]
    pub team_a_difficulty: i64,
    #[serde(rename = "pulse_id")]
    pub pulse_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub identifier: String,
    pub a: Vec<A>,
    pub h: Vec<H>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A {
    pub value: i64,
    pub element: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct H {
    pub value: i64,
    pub element: i64,
}


