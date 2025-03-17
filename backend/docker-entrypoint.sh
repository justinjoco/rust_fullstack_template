#!/bin/sh
echo "Waiting for postgres..."

while ! nc -z "postgres" "5432"; do
    sleep 0.1
done

echo "PostgreSQL started"

sqlx migrate run --database-url "postgres://admin:password@postgres:5432/app_db"
exec /bin/app