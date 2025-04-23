# fan-manager

a small app to control the fan speed via pwm of the radxa X4

# Running

```bash
docker run --privileged ghcr.io/0xc9c3/radxa-x4-cluster/fan-manager:main -p /dev/ttyS4
```

# Installation

the uf2 can be installed via the -i flag i.e. via copying the binary from the container

```bash
docker run -v ".:/out" --entrypoint bash ghcr.io/0xc9c3/radxa-x4-cluster/fan-manager:main -c "cp /usr/bin/fan-manager /out"
```

and then running it on the host

```bash
./fan-manager -i
```

You need gpioset / libgpio installed to run the fan-manager installation. Alternatively you can build the uf2 from the
source and copy it manually.

# Kubernetes

for kubernetes to run on every node

1. create a privileged namespace
    ```bash
   kubectl create ns fan-manager
   ```

   ```bash 
   kubectl label namespace fan-manager pod-security.kubernetes.io/enforce=privileged
   ```

2. create the demonset
    ```bash
   kubectl apply -f fan-manager.yaml
   ```

```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fan-manager
  namespace: fan-manager
  labels:
    app: fan-manager
spec:
  selector:
    matchLabels:
      app: fan-manager
  template:
    metadata:
      labels:
        app: fan-manager
    spec:
      tolerations:
        - key: "node-role.kubernetes.io/master"
          operator: Exists
          effect: NoSchedule
        - key: "node-role.kubernetes.io/control-plane"
          operator: Exists
          effect: NoSchedule
      containers:
        - name: fan-manager
          image: ghcr.io/0xc9c3/radxa-x4-cluster/fan-manager:main
          imagePullPolicy: Always
          args: [ "-p", "/dev/ttyS0" ]
          securityContext:
            privileged: true
```

```bash
kubectl apply -f https://github.com/0xC9C3/radxa-x4-cluster/tree/main/fan-manager/daemonset.yaml
```