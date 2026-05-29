BEGIN BATCH

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, balance
) VALUES (
    'Vera ADRIAN', 0
) IF NOT EXISTS;

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 1, 7.95, 'Breakfast', false
);

APPLY BATCH;

BEGIN BATCH

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 2, 13.44, 'Lunch', true
);

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 3, 25.00, 'Dinner', false
);

UPDATE cycling.cyclist_expenses SET balance = -32.95
WHERE cyclist_name = 'Vera ADRIAN' IF balance = -7.95;

APPLY BATCH;

BEGIN BATCH

UPDATE cycling.cyclist_expenses SET balance = 0
WHERE cyclist_name = 'Vera ADRIAN' IF balance = -32.95;

UPDATE cycling.cyclist_expenses SET paid = true
WHERE cyclist_name = 'Vera ADRIAN' AND expense_id = 1 IF paid = false;

UPDATE cycling.cyclist_expenses SET paid = true
WHERE cyclist_name = 'Vera ADRIAN' AND expense_id = 3 IF paid = false;

APPLY BATCH;

BEGIN BATCH

INSERT INTO cycling.cyclist_name (
    id, lastname, firstname
) VALUES (
    6ab09bec-e68e-48d9-a5f8-97e6fb4c9b12, 'SMITH', 'John'
);

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'John SMITH', 3, 15.00, 'Lunch', false
);

APPLY BATCH;

BEGIN BATCH USING TIMESTAMP 1481124356754405

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 2, 13.44, 'Lunch', true
);

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 3, 25.00, 'Dinner', true
);

APPLY BATCH;

BEGIN BATCH USING TIMESTAMP 1481124356754405

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 2, 13.44, 'Lunch', true
);

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Vera ADRIAN', 3, 25.00, 'Dinner', false
) IF NOT EXISTS;

APPLY BATCH;

BEGIN BATCH

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id
) VALUES (
    'Joe WALLS', 1
)
IF NOT EXISTS;

INSERT INTO cycling.cyclist_expenses (
    cyclist_name, expense_id, amount, description, paid
) VALUES (
    'Joe WALLS', 1, 8, 'burrito', false
);

APPLY BATCH;

BEGIN COUNTER BATCH

UPDATE cycling.popular_count
SET popularity = popularity + 1
WHERE id = 6ab09bec-e68e-48d9-a5f8-97e6fb4c9b47;

UPDATE cycling.popular_count
SET popularity = popularity + 125
WHERE id = 6ab09bec-e68e-48d9-a5f8-97e6fb4c9b47;

UPDATE cycling.popular_count
SET popularity = popularity - 64
WHERE id = 6ab09bec-e68e-48d9-a5f8-97e6fb4c9b47;

APPLY BATCH;
