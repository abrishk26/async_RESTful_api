CREATE TYPE CARDSTATUS AS ENUM ('todo', 'in-progress', 'done');

CREATE TABLE IF NOT EXISTS cards (
    id BIGSERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    status CARDSTATUS NOT NULL DEFAULT 'todo',
    board_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
    CONSTRAINT board_fk
        FOREIGN KEY (board_id) 
        REFERENCES boards (id) 
        ON DELETE CASCADE
);

-- seed data
INSERT INTO cards
(description, status, board_id)
VALUES
('Create a new project', 'todo', 1),
('Update the README', 'todo', 1),
('Add a new feature', 'in-progress', 1),
('Fix a bug', 'done', 1),
('Create a new project', 'todo', 2),
('Update the README', 'todo', 2),
('Add a new feature', 'in-progress', 2),
('Fix a bug', 'done', 2),
('Create a new project', 'todo', 3),
('Update the README', 'todo', 3),
('Add a new feature', 'in-progress', 3),
('Fix a bug', 'done', 3),
('Create a new project', 'todo', 4),
('Update the README', 'todo', 4),
('Add a new feature', 'in-progress', 4),
('Fix a bug', 'done', 4),
('Create a new project', 'todo', 5),
('Update the README', 'todo', 5),
('Add a new feature', 'in-progress', 5),
('Fix a bug', 'done', 5),
('Create a new project', 'todo', 6),
('Update the README', 'todo', 6),
('Add a new feature', 'in-progress', 6),
('Fix a bug', 'done', 6);