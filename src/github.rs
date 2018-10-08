use reqwest;
use serde_json;
use std::error::Error;
use std::{convert, fmt};

use types::ConfigRepo;
use types::{Config, GitHubError, GraphqlResponse, Repo};

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not reach GitHub.")
    }
}

impl Error for GitHubError {
    fn description(&self) -> &str {
        "Could not reach GitHub."
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl convert::From<reqwest::Error> for GitHubError {
    fn from(e: reqwest::Error) -> Self {
        GitHubError {
            details: e.description().to_string(),
        }
    }
}

impl convert::From<serde_json::Error> for GitHubError {
    fn from(e: serde_json::Error) -> Self {
        GitHubError {
            details: e.description().to_string(),
        }
    }
}

pub trait GithubAPI {
    fn fetch_repo(&self, config: &Config, repo: &ConfigRepo) -> Result<Repo, GitHubError>;
}

fn parse_repo_response(repo_response: String) -> Result<Repo, serde_json::Error> {
    let resp: GraphqlResponse = serde_json::from_str(&repo_response)?;
    Ok(resp.data.repository)
}

pub struct GitHubGraphqlAPI {}

#[allow(dead_code)]
pub struct GitHubMockAPI {}

impl GithubAPI for GitHubGraphqlAPI {
    fn fetch_repo(&self, config: &Config, repo: &ConfigRepo) -> Result<Repo, GitHubError> {
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
                state
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
        })
        .to_string();
        let mut response = reqwest::Client::new()
            .post("https://api.github.com/graphql")
            .bearer_auth(config.token.to_owned())
            .body(body)
            .send()?;
        let response_body = response.text()?;
        let repository = parse_repo_response(response_body)?;
        Ok(repository)
    }
}

