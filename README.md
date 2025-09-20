[WIP] auth0-user-gatekeeper
===

> This repository is not provided or in any way affiliated with Auth0.

Admit users to applications through auth0 based on mail address

## Motivation

The auth0-user-gatekeeper is a simple way to admit users to applications based on their email address.

It allows maintaining the allow list for mails and domains outside the auth0 tenant.

## Components

- Server
    - written in Rust
    - distributed as container image
- node-lib
    - written in TypeScript
    - distributed as npm package
- Auth0 action
    - part of your tenant
    - created and maintained by you per tenant

## Usage

Set up is a straightforward as 1-2-3.

- [1. Set up the server](#1-set-up-the-server)
- [2. Set up the auth0 actions](#2-set-up-the-auth0-actions)
- 3\. Enjoy!

### 1. Set up the server

1. Set up the server using docker-compose or any other container orchestration tool
    ```yaml
    services:
      gatekeeper:
        image: timoreymann/auth0-user-gatekeeper:latest
        platform: linux/amd64
        restart: always
        ports:
          - 2025:2025
        volumes:
          - ./config.yaml:/etc/auth0-user-gatekeeper/config.yml
    ```
2. Create the configuration file `config.yaml`
    ```yaml
    token: <token that needs to be provided by the auth0 action>
    # List of allowed domains (can be omitted)
    allowed_domains:
      - your.tld
    # List of allowed mails (can be omitted)
    allowed_mails:
      - your-private@provider.de
    ```
3. Configure a reverse proxy to forward requests to the server with TLS termination
4. Make sure the reverse proxy is reachable from Auth0

### 2. Set up the auth0 actions

You will have to create two actions for each tenant you want to use the gatekeeper.

1. Head over to the [Auth0 dashboard](https://manage.auth0.com/dashboard)

#### 2.1. Create the pre-registration action

1. In the sidebar on the right, navigate to `Actions > Library`
2. In the upper right corner of the page click on `Create Action`
3. Choose `Create custom action`
4. Name: `permit-only-allowed-mails-to-register`
5. Trigger: `Pre User Registration`
6. Click `Create`
7. In the editor on the left side click on the packages icon
8. Click `Add dependency`
9. Name: `auth0-user-gatekeeper`
10. In the code editor add the verify logic
     ```js
     const {UserGateKeeper} = require("auth0-user-gatekeeper");
    
     /**
     * @param {Event} event - Details about the context and user that is attempting to register.
     * @param {PreUserRegistrationAPI} api - Interface whose methods can be used to change the behavior of the signup.
     */
     exports.onExecutePreUserRegistration = async (event, api) => {
       /** @type {UserGateKeeper} */
       const gatekeeper = new UserGateKeeper({
         baseUrl: "https://your-reverse-proxy.tld",
         timeoutMs: 1 * 1_000,
         token: "<token from server config.yml>"
       });
       const {isAllowed, reason} = await gatekeeper.isAllowedEmail(event.user.email);
    
       if(!isAllowed) {
         api.access.deny(reason, "You are not allowed to register.")
       }
     };
     ```

#### 2.2. Create the login action

1. In the sidebar on the right, navigate to `Actions > Library`
2. In the upper right corner of the page click on `Create Action`
3. Choose `Create custom action`
4. Name: `permit-only-allowed-mails-to-login`
5. Trigger: `Login / Post Login`
6. Click `Create`
7. In the editor on the left side click on the packages icon
8. Click `Add dependency`
9. Name: `auth0-user-gatekeeper`
10. In the code editor add the verify logic
    ```js
    const { UserGateKeeper } = require("auth0-user-gatekeeper");
    
    /**
    * @param {Event} event - Details about the context and user that is attempting to register.
    * @param {PostLoginAPI} api - Interface whose methods can be used to change the behavior of the signup.
    */
    exports.onExecutePostLogin = async (event, api) => {
      /** @type {UserGateKeeper} */
      const gatekeeper = new UserGateKeeper({
        baseUrl: "https://your-reverse-proxy.tld",
        timeoutMs: 1 * 1_000,
        token: "<token from server config.yml>"
      });
      const { isAllowed } = await gatekeeper.isAllowedEmail(event.user.email);
    
      if (!isAllowed) {
        api.access.deny("You are not allowed to register.")
      }
    };
    ```

#### 2.3. Integrate the actions into your login flows

1. In the sidebar on the right, navigate to `Actions > Triggers`
2. Under `Signup & Login`, click on `pre-user-registration`
3. On the sidebar on the left click on `permit-only-allowed-mails-to-register`
4. Drag it in the view after `Start`
5. Click `Apply`
6. In the sidebar on the right, navigate to `Actions > Triggers`
7. Choose `post-login`
8. On the sidebar on the left click on `permit-only-allowed-mails-to-login`
9. Drag it in the view after `Start`
10. Click `Apply`
