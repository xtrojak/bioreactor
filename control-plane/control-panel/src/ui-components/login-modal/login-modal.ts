import { Modal }  from 'bootstrap';
import ACTIVE_HUB from '../../active_hub';
import { ApiHost } from '../../hub-api/api_host';
import CREDENTIALS, { Credentials } from '../../hub-api/credentials';

class LoginModal {
    private modal_ui: HTMLElement;
    private address: HTMLInputElement;
    private password: HTMLInputElement;
    private remember: HTMLInputElement;
    private error: HTMLElement;
    private footer: HTMLElement;
    private hub_button_tempalte: HTMLElement;

    private modal: Modal | null = null;

    constructor(id: string) {
        this.modal_ui = document.getElementById(id)!;
        this.address = this.modal_ui.querySelector("#hub-address")!;
        this.password = this.modal_ui.querySelector("#hub-password")!;
        this.remember = this.modal_ui.querySelector("#hub-remember")!;
        this.error = this.modal_ui.querySelector("#login-form-error")!;
        this.footer = this.modal_ui.querySelector('.modal-footer')!;
        this.hub_button_tempalte = this.modal_ui.querySelector('#template-host-button')!;

        let login_modal = this;
        let form = this.modal_ui.querySelector('#login-form')!;
        form.addEventListener("submit", (e) => {
            if (e.preventDefault) e.preventDefault();
            login_modal.send_login();
            return false;
        });
    }

    /**
     * Show the modal, populating the list of previously connected hubs with known
     * servers.
     * 
     * @param enforce If true, the modal cannot be dismissed (use when login is required to continue).
     */
    public async show(enforce: boolean = true) {
        if (enforce) {
            // When enforce is true, the modal is not cancellable.
            this.modal_ui.dataset['bsBackdrop'] = 'static';
            this.modal_ui.dataset['bsKeyboard'] = 'false';
        } else {
            this.modal_ui.dataset['bsBackdrop'] = 'true';
            this.modal_ui.dataset['bsKeyboard'] = 'true';
        }

        let known_hosts = await CREDENTIALS.get_saved_credentials();
        
        // If there are no known hosts, hide the list completely.
        this.footer.classList.toggle('d-none', known_hosts.length === 0);

        // Remove everything from footer except the initial message.
        while (this.footer.childElementCount > 1) {
            this.footer.removeChild(this.footer.lastChild!);
        }
        
        // Then, run through the possible hosts and add each one as a new button.
        let login_modal = this;
        for (let host of known_hosts) {
            let button = this.hub_button_tempalte.cloneNode(true) as HTMLElement;
            let address = button.querySelector(".hub-address")!;
            address.textContent = host.server;

            // Submit the host when the main button is clicked.
            button  .querySelector('.btn-submit')!
                    .addEventListener('click', async () => {
                        // If the host is expired, we cannot log in automatically,
                        // but we can at least pre-fill it into the form.
                        if (host.expires < new Date()) {
                            login_modal.address.value = host.server;
                            login_modal.password.focus();
                        } else {
                            login_modal.send_renew(host);
                        }
                    });

            // Delete the host from storage when the delete button is clicked.
            button  .querySelector('.btn-delete')!
                    .addEventListener('click', () => {
                        CREDENTIALS.remove_credentials(host.server);
                        let button = login_modal.footer.querySelector(`[data-server="${host.server}"]`);
                        login_modal.footer.removeChild(button!);
                        let is_empty = login_modal.footer.childElementCount == 1;
                        login_modal.footer.classList.toggle('d-none', is_empty);
                    });

            button.dataset['server'] = host.server;
            button.classList.remove('d-none');
            button.id = "";
            this.footer.appendChild(button);
        }

        this.modal = new Modal(this.modal_ui);
        this.modal.show();
    }

    /**
     * If a know hub server is selected, this method checks that the server is
     * online by renewing its existing token.
     * 
     * @param host Existing hub server to which we want to connect.
     */
    async send_renew(host: Credentials) {
        try {
            // Hide previous error once new login request is dispatched.
            this.error.classList.toggle('d-none', true);

            let server = new ApiHost(host.server, host.token);
            let renewed = await server.renew();

            // Update the saved credentials with a new token.
            CREDENTIALS.add_credentials(renewed);

            this.login_success(renewed);
        } catch(e: any) {
            this.show_error(e);
        }
    }

    /**
     * If the user fills out a form, try to connect to the provided server.
     */
    async send_login() {
        try {
            // Hide previous error once new login request is dispatched.
            this.error.classList.toggle('d-none', true);

            console.log("Try log in.");
            // Create an API host with the desired server address and try to log in.
            let host = new ApiHost(this.address.value);
            let response = await host.login(this.password.value);

            // If login is successful, save credentials if desired.
            if (this.remember.checked) {
                CREDENTIALS.add_credentials(response);
            } else {
                CREDENTIALS.remove_credentials(response.server);
            }
        
            // Report login success.
            this.login_success(response);
        } catch (e: any) {
            this.show_error(e);
        }
    }

    /**
     * If either attempt fails (`send_renew` or `send_login`), display an error message
     * in the main modal window.
     * 
     * @param e Error message.
     */
    show_error(e: any) {
        this.error.classList.toggle('d-none', false);
        this.error.innerHTML = `${e} Please check that your credentials are valid and try again.`;
    }

    /**
     * If either attempt succeeds, update the credentials stored by the `ACTIVE_HUB` and hide the modal.
     * 
     * @param credentials Credentials for the newly verified hub server.
     */
    login_success(credentials: Credentials) {
        if (ACTIVE_HUB.is_logged_in()) {
            ACTIVE_HUB.log_out();
        }

        ACTIVE_HUB.log_in_with(credentials);
        this.modal?.hide();
    }

}

let LOGIN_MODAL = new LoginModal('login-modal');
export default LOGIN_MODAL;