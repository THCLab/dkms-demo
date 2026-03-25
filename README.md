# DKMS infrastructure

## Purpose

Demonstrate the practical usage of the [DKMS](https://dkms.colossi.network/) network consisting of DKMS Witnesses and Watchers, based on the KERI protocol.

The network consists of:

- 3 Witnesses
- 1 Watcher
- 1 Mesagkesto (message box)

---

## Option A – Docker Compose

### Step 1: Run the Infrastructure

Navigate to the `infrastructure` directory and start the network:

```bash
cd infrastructure
docker compose up
```

### Step 2: Connect to the Infrastructure

Interact with the running infrastructure using one of the following client (controller) options:

1. **Command Line Interface (CLI):**
   Utilize [`dkms-bin`](https://github.com/THCLab/dkms-bin) for CLI-based interaction.

2. **API Client:**
   - **Rust:** The Rust-based API client is available in the [Keriox Controller Component](https://github.com/THCLab/keriox/tree/master/components/controller).
   - **JavaScript (Node.js):** The Node.js API client is provided in the [DKMS Bindings](https://github.com/THCLab/dkms-bindings/tree/master/bindings/node.js).

Service ports (Docker Compose):

| Service    | Port |
|------------|------|
| witness1   | 3232 |
| witness2   | 3233 |
| witness3   | 3234 |
| watcher    | 3235 |
| mesagkesto | 3236 |

---

## Option B – Kubernetes / Helm (minikube)

### Prerequisites

- [minikube](https://minikube.sigs.k8s.io/docs/start/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Helm 3](https://helm.sh/docs/intro/install/)

### Step 1: Start minikube

```bash
minikube start
```

### Step 2: Obtain the minikube IP

```bash
minikube ip
# e.g. 192.168.49.2
```

This IP is used as the `externalHost` — the address that goes into each service's `public_url` so that external KERI clients can reach the witnesses, watcher, and mesagkesto.

### Step 3: Install the Helm chart

From the repository root:

```bash
helm install dkms-demo ./helm/dkms-demo --set externalHost=$(minikube ip)
```

Helm will print a `NOTES` summary with all service endpoints after the install completes.

### Step 4: Verify the deployment

```bash
# Watch pods come up (watcher and mesagkesto use initContainers to wait for dependencies)
kubectl get pods -w

# Check services and their NodePorts
kubectl get svc
```

Expected NodePorts:

| Service    | NodePort |
|------------|----------|
| witness1   | 30232    |
| witness2   | 30233    |
| witness3   | 30234    |
| watcher    | 30235    |
| mesagkesto | 30236    |

### Step 5: Access the services

```bash
MINIKUBE_IP=$(minikube ip)

curl http://$MINIKUBE_IP:30232/introduce   # witness1
curl http://$MINIKUBE_IP:30233/introduce   # witness2
curl http://$MINIKUBE_IP:30234/introduce   # witness3
curl http://$MINIKUBE_IP:30235/introduce   # watcher
curl http://$MINIKUBE_IP:30236/introduce   # mesagkesto
```

### Upgrading / changing the external host

```bash
helm upgrade dkms-demo ./helm/dkms-demo --set externalHost=$(minikube ip)
```

### Uninstalling

```bash
helm uninstall dkms-demo
```

> **Note:** PersistentVolumeClaims are not deleted automatically. To wipe all data:
> ```bash
> kubectl delete pvc witness1-pvc witness2-pvc witness3-pvc watcher-pvc
> ```

### Chart structure

```
helm/dkms-demo/
├── Chart.yaml
├── values.yaml          # all tuneable parameters
└── templates/
    ├── _helpers.tpl
    ├── NOTES.txt
    ├── witness-configmap.yaml    # one ConfigMap per witness (range)
    ├── witness-deployment.yaml   # one Deployment per witness (range)
    ├── witness-service.yaml      # NodePort Service per witness (range)
    ├── witness-pvc.yaml          # PVC per witness (range)
    ├── watcher-configmap.yaml
    ├── watcher-deployment.yaml   # initContainer waits for all witnesses
    ├── watcher-service.yaml
    ├── watcher-pvc.yaml
    ├── mesagkesto-configmap.yaml
    ├── mesagkesto-deployment.yaml # initContainer waits for watcher
    └── mesagkesto-service.yaml
```
---

## Tests

Navigate to the `test-vectors` dir and run the scripts.
