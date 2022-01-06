# \ClubsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_club_activities_by_id**](ClubsApi.md#get_club_activities_by_id) | **GET** /clubs/{id}/activities | List Club Activities
[**get_club_admins_by_id**](ClubsApi.md#get_club_admins_by_id) | **GET** /clubs/{id}/admins | List Club Administrators
[**get_club_by_id**](ClubsApi.md#get_club_by_id) | **GET** /clubs/{id} | Get Club
[**get_club_members_by_id**](ClubsApi.md#get_club_members_by_id) | **GET** /clubs/{id}/members | List Club Members
[**get_logged_in_athlete_clubs**](ClubsApi.md#get_logged_in_athlete_clubs) | **GET** /athlete/clubs | List Athlete Clubs



## get_club_activities_by_id

> Vec<crate::models::SummaryActivity> get_club_activities_by_id(id, page, per_page)
List Club Activities

Retrieve recent activities from members of a specific club. The authenticated athlete must belong to the requested club in order to hit this endpoint. Pagination is supported. Athlete profile visibility is respected for all activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the club. | [required] |
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<crate::models::SummaryActivity>**](SummaryActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_club_admins_by_id

> Vec<crate::models::SummaryAthlete> get_club_admins_by_id(id, page, per_page)
List Club Administrators

Returns a list of the administrators of a given club.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the club. | [required] |
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<crate::models::SummaryAthlete>**](SummaryAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_club_by_id

> crate::models::DetailedClub get_club_by_id(id)
Get Club

Returns a given club using its identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the club. | [required] |

### Return type

[**crate::models::DetailedClub**](DetailedClub.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_club_members_by_id

> Vec<crate::models::SummaryAthlete> get_club_members_by_id(id, page, per_page)
List Club Members

Returns a list of the athletes who are members of a given club.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the club. | [required] |
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<crate::models::SummaryAthlete>**](SummaryAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_in_athlete_clubs

> Vec<crate::models::SummaryClub> get_logged_in_athlete_clubs(page, per_page)
List Athlete Clubs

Returns a list of the clubs whose membership includes the authenticated athlete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<crate::models::SummaryClub>**](SummaryClub.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

