# Stage 02 — the ingress controller (Traefik), installed with Helm.
terraform {
  required_version = ">= 1.9"
  required_providers {
    helm = {
      source  = "hashicorp/helm"
      version = "~> 3.3"
    }
  }
}

data "terraform_remote_state" "cluster" {
  backend = "local"
  config  = { path = "${path.module}/../01-cluster/terraform.tfstate" }
}

provider "helm" {
  kubernetes = {
    config_path = data.terraform_remote_state.cluster.outputs.kubeconfig_path
  }
}

resource "helm_release" "traefik" {
  name             = "traefik"
  namespace        = "traefik"
  create_namespace = true
  repository       = "https://traefik.github.io/charts"
  chart            = "traefik"
  version          = "41.7.1"
  wait             = true
  atomic           = true

  values = [yamlencode({
    service = { spec = { type = "NodePort" } } # kind has no cloud load balancer
    ports = {
      web       = { nodePort = 30080 }
      websecure = { expose = { default = false } }
    }
    ingressClass = { enabled = true, isDefaultClass = true, name = "traefik" }
  })]
}

output "ingress_class" {
  value = "traefik"
}
