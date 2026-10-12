# PostgreSQL running inside the cluster, as a StatefulSet with its own disk.
# (In production you'd usually use a managed database such as AWS RDS instead.)

resource "kubernetes_namespace_v1" "app" {
  metadata {
    name = "notes"
  }
}

# Terraform invents the passwords. They're stored in Terraform state — keep state private.
resource "random_password" "db" {
  length  = 24
  special = false
}

resource "kubernetes_secret_v1" "db" {
  metadata {
    name      = "postgres"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  data = {
    POSTGRES_USER     = "notes"
    POSTGRES_PASSWORD = random_password.db.result
    POSTGRES_DB       = "notes_db"
  }
}

resource "kubernetes_service_v1" "db" {
  metadata {
    name      = "postgres"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  spec {
    cluster_ip = "None" # headless: stable DNS name for the StatefulSet Pod
    selector   = { app = "postgres" }
    port {
      port = 5432
    }
  }
}

resource "kubernetes_stateful_set_v1" "db" {
  metadata {
    name      = "postgres"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }

  spec {
    service_name = kubernetes_service_v1.db.metadata[0].name
    replicas     = 1

    selector {
      match_labels = { app = "postgres" }
    }

    template {
      metadata {
        labels = { app = "postgres" }
      }
      spec {
        container {
          name  = "postgres"
          image = "postgres:17-alpine"

          env_from {
            secret_ref {
              name = kubernetes_secret_v1.db.metadata[0].name
            }
          }
          env {
            name  = "PGDATA"
            value = "/var/lib/postgresql/data/pgdata"
          }

          port {
            container_port = 5432
          }

          readiness_probe {
            exec {
              command = ["sh", "-c", "pg_isready -U \"$POSTGRES_USER\" -d \"$POSTGRES_DB\""]
            }
            period_seconds = 5
          }

          resources {
            requests = { cpu = "100m", memory = "256Mi" }
            limits   = { memory = "256Mi" }
          }

          volume_mount {
            name       = "data"
            mount_path = "/var/lib/postgresql/data"
          }
        }
      }
    }

    volume_claim_template {
      metadata {
        name = "data"
      }
      spec {
        access_modes = ["ReadWriteOnce"]
        resources {
          requests = { storage = "1Gi" }
        }
      }
    }
  }

  wait_for_rollout = true
}
