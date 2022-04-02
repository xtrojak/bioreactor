# General Info and Authentication

## General API rules

 1. Every request and response should be valid JSON (some exceptions may apply to errors).
 2. Failed requests must not end with HTTP code 200, but use an appropriate error code.
 3. \[Optional, but recommended\] Failed requests should return a single JSON object with a `message` key stating a human readable explanation for the error.
 4. Every endpoint (except for `/` and `/login`) is only accessible with the `Authorization` header set to a valid token issued previously using `/login` or `/renew`.

## \[GET\] `/` (root)

This endpoint (probably the only one) does not require authentication. It outputs general information about the hub server. At the moment, this includes `version`, `name` and optionally `description`:

```json
{
	"name": "Test Server",
	"description": "Example hub server showing basic functionality.",
	"version": "0.1.0"
}
```

Probably the most critical value here is the `version`. The protocol will surely evolve, and even though most changes should be non-breaking (like adding new instruments or enhacing existing ones), the clients should be able to check that the server is online and is runnign a supported version of the protocol.

Less importantly, `name` and `description` can be shown in the user interface to esily identify the server (as opposed to its IP) if the user is connected to multiple servers.

In the future, this will probably include additional information (e.g. distinguish server version and protocol version).

## \[POST\] `/login`

Allows anyone to obtain a new authentication token using a valid password. Note that the token expires after some server-defined amount of time. However, this amount of time should generally be reasonable to cover a single user session (e.g. at least one day).

Request body:

```json
{ "password": "USER_PASSWORD" }
```

Response:

```json
{
	"token": "AUTHENTICATION_TOKEN",
	"expires": "ISO_8601_UCT_TIMESTAMP"
}
```

The obtained token can be used directly as the value of the `Authorization` header in all subsequent requests.

## \[GET\] `/renew`

Has no input except for the `Authorization` header, which proves that the user is authenticated. The response is the same JSON object as for `/login`, but a new token with updated expiration time is returned for each request.
