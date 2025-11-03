# xdata-app Deployment

This directory contains deployment configurations for the xdata-app application.

## Files

- `Dockerfile` - Runtime Docker image for the pre-built Rust application
- `deployment.yaml` - Kubernetes deployment, service, and namespace configuration

## Building the Application

First, build the application on the host machine:

```bash
# From the repository root
cargo build --release --package xdata-app
```

## Building the Docker Image

After building the application, create the Docker image from the repository root:

```bash
docker build -f apps/xdata-app/deploy/Dockerfile -t xdata-app:latest .
```

**For Docker Desktop:** Verify the image is available:
```bash
docker images | grep xdata-app
```

## Running Locally with Docker

```bash
docker run -p 8080:8080 xdata-app:latest
```

## Deploying to Kubernetes

### Prerequisites
- kubectl configured with cluster access
- For Docker Desktop: Image built locally (see above)
- For remote clusters: Docker image pushed to a registry

### Deploy

```bash
kubectl apply -f apps/xdata-app/deploy/deployment.yaml
```

### Verify Deployment

```bash
# Check pods
kubectl get pods -n xedio

# Check service
kubectl get svc -n xedio

# View logs
kubectl logs -n xedio -l app=xdata-app
```

### Access the Application

For local testing with port-forwarding:

```bash
kubectl port-forward -n xedio svc/xdata-app 8080:80
```

Then access at `http://localhost:8080`

### Cleanup

```bash
kubectl delete -f apps/xdata-app/deploy/deployment.yaml
```

## Configuration Notes

- The application runs as a non-root user (UID 1000) for security
- 3 replicas are configured for high availability
- Resource limits are set conservatively (adjust based on your needs)
- The service is exposed as ClusterIP (change to LoadBalancer or NodePort for external access)
- Health checks use TCP socket probes on port 8080

## Customization

### Adjust Replicas

Edit `deployment.yaml` and change the `replicas` field.

### Change Service Type

For external access, edit `deployment.yaml` and change:
```yaml
spec:
  type: LoadBalancer  # or NodePort
```

### Resource Limits

Adjust the `resources` section in `deployment.yaml` based on your application's needs.
