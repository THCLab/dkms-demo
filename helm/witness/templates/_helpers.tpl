{{/*
Common labels
*/}}
{{- define "witness.labels" -}}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
app.kubernetes.io/name: witness
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app: {{ .Values.name }}
{{- end }}

{{/*
StorageClass — omit key when empty so cluster default is used
*/}}
{{- define "witness.storageClass" -}}
{{- if .Values.persistence.storageClass }}
storageClassName: {{ .Values.persistence.storageClass | quote }}
{{- end }}
{{- end }}
