/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActivityStats : A set of rolled-up statistics and totals for an athlete



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActivityStats {
    /// The longest distance ridden by the athlete.
    #[serde(rename = "biggest_ride_distance", skip_serializing_if = "Option::is_none")]
    pub biggest_ride_distance: Option<f64>,
    /// The highest climb ridden by the athlete.
    #[serde(rename = "biggest_climb_elevation_gain", skip_serializing_if = "Option::is_none")]
    pub biggest_climb_elevation_gain: Option<f64>,
    #[serde(rename = "recent_ride_totals", skip_serializing_if = "Option::is_none")]
    pub recent_ride_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "recent_run_totals", skip_serializing_if = "Option::is_none")]
    pub recent_run_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "recent_swim_totals", skip_serializing_if = "Option::is_none")]
    pub recent_swim_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "ytd_ride_totals", skip_serializing_if = "Option::is_none")]
    pub ytd_ride_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "ytd_run_totals", skip_serializing_if = "Option::is_none")]
    pub ytd_run_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "ytd_swim_totals", skip_serializing_if = "Option::is_none")]
    pub ytd_swim_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "all_ride_totals", skip_serializing_if = "Option::is_none")]
    pub all_ride_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "all_run_totals", skip_serializing_if = "Option::is_none")]
    pub all_run_totals: Option<Box<crate::models::ActivityTotal>>,
    #[serde(rename = "all_swim_totals", skip_serializing_if = "Option::is_none")]
    pub all_swim_totals: Option<Box<crate::models::ActivityTotal>>,
}

impl ActivityStats {
    /// A set of rolled-up statistics and totals for an athlete
    pub fn new() -> ActivityStats {
        ActivityStats {
            biggest_ride_distance: None,
            biggest_climb_elevation_gain: None,
            recent_ride_totals: None,
            recent_run_totals: None,
            recent_swim_totals: None,
            ytd_ride_totals: None,
            ytd_run_totals: None,
            ytd_swim_totals: None,
            all_ride_totals: None,
            all_run_totals: None,
            all_swim_totals: None,
        }
    }
}


