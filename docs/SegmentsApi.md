# \SegmentsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**explore_segments**](SegmentsApi.md#explore_segments) | **GET** /segments/explore | Explore segments
[**get_logged_in_athlete_starred_segments**](SegmentsApi.md#get_logged_in_athlete_starred_segments) | **GET** /segments/starred | List Starred Segments
[**get_segment_by_id**](SegmentsApi.md#get_segment_by_id) | **GET** /segments/{id} | Get Segment
[**star_segment**](SegmentsApi.md#star_segment) | **PUT** /segments/{id}/starred | Star Segment



## explore_segments

> crate::models::ExplorerResponse explore_segments(bounds, activity_type, min_cat, max_cat)
Explore segments

Returns the top 10 segments matching a specified query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bounds** | [**Vec<f32>**](f32.md) | The latitude and longitude for two points describing a rectangular boundary for the search: [southwest corner latitutde, southwest corner longitude, northeast corner latitude, northeast corner longitude] | [required] |
**activity_type** | Option<**String**> | Desired activity type. |  |
**min_cat** | Option<**i32**> | The minimum climbing category. |  |
**max_cat** | Option<**i32**> | The maximum climbing category. |  |

### Return type

[**crate::models::ExplorerResponse**](ExplorerResponse.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_in_athlete_starred_segments

> Vec<crate::models::SummarySegment> get_logged_in_athlete_starred_segments(page, per_page)
List Starred Segments

List of the authenticated athlete's starred segments. Private segments are filtered out unless requested by a token with read_all scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<crate::models::SummarySegment>**](SummarySegment.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_segment_by_id

> crate::models::DetailedSegment get_segment_by_id(id)
Get Segment

Returns the specified segment. read_all scope required in order to retrieve athlete-specific segment information, or to retrieve private segments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the segment. | [required] |

### Return type

[**crate::models::DetailedSegment**](DetailedSegment.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## star_segment

> crate::models::DetailedSegment star_segment(id, starred)
Star Segment

Stars/Unstars the given segment for the authenticated athlete. Requires profile:write scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the segment to star. | [required] |
**starred** | **bool** | If true, star the segment; if false, unstar the segment. | [required] |[default to false]

### Return type

[**crate::models::DetailedSegment**](DetailedSegment.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

