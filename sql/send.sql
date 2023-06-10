INSERT INTO messages ( sender, recipient, body, sent_at )
VALUES ( :sender, :recipient, :body, NOW() )
RETURNING id, sender, recipient, body, sent_at;
