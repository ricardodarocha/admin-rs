CREATE TABLE job (
    id varchar(40) not null primary key ,
    description varchar not null,
    jontext jsonb not null,
    execute_at TIMESTAMP WITH TIME zone not null default current_timestamp,
    status VARCHAR(40) not null DEFAULT 'pending'
);