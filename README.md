# Timetable

A work-in-progress web service, allows you to schedule webhook executions.

## Motivation

In my spare time, I'm an admin of a meme group based out of the University of Utah, [UofU Edgy Memes for Salty Skis on Facebook](https://www.facebook.com/groups/uuemss/). While doing this I wanted to start expanding from the Facebook group, and establish a presence on a [Facebook page](https://www.facebook.com/uuemss), [Twitter page](https://twitter.com/uuemss), and [Instagram page](https://www.instagram.com/uuemss/) using the content created in our Facebook group. But, I am a busy college student and posting featured posts on those three accounts sounds _incredibly_ tedious and unnecessarily time-consuming. So the obvious solution here is post scheduling.

Solutions for post scheduling do already exist. [Buffer](https://buffer.com) and [HootSuite](https://hootsuite.com) are seemingly the two leading the field right now. However for my use case of removing grunt work from something I'm doing as a hobby with a few friends these come with some disadvantages, most notably pricing and feature set restriction. After scheduling 10 posts on Buffer's free individual plan it became apparent that there could be a better way to do this.

After thinking about how I could do it better and why the existing solutions charge so much for what seems like very basic functionality a few things occurred to me. I could isolate posting behind [serverless functions](https://en.wikipedia.org/wiki/Serverless_computing), this allows the individual functions to house the specific APIs required for each service while keeping my webhook-level APIs consistent across the different services. As well, using serverless functions means reduced costs because the functions will only need to run in short bursts when they're scheduled to post.

As well it occurred to me during this thought exercise that I couldn't think of a good way to schedule one-time webhook executions. That's where this project, timetable, comes in.

## Technical Choices

- [Rust](https://www.rust-lang.org) - the safe and performant systems language, perfect for what will likely become a piece of software sensitive to timing constraints
- [Diesel](https://diesel.rs/) - fully loaded data-handling library for Rust applications
- [PostgreSQL](https://www.postgresql.org/) - database to hold scheduled webhook executions, came recommended by Diesel's documentation
- [Actix](https://actix.rs) - coming from node.js and an express.js background, `actix-web` seemed like the perfect library for setting up a webserver

## Planned

I've thought a bit about how I want to implement this, below is the high-level API and data shapes I have in mind for the project.

### Data

#### Request Payload

Payload used to schedule a webhook execution. The payload should a JSON object with the following fields:

| Field Name           | Type      | Description                                                                                                                                                               |
| -------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scheduledTimestamp` | `string`  | A timestamp of the desired time of execution                                                                                                                              |
| `targetWebhook`      | `string`  | URL Timetable should send a an HTTP request to at the scheduled time                                                                                                      |
| `responseWebhook`    | `string?` | (**OPTIONAL**) URL Timetable should send target's response to. This allow services to schedule an execution and know if the request was successful after it was requested |

Example payload:

```json
{
  "scheduledTimestamp": "",
  "targetWebhook": "",
  "responseWebhook": ""
}
```

#### Execution Status Object

JSON Object containing the status of a scheduled webhook exeuction, it should contain the following fields:

| Field Name           | Type                                   | Description                                                                      |
| -------------------- | -------------------------------------- | -------------------------------------------------------------------------------- |
| `id`                 | `string`                               | The ID of the scheduled webhook execution this object is reporting the status of |
| `scheduledTimestamp` | `string`                               | The timestamp this scheduled execution will execute or was executed at           |
| `targetWebhook`      | `string`                               | The target webhook this scheduled executed will/did hit when executed            |
| `responseWebook`     | `string`                               | The webhook timetable will send/sent the target webhook's response data to.      |
| `status`             | `"waiting" | "executed" | "cancelled"` | The execution status of the scheduled webhook execution                          |

Example object:

```json
{
  "id": "42",
  "scheduledTimestamp": "",
  "targetWebhook": "",
  "responseWebhook": "",
  "status": "waiting"
}
```

#### Scheduled Execution Object

This is the JSON object Timetable will use in response to requests. It should have the following fields:

| Field Name           | Type      | Description                                                                                                                                                                                                                                 |
| -------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`                 | `string`  | This is the ID of the scheduled execution, and will allow services to check the status of the execution, modify it, and delete it from the timetable.                                                                                       |
| `scheduledTimestamp` | `string`  | This is the scheduled time of execution. Returning it allows services to verify their execution was scheduled correctly                                                                                                                     |
| `targetWebhook`      | `string`  | This is the target webhook the execution will request at the scheduled timestamp. Returning it allows services to verify their scheduled execution will hit the corrent endpoint.                                                           |
| `responseWebhook`    | `string?` | This is the webhook timetable will hit after requesting the target webook, and will return the target's response to. Returning it allows services to verify their scheduled execution will return data correctly, after scheduled execution |

Example response:

```json
{
  "id": "42",
  "scheduledTimestamp": "",
  "targetWebhook": "",
  "responseWebhook": ""
}
```

#### Execution Response Object

After requesting the target webhook at the scheduled time, timetable will send the response from the target webhook to a response webhook if provided. It will provide services with a JSON object with the following fields:

| Field Name           | Type     | Description                                         |
| -------------------- | -------- | --------------------------------------------------- |
| `id`                 | `string` | The id of the execution that ran.                   |
| `scheduledTimestamp` | `string` | The timestamp the execution was run at.             |
| `targetWebook`       | `string` | The webhook the execution hit.                      |
| `status`             | `number` | The HTTP Status from the target webhook's response. |
| `headers`            | `object` | The HTTP Headers from the target webhook's reponse. |
| `body`               | `body`   | The body of the target webhook's response.          |

Example response:

```json
{
  "id": "42",
  "scheduledTimestamp": "",
  "targetWebhook": "",
  "status": 200,
  "headers": {},
  "body": ""
}
```

### APIs

#### Schedule a Webhook Execution

To schedule a webhook execution send a `POST` request to the `/schedule` enpoint with a [request payload](#request-payload).

`POST /schedule`

#### Delete a Scheduled Execution

To delete a scheduled webhook execution send a `DELETE` request to the `/schedule/{id}` endpoint, with `{id}` replaced with the ID of the scheduled execution you which to delete.

`DELETE /schedule/{id}`

#### Query a Scheduled Execution's Status

To get the status of a scheduled webhook execution send a `GET` request to the `/schedule/{id}` endpoint, with `{id}` replaced with the ID of the execution you wish to get the status of. You will recieve an [execution status object](#execution-status-object).

`GET /schedule/{id}`

#### Modify a Scheduled Execution

To modify the details of an already scheduled execution send a `PUT` request to the `/schedule/{id}`, with `{id}` replaced with the ID of the execution you wish to modify and a [request payload](#request-payload) with your modified details.

`PUT /schedule/{id}`

## Ideas for the Future

- Ability to simply defer a webhook's execution by an amount of time.
- Cron for webhooks. This functionality already exists in abundance in other services and solutions. But it might make sense to include it in this service if it fits the need and codebase.
