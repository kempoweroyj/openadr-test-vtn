# Kempower Test VTN

## Overview

This is standalone application intended to mimic the basic behavior of
an OpenADR 3.0 Virtual Top Node (VTN) for testing purposes. Most behavior is
simulated and accomplished with dummy behavior and responses.

The application is mostly intended to be a tool for automated testing and not a real VTN and as a result
doesn't implement the entire OpenADR spec and lacks some basics for a production application
such as persistent storage, opting for simple in memory storage instead.

OpenADR 3.0 is a standards specification for demand response and energy management and provides an standardized
interface and protocols for communication between energy providers and customers. In the context of EV Charging,
it can be used, as an example, to manage grid load by limiting available charging power during specified time periods.
Kempower ChargEye features OpenADR 3.0 support, allowing for integration with energy providers and grid operators.

## Usage

The Test VTN has simple endpoints retrieving events and managing subscriptions.
It does not support multiple organizations, so the assumption is that only one VEN is connected to it.
Aside from the basic VTN endpoints, the application has "admin" endpoints to trigger certain
behavior like generating new events or clearing the event list. These are intended to be used as
part of automated test flow.

The Test VTN supports basic OpenADR 3.0 functionality:

- Receive events via PUSH/PULL methods
- Manage subscriptions
- Trigger events for testing purposes

Not implemented (yet):

- Reporting
    - Requiring or receiving reports from VENs
- Different payload types
    - Currently only supports simple event payloads

### Basic Endpoints

#### Auth

Endpoints required a valid bearer token fetched from the `/auth` endpoint.
The token is a dummy token and is hardcoded, nothing secret should be stored in the application memory.

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
something more secure. If security is a concern, proper token generation and authorization should be implemented.

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

### Secrets and environment variables

Shuttle passes secrets to the application at runtime using the `Secrets.toml file`. An example of required secrets is
provided in the `Secrets.example.toml` file.

## Test flow examples

Following are example test flows that can be implemented using the Test VTN and a production VEN to test OpenADR 3.0
functionality and behavior.

### VEN Polling

1. OpenADR 3.0 VEN polls the VTN for events with no active events.
    - Authentication token fetched from the `/auth` endpoint.
    - OpenADR 3.0 VEN polls the VTN for events - `GET /events`.
    - OpenADR 3.0 VEN receives the event list with no active events.
    - OpenADR 3.0 VEN processes the events.
    - Lack of active events results in no action taken by OpenADR 3.0 VEN.
2. OpenADR 3.0 VEN polls the VTN for events with an active event.
    - Virtual Charger set up to charge.
    - Automated test suite generates an event with a start time in the future.
        - `POST /admin/trigger/event` with desired parameters
    - Authentication token fetched from the `/auth` endpoint.
    - OpenADR 3.0 VEN polls the VTN for events - `GET /events`.
    - OpenADR 3.0 VEN receives the event list with an active event.
    - OpenADR 3.0 VEN processes the events.
    - Assets monitored to ensure completion of event.

### VEN Subscription

1. OpenADR 3.0 VEN creates a subscription and receives event. (PUSH method of receiving events)
    - Virtual charger set up to charge.
    - A subscription object is created with desired parameters.
        - Intended to mimic frontend subscription creation.
    - OpenADR 3.0 VEN attempts to register the subscription with the VTN - `POST /subscription`.
    - OpenADR 3.0 VEN receives a successful response.
    - A subscription event is triggered by the VTN - `POST /admin/trigger/subscription/{id}`.
    - OpenADR 3.0 VEN receives the event and processes it.
    - Assets monitored to ensure completion of event.
2. Refreshing access token of an already created subscription
    - Virtual charger set up to charge
    - Create initial subscription in test VTN - `POST /admin/trigger/initial_subscription`
    - OpenADR 3.0 VEN fetches subscription from VTN - `GET /subscription/test`
    - OpenADR 3.0 VEN refreshes the access token of the subscription and updates the VTN record -
      `PUT /subscription/test`
    - New event is triggered by the VTN - `POST /admin/trigger/subscription/{id}`
    - Assets monitored to ensure completion of event.

## Tech

The application is built with [axum](https://github.com/tokio-rs/axum) as the main framework and deployed
with [Shuttle.rs](https://www.shuttle.rs/).
