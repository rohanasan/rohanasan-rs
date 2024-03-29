# This directory is made specifically for testing purposes

### The bechmarking tool mostly used is wrk
**Command used to test the framework:**

```yaml
wrk -t 2 -c 400 http://localhost:8080
```

```yaml
wrk -t2 -c100 -d30s -R2000 http://127.0.0.1:8080
# This is wrk2
```

```yaml
rewrk -h http://127.0.0.1:8080 -t 12 -c 60 -d 5s
```