import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DIST_DIR = REPO_ROOT / "dist"
VERSION_PATTERN = r'^version\s*=\s*"([^"]+)"'


def run(args):
    print("$", " ".join(args))
    result = subprocess.run(args, cwd=REPO_ROOT)
    if result.returncode != 0:
        sys.exit(result.returncode)


def find_maturin():
    repo_venv = REPO_ROOT.parent.parent / ".venv"
    for candidate in (repo_venv / "Scripts" / "maturin.exe", repo_venv / "bin" / "maturin"):
        if candidate.exists():
            return str(candidate)
    return "maturin"


def read_version(path):
    match = re.search(VERSION_PATTERN, path.read_text(encoding="utf-8"), re.MULTILINE)
    return match.group(1) if match else None


def checked_version():
    cargo_version = read_version(REPO_ROOT / "Cargo.toml")
    pyproject_version = read_version(REPO_ROOT / "pyproject.toml")
    if cargo_version != pyproject_version:
        sys.exit(f"version mismatch: Cargo.toml={cargo_version} pyproject.toml={pyproject_version}")
    return cargo_version


def main():
    parser = argparse.ArgumentParser(
        description="Build seraplot from this machine's full source (including the sera-pulse/sera-secure "
        "modules that are gitignored and never reach CI) and upload it to PyPI. Set MATURIN_PYPI_TOKEN "
        "before running this."
    )
    parser.add_argument("--skip-build", action="store_true", help="upload whatever is already in dist/ instead of rebuilding")
    parser.add_argument("--repository", default="pypi", help="maturin repository name, e.g. testpypi")
    args = parser.parse_args()

    version = checked_version()
    maturin = find_maturin()
    print(f"seraplot {version} via {maturin}")

    if not args.skip_build:
        DIST_DIR.mkdir(exist_ok=True)
        run([maturin, "build", "--release", "--out", str(DIST_DIR)])

    wheels = sorted(str(p) for p in DIST_DIR.glob("*.whl"))
    if not wheels:
        sys.exit(f"no wheels found in {DIST_DIR} -- run without --skip-build first")

    run([maturin, "upload", "--repository", args.repository, "--skip-existing", *wheels])


if __name__ == "__main__":
    main()
