// --feature-flag name: user-authentication-flag, status: active
// This will display only "user-authentication-flag" in tooltip

// --feature-experiment title: Checkout Flow Experiment, variant: B
// This will display only "Checkout Flow Experiment" in tooltip

// --feature-toggle key: dark-mode-toggle, enabled: true
// This will display only "dark-mode-toggle" in tooltip

// --feature-config id: api-config-prod, endpoint: https://api.example.com
// This will display only "api-config-prod" in tooltip

// --feature-flag type: backend, status: stable, version: 2.0
// This has no identifier, so will display all properties: "type: backend, status: stable, version: 2.0"

export function metadataDisplayTest() {
  return "testing metadata display formats";
}
