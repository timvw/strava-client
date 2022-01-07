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
pub struct SmoothVelocityStreamAllOf {
    /// The sequence of velocity values for this stream, in meters per second
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<f32>>,
}

impl SmoothVelocityStreamAllOf {
    pub fn new() -> SmoothVelocityStreamAllOf {
        SmoothVelocityStreamAllOf { data: None }
    }
}
