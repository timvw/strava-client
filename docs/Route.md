# Route

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**athlete** | Option<[**crate::models::SummaryAthlete**](SummaryAthlete.md)> |  | [optional]
**description** | Option<**String**> | The description of the route | [optional]
**distance** | Option<**f32**> | The route's distance, in meters | [optional]
**elevation_gain** | Option<**f32**> | The route's elevation gain. | [optional]
**id** | Option<**i64**> | The unique identifier of this route | [optional]
**id_str** | Option<**String**> | The unique identifier of the route in string format | [optional]
**map** | Option<[**crate::models::PolylineMap**](PolylineMap.md)> |  | [optional]
**name** | Option<**String**> | The name of this route | [optional]
**private** | Option<**bool**> | Whether this route is private | [optional]
**starred** | Option<**bool**> | Whether this route is starred by the logged-in athlete | [optional]
**timestamp** | Option<**i32**> | An epoch timestamp of when the route was created | [optional]
**_type** | Option<**i32**> | This route's type (1 for ride, 2 for runs) | [optional]
**sub_type** | Option<**i32**> | This route's sub-type (1 for road, 2 for mountain bike, 3 for cross, 4 for trail, 5 for mixed) | [optional]
**created_at** | Option<**String**> | The time at which the route was created | [optional]
**updated_at** | Option<**String**> | The time at which the route was last updated | [optional]
**estimated_moving_time** | Option<**i32**> | Estimated time in seconds for the authenticated athlete to complete route | [optional]
**segments** | Option<[**Vec<crate::models::SummarySegment>**](SummarySegment.md)> | The segments traversed by this route | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


