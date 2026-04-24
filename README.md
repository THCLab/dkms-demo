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

## Option B – Kubernetes / Helm

The Helm charts are split into three independent charts, one per service type. This allows deploying each instance separately — for example across different nodes or clusters.

```
helm/
├── witness/       # single witness instance
├── watcher/       # watcher instance
└── mesagkesto/    # mesagkesto instance
```

### Prerequisites

- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Helm 3](https://helm.sh/docs/intro/install/)

---

### Local testing (no ingress)

#### Step 1: Create secrets

Create the secrets manually before installing any chart:

```bash
kubectl create secret generic witness1-secret --from-literal=seed=<seed>
kubectl create secret generic witness2-secret --from-literal=seed=<seed>
kubectl create secret generic witness3-secret --from-literal=seed=<seed>
kubectl create secret generic watcher-secret  --from-literal=seed=<seed>
kubectl create secret generic mesagkesto-secret \
  --from-literal=seed=<seed> \
  --from-literal=serverKey=<serverKey>
```

#### Step 2: Install the charts

```bash
# Witness 1
helm install witness1 ./helm/witness \
  --set name=witness1 \
  --set ingress.enabled=false \
  --set publicUrl=http://localhost:3232

# Witness 2
helm install witness2 ./helm/witness \
  --set name=witness2 \
  --set port=3233 \
  --set ingress.enabled=false \
  --set publicUrl=http://localhost:3233

# Witness 3
helm install witness3 ./helm/witness \
  --set name=witness3 \
  --set port=3234 \
  --set ingress.enabled=false \
  --set publicUrl=http://localhost:3234

# Watcher
helm install watcher ./helm/watcher \
  --set ingress.enabled=false \
  --set publicUrl=http://localhost:3235 \
  --set "initialOobis[0].url=http://witness1:3232/" \
  --set "initialOobis[1].url=http://witness2:3233/" \
  --set "initialOobis[2].url=http://witness3:3234/"

# Mesagkesto
helm install mesagkesto ./helm/mesagkesto \
  --set ingress.enabled=false \
  --set publicUrl=http://localhost:3236 \
  --set watcherOobi.url=http://watcher:3235
```

#### Step 3: Port-forward to access services locally

```bash
kubectl port-forward svc/witness1   3232:3232 &
kubectl port-forward svc/witness2   3233:3233 &
kubectl port-forward svc/witness3   3234:3234 &
kubectl port-forward svc/watcher    3235:3235 &
kubectl port-forward svc/mesagkesto 3236:3236 &
```

#### Step 4: Verify

```bash
curl http://localhost:3232/introduce   # witness1
curl http://localhost:3233/introduce   # witness2
curl http://localhost:3234/introduce   # witness3
curl http://localhost:3235/introduce   # watcher
curl http://localhost:3236/introduce   # mesagkesto
```

---

### Production deployment (with ingress)

Requires an ingress controller (nginx) and a wildcard DNS record pointing to your cluster:

```
*.nextgen.hiro-develop.nl  →  <ingress controller IP>
```

#### Step 1: Create secrets (same as above)

#### Step 2: Install the charts

```bash
# Witness 1
helm install witness1 ./helm/witness \
  --namespace production \
  --set name=witness1

# Witness 2
helm install witness2 ./helm/witness \
  --namespace production \
  --set name=witness2 \
  --set port=3233

# Witness 3
helm install witness3 ./helm/witness \
  --namespace production \
  --set name=witness3 \
  --set port=3234

# Watcher — pass all three fields per witness to avoid Helm dropping eid and scheme
# initialOobis URLs should use the ingress domains assigned to each witness
helm install watcher ./helm/watcher \
  --namespace production \
  --set "initialOobis[0].eid=<witness1-eid>" \
  --set "initialOobis[0].scheme=http" \
  --set "initialOobis[0].url=http://<witness1-ingress-domain>/" \
  --set "initialOobis[1].eid=<witness2-eid>" \
  --set "initialOobis[1].scheme=http" \
  --set "initialOobis[1].url=http://<witness2-ingress-domain>/" \
  --set "initialOobis[2].eid=<witness3-eid>" \
  --set "initialOobis[2].scheme=http" \
  --set "initialOobis[2].url=http://<witness3-ingress-domain>/"

# Mesagkesto — watcherOobi URL should use the ingress domain assigned to the watcher
helm install mesagkesto ./helm/mesagkesto \
  --namespace production \
  --set watcherOobi.url=http://<watcher-ingress-domain>
```

Services will be reachable at:

| Service    | URL |
|------------|-----|
| witness1   | `http://ds-witness.production.nextgen.hiro-develop.nl` |
| witness2   | `http://ds-witness.production.nextgen.hiro-develop.nl` |
| witness3   | `http://ds-witness.production.nextgen.hiro-develop.nl` |
| watcher    | `http://ds-watcher.production.nextgen.hiro-develop.nl` |
| mesagkesto | `http://ds-mesagkesto.production.nextgen.hiro-develop.nl` |

> **Note:** All three witnesses share the same subdomain by default. Override `ingress.subdomain` per install to give each witness a unique subdomain.

---

### Upgrading

```bash
helm upgrade witness1 ./helm/witness --set name=witness1 ...
```

### Uninstalling

```bash
helm uninstall witness1
helm uninstall witness2
helm uninstall witness3
helm uninstall watcher
helm uninstall mesagkesto
```

> **Note:** PersistentVolumeClaims are not deleted automatically. To wipe all data:
> ```bash
> kubectl delete pvc witness1-pvc witness2-pvc witness3-pvc watcher-pvc
> ```

---

## Tests

Navigate to the `test-vectors` dir and run the scripts.
