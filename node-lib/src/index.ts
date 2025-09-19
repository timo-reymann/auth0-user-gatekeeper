import fetch from "node-fetch";

/**
 * Configuration for the client.
 */
export interface Configuration {
    /**
     * The base URL of the API.
     */
    baseUrl: string;
    /**
     * The timeout in milliseconds for requests.
     */
    timeoutMs: number;
    /**
     * The token to use for authentication.
     */
    token: string;
}

/**
 * The status of the mail allowance.
 */
export interface MailAllowanceStatus {
    /**
     * Whether the email is allowed to register and login.
     */
    isAllowed: boolean;
    /**
     * The reason for the status.
     */
    reason: string | "no_token" | "invalid_token" | "invalid_email_format" | "email_allowed" | "domain_allowed" | "not_allowed";
}

export class UserGateKeeper {
    constructor(private readonly configuration: Configuration) {
    }

    public async isAllowedEmail(email: string): Promise<MailAllowanceStatus> {
        const url = new URL(this.configuration.baseUrl);
        url.pathname = "/isAllowed"
        const response = await fetch(url, {
            method: "POST",
            body: JSON.stringify({
                "email": email,
            }),
            headers: {
                "Content-Type": "application/json",
                'Authorization': `Bearer ${this.configuration.token}`,
            }
        });
        const responseBody = await response.text();

        return {
            isAllowed: response.status === 200,
            reason: responseBody,
        };
    }
}
