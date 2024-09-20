# Kempower Test VTN

## Overview

This is standalone application intended to mimic the basic behavior of
a OpenADR 3.0 Virtual Top Node (VTN) for testing purposes. Most behavior is
simulated and accomplished with dummy behavior and responses.

The application is mostly intended to be a proof of concept and as a result
doesn't follow the normal architecture for our applications and lacks some basics
such as persistent storage, opting for simple in memory storage instead.

Contact @juusohel or the Bus & Truck team for more info.

## Usage

The Test VTN has simple endpoints retrieving events and managing subscriptions.
It does not support multiple organizations, so the assumption is that only one VEN is connected to it.
Aside from the basic VTN endpoints, the application has "admin" endpoints to trigger certain
behavior like generating new events or clearing the event list.

### Basic Endpoints

#### Auth

Endpoints required a valid bearer token fetched from the `/auth` endpoint.
The current token is a dummy token and is hardcoded, nothing secret should be stored in the application memory.

#### Endpoints

- `GET /ping` - Basic health check.
- `POST /auth` - Get an authentication token to use for the other endpoints.
    - Intended to mimic oauth2 flow
    - Uses basic auth with username and password
    - Requires grant_type `client_credentials` and any scope in the body of the request.
    - Returns a dummy token.
- `GET /events` - Retrieve all events. Application starts with 1 dummy event in the past by default
- `GET /subscription` - Retrieve all stored subscriptions.
- `GET /subscription/{id}` - Retrieve a specific subscription.
- `POST /subscription` - Create a new subscription.
    - For the purposes of the proof of concept, there's no collision detection or validation past basic schema.
- `DELETE /subscription/{id}` - Delete a specific subscription.
- `PUT /subscription/{id}` - Update a specific subscription.

### Admin Endpoints

These endpoints are meant to manage the VTN to trigger behaviors such as generating events
and subscriptions and clearing the event list.

#### Auth

Currently, requires the same authentication token as the basic endpoints. In production use we might want this to be
something more secure.

#### Endpoints

- `/admin/trigger/event` - Generate a new event.
    - Generates a new event and places it in the in-memory event storage.
    - see schema in 'create_test_oadr_event.rs' or docs
    - Event will appear in the normal `GET /events` endpoint.
- `/admin/trigger/clear_events` - Clear all events.
    - Removes all events from the in-memory storage, including the initial dummy event
    - Can be used to test the behavior of the VEN when no events are polled.
- `/admin/trigger/subscription/{id}` - Trigger a subscription event push to the VEN
    - Creates an event according to the provided parameters and sends it to the VEN according to the stored subscription
      parameters.
    - The generated event will NOT be stored in memory after generation.
    - Intended to simulate a VTN pushing an event to a VEN when subscriptions are enabled and polling is not active.
- `/admin/trigger/initial_subscription` - Create a basic subscription object with preset values.
    - Creates and stores a basic subscription object pointing towards kempower dev OpenADR API.
    - Intended to provide a basic subscription which can be fetched and modified with a new bearer token according to
      the subscription refresh flow.
    - Mimics a known use case with E.On where the initial subscription is created via the UI

## Deployment

The application is deployed as a standalone application using [Shuttle.rs](https://www.shuttle.rs/).
To run the application locally: `cargo shuttle run` - You will need rust and cargo installed.
[Shuttle docs](https://docs.shuttle.rs/introduction/welcome)

## Test flow examples

None of these test cases are currently implemented, but could be configured for the OpenADR 3.0 API Automated tests in
the future.

### VEN Polling

1. ChargEye OpenADR 3.0 polls the VTN for events with no active events.
    - Authentication token fetched from the `/auth` endpoint.
    - ChargEye OpenADR 3.0 polls the VTN for events - `GET /events`.
    - ChargEye OpenADR 3.0 receives the event list with no active events.
    - ChargEye OpenADR 3.0 processes the events.
    - Lack of active events results in no action taken by ChargEye OpenADR 3.0.
2. ChargEye OpenADR 3.0 polls the VTN for events with an active event.
    - Virtual Charger set up to charge.
    - Automated test suite generates an event with a start time in the future.
        - `POST /admin/trigger/event` with desired parameters
    - Authentication token fetched from the `/auth` endpoint.
    - ChargEye OpenADR 3.0 polls the VTN for events - `GET /events`.
    - ChargEye OpenADR 3.0 receives the event list with an active event.
    - ChargEye OpenADR 3.0 processes the events.
    - Virtual charger monitored to ensure application of event limits.

### VEN Subscription

1. ChargEye OpenADR 3.0 creates a subscription and receives event. (PUSH method of receiving events)
    - Virtual charger set up to charge.
    - A subscription object is created with desired parameters.
        - Intended to mimic frontend subscription creation.
    - ChargEye OpenADR 3.0 attempts to register the subscription with the VTN - `POST /subscription`.
    - ChargEye OpenADR 3.0 receives a successful response.
    - A subscription event is triggered by the VTN - `POST /admin/trigger/subscription/{id}`.
    - ChargEye OpenADR 3.0 receives the event and processes it.
    - Virtual charger monitored to ensure application of event limits.
2. Refreshing access token of an already created subscription
    - Virtual charger set up to charge
    - Create initial subscription in test VTN - `POST /admin/trigger/initial_subscription`
    - ChargEye OpenADR 3.0 fetches subscription from VTN - `GET /subscription/test`
    - ChargEye OpenADR 3.0 refreshes the access token of the subscription and updates the VTN record -
      `PUT /subscription/test`
    - New event is triggered by the VTN - `POST /admin/trigger/subscription/{id}`
    - ChargEye OpenADR 3.0 receives the event and processes it.
    - Virtual charger monitored to ensure application of event limits.

## Tech

The application is built with [axum](https://github.com/tokio-rs/axum) as the main framework and deployed
with [Shuttle.rs](https://www.shuttle.rs/).
It's very much a quick and dirty proof of concept intended to gauge the viability of building a VTN to test OpenADR 3.0
services without having
to rely on 3rd party VTN's or real VTN logic.






