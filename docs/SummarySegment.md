# SummarySegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique identifier of this segment | [optional]
**name** | Option<**String**> | The name of this segment | [optional]
**activity_type** | Option<**String**> |  | [optional]
**distance** | Option<**f32**> | The segment's distance, in meters | [optional]
**average_grade** | Option<**f32**> | The segment's average grade, in percents | [optional]
**maximum_grade** | Option<**f32**> | The segments's maximum grade, in percents | [optional]
**elevation_high** | Option<**f32**> | The segments's highest elevation, in meters | [optional]
**elevation_low** | Option<**f32**> | The segments's lowest elevation, in meters | [optional]
**start_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**end_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**climb_category** | Option<**i32**> | The category of the climb [0, 5]. Higher is harder ie. 5 is Hors catégorie, 0 is uncategorized in climb_category. | [optional]
**city** | Option<**String**> | The segments's city. | [optional]
**state** | Option<**String**> | The segments's state or geographical region. | [optional]
**country** | Option<**String**> | The segment's country. | [optional]
**private** | Option<**bool**> | Whether this segment is private. | [optional]
**athlete_pr_effort** | Option<[**crate::models::SummarySegmentEffort**](SummarySegmentEffort.md)> |  | [optional]
**athlete_segment_stats** | Option<[**crate::models::SummaryPrSegmentEffort**](SummaryPRSegmentEffort.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


