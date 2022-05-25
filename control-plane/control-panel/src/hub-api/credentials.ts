
const CREDENTIAL_STORAGE_KEY = "CREDENTIALS";

export type Credentials = {
    server: string;
    token: string;
    expires: Date;
}

/**
 * Credentials class is used to maintain and store access tokens for the individual hubs
 * that the control panel can be connected to.
 * 
 * 
 */
class CredentialsStore {
    private credentials: Credentials[] | null = null;

    public async get_saved_credentials() {
        if (this.credentials === null) {
            await this.load_credentials();
        }

        if (this.credentials === null) {
            // Credentials could still be null if local storage is not available.
            return [];
        } else {
            return this.credentials;
        }
    }

    public async add_credentials(credentials: Credentials) {
        if (this.credentials === null) {
            return;
        }

        for (let c of this.credentials) {
            if (c.server === credentials.server) {
                c.token = credentials.token;
                c.expires = credentials.expires;
                await this.store_credentials();
                return;
            }
        }

        this.credentials.push(credentials);
        await this.store_credentials();
    }

    public async remove_credentials(server: string) {
        if (this.credentials === null) {
            return;
        }

        this.credentials = this.credentials.filter((it) => it.server != server);
        await this.store_credentials();
    }

    private async load_credentials(): Promise<void> {
        let THIS = this;
        return new Promise((accept, reject) => {
            try {
                let stored = window.localStorage.getItem(CREDENTIAL_STORAGE_KEY);
                if (stored === null) {
                    THIS.credentials = [];
                } else {
                    THIS.credentials = JSON.parse(stored);
                }
                accept();
            } catch(error) {
                console.warn("Cannot access local storage:", error);
                reject();
            }
        });
    }

    private async store_credentials(): Promise<void> {
        let THIS = this;
        return new Promise((accept, reject) => {
            try {
                window.localStorage.setItem(CREDENTIAL_STORAGE_KEY, JSON.stringify(THIS.credentials));
                accept();
            } catch(error) {
                console.warn("Cannot write to local storage:", error);
                reject();
            }
        })
    }

}

export let CREDENTIALS = new CredentialsStore();
export default CREDENTIALS;