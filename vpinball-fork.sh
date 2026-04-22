#!/usr/bin/env bash
#
# vpinball-fork.sh — Sync, build, and release VPinballX from a GitHub fork.
#
# This script manages a fork of vpinball/vpinball that keeps CI workflows
# set to manual dispatch (workflow_dispatch) instead of push triggers.
#
# Usage:
#   ./vpinball-fork.sh sync       Sync fork with upstream + patch CI + trigger builds
#   ./vpinball-fork.sh release    Create a GitHub Release from successful build artifacts
#   ./vpinball-fork.sh status     Show the state of recent workflow runs
#
# Prerequisites:
#   - gh CLI installed and authenticated
#   - jq installed
#   - A fork of vpinball/vpinball on your GitHub account
#
# The fork repo is auto-detected from your GitHub username.

set -euo pipefail

UPSTREAM_REPO="vpinball/vpinball"
UPSTREAM_BRANCH="master"

# Upstream commits to revert from every sync. Use this to keep tracking
# master (for fresh features / SDL3 fixes / X11 improvements) while rolling
# back specific known-broken commits. Each entry must be a full SHA from
# vpinball/vpinball@master.
#
# Revert order is blacklist order. If reverting any entry conflicts (a
# later upstream commit touches the same code), sync bails with a clear
# error — update the blacklist or drop the entry and pin instead.
BLACKLIST=(
    # dbd2a00 "BGFX: deprecate GLES option and remove fullscreen exclusive"
    # Removed the FSE code path; on X11/Mutter the surviving borderless
    # fullscreen path doesn't honor multi-monitor placement as reliably,
    # and the cabinet PF window vanishes after the update.
    "dbd2a00aaf32cedaae441f00f3de5a59b2218bb4"
)

# CI workflow files to patch (push -> workflow_dispatch)
WORKFLOW_FILES=(
    ".github/workflows/vpinball.yml"
    ".github/workflows/vpinball-sbc.yml"
)

# Build workflow names to trigger after sync
BUILD_WORKFLOWS=(
    "vpinball"
    "vpinball-sbc"
)

# Prerelease workflow that creates the GitHub Release
PRERELEASE_WORKFLOW="prerelease"

#--- Helpers -------------------------------------------------------------------

die() { echo "ERROR: $*" >&2; exit 1; }
info() { echo ":: $*"; }

get_fork_repo() {
    local user
    user=$(gh api user --jq '.login') || die "Cannot get GitHub username. Is gh authenticated?"
    echo "${user}/${UPSTREAM_REPO#*/}"
}

