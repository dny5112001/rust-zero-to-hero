# Stage 01 — a local Kubernetes cluster (kind). On AWS this would be EKS.
terraform {
  required_version = ">= 1.9"
  required_providers {
    kind = {
      source  = "tehcyx/kind"
      version = "~> 0.11"
    }
  }
}

variable "cluster_name" {
  type    = string
  default = "rust"
}

variable "http_port" {
  description = "Port on your Mac for the ingress (http://api.localhost:<port>)"
  type        = number
  default     = 8070
}

resource "kind_cluster" "this" {
  name            = var.cluster_name
  node_image      = "kindest/node:v1.35.8@sha256:07b2536e30b803ed61d1677a79df6115f798ce64c80f9e22f6ed45afd09323c0"
  wait_for_ready  = true
  kubeconfig_path = abspath("${path.module}/kubeconfig")

  kind_config {
    kind        = "Cluster"
    api_version = "kind.x-k8s.io/v1alpha4"

    node {
      role = "control-plane"
      extra_port_mappings {
        container_port = 30080 # Traefik's NodePort (stage 02)
        host_port      = var.http_port
      }
    }
    node {
      role = "worker"
    }
  }
}

output "cluster_name" {
  value = kind_cluster.this.name
}

output "kubeconfig_path" {
  value = kind_cluster.this.kubeconfig_path
}

output "http_port" {
  value = var.http_port
}
