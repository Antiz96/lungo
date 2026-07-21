_lungo() {
	local arg="${2}"
	local -a opts 

	opts=('-h --help
	       -V --version')

	COMPREPLY=( $(compgen -W "${opts[*]}" -- "${arg}") )
}

complete -F _lungo lungo