impl GithubAPI for GitHubMockAPI {
    fn fetch_repo(&self, _config: &Config, _repo: &ConfigRepo) -> Result<Repo, GitHubError> {
        let s = r###"
        {
  "data": {
    "repository": {
      "name": "atst",
      "pullRequests": {
        "edges": [
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE1MDExMzc3",
              "state": "OPEN",
              "title": "[WIP] CircleCI CD",
              "body": "This PR adds deployment capabilities to the CircleCI configuration, and disables image pushing and deployment from Travis.",
              "number": 276,
              "url": "https://github.com/dod-ccpo/atst/pull/276",
              "createdAt": "2018-09-12T15:49:42Z",
              "updatedAt": "2018-09-24T15:25:36Z",
              "closedAt": null,
              "mergedAt": null,
              "mergeable": "MERGEABLE",
              "author": {
                "login": "ddsdevon"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": []
              },
              "reviews": {
                "edges": []
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE4MTEyNTg3",
              "state": "OPEN",
              "title": "Help Content",
              "body": "Begin adding content for the help document:\r\nhttps://docs.google.com/document/d/1Y6RbHd0YMwDpxxowP07MvJvkfvxhsAgmKiYNjoVm9bo/edit#/\r\n\r\nContent is not final. Some work still needs to get done to handle overflowing tables.\r\n\r\n![screencapture-localhost-8000-help-2018-09-25-16_29_33](https://user-images.githubusercontent.com/38014252/46041275-5d942f80-c0e0-11e8-84eb-aa078ea26007.png)\r\n",
              "number": 340,
              "url": "https://github.com/dod-ccpo/atst/pull/340",
              "createdAt": "2018-09-25T20:41:46Z",
              "updatedAt": "2018-10-01T12:48:55Z",
              "closedAt": null,
              "mergedAt": null,
              "mergeable": "MERGEABLE",
              "author": {
                "login": "luisgov"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDc2NDczMQ==",
                      "bodyText": "Can we put some manual column widths (totaling 100%) on those tables, so they don't extend off the page?",
                      "author": {
                        "login": "andrewdds"
                      },
                      "createdAt": "2018-09-26T15:43:08Z",
                      "updatedAt": "2018-09-26T15:43:08Z"
                    }
                  }
                ]
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU5MDQ4Mjc2",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-09-26T15:06:31Z",
                      "updatedAt": "2018-09-26T15:11:45Z",
                      "bodyText": "",
                      "state": "APPROVED",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "patricksmithdds"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDYwMzYxMw==",
                              "bodyText": "Should this link be to #how-are-the-jedi-idiq-clins-structured?",
                              "diffHunk": "@@ -40,15 +40,148 @@\n       <div class='panel'>\n         <div class='panel__heading panel__heading--divider'>\n           <h1>\n-            <div class='h4'>JEDI Cloud Help Documentation</div>\n-            <span class='h1'>Help Topic</span>\n+            <div class='h4'>\n+              JEDI Cloud Help Documentation\n+            </div>\n+            <span class='h1'>\n+              Getting Started\n+            </span>\n           </h1>\n+\n+          <ul>\n+            <li><a href=\"#how-to-prepare-for-financial-verification-step\">How to prepare for Financial Verification step?</a></li>\n+            <li><a href=\"#how-are-the-jedi-idiq-clins-structured\">How are the JEDI ID/IQ CLINs structured?</a></li>\n+            <li><a href=\"#how-are-projects-organized-in-the-jedi-cloud\">How are projects organized in the JEDI Cloud?</a></li>\n+          </ul>\n         </div>\n \n         <div class='panel__content'>\n-          <p>So you see, since we're a small operation, we don't fall into the...uh...jurisdiction of the Empire. So you're part of the mining guild then? No, not actually. Our operation is small enough not to be noticed...which is advantageous for everybody since our customers are anxious to avoid attracting attention to themselves. Aren't you afraid the Empire's going to find out about this little operation and shut you down? That's always been a danger looming like a shadow over everything we've built here. But things have developed that will insure security. I've just made a deal that will keep the Empire out of here forever. We would be honored if you would join us. I had no choice. They arrived right before you did. I'm sorry. I'm sorry, too.</p>\n-          <p>Now will you move along, little fella? We're got a lot of work to do. No! No, no! Stay and help you, I will. Find your friend, hmm? I'm not looking for a friend, I'm looking for a Jedi Master. Oohhh. Jedi Master. Yoda. You seek Yoda. You know him? Mmm. Take you to him, I will. Yes, yes. But now, we must eat. Come. Good food. Come. Come, come. Stay here and watch after the camp, Artoo.</p>\n-          <p>Well done. Hold them in the security tower - and keep it quiet. Move. What do you think you're doing? We're getting out of here. I knew all along it had to be a mistake. Do you think that after what you did to Han we're going to trust you? I had no choice... What are you doing? Trust him, trust him! Oh, so we understand, don't we, Chewie? He had no choice. I'm just trying to help... We don't need any of your help. H-a-a-a... What? It sounds like Han. There's still a chance to save Han...I mean, at the East Platform... Chewie. I'm terribly sorry about all this. After all, he's only a Wookiee.</p>\n+          <h2 id='financial-verification'>Financial Verification</h2>\n+\n+          <h3 id='how-to-prepare-for-financial-verification-step'>How to prepare for Financial Verification step?</h3>\n+          <p>Once your request is approved, the next step is to create a Task Order (T.O.) associated with the JEDI Cloud ID/IQ.  Please contact a Contracting Officer (KO) or Contracting Officer Representative (COR) to help with this step. </p>\n+          <p>This may also involve talking to your Financial Manager (FM) to secure funding.</p>\n+          <p>Once the Task Order (T.O.) has been created, you will need to provide information related to the task order and funding in AT-AT. This step is referred to as “Financial Verification.”</p>\n+          <p><em>We also recommend getting familiar with the <a href=\"#\">JEDI Cloud CLIN structures</a> so that you know which specific services are available under JEDI and categorized for contracting purposes. This will help you and the Contracting Officer create a Task Order.</em></p>",
                              "createdAt": "2018-09-26T15:06:31Z",
                              "updatedAt": "2018-09-27T18:29:09Z"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU5MDc1MzEy",
                      "author": {
                        "login": "luisgov"
                      },
                      "createdAt": "2018-09-26T16:00:12Z",
                      "updatedAt": "2018-09-26T16:00:12Z",
                      "bodyText": "",
                      "state": "COMMENTED",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "luisgov"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDYyNTIxNw==",
                              "bodyText": "👍",
                              "diffHunk": "@@ -40,15 +40,148 @@\n       <div class='panel'>\n         <div class='panel__heading panel__heading--divider'>\n           <h1>\n-            <div class='h4'>JEDI Cloud Help Documentation</div>\n-            <span class='h1'>Help Topic</span>\n+            <div class='h4'>\n+              JEDI Cloud Help Documentation\n+            </div>\n+            <span class='h1'>\n+              Getting Started\n+            </span>\n           </h1>\n+\n+          <ul>\n+            <li><a href=\"#how-to-prepare-for-financial-verification-step\">How to prepare for Financial Verification step?</a></li>\n+            <li><a href=\"#how-are-the-jedi-idiq-clins-structured\">How are the JEDI ID/IQ CLINs structured?</a></li>\n+            <li><a href=\"#how-are-projects-organized-in-the-jedi-cloud\">How are projects organized in the JEDI Cloud?</a></li>\n+          </ul>\n         </div>\n \n         <div class='panel__content'>\n-          <p>So you see, since we're a small operation, we don't fall into the...uh...jurisdiction of the Empire. So you're part of the mining guild then? No, not actually. Our operation is small enough not to be noticed...which is advantageous for everybody since our customers are anxious to avoid attracting attention to themselves. Aren't you afraid the Empire's going to find out about this little operation and shut you down? That's always been a danger looming like a shadow over everything we've built here. But things have developed that will insure security. I've just made a deal that will keep the Empire out of here forever. We would be honored if you would join us. I had no choice. They arrived right before you did. I'm sorry. I'm sorry, too.</p>\n-          <p>Now will you move along, little fella? We're got a lot of work to do. No! No, no! Stay and help you, I will. Find your friend, hmm? I'm not looking for a friend, I'm looking for a Jedi Master. Oohhh. Jedi Master. Yoda. You seek Yoda. You know him? Mmm. Take you to him, I will. Yes, yes. But now, we must eat. Come. Good food. Come. Come, come. Stay here and watch after the camp, Artoo.</p>\n-          <p>Well done. Hold them in the security tower - and keep it quiet. Move. What do you think you're doing? We're getting out of here. I knew all along it had to be a mistake. Do you think that after what you did to Han we're going to trust you? I had no choice... What are you doing? Trust him, trust him! Oh, so we understand, don't we, Chewie? He had no choice. I'm just trying to help... We don't need any of your help. H-a-a-a... What? It sounds like Han. There's still a chance to save Han...I mean, at the East Platform... Chewie. I'm terribly sorry about all this. After all, he's only a Wookiee.</p>\n+          <h2 id='financial-verification'>Financial Verification</h2>\n+\n+          <h3 id='how-to-prepare-for-financial-verification-step'>How to prepare for Financial Verification step?</h3>\n+          <p>Once your request is approved, the next step is to create a Task Order (T.O.) associated with the JEDI Cloud ID/IQ.  Please contact a Contracting Officer (KO) or Contracting Officer Representative (COR) to help with this step. </p>\n+          <p>This may also involve talking to your Financial Manager (FM) to secure funding.</p>\n+          <p>Once the Task Order (T.O.) has been created, you will need to provide information related to the task order and funding in AT-AT. This step is referred to as “Financial Verification.”</p>\n+          <p><em>We also recommend getting familiar with the <a href=\"#\">JEDI Cloud CLIN structures</a> so that you know which specific services are available under JEDI and categorized for contracting purposes. This will help you and the Contracting Officer create a Task Order.</em></p>",
                              "createdAt": "2018-09-26T16:00:12Z",
                              "updatedAt": "2018-09-27T18:29:09Z"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTYwMjc4ODM3",
                      "author": {
                        "login": "andrewdds"
                      },
                      "createdAt": "2018-10-01T12:48:55Z",
                      "updatedAt": "2018-10-01T12:48:55Z",
                      "bodyText": "Could you put manual widths on the table columns?",
                      "state": "CHANGES_REQUESTED",
                      "comments": {
                        "edges": []
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE4NzY4ODA1",
              "state": "OPEN",
              "title": "Coverage #160694116",
              "body": "This adds `pytest-cov` per PT story https://www.pivotaltracker.com/n/projects/2160940/stories/160694116. Coverage runs by default in pytest and a total coverage score below 90 fails.\r\n\r\nWe're currently at 89%; I'm waiting on https://github.com/dod-ccpo/atst/pull/343 to be merged, since that should put us over the bar (thanks @montana-mil ).\r\n\r\n**note**\r\nThis locks our Python version at 3.6.6, instead of 3.6.*. `pytest-cov` breaks on lesser versions. This means if you're running some other minor version you will have to switch. We should prepare everyone and merge at a scheduled time we all know about.",
              "number": 350,
              "url": "https://github.com/dod-ccpo/atst/pull/350",
              "createdAt": "2018-09-27T19:49:19Z",
              "updatedAt": "2018-10-02T13:36:58Z",
              "closedAt": null,
              "mergedAt": null,
              "mergeable": "MERGEABLE",
              "author": {
                "login": "dandds"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNTIyMzE2NA==",
                      "bodyText": "Also noting here that if you're running a different version, pipenv install --python 3.6.6 --dev should reinstall your dependencies correctly after you install 3.6.6.\nWe should probably also change script/update and script/setup to specify the version number.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-09-27T19:58:42Z",
                      "updatedAt": "2018-09-27T19:58:42Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNjAxOTc5Mw==",
                      "bodyText": "Aaand we still need more tests to pass. :( I'll get to that this week.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-10-01T18:48:28Z",
                      "updatedAt": "2018-10-01T18:48:28Z"
                    }
                  }
                ]
              },
              "reviews": {
                "edges": []
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE5NDUzNDIw",
              "state": "OPEN",
              "title": "End to end test #160690740",
              "body": "PT story: https://www.pivotaltracker.com/story/show/160690740\r\n\r\nThis adds a harness for end-to-end testing with Selenium and BrowserStack. This PR only adds two very basic acceptance tests as examples. I'd like to get eyes on the setup before we dive into adding a lot of new end-to-end tests.\r\n\r\n**note**\r\nI'm having the top-level script use a different database, `atat_selenium`. We could write a separate seed script to pre-populate this with data useful for the end-to-end tests without affecting the other databases. It's also possible the test factories will just work in this context, but I haven't gotten that far with it.",
              "number": 358,
              "url": "https://github.com/dod-ccpo/atst/pull/358",
              "createdAt": "2018-10-01T17:24:46Z",
              "updatedAt": "2018-10-02T17:58:25Z",
              "closedAt": null,
              "mergedAt": null,
              "mergeable": "MERGEABLE",
              "author": {
                "login": "dandds"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNTk5MjQ1OA==",
                      "bodyText": "Hmm, I'm going to leave the WIP tag on until #350 is merged and we're pinned at 3.6.6. There are some discrepancies in the way files are being formatted with different versions of black.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-10-01T17:26:49Z",
                      "updatedAt": "2018-10-01T17:26:49Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNjA0MjIwNg==",
                      "bodyText": "I take it back, since we're waiting on the other one.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-10-01T19:59:00Z",
                      "updatedAt": "2018-10-01T19:59:00Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNjI2MTQ4Nw==",
                      "bodyText": "I'm just going to keep dropping notes here, I guess.\nI did a little research and it seems like it's difficult to get Selenium to supply client certs for SSL transactions. Most people talk about using a Firefox profile with the certs included, like this SO thread. That would really constrain us, obviously. The other option is to not do client auth in the environment we use for Selenium testing. We'd have to be careful to not leave security holes and find a way to stub the CAC info we'd normally get from SSL. (It's possible we could monkeypatch the running application under test and force a reload, but that seems like a big lift.)",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-10-02T12:51:59Z",
                      "updatedAt": "2018-10-02T12:51:59Z"
                    }
                  }
                ]
              },
              "reviews": {
                "edges": []
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE5Nzk3NzMw",
              "state": "OPEN",
              "title": "Add Default String Representations for Models",
              "body": "## Description\r\nModels now have a `__repr__` method that prints some relevant information that should be helpful for debugging purposes.\r\nIt also adds another `auth` test.\r\nCollaboration with @dandds \r\n\r\n## Pivotal Tracker\r\nhttps://www.pivotaltracker.com/story/show/160694209",
              "number": 370,
              "url": "https://github.com/dod-ccpo/atst/pull/370",
              "createdAt": "2018-10-02T18:16:32Z",
              "updatedAt": "2018-10-02T20:25:51Z",
              "closedAt": null,
              "mergedAt": null,
              "mergeable": "MERGEABLE",
              "author": {
                "login": "montana-mil"
              },
              "labels": {
                "edges": []
              },
              "comments": {
                "edges": []
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTYwOTA2ODg3",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-10-02T20:23:31Z",
                      "updatedAt": "2018-10-02T20:25:51Z",
                      "bodyText": "",
                      "state": "COMMENTED",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "patricksmithdds"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMjA5ODYyOA==",
                              "bodyText": "We can axe this test.",
                              "diffHunk": "@@ -16,3 +16,7 @@ def test_add_user_to_environment():\n \n     dev_environment = Environments.add_member(dev_environment, developer, \"developer\")\n     assert developer in dev_environment.users\n+\n+\n+def test_repr():\n+    pass",
                              "createdAt": "2018-10-02T20:23:31Z",
                              "updatedAt": "2018-10-02T20:25:51Z"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
        "###;
        let repository = parse_repo_response(s.to_string())?;
        Ok(repository)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mock_api = GitHubMockAPI {};
        let config = Config {
            me: "Richard".to_string(),
            token: "hello".to_string(),
        };
        let repo = ConfigRepo {
            owner: String::from("me"),
            name: String::from("repo"),
        };
        let _repo = mock_api.fetch_repo(&config, &repo).unwrap();
        assert!(true)
    }
}
