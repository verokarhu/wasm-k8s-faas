output "builder_key" {
  sensitive = true
  value     = google_service_account_key.builder.private_key
}

output "hostname" {
  value = google_compute_instance.vm_instance.name
}

output "project" {
  value = google_compute_instance.vm_instance.project
}

output "zone" {
  value = google_compute_instance.vm_instance.zone
}
