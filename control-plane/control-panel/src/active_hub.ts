import type { Credentials } from "./hub-api/credentials";


class ActiveHub {
    private credentials: Credentials | null = null;

    public is_logged_in() {
        return this.credentials != null;
    }

    public log_in_with(credentials: Credentials) {
        if (this.credentials === null) {
            console.log("Logged in as", credentials);
            this.credentials = credentials;
        } else {
            throw 'Already logged in. Please Logout first.';
        }
    }

    public log_out() {
        this.credentials = null;
    }

}

let ACTIVE_HUB = new ActiveHub();
export default ACTIVE_HUB;