import type { Credentials } from "./credentials";

const ENDPOINT_LOGIN: string = "/login";
const ENDPOINT_RENEW: string = "/renew";

export class ApiHost {
    private base_url: string;
    private token: string | null;

    constructor(base: string, token: string | null = null) {
        this.base_url = base;
        // If token is null, this api host will be very limited - most requests will fail.
        // But login should still work...
        this.token = token;
    }

    public async login(password: string): Promise<Credentials> {
        let response = await this.request("POST", ENDPOINT_LOGIN, { 'password': password });
        return {
            server: this.base_url,
            token: response.token,
            expires: new Date(response.expires),
        }
    }

    public async renew(): Promise<Credentials> {
        let response = await this.request("GET", ENDPOINT_RENEW);
        return {
            server: this.base_url,
            token: response.token,
            expires: new Date(response.expires),
        }
    }

    private async request(method: string, path: string, body: any | undefined = undefined): Promise<any> {
        let base_url = this.base_url;
        let token = this.token;
        return new Promise((ok, error) => {
            let request = new XMLHttpRequest();
            request.open(method, base_url + path, true);
            request.setRequestHeader("Content-Type", "application/json");
            request.setRequestHeader("Accept", "application/json");
            if (token !== null) {
                request.setRequestHeader("Authorization", token);
            }

            request.onload = function () {
                console.log("on load");
                if (request.readyState === XMLHttpRequest.DONE && request.status === 200) {
                    let response = null;
                    try {
                        response = JSON.parse(request.responseText);
                    } catch (e) {
                        console.warn("Invalid request response.");
                        error(e);
                    }
                    if (response !== null) {
                        ok(response);
                    }
                } else {
                    // The request went through, but the response code is not ok.
                    try {
                        let response = JSON.parse(request.responseText);
                        error(`${request.statusText}: ${response['message']}`);
                    } catch(e) {
                        if (request.status === null) {
                            error(`Host is unreachable.`);
                        } else {
                            error(request.statusText);
                        }
                    }
                }
            };

            request.ontimeout = function () {
                console.log(request.status);
                if (request.status === 0) {
                    error(`Host is unreachable.`);
                } else {
                    error(request.statusText);
                }
            };

            request.onerror = function () {
                console.log(request.status);
                if (request.status === 0) {
                    error(`Host is unreachable.`);
                } else {
                    error(request.statusText);
                }
            };

            if (body !== undefined) {
                request.send(JSON.stringify(body));
            } else {
                request.send(null);
            }
        })
    }

}