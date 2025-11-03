#!/usr/bin/env bash
set -euo pipefail

# Config
HOST="http://sonarcuteapi:8080"
API_BASE="$HOST/api"
GATE_NAME="Kiosk Gate"

require() {
  command -v "$1" >/dev/null 2>&1 || { echo "Error: $1 is required but not installed." >&2; exit 1; }
}

require curl
require jq

echo "[1/5] Creating quality gate: $GATE_NAME"
create_payload=$(jq -n --arg name "$GATE_NAME" '{name: $name}')
curl -sS --fail --location "$API_BASE/quality-gates" \
  --header 'Content-Type: application/json' \
  --data "$create_payload" >/dev/null

echo "[2/5] Fetching quality gate details"
encoded_name=${GATE_NAME// /%20}
details_json=$(curl -sS --fail --location "$API_BASE/quality-gates/details?name=$encoded_name")

echo "[info] Raw details received:" >&2
echo "$details_json" | jq '.' >&2 || true

echo "[3/5] Finding all existing condition ids"
# Extract all condition IDs from the quality gate details
# This finds all objects that have both "id" and "metric" fields (typical condition structure)
# Convert IDs to strings to ensure compatibility with API
delete_ids_json=$(echo "$details_json" | jq '[.. | objects | select(has("id") and has("metric")) | .id | tostring] | unique')

condition_count=$(echo "$delete_ids_json" | jq 'length')
if [[ "$condition_count" -eq 0 ]]; then
  echo "[info] No existing conditions found to delete." >&2
else
  echo "[info] Found $condition_count existing condition(s) to delete:" >&2
  echo "$delete_ids_json" | jq -r '.[]' | while read -r id; do
    # Handle both string and numeric IDs
    metric=$(echo "$details_json" | jq -r --arg id "$id" '.. | objects | select(has("id") and has("metric") and (.id|tostring)==$id) | .metric // "unknown"' | head -n1)
    echo "  - id: $id (metric: $metric)" >&2
  done
fi

echo "[4/5] Updating quality gate: Delete all old quality gate rule conditions"
update_payload=$(jq -n \
  --arg name "$GATE_NAME" \
  --argjson delete_ids "$delete_ids_json" \
  '{
     name: $name,
     delete_condition_ids: $delete_ids
   }')

curl -sS --fail --location --request PUT "$API_BASE/quality-gates" \
  --header 'Content-Type: application/json' \
  --data "$update_payload" | jq '.'

echo "[5/5] Updating quality gate: Add custom quality gate rule conditions"
# Add conditions one at a time to better identify failures
conditions=(
  '{"metric": "new_software_quality_reliability_rating", "op": "GT", "error": "1"}'
  '{"metric": "new_software_quality_security_rating", "op": "GT", "error": "1"}'
  '{"metric": "new_software_quality_maintainability_rating", "op": "GT", "error": "1"}'
  '{"metric": "new_coverage", "op": "LT", "error": "80"}'
  '{"metric": "new_duplicated_lines_density", "op": "GT", "error": "3"}'
  '{"metric": "new_software_quality_blocker_issues", "op": "GT", "error": "0"}'
  '{"metric": "new_software_quality_high_issues", "op": "GT", "error": "0"}'


  '{"metric": "software_quality_reliability_rating", "op": "GT", "error": "2"}'
  '{"metric": "software_quality_security_rating", "op": "GT", "error": "2"}'
  '{"metric": "software_quality_maintainability_rating", "op": "GT", "error": "2"}'
  '{"metric": "coverage", "op": "LT", "error": "80"}'
  '{"metric": "duplicated_lines_density", "op": "GT", "error": "10"}'
  '{"metric": "software_quality_blocker_issues", "op": "GT", "error": "0"}'
  '{"metric": "software_quality_high_issues", "op": "GT", "error": "0"}'
)

# Convert conditions array to JSON array
add_conditions_json=$(printf '%s\n' "${conditions[@]}" | jq -s '.')

update_payload=$(jq -n \
  --arg name "$GATE_NAME" \
  --argjson add_conditions "$add_conditions_json" \
  '{
     name: $name,
     add_conditions: $add_conditions
   }')

# Make the request and capture both stdout and stderr
response=$(curl -sS -w "\nHTTP_STATUS:%{http_code}" --location --request PUT "$API_BASE/quality-gates" \
  --header 'Content-Type: application/json' \
  --data "$update_payload" 2>&1)

# Extract HTTP status code and body
http_status=$(echo "$response" | grep -o "HTTP_STATUS:[0-9]*" | cut -d: -f2)
body=$(echo "$response" | sed '/HTTP_STATUS:/d')

# Check if request failed
if [[ -z "$http_status" ]] || [[ "$http_status" -ge 400 ]]; then
  echo "[error] Failed to add quality gate conditions" >&2
  echo "[error] HTTP Status: ${http_status:-unknown}" >&2
  echo "[error] Response body:" >&2
  echo "$body" | jq '.' >&2 || echo "$body" >&2
  
  # Try to parse and show individual condition errors if available
  if echo "$body" | jq -e '.error' >/dev/null 2>&1; then
    error_msg=$(echo "$body" | jq -r '.error // empty')
    if [[ -n "$error_msg" ]]; then
      echo "[error] Error message: $error_msg" >&2
    fi
  fi
  
  exit 1
else
  echo "[success] Quality gate conditions added successfully" >&2
  echo "$body" | jq '.' || echo "$body"
fi

echo "[done] Quality gate update complete."


