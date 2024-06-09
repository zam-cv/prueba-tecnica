terraform {
  required_providers {
    oci = {
      source  = "hashicorp/oci"
      version = "= 5.40.0"
    }
  }
}

provider "oci" {
  config_file_profile = "DEFAULT"
  user_ocid           = var.user_ocid
  fingerprint         = var.fingerprint
  private_key_path    = var.private_key_path
  tenancy_ocid        = var.tenancy_ocid
  region              = var.region
}

resource "oci_core_vcn" "tctc_vcn" {
  compartment_id = var.compartment_id
  cidr_block     = "10.0.0.0/16"
  display_name   = "tctc_vcn"

  freeform_tags = {
    "project-name" = "tctc"
  }
}

resource "oci_core_security_list" "public_sn_sl" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.tctc_vcn.id
  display_name   = "security list for the public subnet"

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "10.0.0.0/24"
    description = "access to container instance port 5432 from home"

    tcp_options {
      min = 5432
      max = 5432
    }
  }

  egress_security_rules {
    protocol         = 6
    destination_type = "CIDR_BLOCK"
    destination      = "10.0.0.0/24"
    description      = "access to container instance port 5432"

    tcp_options {
      min = 5432
      max = 5432
    }
  }

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "0.0.0.0/0"
    description = "access to container instance port 8080 from anywhere"

    tcp_options {
      min = 8080
      max = 8080
    }
  }

  egress_security_rules {
    protocol         = 6
    destination_type = "CIDR_BLOCK"
    destination      = "0.0.0.0/0"
    description      = "access to container registries via HTTP"

    tcp_options {
      min = 80
      max = 80
    }
  }

  egress_security_rules {
    protocol         = 6
    destination_type = "CIDR_BLOCK"
    destination      = "0.0.0.0/0"
    description      = "access to container registries via HTTPS"

    tcp_options {
      min = 443
      max = 443
    }
  }

  freeform_tags = {
    "project-name" = "tctc"
  }
}

resource "oci_core_subnet" "tctc_subnet" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.tctc_vcn.id
  cidr_block     = "10.0.0.0/24"
  display_name   = "tctc_subnet"
  route_table_id = oci_core_route_table.igw_rt.id

  security_list_ids = [
    oci_core_security_list.public_sn_sl.id
  ]

  freeform_tags = {
    "project-name" = "tctc"
  }
}

resource "oci_core_internet_gateway" "internet_gateway" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.tctc_vcn.id
  display_name   = "internet_gateway"
  enabled        = true
}

resource "oci_core_route_table" "igw_rt" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.tctc_vcn.id
  display_name   = "Internet gateway route table"

  route_rules {
    network_entity_id = oci_core_internet_gateway.internet_gateway.id
    destination       = "0.0.0.0/0"
  }

  freeform_tags = {
    "project-name" = "tctc"
  }
}

data "oci_identity_availability_domains" "local_ads" {
  compartment_id = var.compartment_id
}

resource "oci_container_instances_container_instance" "database" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "database"
  freeform_tags            = { "project-name" = "tctc" }
  container_restart_policy = "ALWAYS"
  shape                    = "CI.Standard.E4.Flex"

  shape_config {
    ocpus         = 2
    memory_in_gbs = 8
  }

  vnics {
    subnet_id = oci_core_subnet.tctc_subnet.id
  }

  containers {
    image_url    = "postgres:13"
    display_name = "database-server"
    environment_variables = {
      POSTGRES_PASSWORD = var.postgres_password
      POSTGRES_DB       = var.postgres_db_name
      POSTGRES_USER     = var.postgres_user
    }
  }
}

resource "oci_container_instances_container_instance" "app" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "app"
  freeform_tags            = { "project-name" = "tctc" }
  container_restart_policy = "ALWAYS"
  shape                    = "CI.Standard.E4.Flex"

  shape_config {
    ocpus         = 4
    memory_in_gbs = 8
  }

  vnics {
    subnet_id = oci_core_subnet.tctc_subnet.id
  }

  depends_on = [
    oci_container_instances_container_instance.database
  ]

  containers {
    image_url    = "zamcv/tctc"
    display_name = "app-server"
    environment_variables = {
      RUST_LOG      = var.rust_log
      HOST          = var.host
      PORT          = var.port
      SECRET_KEY    = var.secret_key
      DATABASE_HOST = oci_container_instances_container_instance.database.vnics.0.private_ip
      DATABASE_URL  = "postgres://${var.postgres_user}:${var.postgres_password}@${oci_container_instances_container_instance.database.vnics.0.private_ip}:5432/${var.postgres_db_name}"
    }
  }
}

data "oci_core_vnic" "app_vnic" {
  vnic_id = oci_container_instances_container_instance.app.vnics[0].vnic_id
}

output "app_public_ip" {
  value = data.oci_core_vnic.app_vnic.public_ip_address
}
