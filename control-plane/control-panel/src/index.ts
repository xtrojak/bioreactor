
import ACTIVE_HUB from './active_hub';
import './index.scss';
import {default as LoginModal} from './ui-components/login-modal/login-modal';

(async () => {

    if (!ACTIVE_HUB.is_logged_in()) {
        await LoginModal.show(true);
    }

})();