# Apply the commit BLACKLIST as `git revert` commits on top of the fork's
# master. Uses a throwaway local clone because GitHub's REST API can't do
# revert server-side. Each revert is `--no-edit` for non-interactive use;
# a conflict bails out loudly so the user updates the blacklist rather
# than shipping a half-broken fork.
#
# Usage: apply_blacklist <fork_repo>
# Preconditions: fork master is already reset to the target upstream SHA;
# `gh auth setup-git` has been run at least once so git can push via the
# gh-stored token (or the user has an SSH remote — we use HTTPS here).
apply_blacklist() {
    local fork_repo="$1"

    if (( ${#BLACKLIST[@]} == 0 )); then
        return 0
    fi

    info "Applying ${#BLACKLIST[@]} blacklisted revert(s) locally..."

    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf ${tmpdir}" RETURN

    # Shallow-ish clone — enough depth to reach every blacklisted commit.
    # 500 covers roughly 3 months of upstream activity; bump if needed.
    info "  Shallow-cloning ${fork_repo}..."
    git clone --quiet --depth 500 \
        "https://github.com/${fork_repo}.git" "${tmpdir}/repo" \
        || die "Clone of fork failed"

    # Keep commits we need to cherry-pick within reach: fetch upstream
    # shallowly so blacklisted SHAs can be revert-applied (git revert
    # needs both the commit and its parent).
    (cd "${tmpdir}/repo" &&
        git remote add upstream "https://github.com/${UPSTREAM_REPO}.git" &&
        git fetch --quiet --depth 500 upstream "${UPSTREAM_BRANCH}" &&
        git config user.name "vpinball-fork.sh" &&
        git config user.email "fork-bot@localhost") \
        || die "Fork repo setup failed"

    # Apply reverts. Each may ff-fail if upstream has since rewritten the
    # same code — we abort the partial revert and bail so the user can
    # update the blacklist deliberately.
    for sha in "${BLACKLIST[@]}"; do
        info "  Reverting ${sha:0:7}..."
        if ! (cd "${tmpdir}/repo" && git cat-file -e "${sha}^{commit}" 2>/dev/null); then
            info "    ${sha:0:7} not in fetched history — skipping (not yet in upstream?)"
            continue
        fi
        # Also skip if the commit isn't an ancestor of our current HEAD
        # (fork master may be pinned to before this SHA).
        if ! (cd "${tmpdir}/repo" && git merge-base --is-ancestor "${sha}" HEAD 2>/dev/null); then
            info "    ${sha:0:7} not in fork history — skipping"
            continue
        fi
        if ! (cd "${tmpdir}/repo" && git revert --no-edit "${sha}" >/dev/null 2>&1); then
            (cd "${tmpdir}/repo" && git revert --abort 2>/dev/null) || true
            die "Revert of ${sha:0:7} conflicts with later upstream work. Drop it from BLACKLIST and use 'sync <older-sha>' to pin instead."
        fi
        info "    Reverted."
    done

    # Force-push back.
    info "  Force-pushing reverted master to ${fork_repo}..."
    (cd "${tmpdir}/repo" && git push --quiet --force origin "HEAD:${UPSTREAM_BRANCH}") \
        || die "Push to fork failed — have you run 'gh auth setup-git'?"

    info "  Blacklist applied."
}

# Wait for the most recent run of a workflow to complete (max ~30 min).
# Usage: wait_for_workflow <fork_repo> <workflow_name>
wait_for_workflow() {
    local fork_repo="$1" workflow="$2"
    local run_id status elapsed=0 interval=30 max_wait=2400

    info "Waiting for '${workflow}' to complete..."

    # Find the most recent run
    run_id=$(gh run list --repo "${fork_repo}" --workflow="${workflow}.yml" --limit 1 \
        --json databaseId --jq '.[0].databaseId') \
        || die "Cannot find run for ${workflow}"

    while (( elapsed < max_wait )); do
        status=$(gh run view "${run_id}" --repo "${fork_repo}" --json status,conclusion \
            --jq 'if .status == "completed" then .conclusion else .status end')

        case "$status" in
            success)
                info "  '${workflow}' completed successfully."
                return 0
                ;;
            failure|cancelled|timed_out)
                die "'${workflow}' run ${run_id} ended with: ${status}"
                ;;
            *)
                printf "  [%dm%02ds] Status: %s\r" $((elapsed/60)) $((elapsed%60)) "$status"
                sleep "$interval"
                elapsed=$((elapsed + interval))
                ;;
        esac
    done
    die "Timed out waiting for '${workflow}' (${max_wait}s)"
}

#--- Commands ------------------------------------------------------------------

