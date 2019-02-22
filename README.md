# pears

## Usage

Create a json configuration file at `~/.config/pears/pears.json`. See [Configuration](#configuration) for details on the config file format.

```
USAGE:
    pears [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Set a custom config file [default: ~/.config/pears/pears.json]
    -r, --repo <repo>      Specify a repository. Format: <owner>/<repo>

SUBCOMMANDS:
    config    Show config
    help      Prints this message or the help of the given subcommand(s)
    list      lists active pull requests
    show      details for a pull request
```

## Configuration

An example `pears.json` file:

```javascript
{
    // --- Required ---
    "me": "my-github-username",

    // Create one of these here: https://github.com/settings/tokens
    "token": "github-token",

    // --- Optional ---

    // For multi repo groups
    "groups": [
        {
            "name": "$group_name",
            "repos": [
                {"owner": "$owner_name1", "name": "$repo_name1"},
                {"owner": "$owner_name2", "name": "$repo_name2"},
                {"owner": "$owner_name3", "name": "$repo_name3"}
            ]
        }
    ]
}
```
