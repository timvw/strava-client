# DetailedActivityAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the activity | [optional]
**photos** | Option<[**crate::models::PhotosSummary**](PhotosSummary.md)> |  | [optional]
**gear** | Option<[**crate::models::SummaryGear**](SummaryGear.md)> |  | [optional]
**calories** | Option<**f32**> | The number of kilocalories consumed during this activity | [optional]
**segment_efforts** | Option<[**Vec<crate::models::DetailedSegmentEffort>**](DetailedSegmentEffort.md)> |  | [optional]
**device_name** | Option<**String**> | The name of the device used to record the activity | [optional]
**embed_token** | Option<**String**> | The token used to embed a Strava activity | [optional]
**splits_metric** | Option<[**Vec<crate::models::Split>**](Split.md)> | The splits of this activity in metric units (for runs) | [optional]
**splits_standard** | Option<[**Vec<crate::models::Split>**](Split.md)> | The splits of this activity in imperial units (for runs) | [optional]
**laps** | Option<[**Vec<crate::models::Lap>**](Lap.md)> |  | [optional]
**best_efforts** | Option<[**Vec<crate::models::DetailedSegmentEffort>**](DetailedSegmentEffort.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