cmd_sync() {
    local fork_repo="$1"
    local pin_sha="${2:-}"

    info "Syncing ${fork_repo} with ${UPSTREAM_REPO}/${UPSTREAM_BRANCH}..."

    # Get target SHA: either a user-pinned commit (to work around upstream
    # regressions like dbd2a00 "remove fullscreen exclusive") or the
    # current upstream HEAD.
    local upstream_sha
    if [[ -n "$pin_sha" ]]; then
        info "Pin mode: using explicit SHA ${pin_sha:0:7} instead of upstream HEAD"
        upstream_sha=$(gh api "repos/${UPSTREAM_REPO}/commits/${pin_sha}" --jq '.sha') \
            || die "Cannot resolve pinned SHA ${pin_sha} on ${UPSTREAM_REPO}"
    else
        upstream_sha=$(gh api "repos/${UPSTREAM_REPO}/commits/${UPSTREAM_BRANCH}" --jq '.sha') \
            || die "Cannot fetch upstream HEAD"
    fi
    local upstream_short="${upstream_sha:0:7}"

    # Force-reset fork master to upstream
    info "Resetting fork master to upstream (${upstream_short})..."
    gh api "repos/${fork_repo}/git/refs/heads/${UPSTREAM_BRANCH}" \
        --method PATCH --input - <<EOF >/dev/null
{"sha": "${upstream_sha}", "force": true}
EOF

    # Apply the blacklist — reverts of specific upstream commits we want
    # to keep out of our builds. Runs BEFORE the CI patches so the revert
    # commits are part of the "code" history, and the CI-only patches sit
    # on top (separate, easy to rebase if anything goes wrong).
    apply_blacklist "${fork_repo}"

    # Apply CI patches: change "push:" trigger to "workflow_dispatch:"
    for workflow in "${WORKFLOW_FILES[@]}"; do
        info "Patching ${workflow}..."

        local file_data
        file_data=$(gh api "repos/${fork_repo}/contents/${workflow}" \
            --jq '{sha: .sha, content: .content}') \
            || die "Cannot fetch ${workflow}"

        local file_sha content patched
        file_sha=$(echo "$file_data" | jq -r '.sha')
        content=$(echo "$file_data" | jq -r '.content' | base64 -d)

        if echo "$content" | head -3 | grep -q "workflow_dispatch:"; then
            info "  Already patched, skipping."
            continue
        fi

        patched=$(echo "$content" | sed '/^on:/{n;s/^  push:$/  workflow_dispatch:/;}')

        local encoded wf_name
        encoded=$(echo "$patched" | base64 -w 0)
        wf_name=$(basename "$workflow")

        gh api "repos/${fork_repo}/contents/${workflow}" \
            --method PUT --input - <<EOF >/dev/null
{
    "message": "ci: set ${wf_name} to manual dispatch",
    "content": "${encoded}",
    "sha": "${file_sha}"
}
EOF
        info "  Committed."
    done

    # Patch prerelease.yml: use latest successful run instead of commit-filtered
    info "Patching prerelease.yml (use latest successful run)..."
    local pre_data
    pre_data=$(gh api "repos/${fork_repo}/contents/.github/workflows/prerelease.yml" \
        --jq '{sha: .sha, content: .content}') \
        || die "Cannot fetch prerelease.yml"

    local pre_sha pre_content pre_patched
    pre_sha=$(echo "$pre_data" | jq -r '.sha')
    pre_content=$(echo "$pre_data" | jq -r '.content' | base64 -d)

    if echo "$pre_content" | grep -q -- '--status=success --limit=1'; then
        info "  Already patched, skipping."
    else
        # Remove --commit= filter so it picks the latest successful run
        pre_patched=$(echo "$pre_content" | sed 's/--status=success --commit=\${{ needs.version.outputs.sha }} --limit=1/--status=success --limit=1/')

        local pre_encoded
        pre_encoded=$(echo "$pre_patched" | base64 -w 0)
        gh api "repos/${fork_repo}/contents/.github/workflows/prerelease.yml" \
            --method PUT --input - <<EOF >/dev/null
{
    "message": "ci: prerelease uses latest successful build run",
    "content": "${pre_encoded}",
    "sha": "${pre_sha}"
}
EOF
        info "  Committed."
    fi

    # Trigger builds
    echo ""
    for workflow in "${BUILD_WORKFLOWS[@]}"; do
        info "Dispatching '${workflow}' build..."
        gh workflow run "${workflow}" \
            --repo "${fork_repo}" \
            --ref "${UPSTREAM_BRANCH}" \
            || die "Failed to dispatch ${workflow}"
        info "  Dispatched."
    done

    echo ""
    info "Sync complete! Builds are running."
    info "  Upstream: ${upstream_short}"
    info ""
    info "Monitor with:  $0 status"
    info "When ready:    $0 release"
}

