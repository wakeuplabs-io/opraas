# Instructions

With terraform:

1. Fill the values in `helm/values.yaml`
2. Fill up variables in `aws/variables.tf`
3. Deploy infra with `terraform apply` from aws dir
4. Use `terraform output <output_name>` to quickly gain data from your deployment like `elb_dnsname` for setting up the domain records, or `configure_kubectl` to help you connect to the cluster or to get output urls.

Without terraform:

1. Spin up your cluster
2. Deploy nginx as follows

```bash
  helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx && helm repo update
helm install ingress-nginx ingress-nginx/ingress-nginx --namespace ingress-nginx --create-namespace
```

3. Deploy Cert manager

```bash
helm repo add jetstack https://charts.jetstack.io && helm repo update
helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.10.0 --set installCRDs=true
```

4. Update values.yaml accordingly
5. Deploy opraas chart

```bash
helm install opraas ./helm --namespace opraas --create-namespace -f ./helm/values.yaml
```

## SSL certificate

For https domain make sure to create an A record pointing to `elb_dnsname` as specified here: https://github.com/amcginlay/venafi-demos/tree/main/demos/01-eks-ingress-nginx-cert-manager#configure-route53

You can get `elb_dnsname` with `terraform output elb_dnsname` or with `kubectl -n ingress-nginx get service ingress-nginx-controller -o jsonpath='{.status.loadBalancer.ingress[0].hostname}'`

Also, all ingress should be defined in `helm/templates/ingress.yaml`

## Volume claims

db-pvc.yaml

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: postgres-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 5gi
  storageClassName: gp2
```

db-svc.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: postgres-service
spec:
  type: ClusterIP
  ports:
    - port: 5432
      targetPort: 5432
  selector:
    app: postgres
```

db-depl.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgres-deployment
  labels:
    app: postgres
spec:
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
        - name: postgres
          image: postgres:latest # Use a specific version for production
          env:
            - name: POSTGRES_DB
              value: "dbname" # Change to your desired database name
            - name: POSTGRES_USER
              value: "user" # Change to your desired username
            - name: POSTGRES_PASSWORD
              value: "password" # Change to your desired password
            - name: PGDATA
              value: /var/lib/postgresql/data/pgdata
          ports:
            - containerPort: 5432
          volumeMounts:
            - name: postgres-storage
              mountPath: /var/lib/postgresql/data # Persistent storage path
      volumes:
        - name: postgres-storage
          persistentVolumeClaim:
            claimName: postgres-pvc # Reference to the PVC
```

## Wait for service

```yaml
initContainers:
  - name: wait-for-db
    image: busybox
    command:
      [
        "sh",
        "-c",
        "until nc -z postgres-service 5432; do echo waiting for db; sleep 2; done;",
      ]
```
