terraform {
  required_providers {
    google = {
      source = "hashicorp/google"
    }
  }
}

provider "google" {
  project = var.project
  zone    = var.zone
}

resource "google_compute_instance" "vm_instance" {
  name         = "ubuntu"
  machine_type = "e2-standard-4"

  boot_disk {
    initialize_params {
      image = "ubuntu-2204-lts"
      size  = 100
      type  = "pd-ssd"
    }
  }

  network_interface {
    network = "default"

    access_config {}
  }
}

resource "google_project_service" "containerregistry" {
  service = "containerregistry.googleapis.com"
}

resource "google_container_registry" "registry" {
  depends_on = [
    google_project_service.containerregistry
  ]
}

resource "google_storage_bucket_iam_member" "reader" {
  bucket = google_container_registry.registry.id
  member = "allUsers"
  role   = "roles/storage.objectViewer"
}

resource "google_service_account" "builder" {
  account_id   = "registry-writer"
  display_name = "Service account for pushing images"
}

resource "google_service_account_key" "builder" {
  service_account_id = google_service_account.builder.name
}

resource "google_project_iam_member" "admin" {
  member  = "serviceAccount:${google_service_account.builder.email}"
  project = var.project
  role    = "roles/storage.admin"
}
