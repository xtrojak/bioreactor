# Hub Protocol

In this part of the repository, we describe the internal REST protocol for communication between bioreactor *hub* and other more high-level devices.

### Motivation

Bioreactor control plane consists of three main layers: low-level *driver*, intermediary *hub*, and a high-level *control panel*. These may be fully separate, or may be integrated into a single device, depending on the actual architecture. The main purpose of this distinction is to provide a clear separation of concerns.

The *hub protocol* describes the API provided by the *hub* that is used by the *control panel* or other high-level automation and data collection tools. Architecturally, the frontend protocol should allow anyone to connect to a *hub* over a public network, observe the state of the connected bioreactors and make changes to their instruments.

## Security

The initial design assumes that there is a single *master password* (or a collection of master passwords) that allow full access to the hub's functionality. In other words, we do not concern ourselves with user accounts or granular access rights within a single hub. In the future, this could be further refined to properly support different users, but at the moment, it is mostly orthogonal to our primary goal (i.e. a *working* bioreactor).

However, there are several security measures that we nevertheless enforce:

 1. A user must authenticate through a `/login` API endpoint. After successful authentication, the user receives a time-limited token that is used for any further communication with the API instead of the master password---the master password must not be saved on the user's device.
 2. If a user wants to remain "logged in" indefinitely, the token is still time-limited. The client can however exchange the token for a new one using `/renew` (with a renewed expiration date) at any moment. This effectively leads to a system where, as long as the user is repeatedly using the API, the token is renewed periodically. An automatic logout occurs after a certain period of inactivity when the token is not renewed (e.g. a month).
 3. The token design should allow a token to survive situations like a hub restart or a migration (i.e. tokens are not *stored* by the hub, they are verified cryptographically). However, a hub reserves the right to revoke its tokens at any time, at which point a new login must be performed.
 4. Finally, when running a hub on a public network, **always use HTTPS** to avoid leaking the master password.

#### Token design

Here, we outline the recommended principles for token generation. The low-level settings (the chosen hashing algorithm, length of the token, etc.) are up to the specific hub implementation and may not even be public. The client only needs to by able to store and reproduce the token, not to manipulate it in any way. 

> In the future, we may switch to some standard token system (e.g. JWT), but at the moment, this custom scheme appears to be sufficient with much less attached complexity.

As part of its configuration, a hub receives one or more *user passwords* (e.g. `apple`) and a single *server password* (e.g. `snake`). Of course, all typical recommendations for selecting secure passwords apply. 

**Token generation:** When requesting a token, user attaches one of the user passwords (e.g. `apple`) to the request. If the user password is correct, the hub takes the timestamp until which the token is valid (e.g. `123456789`), concatenates it with the server password (e.g. `123456789;snake`) and produces a hash of this string (e.g. `4af40cb89`). Subsequently, this hash is again concatenated with the timestamp (e.g. `123456789;4af40cb89`). This is the returned token. It is either provided as is, or in some standard encoding (e.g. `base64`). The expiration timestamp should be also explicitly provided along with the token. 

The advantage of additional token encoding is that it can potentially detect or repair small errors during transmission. But most communication channels already do this on a lower-level, so at this point it is mostly an obfuscation step. Furthermore, the hub is free to attach any additional metadata aside from timestamp to the token. This can be later used to implement, for example, permissions or user accounts.

**Token verification:** Once the client has a valid token, it must be attached to every API request. The hub then decodes the token, extracts the hash (e.g. `4af40cb89`) and the timestamp (e.g. `123456789`). If the token is stale, the request is automatically rejected. It then computes the same hash using the server password as when the token was generated (e.g. `4af40cb89 = hash(123456789;snake)`). If the hash matches the hash in the token, the request is valid, otherwise it is rejected.

Consequently, the security of this approach lies in the security of the hash function, and the security of the server password. If both the hash function and the password are robust, it is not possible to retrieve the server password from a token. It is also not possible to generate a new valid token from an expired one, as this requires a new hash. 

As with any hash, the longer, the better; `SHA3-256` or `SHA3-512` are the recommended choices. Also, since the server password does not need to be actually typed in or remembered by anyone, around 512 random characters is a good baseline. If the tokens do not need to survive hub restarts, the server password can be even generated randomly (as opposed to being part of the configuration).

## Architecture

Here, we define some general terms that are useful for understanding the actual API documentation:

 - **Device:** Every hub can manage multiple bioreactors, such that each reactor possibly has different capabilities. In the future, we may even support device groups or some other hierarchy. But for now, we generally talk about singular devices. Each device generally has a unique string identifier.
 - **Instrument:** Every bioreactor (a device) can have multiple instruments. Some instruments only provide "outputs", some only provide "inputs", and some may provide both. Each instrument also has a string identifier (unique within the particular device) and a string type (in the future, maybe even multiple types, but that is debateable). The possible instrument types will be described in this documentation (and the client can ignore instruments of unknown type).
 - **Readout:** Each instrument type should describe one or more readouts. Every readout describes a single value (or a collection of inseparable values) that is part of the instrument. Some readouts are read-only, some are modifiable, in which case they are also called controls. However, note that changes to controls do not have to produce an immediate effect. Therefore, user should always be presented with the actual readouts, not the inputs that they provided. Instruments that take a long time to reach the desired state may even provide a "target" and "actual" value as part of their readout.

To keep the whole situation as simple as possible, the hub API is then split along this hierarchy. Everything is communicated using JSON.

 - `/hub/`: Provides info about the whole hub. In particular, the list of devices and a short summary of instruments available on each device.
 - `/hub/<device-id>/`: Provides detailed info about a particular device, including instruments and a short summary of their readouts.
 - `/hub/<device-id>/<instrument-id>/`: Provides detailed info about a particular instrument, including a summary of its readouts.
 - `/hub/<device-id>/<instrument-id>/<readout-id>/`: Provides a detailed info about a particular readout.

> The `hub` prefix is used to avoid collisions with other future functionality, such as experiment replay or authentication/management.

Here, the definition of "short summary" and "detailed info" is standardized for the whole hub and individual devices (see individual documents in this specification). But for the instrument and readout, it is generally dependent on their function. This is again standardized in this specification, just note that very different information can be provided by instruments of different type. 

As an example, an instrument summary probably only lists its ID and type, maybe with one representative or summary readout value. A more detailed info likely includes all readouts and their current values. Finally, individual readouts can include stuff like measurement error and precision, target vs. current value, and so on.

A general rule is that `GET` requests to a particular URL read the information, and `POST` request to the same URL can submit a JSON with new values of modifiable fields (the JSON only includes values that should change). Such modifying requests should ideally succeed in full, or fail without achieving any changes. Also, the returned value is the same JSON as in the `GET` request, summarising the new state of the instrument/readout after the change.

As suggested, the JSON format (including what values are read-only, and what values are user modifiable) is part in this folder in a separate set of files.

**Experiments and logging:** Finally, the hub is generally also expected to periodically log the device status and the changes requested through the API. At the moment, the API does not expose this, but it will definitely be part of the future revisions. This will probably include access to raw logs, and also some user controllable "experiment" sessions, where logs can be grouped or filtered based on time and instruments. Also, logging period for different instruments should be adjustable.

**Triggers and scripting:** In the future, the API will also most likely include some capacity to define hooks and scripts that should be executed due to some basic triggers. For example, a user may be able to define a profile that controls the intensity of the lighting to simulate day/night transitions. Similarly, a trigger based on optical density can be used to automatically reduce the population in a reactor by replicing some of the contents with fresh medium.