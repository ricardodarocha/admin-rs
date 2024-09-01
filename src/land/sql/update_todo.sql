UPDATE todos
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            completed = COALESCE($4, completed)
        WHERE id = $1
        RETURNING id, title, description, completed