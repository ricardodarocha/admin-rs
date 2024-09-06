CREATE TABLE job (
    id varchar(40) not null primary key ,
    description varchar not null,
    context jsonb not null,
    execute_at TIMESTAMP WITH TIME zone not null default current_timestamp,
    status VARCHAR(40) not null DEFAULT 'pending',
	job_key varchar(40) not null default 'None',
	id_empresa varchar(40) not null default 'INDEFINIDO',

);

insert into job
	select current_timestamp::varchar, 
	'hello world', 
	'{"name": "Ada", "role": "Computing"}' :: jsonb,
	current_timestamp + (1 * interval '1 minute'),
	'pending'
	returning *;

