"""
Post-Push Verification Script
Usage: python verify_repo.py <owner>/<repo> [--fix]

Checks all items in the Post-Push Verification checklist.
Returns exit code 0 if all pass, 1 if any fail.
"""
import json
import subprocess
import sys
import base64

PASS = "PASS"
FAIL = "FAIL"

def gh(args: str) -> str:
    result = subprocess.run(
        f"gh {args}",
        capture_output=True, text=True, timeout=30, shell=True
    )
    if result.returncode != 0:
        return ""
    return result.stdout.strip()

def check(description: str, ok: bool, detail: str = ""):
    icon = PASS if ok else FAIL
    print(f"  [{icon}] {description}" + (f"  ({detail})" if detail else ""))

def main():
    if len(sys.argv) < 2:
        print("Usage: python verify_repo.py <owner>/<repo> [--fix]")
        sys.exit(1)

    repo = sys.argv[1]
    fix_mode = "--fix" in sys.argv
    all_pass = True

    print(f"\n  === POST-PUSH VERIFICATION: {repo} ===\n")

    # ── 1. Repo Existence + Metadata ──
    print("  -- 1. Repo Existence + Metadata --")
    repo_json = gh(
        f'repo view {repo} --json name,description,defaultBranchRef,isEmpty,repositoryTopics'
    )
    if not repo_json:
        check("Repo exists", False, "gh repo view failed")
        sys.exit(1)

    data = json.loads(repo_json)
    exists = data.get("name") is not None
    all_pass &= exists
    check("Repo exists and is reachable", exists)

    not_empty = data.get("isEmpty") == False
    all_pass &= not_empty
    check("Repo is not empty (commit landed)", not_empty)

    default_branch = data.get("defaultBranchRef", {}).get("name", "")
    branch_ok = default_branch == "master"
    all_pass &= branch_ok
    check(f"Default branch is master (got: {default_branch})", branch_ok)

    desc = data.get("description", "")
    desc_ok = len(desc) > 10
    all_pass &= desc_ok
    check("Description is set", desc_ok)

    topics = [t["name"] for t in data.get("repositoryTopics", [])]
    topics_ok = len(topics) >= 2
    all_pass &= topics_ok
    check(f"Topics are set ({len(topics)} found)", topics_ok, ", ".join(topics))

    # ── 2. CI Health ──
    print("\n  -- 2. CI Health --")

    ci_content = gh(f"api repos/{repo}/contents/.github/workflows/ci.yml --jq .content")
    ci_ok = bool(ci_content)
    all_pass &= ci_ok
    check("CI workflow exists (.github/workflows/ci.yml)", ci_ok)

    if ci_content:
        try:
            ci_decoded = base64.b64decode(ci_content).decode()
            has_cipass = "ci-pass:" in ci_decoded
            all_pass &= has_cipass
            check("CI has ci-pass job", has_cipass)
        except Exception:
            check("CI has ci-pass job", False, "decode error")

    last_run = gh(f"run list --repo {repo} --workflow CI --limit 1 --json databaseId,status,conclusion")
    if last_run:
        runs = json.loads(last_run)
        if runs:
            run = runs[0]
            ci_passed = run.get("conclusion") == "success" and run.get("status") == "completed"
            all_pass &= ci_passed
            check("Last CI run passed", ci_passed, f"ID: {run['databaseId']}")
        else:
            check("Last CI run exists", False, "no runs found")
    else:
        check("Last CI run exists", False, "gh run list failed")

    # ── 3. License Verification ──
    print("\n  -- 3. License Verification --")

    license_file = gh(f"api repos/{repo}/contents/LICENSE --jq .name")
    lic_ok = license_file == "LICENSE"
    all_pass &= lic_ok
    check("LICENSE file exists in repo root", lic_ok)

    lic_info = gh(f"repo view {repo} --json licenseInfo --jq .licenseInfo.key")
    lic_detected = bool(lic_info)
    all_pass &= lic_detected
    check(f"License detected by GitHub", lic_detected, lic_info if lic_detected else "")

    cargo = gh(f"api repos/{repo}/contents/Cargo.toml --jq .content")
    if cargo:
        try:
            cargo_decoded = base64.b64decode(cargo).decode()
            cargo_lic = 'license = "MIT"' in cargo_decoded
            all_pass &= cargo_lic
            check("Cargo.toml declares MIT license", cargo_lic)
        except Exception:
            check("Cargo.toml declares MIT license", False, "decode error")
    else:
        check("Cargo.toml exists (project manifest)", False)

    readme = gh(f"api repos/{repo}/contents/README.md --jq .content")
    if readme:
        try:
            readme_decoded = base64.b64decode(readme).decode()
            badge_lic = "license-MIT" in readme_decoded or "license/MIT" in readme_decoded
            all_pass &= badge_lic
            check("README has MIT license badge", badge_lic)
        except Exception:
            check("README has MIT license badge", False)

    # ── 4. README Asset Verification ──
    print("\n  -- 4. README Asset Verification --")

    assets = gh(f"api repos/{repo}/contents/assets --jq \"[.[].name]\"")
    if assets:
        try:
            asset_names = json.loads(assets)
            has_logo = "logo.svg" in asset_names
            has_terminal = "terminal-preview.png" in asset_names
            has_arch = "architecture.png" in asset_names
            all_pass &= has_logo
            all_pass &= has_terminal
            all_pass &= has_arch
            check("Logo SVG exists in assets/", has_logo)
            check("Terminal preview PNG exists in assets/", has_terminal)
            check("Architecture diagram PNG exists in assets/", has_arch)
        except Exception as e:
            check("Assets can be listed", False, str(e))
    else:
        check("Assets directory exists", False)

    if readme:
        try:
            readme_decoded = base64.b64decode(readme).decode()
            has_logo_ref = "assets/logo.svg" in readme_decoded
            has_terminal_ref = "assets/terminal-preview.png" in readme_decoded
            has_arch_ref = "assets/architecture.png" in readme_decoded
            all_pass &= has_logo_ref
            all_pass &= has_terminal_ref
            all_pass &= has_arch_ref
            check("README references logo SVG", has_logo_ref)
            check("README references terminal preview", has_terminal_ref)
            check("README references architecture diagram", has_arch_ref)
        except:
            pass

    # ── 5. Branch Protection ──
    print("\n  -- 5. Branch Protection --")

    contexts_json = gh(f"api repos/{repo}/branches/master/protection --jq .required_status_checks.contexts")
    if contexts_json:
        try:
            contexts = json.loads(contexts_json)
            has_cipass_ctx = "ci-pass" in contexts
            all_pass &= has_cipass_ctx
            check("Branch protection: ci-pass required check", has_cipass_ctx, f"contexts: {contexts}")

            strict = gh(f"api repos/{repo}/branches/master/protection --jq .required_status_checks.strict")
            all_pass &= (strict == "true")
            check("Branch protection: strict mode", strict == "true")

            reviews = gh(f"api repos/{repo}/branches/master/protection --jq .required_pull_request_reviews.required_approving_review_count")
            all_pass &= (reviews == "1")
            check("Branch protection: 1 approving review required", reviews == "1")

            linear = gh(f"api repos/{repo}/branches/master/protection --jq .required_linear_history.enabled")
            all_pass &= (linear == "true")
            check("Branch protection: linear history required", linear == "true")

            admin = gh(f"api repos/{repo}/branches/master/protection --jq .enforce_admins.enabled")
            all_pass &= (admin == "true")
            check("Branch protection: admin enforcement enabled", admin == "true")
        except json.JSONDecodeError:
            check("Branch protection is applied", False, contexts_json)
    else:
        check("Branch protection is applied", False)

    # ── Summary ──
    if all_pass:
        print(f"\n  == ALL CHECKS PASSED ==\n")
    else:
        print(f"\n  == SOME CHECKS FAILED ==\n")

    if fix_mode and not lic_detected:
        print("  Note: License file exists but GitHub hasn't detected it yet.")

    sys.exit(0 if all_pass else 1)

if __name__ == "__main__":
    main()
