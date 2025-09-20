auth0-user-gatekeeper
===
[![GitHub Release](https://img.shields.io/github/v/tag/timo-reymann/auth0-user-gatekeeper?label=version)](https://github.com/timo-reymann/auth0-user-gatekeeper/releases)
[![Docker Pulls](https://img.shields.io/docker/pulls/timoreymann/auth0-user-gatekeeper?style=flat)](https://hub.docker.com/r/timoreymann/auth0-user-gatekeeper)
[![LICENSE](https://img.shields.io/github/license/timo-reymann/auth0-user-gatekeeper)](https://github.com/timo-reymann/auth0-user-gatekeeper/blob/main/LICENSE)
[![CircleCI](https://circleci.com/gh/timo-reymann/auth0-user-gatekeeper.svg?style=shield)](https://app.circleci.com/pipelines/github/timo-reymann/auth0-user-gatekeeper)
[![Renovate](https://img.shields.io/badge/renovate-enabled-green?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAzNjkgMzY5Ij48Y2lyY2xlIGN4PSIxODkuOSIgY3k9IjE5MC4yIiByPSIxODQuNSIgZmlsbD0iI2ZmZTQyZSIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoLTUgLTYpIi8+PHBhdGggZmlsbD0iIzhiYjViNSIgZD0iTTI1MSAyNTZsLTM4LTM4YTE3IDE3IDAgMDEwLTI0bDU2LTU2YzItMiAyLTYgMC03bC0yMC0yMWE1IDUgMCAwMC03IDBsLTEzIDEyLTktOCAxMy0xM2ExNyAxNyAwIDAxMjQgMGwyMSAyMWM3IDcgNyAxNyAwIDI0bC01NiA1N2E1IDUgMCAwMDAgN2wzOCAzOHoiLz48cGF0aCBmaWxsPSIjZDk1NjEyIiBkPSJNMzAwIDI4OGwtOCA4Yy00IDQtMTEgNC0xNiAwbC00Ni00NmMtNS01LTUtMTIgMC0xNmw4LThjNC00IDExLTQgMTUgMGw0NyA0N2M0IDQgNCAxMSAwIDE1eiIvPjxwYXRoIGZpbGw9IiMyNGJmYmUiIGQ9Ik04MSAxODVsMTgtMTggMTggMTgtMTggMTh6Ii8+PHBhdGggZmlsbD0iIzI1YzRjMyIgZD0iTTIyMCAxMDBsMjMgMjNjNCA0IDQgMTEgMCAxNkwxNDIgMjQwYy00IDQtMTEgNC0xNSAwbC0yNC0yNGMtNC00LTQtMTEgMC0xNWwxMDEtMTAxYzUtNSAxMi01IDE2IDB6Ii8+PHBhdGggZmlsbD0iIzFkZGVkZCIgZD0iTTk5IDE2N2wxOC0xOCAxOCAxOC0xOCAxOHoiLz48cGF0aCBmaWxsPSIjMDBhZmIzIiBkPSJNMjMwIDExMGwxMyAxM2M0IDQgNCAxMSAwIDE2TDE0MiAyNDBjLTQgNC0xMSA0LTE1IDBsLTEzLTEzYzQgNCAxMSA0IDE1IDBsMTAxLTEwMWM1LTUgNS0xMSAwLTE2eiIvPjxwYXRoIGZpbGw9IiMyNGJmYmUiIGQ9Ik0xMTYgMTQ5bDE4LTE4IDE4IDE4LTE4IDE4eiIvPjxwYXRoIGZpbGw9IiMxZGRlZGQiIGQ9Ik0xMzQgMTMxbDE4LTE4IDE4IDE4LTE4IDE4eiIvPjxwYXRoIGZpbGw9IiMxYmNmY2UiIGQ9Ik0xNTIgMTEzbDE4LTE4IDE4IDE4LTE4IDE4eiIvPjxwYXRoIGZpbGw9IiMyNGJmYmUiIGQ9Ik0xNzAgOTVsMTgtMTggMTggMTgtMTggMTh6Ii8+PHBhdGggZmlsbD0iIzFiY2ZjZSIgZD0iTTYzIDE2N2wxOC0xOCAxOCAxOC0xOCAxOHpNOTggMTMxbDE4LTE4IDE4IDE4LTE4IDE4eiIvPjxwYXRoIGZpbGw9IiMzNGVkZWIiIGQ9Ik0xMzQgOTVsMTgtMTggMTggMTgtMTggMTh6Ii8+PHBhdGggZmlsbD0iIzFiY2ZjZSIgZD0iTTE1MyA3OGwxOC0xOCAxOCAxOC0xOCAxOHoiLz48cGF0aCBmaWxsPSIjMzRlZGViIiBkPSJNODAgMTEzbDE4LTE3IDE4IDE3LTE4IDE4ek0xMzUgNjBsMTgtMTggMTggMTgtMTggMTh6Ii8+PHBhdGggZmlsbD0iIzk4ZWRlYiIgZD0iTTI3IDEzMWwxOC0xOCAxOCAxOC0xOCAxOHoiLz48cGF0aCBmaWxsPSIjYjUzZTAyIiBkPSJNMjg1IDI1OGw3IDdjNCA0IDQgMTEgMCAxNWwtOCA4Yy00IDQtMTEgNC0xNiAwbC02LTdjNCA1IDExIDUgMTUgMGw4LTdjNC01IDQtMTIgMC0xNnoiLz48cGF0aCBmaWxsPSIjOThlZGViIiBkPSJNODEgNzhsMTgtMTggMTggMTgtMTggMTh6Ii8+PHBhdGggZmlsbD0iIzAwYTNhMiIgZD0iTTIzNSAxMTVsOCA4YzQgNCA0IDExIDAgMTZMMTQyIDI0MGMtNCA0LTExIDQtMTUgMGwtOS05YzUgNSAxMiA1IDE2IDBsMTAxLTEwMWM0LTQgNC0xMSAwLTE1eiIvPjxwYXRoIGZpbGw9IiMzOWQ5ZDgiIGQ9Ik0yMjggMTA4bC04LThjLTQtNS0xMS01LTE2IDBMMTAzIDIwMWMtNCA0LTQgMTEgMCAxNWw4IDhjLTQtNC00LTExIDAtMTVsMTAxLTEwMWM1LTQgMTItNCAxNiAweiIvPjxwYXRoIGZpbGw9IiNhMzM5MDQiIGQ9Ik0yOTEgMjY0bDggOGM0IDQgNCAxMSAwIDE2bC04IDdjLTQgNS0xMSA1LTE1IDBsLTktOGM1IDUgMTIgNSAxNiAwbDgtOGM0LTQgNC0xMSAwLTE1eiIvPjxwYXRoIGZpbGw9IiNlYjZlMmQiIGQ9Ik0yNjAgMjMzbC00LTRjLTYtNi0xNy02LTIzIDAtNyA3LTcgMTcgMCAyNGw0IDRjLTQtNS00LTExIDAtMTZsOC04YzQtNCAxMS00IDE1IDB6Ii8+PHBhdGggZmlsbD0iIzEzYWNiZCIgZD0iTTEzNCAyNDhjLTQgMC04LTItMTEtNWwtMjMtMjNhMTYgMTYgMCAwMTAtMjNMMjAxIDk2YTE2IDE2IDAgMDEyMiAwbDI0IDI0YzYgNiA2IDE2IDAgMjJMMTQ2IDI0M2MtMyAzLTcgNS0xMiA1em03OC0xNDdsLTQgMi0xMDEgMTAxYTYgNiAwIDAwMCA5bDIzIDIzYTYgNiAwIDAwOSAwbDEwMS0xMDFhNiA2IDAgMDAwLTlsLTI0LTIzLTQtMnoiLz48cGF0aCBmaWxsPSIjYmY0NDA0IiBkPSJNMjg0IDMwNGMtNCAwLTgtMS0xMS00bC00Ny00N2MtNi02LTYtMTYgMC0yMmw4LThjNi02IDE2LTYgMjIgMGw0NyA0NmM2IDcgNiAxNyAwIDIzbC04IDhjLTMgMy03IDQtMTEgNHptLTM5LTc2Yy0xIDAtMyAwLTQgMmwtOCA3Yy0yIDMtMiA3IDAgOWw0NyA0N2E2IDYgMCAwMDkgMGw3LThjMy0yIDMtNiAwLTlsLTQ2LTQ2Yy0yLTItMy0yLTUtMnoiLz48L3N2Zz4=)](https://renovatebot.com)
[![codecov](https://codecov.io/gh/timo-reymann/auth0-user-gatekeeper/graph/badge.svg?token=9oxnYDeJHI)](https://codecov.io/gh/timo-reymann/auth0-user-gatekeeper)

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
      if(!event.user.email_verified) {
        api.access.deny("E-Mail not verified")
        return
      }
    
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
 
