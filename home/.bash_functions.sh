# Create a new directory and enter it
function mkd() {
	mkdir -p "$@" && cd "$_";
}

# Normalize `open` across Linux, macOS, and Windows (WSL)
if [[ "$(uname -s)" == "Darwin" ]]; then # macOS
	open() {
		command open "$@"
	}

elif [[ -n "$WSL_DISTRO_NAME" ]]; then # WSL
	open() {
		if [ $# -eq 0 ]; then
			explorer.exe .
		else
			for path in "$@"; do # Convert WSL paths (/mnt/c/...) to Windows paths (C:\...)
				explorer.exe "$(wslpath -w "$path")"
			done
		fi
	}

else # Linux
	open() {
		command xdg-open "$@"
	}
fi

# Open the given location or the current directory if no location is given.
function o() {
	if [ $# -eq 0 ]; then
		open .
	else
		open "$@"
	fi
}

# Display the current directory tree
function tre() {
	tree -aC -I '.git|node_modules|bower_components|target' --dirsfirst "$@" | less -FRNX;
}