# Installs packages listed in ~/.apt_packages.txt

set -euo pipefail
sudo apt update
grep -v '^\s*#' ~/.apt_packages.txt | sed 's/#.*//' | xargs sudo apt install -y