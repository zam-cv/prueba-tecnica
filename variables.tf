variable "user_ocid" {
  description = "User OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "fingerprint" {
  description = "Public Key Fingerprint for Oracle Cloud Infrastructure"
  type        = string
}

variable "private_key_path" {
  description = "Path to the Private Key used for Oracle Cloud Infrastructure"
  type        = string
}

variable "tenancy_ocid" {
  description = "Tenancy OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "region" {
  description = "Region for Oracle Cloud Infrastructure"
  type        = string
}

variable "availability_domain" {
  description = "Availability Domain for Oracle Cloud Infrastructure"
  type        = string
}

variable "compartment_id" {
  description = "Compartment OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "postgres_db_name" {
  description = "Database Name for PostgreSQL"
  type        = string
}

variable "postgres_user" {
  description = "User for PostgreSQL"
  type        = string
}

variable "postgres_password" {
  description = "Password for PostgreSQL"
  type        = string
}

variable "rust_log" {
  description = "Rust Log Level"
  type        = string
}

variable "host" {
  description = "Host for the Application"
  type        = string
}

variable "port" {
  description = "Port for the Application"
  type        = number
}

variable "secret_key" {
  description = "Secret Key for the Application"
  type        = string
}