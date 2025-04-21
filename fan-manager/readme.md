# fan-manager

a small app to control the fan speed via pwm of the radxa X4

run via i.e.

```bash
docker run --privileged -v "/dev:/dev" ghcr.io/0xC9C3/radxa-x4-cluster/fan-manager:main -p /dev/ttyS4
```

the u2f can be installed via the -i flag i.e. via

```bash
docker run --privileged -v "/dev:/dev" ghcr.io/0xC9C3/radxa-x4-cluster/fan-manager:main -i
```