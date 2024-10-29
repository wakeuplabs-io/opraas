
variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-2"
}

variable "proy-name" {
  description = "Name of the project"
  type        = string
  default     = "opraas-chain"
}

variable "cluster-name" {
  description = "Name of the cluster"
  type        = string
  default     = "opraas-cluster"
}
