#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

usage() {
    cat <<'EOF'
Usage:
  scripts/check_reference_links.sh
  scripts/check_reference_links.sh --compare-to-ref <ref>

Options:
  --compare-to-ref   Check only reference_link URLs that are present in the
                     current branch but not present on the specified ref.
  -h, --help         Show this help and exit.

Exit codes:
  0 if all checked links are valid.
  1 if any checked link failed (HTTP error, anchor mismatch, URL format error,
    or network error).
  2 on usage/configuration errors.
EOF
}

fatal() {
    >&2 printf 'error: %s\n' "$1"
    exit 2
}

TOPLEVEL="$(git rev-parse --show-toplevel)"
cd "$TOPLEVEL"

COMPARE_REF=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --compare-to-ref)
            shift
            [[ $# -gt 0 ]] || fatal "missing value for --compare-to-ref"
            [[ -z "$COMPARE_REF" ]] || fatal "--compare-to-ref was provided more than once"
            COMPARE_REF="$1"
            shift
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            fatal "unknown argument: $1"
            ;;
    esac
done

extract_current_links() {
    # Prints tab-separated records:
    #   <file_path>\t<url>
    grep -H -E 'reference_link:[[:space:]]*Some\(".*"\)' src/lints/*.ron \
        | sed -E 's#^([^:]+):.*reference_link:[[:space:]]*Some\("([^"]+)"\).*#\1\t\2#'
}

extract_links_from_ref() {
    # Prints tab-separated records:
    #   <file_path>\t<url>
    local ref="$1"
    local file=""

    git rev-parse --verify "${ref}^{commit}" >/dev/null 2>&1 \
        || fatal "base ref '$ref' not found. Try fetching it first."

    while IFS= read -r file; do
        git show "${ref}:${file}" \
            | sed -n -E "s#.*reference_link:[[:space:]]*Some\\(\"([^\"]+)\"\\).*#${file}\t\\1#p"
    done < <(git ls-tree -r --name-only "$ref" -- src/lints | grep -E '\.ron$' || true)
}

fetch_url() {
    # Args: <url> <body_file> <err_file>
    local url="$1"
    local body_file="$2"
    local err_file="$3"

    curl -L \
        --connect-timeout 15 \
        --max-time 90 \
        --retry 2 \
        --retry-delay 1 \
        --retry-all-errors \
        --compressed \
        -sS \
        -o "$body_file" \
        -w '%{http_code}' \
        "$url" 2>"$err_file" || true
}

extract_meta_refresh_target() {
    # Args: <body_file>
    # Prints URL target if present, otherwise empty.
    local body_file="$1"

    grep -Eio "content=['\"]0;[[:space:]]*URL=[^'\"]+['\"]" "$body_file" \
        | head -n 1 \
        | sed -E "s#.*URL=([^'\"]+)['\"].*#\\1#"
}

resolve_relative_url() {
    # Args: <base_url> <relative_or_absolute_url>
    local base_url="$1"
    local target="$2"
    local origin=""
    local directory=""

    if [[ "$target" =~ ^https?:// ]]; then
        printf '%s\n' "$target"
        return
    fi

    origin="$(printf '%s' "$base_url" | sed -E 's#^(https?://[^/]+).*$#\1#')"
    if [[ "$target" == /* ]]; then
        printf '%s%s\n' "$origin" "$target"
        return
    fi

    directory="${base_url%/*}"
    printf '%s/%s\n' "$directory" "$target"
}

anchor_exists_in_body() {
    # Args: <fragment_without_hash> <body_file>
    local fragment="$1"
    local body_file="$2"

    grep -F -q "id=\"$fragment\"" "$body_file" \
        || grep -F -q "id='$fragment'" "$body_file" \
        || grep -F -q "name=\"$fragment\"" "$body_file" \
        || grep -F -q "name='$fragment'" "$body_file" \
        || grep -F -q "href=\"#$fragment\"" "$body_file" \
        || grep -F -q "href='#$fragment'" "$body_file" \
        || grep -F -q "#$fragment" "$body_file"
}

validate_url_structure() {
    # Args: <url>
    # Prints one of:
    #   missing_scheme
    #   missing_hostname
    # on failure. Prints nothing on success.
    local url="$1"
    local url_without_fragment="$url"
    local after_scheme=""
    local authority=""
    local hostport=""
    local host=""

    if [[ "$url_without_fragment" == *#* ]]; then
        url_without_fragment="${url_without_fragment%%#*}"
    fi

    if [[ ! "$url_without_fragment" =~ ^[A-Za-z][A-Za-z0-9+.-]*:// ]]; then
        printf 'missing_scheme\n'
        return 1
    fi

    after_scheme="${url_without_fragment#*://}"
    authority="${after_scheme%%/*}"
    authority="${authority%%\?*}"

    if [[ -z "$authority" ]]; then
        printf 'missing_hostname\n'
        return 1
    fi

    hostport="$authority"
    if [[ "$hostport" == *"@"* ]]; then
        hostport="${hostport##*@}"
    fi

    host="$hostport"
    if [[ "$host" == \[*\]* ]]; then
        host="${host#\[}"
        host="${host%%\]*}"
    elif [[ "$host" == *:* ]]; then
        host="${host%%:*}"
    fi

    if [[ -z "$host" ]]; then
        printf 'missing_hostname\n'
        return 1
    fi
}

current_map="$(mktemp)"
target_map="$(mktemp)"
base_map="$(mktemp)"
target_urls="$(mktemp)"
validation_results="$(mktemp)"

cleanup() {
    rm -f "$current_map" "$target_map" "$base_map" "$target_urls" "$validation_results"
}
trap cleanup EXIT

extract_current_links >"$current_map"

if [[ -z "$COMPARE_REF" ]]; then
    cp "$current_map" "$target_map"
else
    extract_links_from_ref "$COMPARE_REF" >"$base_map"

    comm -23 \
        <(cut -f2 "$current_map" | LC_ALL=C sort -u) \
        <(cut -f2 "$base_map" | LC_ALL=C sort -u) \
        >"$target_urls"

    if [[ ! -s "$target_urls" ]]; then
        printf 'No new reference_link URLs relative to %s; nothing to check.\n' "$COMPARE_REF"
        exit 0
    fi

    awk -F '\t' 'NR==FNR { keep[$1] = 1; next } ($2 in keep) { print }' \
        "$target_urls" "$current_map" >"$target_map"
fi

total_entries="$(wc -l <"$target_map" | tr -d ' ')"
total_unique_urls="$(cut -f2 "$target_map" | LC_ALL=C sort -u | wc -l | tr -d ' ')"

if [[ -z "$COMPARE_REF" ]]; then
    printf 'Mode: all\n'
else
    printf 'Mode: compare-to-ref\n'
    printf 'Compare ref: %s\n' "$COMPARE_REF"
fi
printf 'Checking %s reference_link entries (%s unique URL(s)).\n' "$total_entries" "$total_unique_urls"

while IFS= read -r url; do
    base_url="$url"
    fragment=""
    current_url=""
    body_file=""
    err_file=""
    http_code=""
    message=""
    refresh_target=""
    refresh_hops=0
    structure_error=""

    structure_error="$(validate_url_structure "$url" || true)"
    if [[ -n "$structure_error" ]]; then
        if [[ "$structure_error" == "missing_scheme" ]]; then
            printf 'invalid_url_missing_scheme\t%s\tURL is missing a scheme (expected e.g. https://...)\n' "$url" \
                >>"$validation_results"
        else
            printf 'invalid_url_missing_hostname\t%s\tURL is missing a hostname\n' "$url" \
                >>"$validation_results"
        fi
        continue
    fi

    if [[ "$url" == *#* ]]; then
        base_url="${url%%#*}"
        fragment="${url#*#}"
    fi
    current_url="$base_url"
    body_file="$(mktemp)"
    err_file="$(mktemp)"

    http_code="$(fetch_url "$current_url" "$body_file" "$err_file")"
    if [[ ! "$http_code" =~ ^[0-9]{3}$ ]] || [[ "$http_code" == "000" ]]; then
        message="$(head -n 1 "$err_file" | tr '\t' ' ')"
        [[ -n "$message" ]] || message="curl failed"
        printf 'network_error\t%s\t%s\n' "$url" "$message" >>"$validation_results"
        rm -f "$body_file" "$err_file"
        continue
    fi

    while (( refresh_hops < 4 )); do
        refresh_target="$(extract_meta_refresh_target "$body_file" || true)"
        if [[ -z "$refresh_target" ]]; then
            break
        fi

        current_url="$(resolve_relative_url "$current_url" "$refresh_target")"
        http_code="$(fetch_url "$current_url" "$body_file" "$err_file")"
        if [[ ! "$http_code" =~ ^[0-9]{3}$ ]] || [[ "$http_code" == "000" ]]; then
            message="$(head -n 1 "$err_file" | tr '\t' ' ')"
            [[ -n "$message" ]] || message="curl failed after HTML redirect"
            printf 'network_error\t%s\t%s\n' "$url" "$message" >>"$validation_results"
            break
        fi

        refresh_hops=$((refresh_hops + 1))
    done

    if [[ ! "$http_code" =~ ^[0-9]{3}$ ]] || [[ "$http_code" == "000" ]]; then
        rm -f "$body_file" "$err_file"
        continue
    fi

    code_num=$((10#$http_code))
    if (( code_num >= 400 )); then
        if (( code_num == 404 )); then
            printf 'http_404\t%s\tHTTP %s\n' "$url" "$http_code" >>"$validation_results"
        elif (( code_num >= 500 )); then
            printf 'http_5xx\t%s\tHTTP %s\n' "$url" "$http_code" >>"$validation_results"
        else
            printf 'http_4xx\t%s\tHTTP %s\n' "$url" "$http_code" >>"$validation_results"
        fi
        rm -f "$body_file" "$err_file"
        continue
    fi

    if [[ -n "$fragment" ]] && ! anchor_exists_in_body "$fragment" "$body_file"; then
        printf 'bad_anchor\t%s\tfragment #%s not found in response body\n' "$url" "$fragment" \
            >>"$validation_results"
    fi

    rm -f "$body_file" "$err_file"
done < <(cut -f2 "$target_map" | LC_ALL=C sort -u)

if [[ ! -s "$validation_results" ]]; then
    printf 'All checked reference_link URLs are reachable, and all anchors are present.\n'
    exit 0
fi

printf 'Found %s failing URL(s).\n' "$(wc -l <"$validation_results" | tr -d ' ')"
printf '\n'

while IFS= read -r kind; do
    printf '[%s]\n' "$kind"
    while IFS=$'\t' read -r _ url detail; do
        printf '%s\n' "$url"
        printf '  files:\n'
        while IFS= read -r file; do
            printf '    - %s\n' "$file"
        done < <(awk -F '\t' -v target="$url" '$2 == target { print $1 }' "$target_map" | LC_ALL=C sort -u)
        printf '  detail: %s\n' "$detail"
    done < <(awk -F '\t' -v target_kind="$kind" '$1 == target_kind { print }' "$validation_results" \
        | LC_ALL=C sort -t $'\t' -k2,2 -k3,3)
    printf '\n'
done < <(cut -f1 "$validation_results" | LC_ALL=C sort -u)

exit 1
