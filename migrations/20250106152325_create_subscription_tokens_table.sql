CREATE TABLE subscription_tokens (
    subscription_token VARCHAR(255) PRIMARY KEY,
    subscription_id UUID NOT NULL REFERENCES subscriptions(id)
);
