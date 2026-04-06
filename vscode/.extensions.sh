# Installs packages listed in $VSCODE_USER_HOME/.extensions.txt

set -euo pipefail
grep -v '^\s*#' "$VSCODE_HOME/.extensions.txt" | sed 's/#.*//' | xargs -n 1 -I {} sh -c 'code --install-extension "{}" || echo "Failed: {}"'