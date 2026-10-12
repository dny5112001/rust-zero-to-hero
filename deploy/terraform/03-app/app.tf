# The Rust notes-api: Secret, Deployment, Service, Ingress, PodDisruptionBudget.

locals {
  labels = {
    "app.kubernetes.io/name"       = "notes-api"
    "app.kubernetes.io/managed-by" = "terraform"
  }
  db_host = "${kubernetes_service_v1.db.metadata[0].name}.${kubernetes_namespace_v1.app.metadata[0].name}.svc.cluster.local"
}

resource "random_password" "jwt" {
  length  = 48
  special = false
}

resource "kubernetes_secret_v1" "app" {
  metadata {
    name      = "notes-api"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  data = {
    DATABASE_URL = "postgres://notes:${random_password.db.result}@${local.db_host}:5432/notes_db"
    JWT_SECRET   = random_password.jwt.result
  }
}

resource "kubernetes_deployment_v1" "app" {
  metadata {
    name      = "notes-api"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
    labels    = local.labels
  }

  # The app runs its migrations on start, so the database must be up first.
  depends_on = [kubernetes_stateful_set_v1.db]

  spec {
    replicas = var.replicas

    selector {
      match_labels = local.labels
    }

    strategy {
      type = "RollingUpdate"
      rolling_update {
        max_unavailable = "0"
        max_surge       = "1"
      }
    }

    template {
      metadata {
        labels = local.labels
        annotations = {
          # Roll the Pods when the secret changes (env vars are read only at start).
          "checksum/secret" = sha256(jsonencode(kubernetes_secret_v1.app.data))
        }
      }

      spec {
        automount_service_account_token  = false
        termination_grace_period_seconds = 30

        security_context {
          run_as_non_root = true
          seccomp_profile {
            type = "RuntimeDefault"
          }
        }

        container {
          name              = "notes-api"
          image             = var.image
          image_pull_policy = "IfNotPresent" # loaded into kind with `kind load`, not pulled

          port {
            name           = "http"
            container_port = 8800
          }

          env_from {
            secret_ref {
              name = kubernetes_secret_v1.app.metadata[0].name
            }
          }
          env {
            name  = "RUST_LOG"
            value = "notes_api=info,tower_http=info"
          }
          env {
            name  = "SHUTDOWN_DELAY_SECONDS" # keep serving briefly after SIGTERM (see main.rs)
            value = "5"
          }

          resources {
            requests = { cpu = "50m", memory = "32Mi" }
            limits   = { memory = "128Mi" }
          }

          readiness_probe {
            http_get {
              path = "/health" # checks the database
              port = "http"
            }
            period_seconds = 5
          }

          liveness_probe {
            http_get {
              path = "/health/live" # process only — never the database
              port = "http"
            }
            period_seconds    = 10
            failure_threshold = 3
          }

          security_context {
            allow_privilege_escalation = false
            read_only_root_filesystem  = true
            capabilities {
              drop = ["ALL"]
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_service_v1" "app" {
  metadata {
    name      = "notes-api"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  spec {
    selector = local.labels
    port {
      name        = "http"
      port        = 80
      target_port = "http"
    }
  }
}

resource "kubernetes_ingress_v1" "app" {
  metadata {
    name      = "notes-api"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  spec {
    ingress_class_name = data.terraform_remote_state.platform.outputs.ingress_class
    rule {
      host = var.hostname
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = kubernetes_service_v1.app.metadata[0].name
              port {
                name = "http"
              }
            }
          }
        }
      }
    }
  }
}

# Never take more than one copy down at a time during node maintenance.
resource "kubernetes_pod_disruption_budget_v1" "app" {
  metadata {
    name      = "notes-api"
    namespace = kubernetes_namespace_v1.app.metadata[0].name
  }
  spec {
    max_unavailable = "1"
    selector {
      match_labels = local.labels
    }
  }
}
