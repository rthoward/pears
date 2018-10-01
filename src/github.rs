use std::{error,convert,fmt};
use reqwest;
use serde_json;

use types::ConfigRepo;
use types::{GitHubGraphQLResponse, GitHubPullRequest, GitHubRepo};

#[derive(Debug, Clone)]
pub struct GitHubError;

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not reach GitHub.")
    }
}

impl error::Error for GitHubError {
    fn description(&self) -> &str {
        "Could not reach GitHub."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl convert::From<reqwest::Error> for GitHubError {
    fn from(_e: reqwest::Error) -> Self {
        GitHubError {}
    }
}

impl convert::From<serde_json::Error> for GitHubError {
    fn from(_e: serde_json::Error) -> Self {
        GitHubError {}
    }
}

pub trait GithubAPI {
    fn fetch_repo(&self, repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError>;
}

fn parse_repo_response(repo_response: String) -> Result<GitHubRepo, serde_json::Error> {
    let resp: GitHubGraphQLResponse = serde_json::from_str(&repo_response)?;
    Ok(resp.data.repository)
}

pub struct GitHubMockAPI;

impl GithubAPI for GitHubMockAPI {
    fn fetch_repo(&self, repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError> {
        let query = r###"
        query fetchPullRequests($repo_owner: String!, $repo_name: String!) {
  repository(owner: $repo_owner, name: $repo_name) {
    name
    pullRequests(last: 20, states: [OPEN]) {
      edges {
        node {
          id
          state
          title
          body
          number
          url
          createdAt
          updatedAt
          closedAt
          mergedAt
          mergeable
          author {
            login
          }
          labels(first: 100) {
            edges {
              node {
                id
                name
              }
            }
          }
          comments(last: 100) {
            edges {
              node {
                id
                bodyText
                author {
                  login
                }
                createdAt
                updatedAt
              }
            }
          }
          reviews(last: 100) {
            edges {
              node {
                id
                author {
                  login
                }
                createdAt
                updatedAt
                bodyText
                comments(last: 100) {
                  edges {
                    node {
                      author {
                        login
                      }
                      id
                      bodyText
                      diffHunk
                      createdAt
                      updatedAt
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
        "###;
        let body = json!({
           "query": query,
           "variables": {
               "repo_owner": repo.owner,
               "repo_name": repo.name,
           }
        }).to_string();
        let mut response = reqwest::Client::new().post("").bearer_auth("bearer 98914d84313d0c299d68fc8a0bf8e1cceac87575").body(body).send()?;
        let response_body = response.text()?;
        let repository = parse_repo_response(response_body)?;
        Ok(repository)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mock_api = GitHubMockAPI {};
        let repo = ConfigRepo {
            owner: String::from("me"),
            name: String::from("repo"),
        };
        let repo = mock_api.fetch_repo(&repo).unwrap();
        assert!(true)
    }
}
