terraform {
  required_version = ">= 1.9"
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 3.3"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.7"
    }
  }
}

data "terraform_remote_state" "cluster" {
  backend = "local"
  config  = { path = "${path.module}/../01-cluster/terraform.tfstate" }
}

data "terraform_remote_state" "platform" {
  backend = "local"
  config  = { path = "${path.module}/../02-platform/terraform.tfstate" }
}

provider "kubernetes" {
  config_path = data.terraform_remote_state.cluster.outputs.kubeconfig_path
}
