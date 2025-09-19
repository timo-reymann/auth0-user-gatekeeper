import { describe, it, expect, Mock, vi, beforeEach, afterEach } from "vitest";
import fetch from "node-fetch";
import { UserGateKeeper, type Configuration } from "../src";

// Mock node-fetch
vi.mock("node-fetch", () => {
    return {
        default: vi.fn(),
    };
});

const mockedFetch = fetch as unknown as Mock;

describe("UserGateKeeper.isAllowedEmail", () => {
    const config: Configuration = {
        baseUrl: "https://api.example.com",
        timeoutMs: 5000,
        token: "test-token",
    };

    let gatekeeper: UserGateKeeper;

    beforeEach(() => {
        gatekeeper = new UserGateKeeper(config);
        mockedFetch.mockReset();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    it("sends POST to /isAllowed with correct headers and body", async () => {
        const responseBody = "email_allowed";
        mockedFetch.mockResolvedValueOnce({
            status: 200,
            text: vi.fn().mockResolvedValue(responseBody),
        });

        const email = "user@example.com";
        const result = await gatekeeper.isAllowedEmail(email);

        // Assertions about fetch call
        expect(mockedFetch).toHaveBeenCalledTimes(1);
        const callArgs = mockedFetch.mock.calls[0];
        const calledUrl = new URL(callArgs[0].toString());

        expect(calledUrl.origin + calledUrl.pathname).toBe("https://api.example.com/isAllowed");

        const options = callArgs[1] as RequestInit & {
            headers?: Record<string, string>;
            body?: string;
            method?: string;
        };

        expect(options.method).toBe("POST");
        expect(options.headers).toMatchObject({
            "Content-Type": "application/json",
            Authorization: `Bearer ${config.token}`,
        });
        expect(options.body).toBe(JSON.stringify({ email }));

        // Result mapping
        expect(result).toEqual({
            isAllowed: true,
            reason: responseBody,
        });
    });

    it("returns isAllowed=false and reason with non-200 status", async () => {
        const responseBody = "not_allowed";
        mockedFetch.mockResolvedValueOnce({
            status: 403,
            text: vi.fn().mockResolvedValue(responseBody),
        });

        const result = await gatekeeper.isAllowedEmail("blocked@example.com");

        expect(result).toEqual({
            isAllowed: false,
            reason: responseBody,
        });
    });

    it("propagates fetch rejections", async () => {
        const error = new Error("network error");
        mockedFetch.mockRejectedValueOnce(error);

        await expect(gatekeeper.isAllowedEmail("x@y.z")).rejects.toThrow("network error");
    });

    it("builds URL by replacing pathname to /isAllowed", async () => {
        const localConfig: Configuration = {
            ...config,
            baseUrl: "https://api.example.com/v1/anything",
        };
        const localGatekeeper = new UserGateKeeper(localConfig);

        mockedFetch.mockResolvedValueOnce({
            status: 200,
            text: vi.fn().mockResolvedValue("email_allowed"),
        });

        await localGatekeeper.isAllowedEmail("user@example.com");

        const callArgs = mockedFetch.mock.calls[0];
        const calledUrl = new URL(callArgs[0].toString());
        expect(calledUrl.origin).toBe("https://api.example.com");
        expect(calledUrl.pathname).toBe("/isAllowed");
    });
});
