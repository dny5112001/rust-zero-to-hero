variable "image" {
  description = "The notes-api image. `make deploy` passes a tag based on the source code hash."
  type        = string
  default     = "notes-api:dev"
}

variable "replicas" {
  type    = number
  default = 2
}

variable "hostname" {
  type    = string
  default = "api.localhost"
}
