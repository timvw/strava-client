# \UploadsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_upload**](UploadsApi.md#create_upload) | **POST** /uploads | Upload Activity
[**get_upload_by_id**](UploadsApi.md#get_upload_by_id) | **GET** /uploads/{uploadId} | Get Upload



## create_upload

> crate::models::Upload create_upload(file, name, description, trainer, commute, data_type, external_id)
Upload Activity

Uploads a new data file to create an activity from. Requires activity:write scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> | The uploaded file. |  |
**name** | Option<**String**> | The desired name of the resulting activity. |  |
**description** | Option<**String**> | The desired description of the resulting activity. |  |
**trainer** | Option<**String**> | Whether the resulting activity should be marked as having been performed on a trainer. |  |
**commute** | Option<**String**> | Whether the resulting activity should be tagged as a commute. |  |
**data_type** | Option<**String**> | The format of the uploaded file. |  |
**external_id** | Option<**String**> | The desired external identifier of the resulting activity. |  |

### Return type

[**crate::models::Upload**](Upload.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_upload_by_id

> crate::models::Upload get_upload_by_id(upload_id)
Get Upload

Returns an upload for a given identifier. Requires activity:write scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **i64** | The identifier of the upload. | [required] |

### Return type

[**crate::models::Upload**](Upload.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

