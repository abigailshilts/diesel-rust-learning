-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE activities (
    id SERIAL PRIMARY KEY,
    topic VARCHAR NOT NULL,
    dy VARCHAR NOT NULL,
    start_at VARCHAR NOT NULL,
    end_at VARCHAR NOT NULL
)