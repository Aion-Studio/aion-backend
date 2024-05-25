# Aion Game

## Startup instructions

1. start docker
2. run this:
```
docker compose up -d postgres
psql "postgresql://root:root@localhost:5432/defaultdb" -a -f Quest.sql
```
