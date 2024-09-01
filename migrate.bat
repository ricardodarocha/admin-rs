setlocal
FOR /F "tokens=*" %%i in ('type .env') do SET %%i

@REM test connection
@REM psql -h localhost -p 5432 -d proto -U postgres -

@REM cargo sqlx database create --database-url %DATABASE_URL%

@REM cargo sqlx migrate add nome_tabela
sqlx migrate run --database-url %DATABASE_URL%
endlocal