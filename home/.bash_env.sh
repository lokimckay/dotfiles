# exports with *_WIN suffix are duplicates that are resolvable by Windows.

export HOME_WIN=$(wslpath -w "$HOME")

export USER_PROFILE=$(wslpath "$(cmd.exe /C echo %USERPROFILE% 2>/dev/null | tr -d '\r')")
export USER_PROFILE_WIN=$(wslpath -w "$USER_PROFILE")
export APPDATA=$(wslpath "$(cmd.exe /C echo %APPDATA% 2>/dev/null | tr -d '\r')")
export APPDATA_WIN=$(wslpath -w "$APPDATA")

export VSCODE_SERVER_HOME="$HOME/.vscode-server/data/User"
export VSCODE_HOME="$APPDATA/Code/User"
export VSCODE_HOME_WIN=$(wslpath -w "$VSCODE_HOME")

. "$HOME/.cargo/env"


