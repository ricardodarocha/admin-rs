INSERT INTO todos (title, description, completed)
        VALUES ($1, $2, false)
        RETURNING id, title, description, completed