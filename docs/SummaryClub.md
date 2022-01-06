# SummaryClub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The club's unique identifier. | [optional]
**resource_state** | Option<**i32**> | Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\" | [optional]
**name** | Option<**String**> | The club's name. | [optional]
**profile_medium** | Option<**String**> | URL to a 60x60 pixel profile picture. | [optional]
**cover_photo** | Option<**String**> | URL to a ~1185x580 pixel cover photo. | [optional]
**cover_photo_small** | Option<**String**> | URL to a ~360x176  pixel cover photo. | [optional]
**sport_type** | Option<**String**> | Deprecated. Prefer to use activity_types. | [optional]
**activity_types** | Option<[**Vec<crate::models::ActivityType>**](ActivityType.md)> | The activity types that count for a club. This takes precedence over sport_type. | [optional]
**city** | Option<**String**> | The club's city. | [optional]
**state** | Option<**String**> | The club's state or geographical region. | [optional]
**country** | Option<**String**> | The club's country. | [optional]
**private** | Option<**bool**> | Whether the club is private. | [optional]
**member_count** | Option<**i32**> | The club's member count. | [optional]
**featured** | Option<**bool**> | Whether the club is featured or not. | [optional]
**verified** | Option<**bool**> | Whether the club is verified or not. | [optional]
**url** | Option<**String**> | The club's vanity URL. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


