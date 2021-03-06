/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetailedActivityAllOf {
    /// The description of the activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "photos", skip_serializing_if = "Option::is_none")]
    pub photos: Option<Box<crate::models::PhotosSummary>>,
    #[serde(rename = "gear", skip_serializing_if = "Option::is_none")]
    pub gear: Option<Box<crate::models::SummaryGear>>,
    /// The number of kilocalories consumed during this activity
    #[serde(rename = "calories", skip_serializing_if = "Option::is_none")]
    pub calories: Option<f32>,
    #[serde(rename = "segment_efforts", skip_serializing_if = "Option::is_none")]
    pub segment_efforts: Option<Vec<crate::models::DetailedSegmentEffort>>,
    /// The name of the device used to record the activity
    #[serde(rename = "device_name", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// The token used to embed a Strava activity
    #[serde(rename = "embed_token", skip_serializing_if = "Option::is_none")]
    pub embed_token: Option<String>,
    /// The splits of this activity in metric units (for runs)
    #[serde(rename = "splits_metric", skip_serializing_if = "Option::is_none")]
    pub splits_metric: Option<Vec<crate::models::Split>>,
    /// The splits of this activity in imperial units (for runs)
    #[serde(rename = "splits_standard", skip_serializing_if = "Option::is_none")]
    pub splits_standard: Option<Vec<crate::models::Split>>,
    #[serde(rename = "laps", skip_serializing_if = "Option::is_none")]
    pub laps: Option<Vec<crate::models::Lap>>,
    #[serde(rename = "best_efforts", skip_serializing_if = "Option::is_none")]
    pub best_efforts: Option<Vec<crate::models::DetailedSegmentEffort>>,
}

impl DetailedActivityAllOf {
    pub fn new() -> DetailedActivityAllOf {
        DetailedActivityAllOf {
            description: None,
            photos: None,
            gear: None,
            calories: None,
            segment_efforts: None,
            device_name: None,
            embed_token: None,
            splits_metric: None,
            splits_standard: None,
            laps: None,
            best_efforts: None,
        }
    }
}
