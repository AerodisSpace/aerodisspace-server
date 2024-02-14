# Run Scylladb Database and Redis on Docker for local development:

## ScyllaDB
```bash
docker run --name aerodisspace_scylla -p 127.0.0.1:9042:9042 -d scylladb/scylla --smp 1 --broadcast-rpc-address 127.0.0.1 --listen-address 0.0.0.0 
```
The -p 127.0.0.1:9042:9042 is to make port 9042 accessible on host (local) machine.
### Connect CQLSH Scylla:

```bash
docker exec -it aerodisspace_scylla cqlsh
```

## charybdis-migrate CLI
use for local dev: 
```bash
migrate --host 127.0.0.1:9042  --keyspace aerodisspace -d
```


---
---

## Redis (cache)
```bash
docker run --name aerodisspace_redis -p 127.0.0.1:6379:6379 -d redis
```

- ### Connect REDIS-CLI to Redis container:
```bash
docker exec -it aerodisspace_redis redis-cli
```


