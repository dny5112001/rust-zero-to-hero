# 20 · Deploying everything with Terraform

> New to Terraform? See [terraform-zero-to-hero](https://github.com/dny5112001/terraform-zero-to-hero).

One command builds the image and creates the whole platform:

```bash
make deploy      # cluster → ingress → database + app → smoke test
make status
make destroy
```

Then open **http://api.localhost:8070/docs/**.

## Three stages, three states

[`deploy/terraform/`](../deploy/terraform):

| Stage | Provider | Creates |
|---|---|---|
| `01-cluster` | `tehcyx/kind` | a 2-node Kubernetes cluster; port 8070 → the ingress |
| `02-platform` | `hashicorp/helm` | Traefik ingress controller |
| `03-app` | `hashicorp/kubernetes` + `random` | namespace, Postgres, Secrets, the app, Service, Ingress, PDB |

Each stage reads the previous ones with `terraform_remote_state` (e.g. the
kubeconfig path). Splitting by layer keeps the **blast radius** small: changing
the app can't touch the cluster.

## What `make deploy` does

```
make cluster   terraform apply in 01-cluster
make platform  terraform apply in 02-platform
make image     docker build -t notes-api:<source-hash> notes-api
               kind load docker-image notes-api:<source-hash>
make app       terraform apply in 03-app -var image=notes-api:<source-hash>
               kubectl rollout status
make test      register → login → create a note → list notes, through the ingress
```

## Highlights in the Terraform code

### Secrets generated, never typed

```hcl
resource "random_password" "jwt" {
  length  = 48
  special = false
}

resource "kubernetes_secret_v1" "app" {
  data = {
    DATABASE_URL = "postgres://notes:${random_password.db.result}@${local.db_host}:5432/notes_db"
    JWT_SECRET   = random_password.jwt.result
  }
}
```

No passwords in git. (They **are** in Terraform state — which is why state is
gitignored, and in a team lives in an encrypted remote backend.)

### Config changes roll the Pods

```hcl
annotations = {
  "checksum/secret" = sha256(jsonencode(kubernetes_secret_v1.app.data))
}
```

Env vars are only read at start-up; changing the hash makes Kubernetes restart the Pods.

### Order without scripts

```hcl
depends_on = [kubernetes_stateful_set_v1.db]   # the app runs migrations at start, so the DB must be up
```

## Verified behaviour

When this repo was built, these were tested end to end:

| Check | Result |
|---|---|
| `make test` through the ingress | register, login, create and list notes ✔ |
| Delete the Postgres Pod | data still there afterwards ✔ |
| `terraform plan` after deploy | no changes (no drift) ✔ |
| Rolling restart under constant load | 0 failed requests out of 5,994 ✔ |

## From laptop to AWS

The shape stays the same; only the stages change:

| Local (this repo) | On AWS |
|---|---|
| `01-cluster`: kind | EKS (module `terraform-aws-modules/eks`) + VPC |
| `02-platform`: Traefik | AWS Load Balancer Controller (an ALB) + cert-manager/ACM for HTTPS |
| Postgres StatefulSet | **RDS for PostgreSQL** (managed backups, failover) |
| `kind load` | push to **ECR**; CI builds the image |
| Secrets from `random_password` | AWS Secrets Manager + External Secrets Operator |
| `make deploy` from your laptop | CI/CD: tests → build → push → Terraform/Argo CD |

A typical CI pipeline for this repo:

```
on pull request:  cargo fmt --check · cargo clippy -D warnings · cargo test (with a Postgres service)
                  docker build · terraform fmt -check · terraform validate · terraform plan
on merge:         push image to registry · terraform apply (with approval for prod)
```

## Try it

1. Change the greeting in `/health/live`, run `make app`, and watch a new rolling update (`kubectl -n notes get pods -w`).
2. Set `replicas = 3` in `03-app/variables.tf` and apply.
3. Run `make destroy`, then `make deploy` — everything comes back from code.

## Remember

- Layered stages, each with its own state, wired together by outputs.
- Terraform generates secrets; hashes in annotations roll Pods on change.
- The same structure scales from kind on a laptop to EKS on AWS.
