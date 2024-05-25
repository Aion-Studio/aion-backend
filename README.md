# Aion Game

## Startup instructions

1. start docker
2. run this:
```
docker compose up -d postgres
export APP_ENVIRONMENT=local
cargo prisma migrate dev
<!-- psql "postgresql://root:root@localhost:5432/defaultdb" -a -f Quest.sql -->
cd seed/base
for sql_file in ./*.sql; do
    echo "Executing $sql_file"
    psql "postgresql://root:root@localhost:5432/defaultdb" -a -f "$sql_file"
done
cd ...
cargo-watch -w src -x run
```
3. To shut down
```
docker ps
# get the container id
docker stop <container_id>
```
