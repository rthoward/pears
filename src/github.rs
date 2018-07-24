extern crate reqwest;
extern crate serde_json;

use std::error::Error;

use self::reqwest::Url;
use types::ConfigRepo;
use types::GitHubPullRequest;

pub fn fetch_prs(repo: &ConfigRepo) -> Result<Vec<GitHubPullRequest>, Box<Error>> {
    let github_response = make_github_request(repo)?;
    let prs = parse_prs_response(github_response)?;
    Ok(prs)
}

fn parse_prs_response(prs_response: String) -> Result<Vec<GitHubPullRequest>, serde_json::Error> {
    let pull_requests: Vec<GitHubPullRequest> = serde_json::from_str(&prs_response)?;
    Ok(pull_requests)
}

fn make_github_request(repo: &ConfigRepo) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        repo.owner, repo.name
    );
    let uri = Url::parse(&url).expect("Could not parse url.");
    let response_text = reqwest::get(uri)
        .expect("Request failed.")
        .text()
        .expect("Could not get json");
    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use super::parse_prs_response;

    fn make_sample_request() -> String {
        let s = r#"
        [
    {
        "url": "https://api.github.com/repos/dod-ccpo/atst/pulls/77",
        "id": 202279448,
        "node_id": "MDExOlB1bGxSZXF1ZXN0MjAyMjc5NDQ4",
        "html_url": "https://github.com/dod-ccpo/atst/pull/77",
        "diff_url": "https://github.com/dod-ccpo/atst/pull/77.diff",
        "patch_url": "https://github.com/dod-ccpo/atst/pull/77.patch",
        "issue_url": "https://api.github.com/repos/dod-ccpo/atst/issues/77",
        "number": 77,
        "state": "open",
        "locked": false,
        "title": "dev users",
        "user": {
        "login": "dandds",
        "id": 38955503,
        "node_id": "MDQ6VXNlcjM4OTU1NTAz",
        "avatar_url": "https://avatars3.githubusercontent.com/u/38955503?v=4",
        "gravatar_id": "",
        "url": "https://api.github.com/users/dandds",
        "html_url": "https://github.com/dandds",
        "followers_url": "https://api.github.com/users/dandds/followers",
        "following_url": "https://api.github.com/users/dandds/following{/other_user}",
        "gists_url": "https://api.github.com/users/dandds/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/dandds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/dandds/subscriptions",
        "organizations_url": "https://api.github.com/users/dandds/orgs",
        "repos_url": "https://api.github.com/users/dandds/repos",
        "events_url": "https://api.github.com/users/dandds/events{/privacy}",
        "received_events_url": "https://api.github.com/users/dandds/received_events",
        "type": "User",
        "site_admin": false
        },
        "body": "with @richard-dds \r\n\r\nThis does two things:\r\n- Adds better permission handling on user login. When a user logs in, ATST will try to fetch their perms from authz. If the user doesn't exist in authz, ATST will add them with a default role of \"developer\". This is naive handling for now and will probably change later.\r\n- Adds additional dev roles and expands the `/login-dev` endpoint. Now, you can hit that endpoint with a `role` query param containing the name of the role you want to log in as (i.e., `/login-dev?role=owner`).",
        "created_at": "2018-07-18T14:49:05Z",
        "updated_at": "2018-07-18T14:49:06Z",
        "closed_at": null,
        "merged_at": null,
        "merge_commit_sha": "c62f8eb90e33b47d529b75c5761dbc23cbd90962",
        "assignee": null,
        "assignees": [

        ],
        "requested_reviewers": [
        {
            "login": "richard-dds",
            "id": 38955572,
            "node_id": "MDQ6VXNlcjM4OTU1NTcy",
            "avatar_url": "https://avatars3.githubusercontent.com/u/38955572?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/richard-dds",
            "html_url": "https://github.com/richard-dds",
            "followers_url": "https://api.github.com/users/richard-dds/followers",
            "following_url": "https://api.github.com/users/richard-dds/following{/other_user}",
            "gists_url": "https://api.github.com/users/richard-dds/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/richard-dds/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/richard-dds/subscriptions",
            "organizations_url": "https://api.github.com/users/richard-dds/orgs",
            "repos_url": "https://api.github.com/users/richard-dds/repos",
            "events_url": "https://api.github.com/users/richard-dds/events{/privacy}",
            "received_events_url": "https://api.github.com/users/richard-dds/received_events",
            "type": "User",
            "site_admin": false
        },
        {
            "login": "patricksmithdds",
            "id": 40774582,
            "node_id": "MDQ6VXNlcjQwNzc0NTgy",
            "avatar_url": "https://avatars3.githubusercontent.com/u/40774582?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/patricksmithdds",
            "html_url": "https://github.com/patricksmithdds",
            "followers_url": "https://api.github.com/users/patricksmithdds/followers",
            "following_url": "https://api.github.com/users/patricksmithdds/following{/other_user}",
            "gists_url": "https://api.github.com/users/patricksmithdds/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/patricksmithdds/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/patricksmithdds/subscriptions",
            "organizations_url": "https://api.github.com/users/patricksmithdds/orgs",
            "repos_url": "https://api.github.com/users/patricksmithdds/repos",
            "events_url": "https://api.github.com/users/patricksmithdds/events{/privacy}",
            "received_events_url": "https://api.github.com/users/patricksmithdds/received_events",
            "type": "User",
            "site_admin": false
        }
        ],
        "requested_teams": [

        ],
        "labels": [

        ],
        "milestone": null,
        "commits_url": "https://api.github.com/repos/dod-ccpo/atst/pulls/77/commits",
        "review_comments_url": "https://api.github.com/repos/dod-ccpo/atst/pulls/77/comments",
        "review_comment_url": "https://api.github.com/repos/dod-ccpo/atst/pulls/comments{/number}",
        "comments_url": "https://api.github.com/repos/dod-ccpo/atst/issues/77/comments",
        "statuses_url": "https://api.github.com/repos/dod-ccpo/atst/statuses/46a8d8aade5ccc08d39eae1b44f5b763be2c0168",
        "head": {
        "label": "dod-ccpo:dev-users",
        "ref": "dev-users",
        "sha": "46a8d8aade5ccc08d39eae1b44f5b763be2c0168",
        "user": {
            "login": "dod-ccpo",
            "id": 38081014,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjM4MDgxMDE0",
            "avatar_url": "https://avatars2.githubusercontent.com/u/38081014?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dod-ccpo",
            "html_url": "https://github.com/dod-ccpo",
            "followers_url": "https://api.github.com/users/dod-ccpo/followers",
            "following_url": "https://api.github.com/users/dod-ccpo/following{/other_user}",
            "gists_url": "https://api.github.com/users/dod-ccpo/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dod-ccpo/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dod-ccpo/subscriptions",
            "organizations_url": "https://api.github.com/users/dod-ccpo/orgs",
            "repos_url": "https://api.github.com/users/dod-ccpo/repos",
            "events_url": "https://api.github.com/users/dod-ccpo/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dod-ccpo/received_events",
            "type": "Organization",
            "site_admin": false
        },
        "repo": {
            "id": 134563299,
            "node_id": "MDEwOlJlcG9zaXRvcnkxMzQ1NjMyOTk=",
            "name": "atst",
            "full_name": "dod-ccpo/atst",
            "owner": {
            "login": "dod-ccpo",
            "id": 38081014,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjM4MDgxMDE0",
            "avatar_url": "https://avatars2.githubusercontent.com/u/38081014?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dod-ccpo",
            "html_url": "https://github.com/dod-ccpo",
            "followers_url": "https://api.github.com/users/dod-ccpo/followers",
            "following_url": "https://api.github.com/users/dod-ccpo/following{/other_user}",
            "gists_url": "https://api.github.com/users/dod-ccpo/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dod-ccpo/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dod-ccpo/subscriptions",
            "organizations_url": "https://api.github.com/users/dod-ccpo/orgs",
            "repos_url": "https://api.github.com/users/dod-ccpo/repos",
            "events_url": "https://api.github.com/users/dod-ccpo/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dod-ccpo/received_events",
            "type": "Organization",
            "site_admin": false
            },
            "private": false,
            "html_url": "https://github.com/dod-ccpo/atst",
            "description": null,
            "fork": false,
            "url": "https://api.github.com/repos/dod-ccpo/atst",
            "forks_url": "https://api.github.com/repos/dod-ccpo/atst/forks",
            "keys_url": "https://api.github.com/repos/dod-ccpo/atst/keys{/key_id}",
            "collaborators_url": "https://api.github.com/repos/dod-ccpo/atst/collaborators{/collaborator}",
            "teams_url": "https://api.github.com/repos/dod-ccpo/atst/teams",
            "hooks_url": "https://api.github.com/repos/dod-ccpo/atst/hooks",
            "issue_events_url": "https://api.github.com/repos/dod-ccpo/atst/issues/events{/number}",
            "events_url": "https://api.github.com/repos/dod-ccpo/atst/events",
            "assignees_url": "https://api.github.com/repos/dod-ccpo/atst/assignees{/user}",
            "branches_url": "https://api.github.com/repos/dod-ccpo/atst/branches{/branch}",
            "tags_url": "https://api.github.com/repos/dod-ccpo/atst/tags",
            "blobs_url": "https://api.github.com/repos/dod-ccpo/atst/git/blobs{/sha}",
            "git_tags_url": "https://api.github.com/repos/dod-ccpo/atst/git/tags{/sha}",
            "git_refs_url": "https://api.github.com/repos/dod-ccpo/atst/git/refs{/sha}",
            "trees_url": "https://api.github.com/repos/dod-ccpo/atst/git/trees{/sha}",
            "statuses_url": "https://api.github.com/repos/dod-ccpo/atst/statuses/{sha}",
            "languages_url": "https://api.github.com/repos/dod-ccpo/atst/languages",
            "stargazers_url": "https://api.github.com/repos/dod-ccpo/atst/stargazers",
            "contributors_url": "https://api.github.com/repos/dod-ccpo/atst/contributors",
            "subscribers_url": "https://api.github.com/repos/dod-ccpo/atst/subscribers",
            "subscription_url": "https://api.github.com/repos/dod-ccpo/atst/subscription",
            "commits_url": "https://api.github.com/repos/dod-ccpo/atst/commits{/sha}",
            "git_commits_url": "https://api.github.com/repos/dod-ccpo/atst/git/commits{/sha}",
            "comments_url": "https://api.github.com/repos/dod-ccpo/atst/comments{/number}",
            "issue_comment_url": "https://api.github.com/repos/dod-ccpo/atst/issues/comments{/number}",
            "contents_url": "https://api.github.com/repos/dod-ccpo/atst/contents/{+path}",
            "compare_url": "https://api.github.com/repos/dod-ccpo/atst/compare/{base}...{head}",
            "merges_url": "https://api.github.com/repos/dod-ccpo/atst/merges",
            "archive_url": "https://api.github.com/repos/dod-ccpo/atst/{archive_format}{/ref}",
            "downloads_url": "https://api.github.com/repos/dod-ccpo/atst/downloads",
            "issues_url": "https://api.github.com/repos/dod-ccpo/atst/issues{/number}",
            "pulls_url": "https://api.github.com/repos/dod-ccpo/atst/pulls{/number}",
            "milestones_url": "https://api.github.com/repos/dod-ccpo/atst/milestones{/number}",
            "notifications_url": "https://api.github.com/repos/dod-ccpo/atst/notifications{?since,all,participating}",
            "labels_url": "https://api.github.com/repos/dod-ccpo/atst/labels{/name}",
            "releases_url": "https://api.github.com/repos/dod-ccpo/atst/releases{/id}",
            "deployments_url": "https://api.github.com/repos/dod-ccpo/atst/deployments",
            "created_at": "2018-05-23T12:02:56Z",
            "updated_at": "2018-07-18T15:49:15Z",
            "pushed_at": "2018-07-18T15:49:42Z",
            "git_url": "git://github.com/dod-ccpo/atst.git",
            "ssh_url": "git@github.com:dod-ccpo/atst.git",
            "clone_url": "https://github.com/dod-ccpo/atst.git",
            "svn_url": "https://github.com/dod-ccpo/atst",
            "homepage": null,
            "size": 1505,
            "stargazers_count": 1,
            "watchers_count": 1,
            "language": "Python",
            "has_issues": true,
            "has_projects": true,
            "has_downloads": true,
            "has_wiki": true,
            "has_pages": false,
            "forks_count": 0,
            "mirror_url": null,
            "archived": false,
            "open_issues_count": 1,
            "license": {
            "key": "mit",
            "name": "MIT License",
            "spdx_id": "MIT",
            "url": "https://api.github.com/licenses/mit",
            "node_id": "MDc6TGljZW5zZTEz"
            },
            "forks": 0,
            "open_issues": 1,
            "watchers": 1,
            "default_branch": "master"
        }
        },
        "base": {
        "label": "dod-ccpo:master",
        "ref": "master",
        "sha": "7d3cd04bdd89c6e84cb652f3f0d0b86867524f6d",
        "user": {
            "login": "dod-ccpo",
            "id": 38081014,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjM4MDgxMDE0",
            "avatar_url": "https://avatars2.githubusercontent.com/u/38081014?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dod-ccpo",
            "html_url": "https://github.com/dod-ccpo",
            "followers_url": "https://api.github.com/users/dod-ccpo/followers",
            "following_url": "https://api.github.com/users/dod-ccpo/following{/other_user}",
            "gists_url": "https://api.github.com/users/dod-ccpo/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dod-ccpo/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dod-ccpo/subscriptions",
            "organizations_url": "https://api.github.com/users/dod-ccpo/orgs",
            "repos_url": "https://api.github.com/users/dod-ccpo/repos",
            "events_url": "https://api.github.com/users/dod-ccpo/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dod-ccpo/received_events",
            "type": "Organization",
            "site_admin": false
        },
        "repo": {
            "id": 134563299,
            "node_id": "MDEwOlJlcG9zaXRvcnkxMzQ1NjMyOTk=",
            "name": "atst",
            "full_name": "dod-ccpo/atst",
            "owner": {
            "login": "dod-ccpo",
            "id": 38081014,
            "node_id": "MDEyOk9yZ2FuaXphdGlvbjM4MDgxMDE0",
            "avatar_url": "https://avatars2.githubusercontent.com/u/38081014?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dod-ccpo",
            "html_url": "https://github.com/dod-ccpo",
            "followers_url": "https://api.github.com/users/dod-ccpo/followers",
            "following_url": "https://api.github.com/users/dod-ccpo/following{/other_user}",
            "gists_url": "https://api.github.com/users/dod-ccpo/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dod-ccpo/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dod-ccpo/subscriptions",
            "organizations_url": "https://api.github.com/users/dod-ccpo/orgs",
            "repos_url": "https://api.github.com/users/dod-ccpo/repos",
            "events_url": "https://api.github.com/users/dod-ccpo/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dod-ccpo/received_events",
            "type": "Organization",
            "site_admin": false
            },
            "private": false,
            "html_url": "https://github.com/dod-ccpo/atst",
            "description": null,
            "fork": false,
            "url": "https://api.github.com/repos/dod-ccpo/atst",
            "forks_url": "https://api.github.com/repos/dod-ccpo/atst/forks",
            "keys_url": "https://api.github.com/repos/dod-ccpo/atst/keys{/key_id}",
            "collaborators_url": "https://api.github.com/repos/dod-ccpo/atst/collaborators{/collaborator}",
            "teams_url": "https://api.github.com/repos/dod-ccpo/atst/teams",
            "hooks_url": "https://api.github.com/repos/dod-ccpo/atst/hooks",
            "issue_events_url": "https://api.github.com/repos/dod-ccpo/atst/issues/events{/number}",
            "events_url": "https://api.github.com/repos/dod-ccpo/atst/events",
            "assignees_url": "https://api.github.com/repos/dod-ccpo/atst/assignees{/user}",
            "branches_url": "https://api.github.com/repos/dod-ccpo/atst/branches{/branch}",
            "tags_url": "https://api.github.com/repos/dod-ccpo/atst/tags",
            "blobs_url": "https://api.github.com/repos/dod-ccpo/atst/git/blobs{/sha}",
            "git_tags_url": "https://api.github.com/repos/dod-ccpo/atst/git/tags{/sha}",
            "git_refs_url": "https://api.github.com/repos/dod-ccpo/atst/git/refs{/sha}",
            "trees_url": "https://api.github.com/repos/dod-ccpo/atst/git/trees{/sha}",
            "statuses_url": "https://api.github.com/repos/dod-ccpo/atst/statuses/{sha}",
            "languages_url": "https://api.github.com/repos/dod-ccpo/atst/languages",
            "stargazers_url": "https://api.github.com/repos/dod-ccpo/atst/stargazers",
            "contributors_url": "https://api.github.com/repos/dod-ccpo/atst/contributors",
            "subscribers_url": "https://api.github.com/repos/dod-ccpo/atst/subscribers",
            "subscription_url": "https://api.github.com/repos/dod-ccpo/atst/subscription",
            "commits_url": "https://api.github.com/repos/dod-ccpo/atst/commits{/sha}",
            "git_commits_url": "https://api.github.com/repos/dod-ccpo/atst/git/commits{/sha}",
            "comments_url": "https://api.github.com/repos/dod-ccpo/atst/comments{/number}",
            "issue_comment_url": "https://api.github.com/repos/dod-ccpo/atst/issues/comments{/number}",
            "contents_url": "https://api.github.com/repos/dod-ccpo/atst/contents/{+path}",
            "compare_url": "https://api.github.com/repos/dod-ccpo/atst/compare/{base}...{head}",
            "merges_url": "https://api.github.com/repos/dod-ccpo/atst/merges",
            "archive_url": "https://api.github.com/repos/dod-ccpo/atst/{archive_format}{/ref}",
            "downloads_url": "https://api.github.com/repos/dod-ccpo/atst/downloads",
            "issues_url": "https://api.github.com/repos/dod-ccpo/atst/issues{/number}",
            "pulls_url": "https://api.github.com/repos/dod-ccpo/atst/pulls{/number}",
            "milestones_url": "https://api.github.com/repos/dod-ccpo/atst/milestones{/number}",
            "notifications_url": "https://api.github.com/repos/dod-ccpo/atst/notifications{?since,all,participating}",
            "labels_url": "https://api.github.com/repos/dod-ccpo/atst/labels{/name}",
            "releases_url": "https://api.github.com/repos/dod-ccpo/atst/releases{/id}",
            "deployments_url": "https://api.github.com/repos/dod-ccpo/atst/deployments",
            "created_at": "2018-05-23T12:02:56Z",
            "updated_at": "2018-07-18T15:49:15Z",
            "pushed_at": "2018-07-18T15:49:42Z",
            "git_url": "git://github.com/dod-ccpo/atst.git",
            "ssh_url": "git@github.com:dod-ccpo/atst.git",
            "clone_url": "https://github.com/dod-ccpo/atst.git",
            "svn_url": "https://github.com/dod-ccpo/atst",
            "homepage": null,
            "size": 1505,
            "stargazers_count": 1,
            "watchers_count": 1,
            "language": "Python",
            "has_issues": true,
            "has_projects": true,
            "has_downloads": true,
            "has_wiki": true,
            "has_pages": false,
            "forks_count": 0,
            "mirror_url": null,
            "archived": false,
            "open_issues_count": 1,
            "license": {
            "key": "mit",
            "name": "MIT License",
            "spdx_id": "MIT",
            "url": "https://api.github.com/licenses/mit",
            "node_id": "MDc6TGljZW5zZTEz"
            },
            "forks": 0,
            "open_issues": 1,
            "watchers": 1,
            "default_branch": "master"
        }
        },
        "_links": {
        "self": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/pulls/77"
        },
        "html": {
            "href": "https://github.com/dod-ccpo/atst/pull/77"
        },
        "issue": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/issues/77"
        },
        "comments": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/issues/77/comments"
        },
        "review_comments": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/pulls/77/comments"
        },
        "review_comment": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/pulls/comments{/number}"
        },
        "commits": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/pulls/77/commits"
        },
        "statuses": {
            "href": "https://api.github.com/repos/dod-ccpo/atst/statuses/46a8d8aade5ccc08d39eae1b44f5b763be2c0168"
        }
        },
        "author_association": "CONTRIBUTOR"
    }
    ]
        "#;
        String::from(s)
    }

    #[test]
    fn parse() {
        let sample = make_sample_request();
        let prs = parse_prs_response(sample).unwrap();
        assert!(prs[0].title == "dev users")
    }

}
