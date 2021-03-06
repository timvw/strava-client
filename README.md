# Rust API client for Strava

The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3.0.0
- Package version: 3.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

```bash
 docker run --rm -it \
    -v "${PWD}:/host" \
    openapitools/openapi-generator-cli \
    "generate" \
    "-i" "https://developers.strava.com/swagger/swagger.json" \
    "-g" "rust" \
    "-o" "/host/strava-client" \
    "--additional-properties=supportMultipleResponses=true,useSingleRequestParameter=true,packageName=strava_client" \
    "--skip-validate-spec"
```


## Installation

Put the package under your project folder in a directory named `strava_client` and add the following to `Cargo.toml` under `[dependencies]`:

```
strava_client = { path = "./strava_client" }
```

## Documentation for API Endpoints

All URIs are relative to *https://www.strava.com/api/v3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ActivitiesApi* | [**create_activity**](docs/ActivitiesApi.md#create_activity) | **POST** /activities | Create an Activity
*ActivitiesApi* | [**get_activity_by_id**](docs/ActivitiesApi.md#get_activity_by_id) | **GET** /activities/{id} | Get Activity
*ActivitiesApi* | [**get_comments_by_activity_id**](docs/ActivitiesApi.md#get_comments_by_activity_id) | **GET** /activities/{id}/comments | List Activity Comments
*ActivitiesApi* | [**get_kudoers_by_activity_id**](docs/ActivitiesApi.md#get_kudoers_by_activity_id) | **GET** /activities/{id}/kudos | List Activity Kudoers
*ActivitiesApi* | [**get_laps_by_activity_id**](docs/ActivitiesApi.md#get_laps_by_activity_id) | **GET** /activities/{id}/laps | List Activity Laps
*ActivitiesApi* | [**get_logged_in_athlete_activities**](docs/ActivitiesApi.md#get_logged_in_athlete_activities) | **GET** /athlete/activities | List Athlete Activities
*ActivitiesApi* | [**get_zones_by_activity_id**](docs/ActivitiesApi.md#get_zones_by_activity_id) | **GET** /activities/{id}/zones | Get Activity Zones
*ActivitiesApi* | [**update_activity_by_id**](docs/ActivitiesApi.md#update_activity_by_id) | **PUT** /activities/{id} | Update Activity
*AthletesApi* | [**get_logged_in_athlete**](docs/AthletesApi.md#get_logged_in_athlete) | **GET** /athlete | Get Authenticated Athlete
*AthletesApi* | [**get_logged_in_athlete_zones**](docs/AthletesApi.md#get_logged_in_athlete_zones) | **GET** /athlete/zones | Get Zones
*AthletesApi* | [**get_stats**](docs/AthletesApi.md#get_stats) | **GET** /athletes/{id}/stats | Get Athlete Stats
*AthletesApi* | [**update_logged_in_athlete**](docs/AthletesApi.md#update_logged_in_athlete) | **PUT** /athlete | Update Athlete
*ClubsApi* | [**get_club_activities_by_id**](docs/ClubsApi.md#get_club_activities_by_id) | **GET** /clubs/{id}/activities | List Club Activities
*ClubsApi* | [**get_club_admins_by_id**](docs/ClubsApi.md#get_club_admins_by_id) | **GET** /clubs/{id}/admins | List Club Administrators
*ClubsApi* | [**get_club_by_id**](docs/ClubsApi.md#get_club_by_id) | **GET** /clubs/{id} | Get Club
*ClubsApi* | [**get_club_members_by_id**](docs/ClubsApi.md#get_club_members_by_id) | **GET** /clubs/{id}/members | List Club Members
*ClubsApi* | [**get_logged_in_athlete_clubs**](docs/ClubsApi.md#get_logged_in_athlete_clubs) | **GET** /athlete/clubs | List Athlete Clubs
*GearsApi* | [**get_gear_by_id**](docs/GearsApi.md#get_gear_by_id) | **GET** /gear/{id} | Get Equipment
*RoutesApi* | [**get_route_as_gpx**](docs/RoutesApi.md#get_route_as_gpx) | **GET** /routes/{id}/export_gpx | Export Route GPX
*RoutesApi* | [**get_route_as_tcx**](docs/RoutesApi.md#get_route_as_tcx) | **GET** /routes/{id}/export_tcx | Export Route TCX
*RoutesApi* | [**get_route_by_id**](docs/RoutesApi.md#get_route_by_id) | **GET** /routes/{id} | Get Route
*RoutesApi* | [**get_routes_by_athlete_id**](docs/RoutesApi.md#get_routes_by_athlete_id) | **GET** /athletes/{id}/routes | List Athlete Routes
*SegmentEffortsApi* | [**get_efforts_by_segment_id**](docs/SegmentEffortsApi.md#get_efforts_by_segment_id) | **GET** /segment_efforts | List Segment Efforts
*SegmentEffortsApi* | [**get_segment_effort_by_id**](docs/SegmentEffortsApi.md#get_segment_effort_by_id) | **GET** /segment_efforts/{id} | Get Segment Effort
*SegmentsApi* | [**explore_segments**](docs/SegmentsApi.md#explore_segments) | **GET** /segments/explore | Explore segments
*SegmentsApi* | [**get_logged_in_athlete_starred_segments**](docs/SegmentsApi.md#get_logged_in_athlete_starred_segments) | **GET** /segments/starred | List Starred Segments
*SegmentsApi* | [**get_segment_by_id**](docs/SegmentsApi.md#get_segment_by_id) | **GET** /segments/{id} | Get Segment
*SegmentsApi* | [**star_segment**](docs/SegmentsApi.md#star_segment) | **PUT** /segments/{id}/starred | Star Segment
*StreamsApi* | [**get_activity_streams**](docs/StreamsApi.md#get_activity_streams) | **GET** /activities/{id}/streams | Get Activity Streams
*StreamsApi* | [**get_route_streams**](docs/StreamsApi.md#get_route_streams) | **GET** /routes/{id}/streams | Get Route Streams
*StreamsApi* | [**get_segment_effort_streams**](docs/StreamsApi.md#get_segment_effort_streams) | **GET** /segment_efforts/{id}/streams | Get Segment Effort Streams
*StreamsApi* | [**get_segment_streams**](docs/StreamsApi.md#get_segment_streams) | **GET** /segments/{id}/streams | Get Segment Streams
*UploadsApi* | [**create_upload**](docs/UploadsApi.md#create_upload) | **POST** /uploads | Upload Activity
*UploadsApi* | [**get_upload_by_id**](docs/UploadsApi.md#get_upload_by_id) | **GET** /uploads/{uploadId} | Get Upload


## Documentation For Models

 - [ActivityStats](docs/ActivityStats.md)
 - [ActivityTotal](docs/ActivityTotal.md)
 - [ActivityType](docs/ActivityType.md)
 - [ActivityZone](docs/ActivityZone.md)
 - [AltitudeStream](docs/AltitudeStream.md)
 - [AltitudeStreamAllOf](docs/AltitudeStreamAllOf.md)
 - [BaseStream](docs/BaseStream.md)
 - [CadenceStream](docs/CadenceStream.md)
 - [CadenceStreamAllOf](docs/CadenceStreamAllOf.md)
 - [Comment](docs/Comment.md)
 - [DetailedActivity](docs/DetailedActivity.md)
 - [DetailedActivityAllOf](docs/DetailedActivityAllOf.md)
 - [DetailedAthlete](docs/DetailedAthlete.md)
 - [DetailedAthleteAllOf](docs/DetailedAthleteAllOf.md)
 - [DetailedClub](docs/DetailedClub.md)
 - [DetailedClubAllOf](docs/DetailedClubAllOf.md)
 - [DetailedGear](docs/DetailedGear.md)
 - [DetailedGearAllOf](docs/DetailedGearAllOf.md)
 - [DetailedSegment](docs/DetailedSegment.md)
 - [DetailedSegmentAllOf](docs/DetailedSegmentAllOf.md)
 - [DetailedSegmentEffort](docs/DetailedSegmentEffort.md)
 - [DetailedSegmentEffortAllOf](docs/DetailedSegmentEffortAllOf.md)
 - [DistanceStream](docs/DistanceStream.md)
 - [DistanceStreamAllOf](docs/DistanceStreamAllOf.md)
 - [Error](docs/Error.md)
 - [ExplorerResponse](docs/ExplorerResponse.md)
 - [ExplorerSegment](docs/ExplorerSegment.md)
 - [Fault](docs/Fault.md)
 - [HeartRateZoneRanges](docs/HeartRateZoneRanges.md)
 - [HeartrateStream](docs/HeartrateStream.md)
 - [HeartrateStreamAllOf](docs/HeartrateStreamAllOf.md)
 - [Lap](docs/Lap.md)
 - [LatLngStream](docs/LatLngStream.md)
 - [LatLngStreamAllOf](docs/LatLngStreamAllOf.md)
 - [MetaActivity](docs/MetaActivity.md)
 - [MetaAthlete](docs/MetaAthlete.md)
 - [MetaClub](docs/MetaClub.md)
 - [MovingStream](docs/MovingStream.md)
 - [MovingStreamAllOf](docs/MovingStreamAllOf.md)
 - [PhotosSummary](docs/PhotosSummary.md)
 - [PhotosSummaryPrimary](docs/PhotosSummaryPrimary.md)
 - [PolylineMap](docs/PolylineMap.md)
 - [PowerStream](docs/PowerStream.md)
 - [PowerStreamAllOf](docs/PowerStreamAllOf.md)
 - [PowerZoneRanges](docs/PowerZoneRanges.md)
 - [Route](docs/Route.md)
 - [SmoothGradeStream](docs/SmoothGradeStream.md)
 - [SmoothGradeStreamAllOf](docs/SmoothGradeStreamAllOf.md)
 - [SmoothVelocityStream](docs/SmoothVelocityStream.md)
 - [SmoothVelocityStreamAllOf](docs/SmoothVelocityStreamAllOf.md)
 - [Split](docs/Split.md)
 - [StreamSet](docs/StreamSet.md)
 - [SummaryActivity](docs/SummaryActivity.md)
 - [SummaryActivityAllOf](docs/SummaryActivityAllOf.md)
 - [SummaryAthlete](docs/SummaryAthlete.md)
 - [SummaryAthleteAllOf](docs/SummaryAthleteAllOf.md)
 - [SummaryClub](docs/SummaryClub.md)
 - [SummaryClubAllOf](docs/SummaryClubAllOf.md)
 - [SummaryGear](docs/SummaryGear.md)
 - [SummaryPrSegmentEffort](docs/SummaryPrSegmentEffort.md)
 - [SummarySegment](docs/SummarySegment.md)
 - [SummarySegmentEffort](docs/SummarySegmentEffort.md)
 - [TemperatureStream](docs/TemperatureStream.md)
 - [TemperatureStreamAllOf](docs/TemperatureStreamAllOf.md)
 - [TimeStream](docs/TimeStream.md)
 - [TimeStreamAllOf](docs/TimeStreamAllOf.md)
 - [TimedZoneRange](docs/TimedZoneRange.md)
 - [TimedZoneRangeAllOf](docs/TimedZoneRangeAllOf.md)
 - [UpdatableActivity](docs/UpdatableActivity.md)
 - [Upload](docs/Upload.md)
 - [ZoneRange](docs/ZoneRange.md)
 - [Zones](docs/Zones.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

[Tim Van Wassenhove](github@timvw.be).

