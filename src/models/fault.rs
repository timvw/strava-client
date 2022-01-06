/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Fault : Encapsulates the errors that may be returned from the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Fault {
    /// The set of specific errors associated with this fault, if any.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
    /// The message of the fault.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Fault {
    /// Encapsulates the errors that may be returned from the API.
    pub fn new() -> Fault {
        Fault {
            errors: None,
            message: None,
        }
    }
}


