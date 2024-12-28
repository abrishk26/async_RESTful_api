CREATE TABLE IF NOT EXISTS boards (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);

-- seed data
INSERT INTO boards 
(name) 
VALUES 
('General'),
('Random'),
('Technology'),
('Programming'),
('Music'),
('Movies');