cmd_release() {
    local fork_repo="$1"

    info "Preparing release for ${fork_repo}..."
    echo ""

    # Wait for both builds to succeed
    wait_for_workflow "$fork_repo" "vpinball"
    wait_for_workflow "$fork_repo" "vpinball-sbc"

    echo ""
    info "All builds successful. Triggering prerelease..."

    # Dispatch the prerelease workflow (creates a GitHub Release from vpinball artifacts)
    gh workflow run "${PRERELEASE_WORKFLOW}" \
        --repo "${fork_repo}" \
        --ref "${UPSTREAM_BRANCH}" \
        || die "Failed to dispatch ${PRERELEASE_WORKFLOW}"

    info "Prerelease workflow dispatched."
    echo ""

    # Wait for prerelease to complete
    sleep 5  # Give GitHub a moment to register the run
    wait_for_workflow "$fork_repo" "${PRERELEASE_WORKFLOW}"

    # Find the release that was just created
    local release_tag
    release_tag=$(gh api "repos/${fork_repo}/releases" --jq '.[0].tag_name') \
        || die "Cannot find release"

    echo ""
    info "Now uploading vpinball-sbc artifacts to the release..."

    # Find the latest successful vpinball-sbc run
    local sbc_run_id
    sbc_run_id=$(gh run list --repo "${fork_repo}" --workflow="vpinball-sbc.yml" \
        --status=success --limit 1 --json databaseId --jq '.[0].databaseId') \
        || die "Cannot find successful vpinball-sbc run"

    # Download SBC artifacts via API and upload to the release
    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf ${tmpdir}" EXIT

    info "Downloading vpinball-sbc artifacts..."

    # Get artifact IDs for the SBC run
    local artifacts
    artifacts=$(gh api "repos/${fork_repo}/actions/runs/${sbc_run_id}/artifacts" \
        --jq '.artifacts[] | "\(.id) \(.name)"') \
        || die "Cannot list SBC artifacts"

    while IFS=' ' read -r artifact_id artifact_name; do
        [ -z "$artifact_id" ] && continue
        info "  Downloading ${artifact_name}..."
        # GitHub wraps artifacts in zip; for tar.gz artifacts the zip is just the raw file
        gh api "repos/${fork_repo}/actions/artifacts/${artifact_id}/zip" \
            > "${tmpdir}/${artifact_name}" 2>/dev/null \
            || { info "  Warning: failed to download ${artifact_name}"; continue; }

        info "  Uploading ${artifact_name}..."
        gh release upload "${release_tag}" "${tmpdir}/${artifact_name}" \
            --repo "${fork_repo}" --clobber \
            || info "  Warning: failed to upload ${artifact_name}"
    done <<< "$artifacts"

    echo ""
    info "Release ${release_tag} is ready!"
    info "  https://github.com/${fork_repo}/releases/tag/${release_tag}"
}

cmd_status() {
    local fork_repo="$1"

    info "Recent workflow runs for ${fork_repo}:"
    echo ""

    for workflow in "${BUILD_WORKFLOWS[@]}" "${PRERELEASE_WORKFLOW}"; do
        echo "--- ${workflow} ---"
        gh run list --repo "${fork_repo}" --workflow="${workflow}.yml" --limit 3 \
            --json status,conclusion,createdAt,headSha,databaseId \
            --jq '.[] | "  \(.status)\t\(.conclusion // "-")\t\(.createdAt[0:16])\t\(.headSha[0:7])\t#\(.databaseId)"'
        echo ""
    done

    # Show latest release
    local latest_release
    latest_release=$(gh api "repos/${fork_repo}/releases" --jq '.[0] | "\(.tag_name) (\(.created_at[0:10])) — \(.assets | length) assets"' 2>/dev/null) \
        || latest_release="(none)"
    info "Latest release: ${latest_release}"
}

#--- Main ----------------------------------------------------------------------

main() {
    local action="${1:-}"

    if [[ -z "$action" ]]; then
        echo "Usage: $0 {sync [SHA]|release|status}"
        echo ""
        echo "  sync [SHA]    Sync fork with upstream, patch CI, trigger builds."
        echo "                Optional SHA pins the fork to a specific upstream"
        echo "                commit (accepts '<sha>^' for parent) — useful to"
        echo "                roll back a breaking upstream change."
        echo "  release       Wait for builds, create GitHub Release with all artifacts"
        echo "  status        Show recent workflow runs and latest release"
        exit 1
    fi

    command -v gh >/dev/null 2>&1 || die "'gh' CLI not found. Install from https://cli.github.com"
    command -v jq >/dev/null 2>&1 || die "'jq' not found. Install with: sudo apt install jq"
    # git + a gh-credential-helper hookup are only strictly required when
    # BLACKLIST is non-empty (the revert-and-push step), but checking
    # upfront gives a clearer error than a mid-run failure.
    if (( ${#BLACKLIST[@]} > 0 )); then
        command -v git >/dev/null 2>&1 || die "'git' not found. Install with: sudo apt install git"
        # Verify `gh auth setup-git` has been run, otherwise the force-push
        # inside apply_blacklist will prompt interactively for credentials.
        if ! git config --global --get-regexp '^credential\..*helper$' | grep -q 'gh'; then
            info "Note: run 'gh auth setup-git' once so blacklist pushes don't hang on auth."
        fi
    fi

    local fork_repo
    fork_repo=$(get_fork_repo)
    info "Fork: ${fork_repo}"
    echo ""

    case "$action" in
        sync)    cmd_sync "$fork_repo" "${2:-}" ;;
        release) cmd_release "$fork_repo" ;;
        status)  cmd_status "$fork_repo" ;;
        *)       die "Unknown action: ${action}. Use: sync [SHA], release, or status" ;;
    esac
}

main "$@"
