INSERT INTO messages ( sender, recipient, body, sent_at )
VALUES ( $1, $2, $3, NOW() )
RETURNING id, sender, recipient, body, sent_at;
