output "url" {
  value = "http://${var.hostname}:${data.terraform_remote_state.cluster.outputs.http_port}"
}

output "docs" {
  value = "http://${var.hostname}:${data.terraform_remote_state.cluster.outputs.http_port}/docs/"
}

output "image" {
  value = var.image
}
