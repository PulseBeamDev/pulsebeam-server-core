# PulseBeam CLI

Generates secure tokens for client-side applications. For example, clients using the `@pulsebeam/peer` WebRTC Peer-to-Peer Communication SDK.

These tokens enable secure interaction with PulseBeam signaling servers for peer-to-peer connections.

For a server-side token-generation alternative and more documentation, see [`@pulsebeam/server`](https://jsr.io/@pulsebeam/server).

## Quick Start

```bash
cargo build
./target/debug/pulsebeam-cli \
    --api-key "your_api_key" \
    --api-secret "your_api_secret" \
    create-token \
    --peer-id "peer-1" \
    --group-id "test" \
    --allow-policy "test:peer-*"

./target/debug/pulsebeam-cli \
    --api-key "your_api_key" \
    --api-secret "your_api_secret" \
    create-token \
    --peer-id "peer-2" \
    --group-id "test" \
    --allow-policy "test:peer-*"
```

Now you should have two tokens. You can use the tokens to connect the two peers together using PulseBeam. They are valid for the next hour.

## Installation

1. Make sure you have Rust and Cargo installed: https://rustup.rs/
2. Clone this repository.
3. Navigate to the project directory in your terminal.
4. Build the project: `cargo build`

## Usage

```bash
./target/debug/pulsebeam-cli create-token --help
```
| Parameter     | Description                                                                                           |
|---------------|-------------------------------------------------------------------------------------------------------|
| `peer-id`     | ID for the peer this token is intended to be used by. [More info](https://jsr.io/@pulsebeam/server/doc/~/PeerClaims#constructor_0) |
| `group-id`    | ID for the group the peer is in, scoped to your application. [More info](https://jsr.io/@pulsebeam/server/doc/~/PeerClaims#constructor_0) |
| `duration`    | Duration in seconds TTL before token expiration.                                                          |
| `allow-policy`| Defines which peer(s) this peer is allowed to connect to. Default: `"*:*"` (connect to any other peer)[^1]. [More info](https://jsr.io/@pulsebeam/server/doc/~/PeerPolicy) |
| `create-token`| Generates a token[^2][^3] based on the provided inputs/defaults.                                               |

[^1]: Even with the `"*:*"` allow-policy, peers can only connect to other peers within the scope of your `<API_KEY>`.

[^2]: Provide the CLI-generated token to your client's to allow them to talk to PulseBeam signaling servers and make connections with each other.

[^3]: See billing section below.

```bash
./target/debug/pulsebeam-cli --help
```

* Set `api-key` and `api-secret` with your credentials obtained from PulseBeam (https://pulsebeam.dev).
* Warning: be sure to protect your <API_KEY> and <API_SECRET>. See information below 

## ⚠️ Security Warning

**Keep your API credentials secure!**

- The `<API_KEY>` and `<API_SECRET>` are sensitive credentials that grant access to your application’s PulseBeam resources. **Do not expose these credentials** in:
  - Public repositories.
  - Client-side code.
  - Shared or unencrypted environments.

- Treat tokens generated by this CLI as sensitive data. Tokens should only be shared with trusted clients.

### Recommended Practices:
1. **Environment Variables:** Store your API credentials securely in environment variables instead of hardcoding them in your codebase.
    ```bash
    export PULSEBEAM_API_KEY="your_api_key"
    export PULSEBEAM_API_SECRET="your_api_secret"
    ```

2. **Access Controls:** Regularly rotate API credentials and limit their scope to only necessary permissions.

3. **Auditing:** Monitor usage of your credentials and investigate any unauthorized activity.

## 💳 Billing and Token Usage

**Be aware:** Generating and using tokens can incur billing charges. Each token enables interactions with PulseBeam's infrastructure, which may contribute to your account’s usage costs.

### Billing Best Practices:
1. **Understand Your Plan:** Familiarize yourself with the details of your billing plan. For more information, visit our [Billing Page](https://pulsebeam.dev/billing).
2. **Monitor Usage:** Keep track of token usage to avoid unexpected charges. Utilize PulseBeam’s dashboards and APIs for real-time monitoring.
3. **Token Expiration:** Set appropriate `duration` values for tokens to limit unnecessary usage. Tokens with a longer duration may result in increased billing if misused.
4. **Audit Token Distribution:** Ensure tokens are only distributed to trusted clients to avoid misuse that could drive up costs.

### ⚠️ Important Billing Warning

- Tokens allow access to PulseBeam's infrastructure and **may result in charges depending on your usage**.
- Misuse or unauthorized distribution of tokens can lead to **unexpected billing costs**.
- **Ensure you monitor your account's activity regularly** and revoke tokens that are no longer needed.

For detailed information on billing and usage policies, visit our [Billing Page](https://pulsebeam.dev/billing).
