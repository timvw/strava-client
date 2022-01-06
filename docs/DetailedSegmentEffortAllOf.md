# DetailedSegmentEffortAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the segment on which this effort was performed | [optional]
**activity** | Option<[**crate::models::MetaActivity**](MetaActivity.md)> |  | [optional]
**athlete** | Option<[**crate::models::MetaAthlete**](MetaAthlete.md)> |  | [optional]
**moving_time** | Option<**i32**> | The effort's moving time | [optional]
**start_index** | Option<**i32**> | The start index of this effort in its activity's stream | [optional]
**end_index** | Option<**i32**> | The end index of this effort in its activity's stream | [optional]
**average_cadence** | Option<**f32**> | The effort's average cadence | [optional]
**average_watts** | Option<**f32**> | The average wattage of this effort | [optional]
**device_watts** | Option<**bool**> | For riding efforts, whether the wattage was reported by a dedicated recording device | [optional]
**average_heartrate** | Option<**f32**> | The heart heart rate of the athlete during this effort | [optional]
**max_heartrate** | Option<**f32**> | The maximum heart rate of the athlete during this effort | [optional]
**segment** | Option<[**crate::models::SummarySegment**](SummarySegment.md)> |  | [optional]
**kom_rank** | Option<**i32**> | The rank of the effort on the global leaderboard if it belongs in the top 10 at the time of upload | [optional]
**pr_rank** | Option<**i32**> | The rank of the effort on the athlete's leaderboard if it belongs in the top 3 at the time of upload | [optional]
**hidden** | Option<**bool**> | Whether this effort should be hidden when viewed within an activity | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


