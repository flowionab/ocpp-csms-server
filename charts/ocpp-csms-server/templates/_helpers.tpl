{{/*
Expand the name of the chart.
*/}}
{{- define "ocpp-csms-server.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "ocpp-csms-server.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "ocpp-csms-server.ocppFullname" -}}
{{- printf "%s-ocpp" (include "ocpp-csms-server.fullname" .) }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "ocpp-csms-server.apiFullname" -}}
{{- printf "%s-ocpp" (include "ocpp-csms-server.fullname" .) }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "ocpp-csms-server.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "ocpp-csms-server.ocppLabels" -}}
helm.sh/chart: {{ include "ocpp-csms-server.chart" . }}
{{ include "ocpp-csms-server.ocppSelectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "ocpp-csms-server.ocppSelectorLabels" -}}
app.kubernetes.io/name: {{ include "ocpp-csms-server.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: ocpp
{{- end }}

{{/*
Common labels
*/}}
{{- define "ocpp-csms-server.apiLabels" -}}
helm.sh/chart: {{ include "ocpp-csms-server.chart" . }}
{{ include "ocpp-csms-server.apiSelectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "ocpp-csms-server.apiSelectorLabels" -}}
app.kubernetes.io/name: {{ include "ocpp-csms-server.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: api
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "ocpp-csms-server.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "ocpp-csms-server.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
