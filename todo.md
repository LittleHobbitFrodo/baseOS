## TODO - base

Redo `util`
- add `use` subcommand
  - chooses language, platform(s)
- `configure` will choose tools
  - tools will not be chosen in the `build` command
- add `reconfigure` command
  - used if project is moved or tools are changed
- path to each tool will be in config file
- change script structure
  - run scripts in `files/base/scripts` instead of one bloated script
  - each lang and arch will have its own `build` script