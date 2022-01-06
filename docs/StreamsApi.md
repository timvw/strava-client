# \StreamsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity_streams**](StreamsApi.md#get_activity_streams) | **GET** /activities/{id}/streams | Get Activity Streams
[**get_route_streams**](StreamsApi.md#get_route_streams) | **GET** /routes/{id}/streams | Get Route Streams
[**get_segment_effort_streams**](StreamsApi.md#get_segment_effort_streams) | **GET** /segment_efforts/{id}/streams | Get Segment Effort Streams
[**get_segment_streams**](StreamsApi.md#get_segment_streams) | **GET** /segments/{id}/streams | Get Segment Streams



## get_activity_streams

> crate::models::StreamSet get_activity_streams(id, keys, key_by_type)
Get Activity Streams

Returns the given activity's streams. Requires activity:read scope. Requires activity:read_all scope for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |
**keys** | [**Vec<String>**](String.md) | Desired stream types. | [required] |
**key_by_type** | **bool** | Must be true. | [required] |[default to true]

### Return type

[**crate::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_route_streams

> crate::models::StreamSet get_route_streams(id)
Get Route Streams

Returns the given route's streams. Requires read_all scope for private routes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the route. | [required] |

### Return type

[**crate::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_segment_effort_streams

> crate::models::StreamSet get_segment_effort_streams(id, keys, key_by_type)
Get Segment Effort Streams

Returns a set of streams for a segment effort completed by the authenticated athlete. Requires read_all scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the segment effort. | [required] |
**keys** | [**Vec<String>**](String.md) | The types of streams to return. | [required] |
**key_by_type** | **bool** | Must be true. | [required] |[default to true]

### Return type

[**crate::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_segment_streams

> crate::models::StreamSet get_segment_streams(id, keys, key_by_type)
Get Segment Streams

Returns the given segment's streams. Requires read_all scope for private segments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the segment. | [required] |
**keys** | [**Vec<String>**](String.md) | The types of streams to return. | [required] |
**key_by_type** | **bool** | Must be true. | [required] |[default to true]

### Return type

[**crate::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

