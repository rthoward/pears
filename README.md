# pears

## Usage

Create a json configuration file at `~/.config/pears/pears.json`. See [Configuration](#configuration) for details on the config file format.

Run `pears` to see a list of pull requests, organized by repo.

For other options, see `pears --help`.

## Configuration

An example `pears.json` file:

```javascript
{
    "me": "my-github-username",
    // Create one of these here: https://github.com/settings/tokens
    "token": "github-token",
    "repos": [
        {
            "owner": "repo-owner",
            "name": "repo-name"
        },
    ]
}
```
