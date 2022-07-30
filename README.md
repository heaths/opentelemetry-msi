# OpenTelemetry Tracer for Windows Installer Packages

This project demonstrates Windows Installer custom actions (CAs) that emit spans to an OpenTelemetry tracer to measure total install time, execution time, and time of certain actions known to be slow and affected by package authoring e.g., `CostFinalize` and `InstallFiles`.
