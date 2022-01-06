# DetailedAthlete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique identifier of the athlete | [optional]
**resource_state** | Option<**i32**> | Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\" | [optional]
**firstname** | Option<**String**> | The athlete's first name. | [optional]
**lastname** | Option<**String**> | The athlete's last name. | [optional]
**profile_medium** | Option<**String**> | URL to a 62x62 pixel profile picture. | [optional]
**profile** | Option<**String**> | URL to a 124x124 pixel profile picture. | [optional]
**city** | Option<**String**> | The athlete's city. | [optional]
**state** | Option<**String**> | The athlete's state or geographical region. | [optional]
**country** | Option<**String**> | The athlete's country. | [optional]
**sex** | Option<**String**> | The athlete's sex. | [optional]
**premium** | Option<**bool**> | Deprecated.  Use summit field instead. Whether the athlete has any Summit subscription. | [optional]
**summit** | Option<**bool**> | Whether the athlete has any Summit subscription. | [optional]
**created_at** | Option<**String**> | The time at which the athlete was created. | [optional]
**updated_at** | Option<**String**> | The time at which the athlete was last updated. | [optional]
**follower_count** | Option<**i32**> | The athlete's follower count. | [optional]
**friend_count** | Option<**i32**> | The athlete's friend count. | [optional]
**measurement_preference** | Option<**String**> | The athlete's preferred unit system. | [optional]
**ftp** | Option<**i32**> | The athlete's FTP (Functional Threshold Power). | [optional]
**weight** | Option<**f32**> | The athlete's weight. | [optional]
**clubs** | Option<[**Vec<crate::models::SummaryClub>**](SummaryClub.md)> | The athlete's clubs. | [optional]
**bikes** | Option<[**Vec<crate::models::SummaryGear>**](SummaryGear.md)> | The athlete's bikes. | [optional]
**shoes** | Option<[**Vec<crate::models::SummaryGear>**](SummaryGear.md)> | The athlete's shoes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


