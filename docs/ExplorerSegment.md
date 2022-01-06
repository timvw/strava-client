# ExplorerSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique identifier of this segment | [optional]
**name** | Option<**String**> | The name of this segment | [optional]
**climb_category** | Option<**i32**> | The category of the climb [0, 5]. Higher is harder ie. 5 is Hors catégorie, 0 is uncategorized in climb_category. If climb_category = 5, climb_category_desc = HC. If climb_category = 2, climb_category_desc = 3. | [optional]
**climb_category_desc** | Option<**String**> | The description for the category of the climb | [optional]
**avg_grade** | Option<**f32**> | The segment's average grade, in percents | [optional]
**start_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**end_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**elev_difference** | Option<**f32**> | The segments's evelation difference, in meters | [optional]
**distance** | Option<**f32**> | The segment's distance, in meters | [optional]
**points** | Option<**String**> | The polyline of the segment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


