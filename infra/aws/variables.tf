
variable "region" {
  description = "AWS region"
  type        = string
  default     = "us-east-2"
}

variable "proy-name" {
  description = "Name of the project. Used for naming resources {proy-name}-{resource}. For example, {}-vpc, {}-cluster"
  type        = string
  default     = "opraas-chain"
}
