-- create todos table
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    due_date DATETIME,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Insert sample records
INSERT INTO todos (title, description, completed, due_date, created_at, updated_at)
VALUES ('Buy groceries', 'Milk, eggs, bread, and fruits', 0, '2023-03-30 12:00:00', '2023-03-24 10:00:00', '2023-03-24 10:00:00');

INSERT INTO todos (title, description, completed, due_date, created_at, updated_at)
VALUES ('Call the bank', 'Ask about the new credit card offer', 0, '2023-03-25 14:00:00', '2023-03-24 11:00:00', '2023-03-24 11:00:00');

INSERT INTO todos (title, description, completed, due_date, created_at, updated_at)
VALUES ('Finish the report', 'Complete the annual sales report', 0, '2023-03-28 18:00:00', '2023-03-24 09:00:00', '2023-03-24 09:00:00');
