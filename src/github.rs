extern crate reqwest;
extern crate serde_json;

use std::error;
use std::fmt;

use types::ConfigRepo;
use types::{GitHubPullRequest, GitHubRepo, GitHubGraphQLResponse};

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

pub trait GithubAPI {
    fn fetch_repo(&self, repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError>;
    fn fetch_prs(&self, repo: &ConfigRepo) -> Result<Vec<GitHubPullRequest>, GitHubError>;
}

fn parse_repo_response(repo_response: String) -> Result<GitHubRepo, serde_json::Error> {
    let resp: GitHubGraphQLResponse = serde_json::from_str(&repo_response)?;
    Ok(resp.data.repository)
}

fn parse_prs_response(prs_response: String) -> Result<Vec<GitHubPullRequest>, serde_json::Error> {
    let pull_requests: Vec<GitHubPullRequest> = serde_json::from_str(&prs_response)?;
    Ok(pull_requests)
}

pub struct GitHubMockAPI;

impl GithubAPI for GitHubMockAPI {
    fn fetch_repo(&self, _repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError> {
        let s = r###"
        {
  "data": {
    "repository": {
      "name": "atst",
      "pullRequests": {
        "edges": [
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE0MDI0OTMw",
              "state": "OPEN",
              "title": "Investigate requests against EDA",
              "body": "This PR adds the ability to make some requests against the real EDA servers. This PR is the result of investigating EDA and _does not_ change the app to use EDA or include credentials for accessing the API.\r\n\r\nThis PR includes a script, `example_fetch_from_eda.py`, that makes a couple example requests to EDA (given that you update your config with the appropriate credentials -- see the below google doc) which will allow us to further poke at & investigate the data EDA returns.\r\n\r\nFor more information, see [this doc on google drive](https://docs.google.com/document/d/1LvpnWHFTdKgwrsENOjgiATFzWjhlmYxZmZ2Ts1_bTck/edit)",
              "number": 260,
              "url": "https://github.com/dod-ccpo/atst/pull/260",
              "createdAt": "2018-09-07T19:56:52Z",
              "updatedAt": "2018-09-18T14:24:09Z",
              "closedAt": null,
              "mergedAt": null,
              "author": {
                "login": "patricksmithdds"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "id": "MDU6TGFiZWw5NjI1ODE0MTY=",
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQxOTkxMjA2Nw==",
                      "bodyText": "@richard-dds I don't think I was copied on the email referenced in the Google Doc that has the account password. Were you? If so, can you plug in the creds and test that this works? I updated it with the defusedxml library the security audit asked for.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-09-10T13:29:21Z",
                      "updatedAt": "2018-09-10T13:29:21Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMDM2NTY1NA==",
                      "bodyText": "@patricksmithdds Richard and I couldn't find the password referenced in the google doc and couldn't test this properly. I made a couple superficial updates. I don't think it will hit conflicts anytime soon, so we'll let it hang until you're back.",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-09-11T18:03:24Z",
                      "updatedAt": "2018-09-11T18:03:24Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMjQxNDE3NQ==",
                      "bodyText": "@patricksmithdds Okay, I'm able to connect now, but I'm getting \"no data found\" responses for all the requests. :(",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-09-18T14:24:09Z",
                      "updatedAt": "2018-09-18T14:24:09Z"
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
              "author": {
                "login": "ddsdevon"
              },
              "labels": {
                "edges": [
                  {
                    "node": {
                      "id": "MDU6TGFiZWw5NjI1ODE0MTY=",
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
              "id": "MDExOlB1bGxSZXF1ZXN0MjE4MDcxMDQ2",
              "state": "OPEN",
              "title": "Fix regression in Projects.create",
              "body": "A new project's creator was not being assigned a role in the project's environments.",
              "number": 335,
              "url": "https://github.com/dod-ccpo/atst/pull/335",
              "createdAt": "2018-09-25T18:09:00Z",
              "updatedAt": "2018-09-25T19:55:13Z",
              "closedAt": null,
              "mergedAt": null,
              "author": {
                "login": "richard-dds"
              },
              "labels": {
                "edges": []
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDQ1MjI3OQ==",
                      "bodyText": "Two things I've noticed: there isn't an owner role in ENVIRONMENT_ROLES, and when I go to update a project and add a new environment I get an error. I'm not really sure how the update project works. Seems mysterious.",
                      "author": {
                        "login": "montana-mil"
                      },
                      "createdAt": "2018-09-25T18:30:18Z",
                      "updatedAt": "2018-09-25T18:33:45Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDQ1MjYxMA==",
                      "bodyText": "@richard-dds We may need to discuss this further -- there is no \"owner\" role in an environment, so adding that would make things inconsistent with the desired CSP roles.\nI was thinking that we can't presume to know what role the user intends for themselves, so we can refrain from adding any env role and allow them to add themselves as desired.",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-09-25T18:31:19Z",
                      "updatedAt": "2018-09-25T18:31:19Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDQ1NjQ0MQ==",
                      "bodyText": "@montana-mil @patricksmithdds You're both right about the owner role. The only environment role we have now is nonsense_role, since we've punted on defining them.\nThe reason we've been adding project creators to a new project's environments in the first place is so they can view the project's environments. Now I'm realizing that the workspace owner's VIEW_ENVIRONMENT_IN_APPLICATION permission will allow them to see all environments anyway.\nI don't see any issues with not adding anyone to a new project's environments, except maybe the fact that the workspace owner may be listed with no environment access on the members page. How do we feel about that?",
                      "author": {
                        "login": "richard-dds"
                      },
                      "createdAt": "2018-09-25T18:43:00Z",
                      "updatedAt": "2018-09-25T18:44:15Z"
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDQ3NjI2MQ==",
                      "bodyText": "I don't see any issues with not adding anyone to a new project's environments, except maybe the fact that the workspace owner may be listed with no environment access on the members page. How do we feel about that?\n\nI think that's intended behavior -- we won't add a user to the environment unless they do so explicitly.",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-09-25T19:44:53Z",
                      "updatedAt": "2018-09-25T19:44:53Z"
                    }
                  }
                ]
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4Njc5Mjg4",
                      "author": {
                        "login": "montana-mil"
                      },
                      "createdAt": "2018-09-25T18:20:05Z",
                      "updatedAt": "2018-09-25T18:20:06Z",
                      "bodyText": "",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "montana-mil"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDMwMjA4MA==",
                              "bodyText": "why did you take this out?",
                              "diffHunk": "@@ -21,18 +21,20 @@ def create(cls, project, name):\n \n     @classmethod\n     def create_many(cls, project, names):\n+        environments = []\n         for name in names:\n             environment = Environment(project=project, name=name)\n-            db.session.add(environment)\n+            environments.append(environment)\n+\n+        db.session.add_all(environments)\n+        return environments\n \n     @classmethod\n     def add_member(cls, environment, user, role):\n         environment_user = EnvironmentRole(\n             user=user, environment=environment, role=role\n         )\n         db.session.add(environment_user)\n-        db.session.commit()",
                              "createdAt": "2018-09-25T18:20:06Z",
                              "updatedAt": "2018-09-25T19:55:13Z"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4Njg1MjA1",
                      "author": {
                        "login": "richard-dds"
                      },
                      "createdAt": "2018-09-25T18:34:03Z",
                      "updatedAt": "2018-09-25T18:34:03Z",
                      "bodyText": "",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "richard-dds"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDMwNjYzNg==",
                              "bodyText": "In the audit logs story I removed some of the commit()s from the Projects.create code path by only adding the new objects to the session and then committing them at the end of Projects.create. The benefits are that it saves us a few trips to the database, and it causes the audit events to be generated in a more readable way.\nIn doing that, I introduced this bug, but in fixing it I also realized that there were more extraneous commit()s to remove.",
                              "diffHunk": "@@ -21,18 +21,20 @@ def create(cls, project, name):\n \n     @classmethod\n     def create_many(cls, project, names):\n+        environments = []\n         for name in names:\n             environment = Environment(project=project, name=name)\n-            db.session.add(environment)\n+            environments.append(environment)\n+\n+        db.session.add_all(environments)\n+        return environments\n \n     @classmethod\n     def add_member(cls, environment, user, role):\n         environment_user = EnvironmentRole(\n             user=user, environment=environment, role=role\n         )\n         db.session.add(environment_user)\n-        db.session.commit()",
                              "createdAt": "2018-09-25T18:34:03Z",
                              "updatedAt": "2018-09-25T19:55:13Z"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE4MDczNDIw",
              "state": "OPEN",
              "title": "User profile screen",
              "body": "Adds a stubbed edit user screen, and various associated thingys\r\n- Links in topbar to the screen\r\n- Form template, so it can be reused on the user-signup flow\r\n- Wtform object, not yet hooked up to real data\r\n- `save_user` route, that does nothing but redirect\r\n- Temporary alert telling user the form isn't working yet\r\n\r\n![edit_user](https://user-images.githubusercontent.com/40467269/46034377-772c7b80-c0ce-11e8-9370-9bdb8940c656.png)\r\n",
              "number": 337,
              "url": "https://github.com/dod-ccpo/atst/pull/337",
              "createdAt": "2018-09-25T18:17:54Z",
              "updatedAt": "2018-09-25T19:46:10Z",
              "closedAt": null,
              "mergedAt": null,
              "author": {
                "login": "andrewdds"
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
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4Njk4MjIy",
                      "author": {
                        "login": "dandds"
                      },
                      "createdAt": "2018-09-25T19:05:50Z",
                      "updatedAt": "2018-09-25T19:05:50Z",
                      "bodyText": "Looks good. Excited to wire this up and have some real user flow.",
                      "comments": {
                        "edges": []
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NzEzOTE0",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-09-25T19:46:10Z",
                      "updatedAt": "2018-09-25T19:46:10Z",
                      "bodyText": "I think there's some duplication that we can resolve between this form and one of the forms of the request process, but that can be handled when this is actually hooked up.",
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
              "id": "MDExOlB1bGxSZXF1ZXN0MjE4MDc4OTAw",
              "state": "OPEN",
              "title": "Make BA code optional",
              "body": "When all other fields fail... this field stands alone\r\n\r\n![screen shot 2018-09-25 at 2 34 22 pm](https://user-images.githubusercontent.com/474639/46035222-8e6c6880-c0d0-11e8-9792-904da89946d5.png)\r\n",
              "number": 338,
              "url": "https://github.com/dod-ccpo/atst/pull/338",
              "createdAt": "2018-09-25T18:38:13Z",
              "updatedAt": "2018-09-25T18:40:14Z",
              "closedAt": null,
              "mergedAt": null,
              "author": {
                "login": "andrewcroce"
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
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4Njg3Nzk1",
                      "author": {
                        "login": "patricksmithdds"
                      },
                      "createdAt": "2018-09-25T18:40:13Z",
                      "updatedAt": "2018-09-25T18:40:14Z",
                      "bodyText": "",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "author": {
                                "login": "patricksmithdds"
                              },
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDMwODcyNA==",
                              "bodyText": "While we're here, can we update the text here to indicate the letter is optional as well.",
                              "diffHunk": "@@ -121,9 +121,9 @@ def is_missing_task_order_number(self):\n     )\n \n     ba_code = StringField(\n-        \"Program Budget Activity (BA) Code\",\n+        \"Program Budget Activity (BA) Code (Optional)\",\n         description=\"BA Code is used to identify the purposes, projects, or types of activities financed by the appropriation fund. <br/><em>It should be two digits, followed by a letter.</em>\",",
                              "createdAt": "2018-09-25T18:40:13Z",
                              "updatedAt": "2018-09-25T18:40:14Z"
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
        Ok(parse_repo_response(String::from(s)).unwrap())
    }




















































    fn fetch_prs(&self, _repo: &ConfigRepo) -> Result<Vec<GitHubPullRequest>, GitHubError> {
        let s = r###"
        {
  "data": {
    "repository": {
      "name": "atst",
      "pullRequests": {
        "edges": [
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE0MDI0OTMw",
              "state": "OPEN",
              "title": "Investigate requests against EDA",
              "body": "This PR adds the ability to make some requests against the real EDA servers. This PR is the result of investigating EDA and _does not_ change the app to use EDA or include credentials for accessing the API.\r\n\r\nThis PR includes a script, `example_fetch_from_eda.py`, that makes a couple example requests to EDA (given that you update your config with the appropriate credentials -- see the below google doc) which will allow us to further poke at & investigate the data EDA returns.\r\n\r\nFor more information, see [this doc on google drive](https://docs.google.com/document/d/1LvpnWHFTdKgwrsENOjgiATFzWjhlmYxZmZ2Ts1_bTck/edit)",
              "number": 260,
              "url": "https://github.com/dod-ccpo/atst/pull/260",
              "createdAt": "2018-09-07T19:56:52Z",
              "updatedAt": "2018-09-18T14:24:09Z",
              "closedAt": null,
              "mergedAt": null,
              "labels": {
                "edges": [
                  {
                    "node": {
                      "id": "MDU6TGFiZWw5NjI1ODE0MTY=",
                      "name": "WIP"
                    }
                  }
                ]
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQxOTkxMjA2Nw==",
                      "bodyText": "@richard-dds I don't think I was copied on the email referenced in the Google Doc that has the account password. Were you? If so, can you plug in the creds and test that this works? I updated it with the defusedxml library the security audit asked for."
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMDM2NTY1NA==",
                      "bodyText": "@patricksmithdds Richard and I couldn't find the password referenced in the google doc and couldn't test this properly. I made a couple superficial updates. I don't think it will hit conflicts anytime soon, so we'll let it hang until you're back."
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMjQxNDE3NQ==",
                      "bodyText": "@patricksmithdds Okay, I'm able to connect now, but I'm getting \"no data found\" responses for all the requests. :("
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
              "labels": {
                "edges": [
                  {
                    "node": {
                      "id": "MDU6TGFiZWw5NjI1ODE0MTY=",
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
              "id": "MDExOlB1bGxSZXF1ZXN0MjE2NzAxMTg4",
              "state": "OPEN",
              "title": "Assign and Update Environment Roles for Workspace Users",
              "body": "## Description\r\n - Users with permissions to edit workspace users can now assign and update their workspace users' environment roles on each project (and each environment on the project).\r\n - Each environment has its own modal in order to capture environment role changes to multiple environments. This means that there will be a field on the posted form for every environment in each project that the user being editing is a member of.\r\n - When a new environment role is chosen, the displayed environment role is updated. Note that the environment role is not actually updated until the user presses `Save`. If a selection was made, pressing `Cancel` will change the display back to the original role. \r\n - This PR also wires up the edit member template to use the user's real data instead of hardcoded example data.\r\n\r\n## Pivotal Tracker\r\nhttps://www.pivotaltracker.com/story/show/158072276\r\nhttps://www.pivotaltracker.com/story/show/160298069\r\n\r\n## Screenshots\r\n![screen shot 2018-09-20 at 11 00 52 am](https://user-images.githubusercontent.com/42577527/45827460-7a85c880-bcc4-11e8-85e2-dd40654357cf.png)\r\n\r\n![screen shot 2018-09-19 at 2 49 48 pm](https://user-images.githubusercontent.com/42577527/45774769-d9dbce00-bc1b-11e8-9ccb-cdc2109ba6f4.png)\r\n",
              "number": 304,
              "url": "https://github.com/dod-ccpo/atst/pull/304",
              "createdAt": "2018-09-19T16:26:40Z",
              "updatedAt": "2018-09-25T13:01:10Z",
              "closedAt": null,
              "mergedAt": null,
              "labels": {
                "edges": []
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMzI5OTkxMQ==",
                      "bodyText": "I've taken care of these comments, but I found a few more bugs. They both happen if the modal is opened, a selection is made, and the modal is closed via the Cancel button.\n\nIf the Save button at the bottom of the page is pressed, the cancelled selection will be saved. This happens even though the display name shows correctly.\nIf the modal is opened again, the default selected item is the selection made the last time the modal was opened. It should default to the selection of the user's actual environment role if they have one."
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyMzU2MDYzOQ==",
                      "bodyText": "If the modal is opened again, the default selected item is the selection made the last time the modal was opened. It should default to the selection of the user's actual environment role if they have one.\n\n\nThis is still happening, but @andrewdds and I agreed we could allow this for now."
                    }
                  }
                ]
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU2OTU1Mzg0",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxODkxMDU1Ng==",
                              "bodyText": "I think you can do for id_and_role in ids_and_roles: and then just use id_and_role in the loop instead of ids_and_roles[i].",
                              "diffHunk": "@@ -39,3 +44,29 @@ def for_user(cls, user, project):\n             .filter(Project.id == Environment.project_id)\n             .all()\n         )\n+\n+    def get(cls, environment_id):\n+        try:\n+            env = db.session.query(Environment).filter_by(id=environment_id).one()\n+        except NoResultFound:\n+            raise NotFoundError(\"environment\")\n+\n+        return env\n+\n+    @classmethod\n+    def update_environment_role(cls, ids_and_roles, workspace_user):\n+        # TODO need to check permissions?\n+        for i in range(len(ids_and_roles)):"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE3NDIxNw==",
                              "bodyText": "We should take care of this before merging.",
                              "diffHunk": "@@ -27,3 +31,30 @@ def add_member(cls, user, environment, member):\n         db.session.commit()\n \n         return environment\n+\n+    @classmethod\n+    def get(cls, environment_id):\n+        try:\n+            env = db.session.query(Environment).filter_by(id=environment_id).one()\n+        except NoResultFound:\n+            raise NotFoundError(\"environment\")\n+\n+        return env\n+\n+    @classmethod\n+    def update_environment_role(cls, ids_and_roles, workspace_user):\n+        # TODO need to check permissions?"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE3NDU4Mg==",
                              "bodyText": "I think this should be taken out of the loop, so we do one commit to add all the roles. I think that would be slightly more performant and prevent a partial update of users' roles.",
                              "diffHunk": "@@ -27,3 +31,30 @@ def add_member(cls, user, environment, member):\n         db.session.commit()\n \n         return environment\n+\n+    @classmethod\n+    def get(cls, environment_id):\n+        try:\n+            env = db.session.query(Environment).filter_by(id=environment_id).one()\n+        except NoResultFound:\n+            raise NotFoundError(\"environment\")\n+\n+        return env\n+\n+    @classmethod\n+    def update_environment_role(cls, ids_and_roles, workspace_user):\n+        # TODO need to check permissions?\n+        for i in range(len(ids_and_roles)):\n+            new_role = ids_and_roles[i][\"role\"]\n+            environment = Environments.get(ids_and_roles[i][\"id\"])\n+            env_role = EnvironmentRole.get(\n+                workspace_user.user_id, ids_and_roles[i][\"id\"]\n+            )\n+            if env_role:\n+                env_role.role = new_role\n+            else:\n+                env_role = EnvironmentRole(\n+                    user=workspace_user.user, environment=environment, role=new_role\n+                )\n+            db.session.add(env_role)\n+            db.session.commit()"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE3NDk2MA==",
                              "bodyText": "Does this need to be an explicit role? Could we infer no access from the fact that the user does not have an environment role?",
                              "diffHunk": "@@ -141,6 +141,45 @@\n     ),\n ]\n \n+ENVIRONMENT_ROLES = [\n+    (\"no_access\", {\"name\": \"no access\", \"description\": \"No environment access.\"}),"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE3NTYyNw==",
                              "bodyText": "We can yank this out.",
                              "diffHunk": "@@ -0,0 +1,40 @@\n+import FormMixin from '../../mixins/form'\n+import textinput from '../text_input'\n+import Selector from '../selector'\n+import Modal from '../../mixins/modal'\n+import toggler from '../toggler'\n+\n+export default {\n+  name: 'edit-workspace-member',\n+\n+  mixins: [FormMixin, Modal],\n+\n+  components: {\n+    toggler,\n+    Modal,\n+    Selector,\n+    textinput\n+  },\n+\n+  props: {\n+    choices: Array,\n+    initialData: String\n+  },\n+\n+  data: function () {\n+    return { value: this.initialData }\n+  },\n+\n+  methods: {\n+    change: function (e) {\n+      this.value = e.target.value\n+    },\n+    readableName: function (role) {\n+      return role.replace(/[_]/g, \" \")\n+    },\n+  },\n+\n+  mounted: function () {\n+    console.log(this.initialData, this.choices)"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE4MzI2OA==",
                              "bodyText": "Just curious -- why change from template to div? I believe the benefit of template was that nothing gets rendered if v-show is false, but otherwise an empty element would be rendered.",
                              "diffHunk": "@@ -1,7 +1,7 @@\n {% from \"components/icon.html\" import Icon %}\n \n {% macro Modal(name, dismissable=False) -%}\n-  <template v-if='modals.{{name}} === true' v-cloak>\n+  <div v-show=\"activeModal === '{{name}}'\" v-cloak>"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE4NDI5MQ==",
                              "bodyText": "Looks like we're missing the \"Developer\" role here.",
                              "diffHunk": "@@ -141,6 +141,45 @@\n     ),\n ]\n \n+ENVIRONMENT_ROLES = [\n+    (\"no_access\", {\"name\": \"no access\", \"description\": \"No environment access.\"}),\n+    ("
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE4NTMwNg==",
                              "bodyText": "Is it possible to revert any selected changes when cancelling the modal?",
                              "diffHunk": "@@ -49,167 +50,88 @@ <h2 class=\"h3\">Manage Access <div class=\"subtitle\">Grant access to an environmen\n     </div>\n   </div>\n \n-  {% call Modal(name='rolesModal', dismissable=False) %}\n-  <div class=\"block-list\">\n-    <header class=\"block-list__header\">\n-      <h2 class=\"block-list__title\">\n-        Environment access for Danny Knight\n-        <div class='subtitle'>Project Name - Environment Name</div>\n-      </h2>\n-    </header>\n-\n-    <form method=\"post\" action=\"\">\n-      <ul>\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>Developer</dt>\n-              <dd>Configures cloud-based IaaS and PaaS computing, networking, and storage services.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>Database Administrator</dt>\n-              <dd>Configures cloud-based database services.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>DevOps</dt>\n-              <dd>Provisions, deprovisions, and deploys cloud-based IaaS and PaaS computing, networking, and storage services, including pre-configured machine images.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>Billing Administrator</dt>\n-              <dd>Views cloud resource usage, budget reports, and invoices; Tracks budgets, including spend reports, cost planning and projections, and sets limits based on cloud service usage.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>Security Administrator</dt>\n-              <dd>Accesses information security and control tools of cloud resources which include viewing cloud resource usage logging, user roles and permissioning history.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-\n-        <li class='block-list__item block-list__item--selectable'>\n-          <input type='radio' name='radio' id='radio-' />\n-          <label for='radio-'>\n-            <dl>\n-              <dt>Financial Auditor</dt>\n-              <dd>Views cloud resource usage and budget reports.</dd>\n-            </dl>\n-          </label>\n-        </li>\n-      </ul>\n-\n-      <div class='block-list__footer'>\n-        <div class='action-group'>\n-          <a v-on:click=\"closeModal('rolesModal')\" class='action-group__action usa-button'>Select Access Role</a>\n-          <a class='action-group__action icon-link icon-link--danger' v-on:click=\"closeModal('rolesModal')\">No Access</a>\n-        </div>\n-      </div>\n-    </form>\n-\n-  </div>\n-\n-  {% endcall %}\n-\n+  {% for project in projects %}\n   <div is='toggler' default-visible class='block-list project-list-item'>\n     <template slot-scope='{ isVisible, toggle }'>\n       <header class='block-list__header'>\n       <button v-on:click='toggle' class='icon-link icon-link--large icon-link--default spend-table__project__toggler'>\n           <template v-if='isVisible'>{{ Icon('caret_down') }}</template>\n           <template v-else>{{ Icon('caret_right') }}</template>\n-          <h3 class=\"block-list__title\">Code.mil</h3>\n+          <h3 class=\"block-list__title\">{{ project.name }}</h3>\n         </button>\n         <span><a href=\"#\" class=\"icon-link icon-link--danger\">revoke all access</a></span>\n       </header>\n       <ul v-show='isVisible'>\n-        <li class='block-list__item project-list-item__environment'>\n-          <span class='project-list-item__environment'>\n-            Development\n-          </span>\n-          <div class='project-list-item__environment__actions'>\n-            <span class=\"label\">no access </span><button v-on:click=\"openModal('rolesModal')\" type=\"button\" class=\"icon-link\">set role</button>\n-          </div>\n-        </li>\n-        <li class='block-list__item project-list-item__environment'>\n-          <span class='project-list-item__environment'>\n-            Sandbox\n-          </span>\n-          <div class='project-list-item__environment__actions'>\n-            <span class=\"label\">no access</span><button v-on:click=\"openModal('rolesModal')\" type=\"button\" class=\"icon-link\">set role</button>\n+        {% for env in project.environments %}\n+\n+          {% set role = EnvironmentRole.get(member.user_id, env.id).role or 'no_access' %}\n+          {% set label_class = 'label' %}\n+          {% if role != 'no_access' %}\n+            {% set label_class = 'label label--success' %}\n+          {% endif %}\n+\n+\n+        <li class='block-list__item'>\n+          <edit-workspace-member inline-template initial-data='{{ role }}' v-bind:choices='{{ form.environment_role.choices | tojson }}'>\n+          <div class='project-list-item__environment'>\n+            <span class='project-list-item__environment__link'>\n+              {{ env.name }}\n+            </span>\n+\n+            <div class='project-list-item__environment__actions'>\n+              <div>{{ form.data[\"environment_role\"] }}</div>\n+\n+              <span class=\"{{ label_class }}\" v-html:on=readableName(value)></span>\n+              <button v-on:click=\"openModal('{{ env.name }}RolesModal')\" type=\"button\" class=\"icon-link\">set role</button>\n+              {% call Modal(name=env.name + 'RolesModal', dismissable=False) %}\n+                  <div class='block-list'>\n+                    <ul>\n+                      {% for choice in form.environment_role.choices %}\n+                      <li class='block-list__item block-list__item--selectable'>\n+\n+                        {% if choice[0] != \"\"  %}\n+                          <input\n+                            name='env_{{ env.id }}'\n+                            v-on:change='change'\n+                            type='radio'\n+                            id=\"env_{{ env.id }}_{{ choice[0] }}\"\n+                            value='{{ choice[0] }}'\n+                            {% if role == choice[0] %}\n+                              checked='checked'\n+                            {% endif %}\n+                          />\n+                          <label for=\"env_{{ env.id }}_{{ choice[0] }}\">\n+                            {% if choice[1].description %}\n+                              <dl>\n+                                <dt>{{ choice[1].name }}</dt>\n+                                <dd>{{ choice[1].description }}</dd>\n+                              </dl>\n+                            {% else %}\n+                              {{ choice[1].name }}\n+                            {% endif %}\n+                          </label>\n+                          {% endif %}\n+                      </li>\n+                      {% endfor %}\n+                    </ul>\n+                    <div class='block-list__footer'>\n+                      <div class='action-group'>\n+                        <a v-on:click=\"closeModal('{{ env.name }}RolesModal')\" class='action-group__action usa-button'>Select Access Role</a>\n+                        <a class='action-group__action icon-link icon-link--danger' v-on:click=\"closeModal('{{ env.name }}RolesModal')\">Cancel</a>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3MzA4MjE4",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTE5MjE2MA==",
                              "bodyText": "I think this could probably be switched back to a template. The key here is we needed v-show instead of v-if so that the form elements inside the modal are not removed from the DOM when the modal is closed.\nIt also might make sense for this behavior (using v-if or v-show) to be parameterized in the macro, like {% macro Modal(name, dismissable=False, persistent=True) -%}. I can imagine this could be a problem for other uses of the Modal.",
                              "diffHunk": "@@ -1,7 +1,7 @@\n {% from \"components/icon.html\" import Icon %}\n \n {% macro Modal(name, dismissable=False) -%}\n-  <template v-if='modals.{{name}} === true' v-cloak>\n+  <div v-show=\"activeModal === '{{name}}'\" v-cloak>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3MzMyNTQ2",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTIxMTA2Ng==",
                              "bodyText": "I can imagine this could be a problem for other uses of the Modal.\n\nWhat sorts of problems has your imagination brought?",
                              "diffHunk": "@@ -1,7 +1,7 @@\n {% from \"components/icon.html\" import Icon %}\n \n {% macro Modal(name, dismissable=False) -%}\n-  <template v-if='modals.{{name}} === true' v-cloak>\n+  <div v-show=\"activeModal === '{{name}}'\" v-cloak>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3MzMyODY5",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTIxMTMxNQ==",
                              "bodyText": "@andrewdds I'm going to leave out the persistent=True for now. If it ends up being a problem, we know where to change it",
                              "diffHunk": "@@ -1,7 +1,7 @@\n {% from \"components/icon.html\" import Icon %}\n \n {% macro Modal(name, dismissable=False) -%}\n-  <template v-if='modals.{{name}} === true' v-cloak>\n+  <div v-show=\"activeModal === '{{name}}'\" v-cloak>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3MzM5MTI1",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTIxNjI0NQ==",
                              "bodyText": "We were having some trouble with combinations besides div and v-show, so I'm keeping those for now.",
                              "diffHunk": "@@ -1,7 +1,7 @@\n {% from \"components/icon.html\" import Icon %}\n \n {% macro Modal(name, dismissable=False) -%}\n-  <template v-if='modals.{{name}} === true' v-cloak>\n+  <div v-show=\"activeModal === '{{name}}'\" v-cloak>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3NDY0OTU0",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTMxNjQ2OA==",
                              "bodyText": "I'm not sure if this is the right permission here, because I'm logged in as the workspace owner but I'm failing a permission check when I try to edit a member. Maybe VIEW_WORKSPACE would be better?",
                              "diffHunk": "@@ -49,3 +49,21 @@ def for_user(self, user, workspace):\n             .filter(EnvironmentRole.user_id == user.id)\n             .all()\n         )\n+\n+    @classmethod\n+    def get_all(cls, workspace_user, workspace):\n+        Authorization.check_workspace_permission(\n+            workspace_user.user,\n+            workspace,\n+            Permissions.VIEW_APPLICATION_IN_WORKSPACE,"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3NDY4NzYw",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTMxOTU5Mg==",
                              "bodyText": "Any particular reason that this is on a model, rather than on a domain object? Both work of course, but our convention so far has been to put these kinds of things on the domain objects.",
                              "diffHunk": "@@ -23,6 +24,18 @@ class EnvironmentRole(Base):\n     user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"))\n     user = relationship(\"User\", backref=\"environment_roles\")\n \n+    @classmethod\n+    def get(cls, user_id, environment_id):"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3NzI2NjUy",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTUyNDgyNw==",
                              "bodyText": "I think i had it in a domain at first, but wasn't sure if it was worth it to create a new domain file for this one method. I can add it in.",
                              "diffHunk": "@@ -23,6 +24,18 @@ class EnvironmentRole(Base):\n     user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"))\n     user = relationship(\"User\", backref=\"environment_roles\")\n \n+    @classmethod\n+    def get(cls, user_id, environment_id):"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3NzMxMzM0",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTUyODM5Mg==",
                              "bodyText": "@richard-dds is the user Danny Knight? I have a feeling he is corrupt. I used these permissions because we used them above in line 26 for the get method.",
                              "diffHunk": "@@ -49,3 +49,21 @@ def for_user(self, user, workspace):\n             .filter(EnvironmentRole.user_id == user.id)\n             .all()\n         )\n+\n+    @classmethod\n+    def get_all(cls, workspace_user, workspace):\n+        Authorization.check_workspace_permission(\n+            workspace_user.user,\n+            workspace,\n+            Permissions.VIEW_APPLICATION_IN_WORKSPACE,"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3ODE3NjA2",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTU5NTU5OA==",
                              "bodyText": "I don't think we want to be passing role=None here all the time.",
                              "diffHunk": "@@ -20,9 +27,9 @@ def create_many(cls, project, names):\n         db.session.commit()\n \n     @classmethod\n-    def add_member(cls, user, environment, member, role=CSPRole.NONSENSE_ROLE):\n+    def add_member(cls, user, environment, member, role=None):\n         environment_user = EnvironmentRole(\n-            user=member, environment=environment, role=role.value\n+            user=member, environment=environment, role=None"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTU5NjgxNQ==",
                              "bodyText": "I believe the user whose permissions should be checked should be the current user, not the workspace_user. For example, if I, as a workspace owner, am editing Danny's roles, we should be checking my permissions to view applications, not Danny's.",
                              "diffHunk": "@@ -49,3 +49,21 @@ def for_user(self, user, workspace):\n             .filter(EnvironmentRole.user_id == user.id)\n             .all()\n         )\n+\n+    @classmethod\n+    def get_all(cls, workspace_user, workspace):\n+        Authorization.check_workspace_permission(\n+            workspace_user.user,\n+            workspace,\n+            Permissions.VIEW_APPLICATION_IN_WORKSPACE,"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTU5NzEwMw==",
                              "bodyText": "We can remove this method now that we have the domain file.",
                              "diffHunk": "@@ -23,6 +24,18 @@ class EnvironmentRole(Base):\n     user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"))\n     user = relationship(\"User\", backref=\"environment_roles\")\n \n+    @classmethod\n+    def get(cls, user_id, environment_id):"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTU5NzQyOA==",
                              "bodyText": "Any particular reason to catch & ignore this error?",
                              "diffHunk": "@@ -235,6 +253,11 @@ def update_member(workspace_id, member_id):\n             role = form.data[\"workspace_role\"]\n             Workspaces.update_member(g.current_user, workspace, member, role)\n \n+        try:\n+            Environments.update_environment_role(g.current_user, ids_and_roles, member)\n+        except UnauthorizedError:\n+            pass"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3ODM0Mzc4",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTYwODI3Mg==",
                              "bodyText": "i guess that's already taken care of in update_environment_role?",
                              "diffHunk": "@@ -235,6 +253,11 @@ def update_member(workspace_id, member_id):\n             role = form.data[\"workspace_role\"]\n             Workspaces.update_member(g.current_user, workspace, member, role)\n \n+        try:\n+            Environments.update_environment_role(g.current_user, ids_and_roles, member)\n+        except UnauthorizedError:\n+            pass"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3ODM0NDQy",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTYwODMxOQ==",
                              "bodyText": "ha oops",
                              "diffHunk": "@@ -23,6 +24,18 @@ class EnvironmentRole(Base):\n     user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"))\n     user = relationship(\"User\", backref=\"environment_roles\")\n \n+    @classmethod\n+    def get(cls, user_id, environment_id):"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU3ODM0NjIy",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTYwODQ2Mg==",
                              "bodyText": "oh whoops",
                              "diffHunk": "@@ -20,9 +27,9 @@ def create_many(cls, project, names):\n         db.session.commit()\n \n     @classmethod\n-    def add_member(cls, user, environment, member, role=CSPRole.NONSENSE_ROLE):\n+    def add_member(cls, user, environment, member, role=None):\n         environment_user = EnvironmentRole(\n-            user=member, environment=environment, role=role.value\n+            user=member, environment=environment, role=None"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE3MzE4Mzcw",
              "state": "OPEN",
              "title": "Audit log v2",
              "body": "Log audit events automatically when any `Auditable` model is changed, where `Auditable` is a new mixin.",
              "number": 315,
              "url": "https://github.com/dod-ccpo/atst/pull/315",
              "createdAt": "2018-09-21T15:19:36Z",
              "updatedAt": "2018-09-24T17:45:58Z",
              "closedAt": null,
              "mergedAt": null,
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
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MTM0MTMw",
                      "comments": {
                        "edges": []
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MTI3NjQ2",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg1NDcwNg==",
                              "bodyText": "Could we add a simple test here that a CCPO can view audit logs? It would be nice to see a request or workspace created or modified and see that the CCPO can see the automatically created audit events.",
                              "diffHunk": "@@ -0,0 +1,20 @@\n+import pytest\n+\n+from atst.domain.audit_log import AuditLog\n+from atst.domain.exceptions import UnauthorizedError\n+from tests.factories import UserFactory\n+\n+\n+@pytest.fixture(scope=\"function\")\n+def ccpo():\n+    return UserFactory.from_atat_role(\"ccpo\")\n+\n+\n+@pytest.fixture(scope=\"function\")\n+def developer():\n+    return UserFactory.from_atat_role(\"default\")\n+\n+\n+def test_non_admin_cannot_view_audit_log(developer):\n+    with pytest.raises(UnauthorizedError):\n+        AuditLog.get_all_events(developer)"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg1NTI5Mg==",
                              "bodyText": "This mixin is great! 😍",
                              "diffHunk": "@@ -0,0 +1,65 @@\n+from sqlalchemy import event\n+from flask import g\n+import re\n+\n+from atst.models.audit_event import AuditEvent\n+\n+ACTION_CREATE = \"create\"\n+ACTION_UPDATE = \"update\"\n+ACTION_DELETE = \"delete\"\n+\n+\n+def getattr_path(obj, path, default=None):\n+    _obj = obj\n+    for item in path.split(\".\"):\n+        _obj = getattr(_obj, item, default)\n+    return _obj\n+\n+\n+def camel_to_snake(camel_cased):\n+    s1 = re.sub(\"(.)([A-Z][a-z]+)\", r\"\\1_\\2\", camel_cased)\n+    return re.sub(\"([a-z0-9])([A-Z])\", r\"\\1_\\2\", s1).lower()\n+\n+\n+class AuditableMixin(object):"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg1NTczOA==",
                              "bodyText": "Similar to workspace, I think we should tracking the associated request, if it exists. Being able to see all the requestor's and approver's actions on a request would be good for an auditor, I think.",
                              "diffHunk": "@@ -0,0 +1,41 @@\n+from sqlalchemy import String, Column, ForeignKey, inspect\n+from sqlalchemy.dialects.postgresql import UUID\n+from sqlalchemy.orm import relationship\n+\n+from atst.models import Base, types\n+from atst.models.mixins.timestamps import TimestampsMixin\n+\n+\n+class AuditEvent(Base, TimestampsMixin):\n+    __tablename__ = \"audit_events\"\n+\n+    id = types.Id()\n+\n+    user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"), index=True)\n+    user = relationship(\"User\", backref=\"audit_events\")\n+\n+    workspace_id = Column(UUID(as_uuid=True), ForeignKey(\"workspaces.id\"), index=True)\n+    workspace = relationship(\"Workspace\", backref=\"audit_events\")"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg1ODg1OA==",
                              "bodyText": "Is resource_type a more accurate name for this? For example, a request has a name & id, so it may be unclear if resource_name and resource_id should be the name and id of the request instead of \"request\" and its id.",
                              "diffHunk": "@@ -0,0 +1,41 @@\n+from sqlalchemy import String, Column, ForeignKey, inspect\n+from sqlalchemy.dialects.postgresql import UUID\n+from sqlalchemy.orm import relationship\n+\n+from atst.models import Base, types\n+from atst.models.mixins.timestamps import TimestampsMixin\n+\n+\n+class AuditEvent(Base, TimestampsMixin):\n+    __tablename__ = \"audit_events\"\n+\n+    id = types.Id()\n+\n+    user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"), index=True)\n+    user = relationship(\"User\", backref=\"audit_events\")\n+\n+    workspace_id = Column(UUID(as_uuid=True), ForeignKey(\"workspaces.id\"), index=True)\n+    workspace = relationship(\"Workspace\", backref=\"audit_events\")\n+\n+    resource_name = Column(String(), nullable=False)"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg1OTkyOA==",
                              "bodyText": "I think some of the logged events are hard to read with just ids in there. Does it make sense to log displayname, if the property exists?\nI'm particularly thinking of the request status event logged event. If we track both the request id and the displayname, we would go from this:\n\nAmanda Adamson performed create on request_status_event 62b76e4c-7cd7-4a92-88ca-f6981e9c0035\n\nto\n\nAmanda Adamson performed create on request_status_event 62b76e4c-7cd7-4a92-88ca-f6981e9c0035 (\"Submitted\") on request fb01955f-2a9d-43f0-b0db-82205b32469c\n\nwhich is more usable information.",
                              "diffHunk": "@@ -0,0 +1,41 @@\n+from sqlalchemy import String, Column, ForeignKey, inspect\n+from sqlalchemy.dialects.postgresql import UUID\n+from sqlalchemy.orm import relationship\n+\n+from atst.models import Base, types\n+from atst.models.mixins.timestamps import TimestampsMixin\n+\n+\n+class AuditEvent(Base, TimestampsMixin):\n+    __tablename__ = \"audit_events\"\n+\n+    id = types.Id()\n+\n+    user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"), index=True)\n+    user = relationship(\"User\", backref=\"audit_events\")\n+\n+    workspace_id = Column(UUID(as_uuid=True), ForeignKey(\"workspaces.id\"), index=True)\n+    workspace = relationship(\"Workspace\", backref=\"audit_events\")\n+\n+    resource_name = Column(String(), nullable=False)\n+    resource_id = Column(UUID(as_uuid=True), index=True, nullable=False)\n+    action = Column(String(), nullable=False)"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MTU1OTMx",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTg3NzAxMw==",
                              "bodyText": "👍",
                              "diffHunk": "@@ -0,0 +1,20 @@\n+import pytest\n+\n+from atst.domain.audit_log import AuditLog\n+from atst.domain.exceptions import UnauthorizedError\n+from tests.factories import UserFactory\n+\n+\n+@pytest.fixture(scope=\"function\")\n+def ccpo():\n+    return UserFactory.from_atat_role(\"ccpo\")\n+\n+\n+@pytest.fixture(scope=\"function\")\n+def developer():\n+    return UserFactory.from_atat_role(\"default\")\n+\n+\n+def test_non_admin_cannot_view_audit_log(developer):\n+    with pytest.raises(UnauthorizedError):\n+        AuditLog.get_all_events(developer)"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MjE4ODQ3",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIxOTkyNzQ2NA==",
                              "bodyText": "👍",
                              "diffHunk": "@@ -0,0 +1,41 @@\n+from sqlalchemy import String, Column, ForeignKey, inspect\n+from sqlalchemy.dialects.postgresql import UUID\n+from sqlalchemy.orm import relationship\n+\n+from atst.models import Base, types\n+from atst.models.mixins.timestamps import TimestampsMixin\n+\n+\n+class AuditEvent(Base, TimestampsMixin):\n+    __tablename__ = \"audit_events\"\n+\n+    id = types.Id()\n+\n+    user_id = Column(UUID(as_uuid=True), ForeignKey(\"users.id\"), index=True)\n+    user = relationship(\"User\", backref=\"audit_events\")\n+\n+    workspace_id = Column(UUID(as_uuid=True), ForeignKey(\"workspaces.id\"), index=True)\n+    workspace = relationship(\"Workspace\", backref=\"audit_events\")"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE3NzU2MjA3",
              "state": "OPEN",
              "title": "New project validation #160064518",
              "body": "PT story: https://www.pivotaltracker.com/story/show/160064518\r\n\r\nThis moves new project and environment form validation into the Vue component. This way we can trigger validation errors before the user is shown the confirmation modal.",
              "number": 326,
              "url": "https://github.com/dod-ccpo/atst/pull/326",
              "createdAt": "2018-09-24T19:03:22Z",
              "updatedAt": "2018-09-25T13:58:39Z",
              "closedAt": null,
              "mergedAt": null,
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
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MjYwNzk5",
                      "comments": {
                        "edges": []
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MzUwOTMx",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNTk4MQ==",
                              "bodyText": "We may need a third error -- currently you if you try to submit a project with one (or more) valid environment names and an empty name, the error shown says that you must provide at least one environment name:\n\nIt should instead say something to the effect that names cannot be empty.",
                              "diffHunk": "@@ -50,6 +62,40 @@ export default {\n       if (this.environments.length > 1) {\n         this.environments.splice(index, 1)\n       }\n+    },\n+\n+    environmentsHaveNames: function () {\n+      return this.environments.every((e) => e.name !== \"\")\n+    },\n+\n+    envNamesAreUnique: function () {\n+      const names = this.environments.map((e) => e.name)\n+      return [...new Set(names)].length == this.environments.length\n+    },\n+\n+    validateAndOpenModal: function (modalName) {\n+      let isValid = this.$children.reduce((previous, newVal) => {\n+        // display textInput error if it is not valid\n+        if (!newVal.showValid) {\n+          newVal.showError = true\n+        }\n+\n+        return newVal.showValid && previous\n+      }, true)\n+\n+      if (!this.environmentsHaveNames()) {"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNjQyNw==",
                              "bodyText": "Will we be showing more than one error at a time? If not, it might be more efficient for the component to have an error attribute with a string error message rather than a bool attribute for every type of error.",
                              "diffHunk": "@@ -42,11 +42,12 @@ <h1>{{ title_text }}</h1>\n       </div>\n     </div>\n \n-    {% if form.environment_names.errors %}\n-      {% for error in form.environment_names.errors %}\n-        {{ Alert(error, level=\"error\") }}\n-      {% endfor %}\n-    {% endif %}\n+    <div v-if=\"showMissingError\">\n+      {{ Alert(\"Provide at least one environment name.\", level=\"error\") }}\n+    </div>\n+    <div v-if=\"showUniqueError\">\n+      {{ Alert(\"Environment names must be unique.\", level=\"error\") }}\n+    </div>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTIyNTg3",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDE3NzY4Nw==",
                              "bodyText": "I don't really like the multiple bools either. You can have more than one error at a time (in this case, two redundant env names and an empty env), though. I was thinking it might eventually be useful to take a more React-ish approach and keep a list of errors and render child Alerts from the parent form, but that seems like a big lift for the moment.",
                              "diffHunk": "@@ -42,11 +42,12 @@ <h1>{{ title_text }}</h1>\n       </div>\n     </div>\n \n-    {% if form.environment_names.errors %}\n-      {% for error in form.environment_names.errors %}\n-        {{ Alert(error, level=\"error\") }}\n-      {% endfor %}\n-    {% endif %}\n+    <div v-if=\"showMissingError\">\n+      {{ Alert(\"Provide at least one environment name.\", level=\"error\") }}\n+    </div>\n+    <div v-if=\"showUniqueError\">\n+      {{ Alert(\"Environment names must be unique.\", level=\"error\") }}\n+    </div>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTU2MjMz",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDIwMzgyNQ==",
                              "bodyText": "I think keeping a list of errors is fine -- I'm not sure what you mean by a \"big lift\" for that, though?",
                              "diffHunk": "@@ -42,11 +42,12 @@ <h1>{{ title_text }}</h1>\n       </div>\n     </div>\n \n-    {% if form.environment_names.errors %}\n-      {% for error in form.environment_names.errors %}\n-        {{ Alert(error, level=\"error\") }}\n-      {% endfor %}\n-    {% endif %}\n+    <div v-if=\"showMissingError\">\n+      {{ Alert(\"Provide at least one environment name.\", level=\"error\") }}\n+    </div>\n+    <div v-if=\"showUniqueError\">\n+      {{ Alert(\"Environment names must be unique.\", level=\"error\") }}\n+    </div>"
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "node": {
              "id": "MDExOlB1bGxSZXF1ZXN0MjE3Nzc3MTk4",
              "state": "OPEN",
              "title": "Dummy help page",
              "body": "This adds a single public '/help' route with some dummy navigation and content.\r\nA handful of \"learn more\" links throughout are now directing to this page.\r\n\r\nAlso fixes a few layout issues with the topbar and footer.\r\n\r\nAlso makes a separate `base_public` template, which is inherited by public pages, such as login and help. The login route now has a separate template inheriting from this.\r\n\r\n![help](https://user-images.githubusercontent.com/40467269/45976977-36712b80-c016-11e8-8f20-9b08392494eb.png)\r\n",
              "number": 327,
              "url": "https://github.com/dod-ccpo/atst/pull/327",
              "createdAt": "2018-09-24T20:24:32Z",
              "updatedAt": "2018-09-25T14:50:16Z",
              "closedAt": null,
              "mergedAt": null,
              "labels": {
                "edges": []
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDMxODQyMg==",
                      "bodyText": "@dandds thanks for fixing the tests"
                    }
                  }
                ]
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MzQ3MjQx",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzMjc5Nw==",
                              "bodyText": "We should use url_for('atst.home') here, too.",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>\n+\n+      {% if g.current_user %}\n+        <a href=\"/\" class=\"topbar__link\">"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzMzkzNg==",
                              "bodyText": "current_user is set on the global context only for authenticated routes: \n  \n    \n      atst/atst/domain/auth.py\n    \n    \n        Lines 21 to 24\n      in\n      04d03f3\n    \n    \n    \n    \n\n        \n          \n           if not _unprotected_route(request): \n        \n\n        \n          \n               user = get_current_user() \n        \n\n        \n          \n               if user: \n        \n\n        \n          \n                   g.current_user = user \n        \n    \n  \n\n\nCurrently, this is showing the Amanda user because of some leftover test data (which is actually removed in #315). After that's merged this will always just show the \"Log in\" button for public routes. If that's ok, perhaps we should remove the conditional here? Or, we could set the current_user for all routes instead.",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>\n+\n+      {% if g.current_user %}"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNDA2NQ==",
                              "bodyText": "Can we use the url_for helper here instead of hardcoding /help? If we rely on url_for for all URLs, we can more easily change URLs in the future.",
                              "diffHunk": "@@ -1,3 +1,12 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n <footer class='app-footer'>\n-  <h5>Joint Enterprise Defense Infrastructure</h5>\n+  <div class='app-footer__info'>\n+    <h5 class='app-footer__info__title'>Joint Enterprise Defense Infrastructure</h5>\n+\n+    <a href='/help' class='icon-link app-footer__info__link' target='_blank' rel='noopener noreferrer'>"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNDE4OA==",
                              "bodyText": "We should use url_for here.",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>\n+\n+      {% if g.current_user %}\n+        <a href=\"/\" class=\"topbar__link\">\n+          <span class=\"topbar__link-label\">{{ g.current_user.first_name + \" \" + g.current_user.last_name }}</span>\n+          {{ Icon('avatar', classes='topbar__link-icon') }}\n+        </a>\n+\n+        <a href=\"{{ url_for('atst.logout') }}\" class=\"topbar__link\" title='Log out of JEDI Cloud'>\n+          {{ Icon('logout', classes='topbar__link-icon') }}\n+        </a>\n+      {% else %}\n+        <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link\" title='Log in'>\n+          <span class=\"topbar__link-label\">Log in</span>\n+          {{ Icon('avatar', classes='topbar__link-icon') }}\n+        </a>\n+      {% endif %}\n+\n+    </nav>\n+  </header>\n+\n+  {% block content %}{% endblock %}\n+\n+  <footer class='app-footer'>\n+    <div class='app-footer__info'>\n+      <h5 class='app-footer__info__title'>Joint Enterprise Defense Infrastructure</h5>\n+\n+      <a href='/help' class='icon-link app-footer__info__link' target='_blank' rel='noopener noreferrer'>"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNDMwOQ==",
                              "bodyText": "Looks like this pattern is repeated a couple times -- is it worth adding a macro for it?",
                              "diffHunk": "@@ -7,6 +9,11 @@\n   Once the Task Order has been created, you will be asked to provide\n   details about the task order in the Financial Verification step.\n </p>\n-<p>\n-  <i>Learn more</i> about the JEDI Cloud Task Order and the Financial Verification process.\n-</p>\n+\n+<div class='alert__actions'>\n+  <a href='/help' class='icon-link'>\n+    {{ Icon('help') }}"
                            }
                          },
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDAzNDczOA==",
                              "bodyText": "For some reason, this link isn't at the top left on IE10 on Win7:\n\nLooks correct in chrome, though.",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTAzNDc0",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDE2MjA2OQ==",
                              "bodyText": "Its possible to be logged in AND on a public route, so I don't think that will work.",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>\n+\n+      {% if g.current_user %}"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTA1NDE5",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDE2MzY5NA==",
                              "bodyText": "theres an 'actions' param in the Alert macro that takes a list of dicts to build the alert links. But because the fragments were used here for the Alert content, I couldn't see a way to use that. I'm open to suggestions.",
                              "diffHunk": "@@ -7,6 +9,11 @@\n   Once the Task Order has been created, you will be asked to provide\n   details about the task order in the Financial Verification step.\n </p>\n-<p>\n-  <i>Learn more</i> about the JEDI Cloud Task Order and the Financial Verification process.\n-</p>\n+\n+<div class='alert__actions'>\n+  <a href='/help' class='icon-link'>\n+    {{ Icon('help') }}"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTA4MTMy",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDE2NTk4Mg==",
                              "bodyText": "Fixed...",
                              "diffHunk": "@@ -0,0 +1,67 @@\n+{% from \"components/icon.html\" import Icon %}\n+\n+<!DOCTYPE html>\n+<html>\n+<head>\n+  <meta charset=\"utf-8\">\n+  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n+  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n+  <title>{% block title %}JEDI Cloud{% endblock %}</title>\n+  {% assets \"css\" %}\n+    <link rel=\"stylesheet\" href=\"{{ ASSET_URL }}\" type=\"text/css\">\n+  {% endassets %}\n+  <link rel=\"icon\" type=\"image/x-icon\" href=\"/static/img/favicon.ico\">\n+</head>\n+<body>\n+\n+<div id='app-root'>\n+\n+  <header class=\"topbar topbar--public\">\n+    <nav class=\"topbar__navigation\">\n+      <a href=\"{{ url_for('atst.home') }}\" class=\"topbar__link topbar__link--home\">\n+        {{ Icon('shield', classes='topbar__link-icon') }}\n+        <span class=\"topbar__link-label\">JEDI Cloud</span>\n+      </a>"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTU1Mjk1",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDIwMzA5MA==",
                              "bodyText": "I was thinking we could have a HelpLink macro:\n{% from \"components/icon.html\" import Icon %}\n\n{% macro HelpLink() -%}\n  <div class='alert__actions'>\n    <a href='/help' class='icon-link'>\n      {{ Icon('help') }}\n      {% if caller %}\n        <div class='alert__message'>{{ caller() }}</div>\n      {% endif %}\n    </a>\n  </div>\n{%- endmacro %}\n\nAnd then call that macro here, like:\n{% call HelpLink() %}\n  Learn more about the JEDI Cloud Task Order and the Financial Verification process.\n{% endcall %}\n\nI don't believe there'd be any issue calling a macro inside this fragment, but I may be misunderstanding what you're saying.",
                              "diffHunk": "@@ -7,6 +9,11 @@\n   Once the Task Order has been created, you will be asked to provide\n   details about the task order in the Financial Verification step.\n </p>\n-<p>\n-  <i>Learn more</i> about the JEDI Cloud Task Order and the Financial Verification process.\n-</p>\n+\n+<div class='alert__actions'>\n+  <a href='/help' class='icon-link'>\n+    {{ Icon('help') }}"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTY1NDEz",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDIxMDgzMg==",
                              "bodyText": "ohh I thought you meant the helplinks specifically inside the Alert. I think there might be too much variation in the help links to make this a valuable macro. There are modifier classes, the markup order might be different, the a attributes might need to vary, etc.\nSeems like a bit too much work to parameterize all that.",
                              "diffHunk": "@@ -7,6 +9,11 @@\n   Once the Task Order has been created, you will be asked to provide\n   details about the task order in the Financial Verification step.\n </p>\n-<p>\n-  <i>Learn more</i> about the JEDI Cloud Task Order and the Financial Verification process.\n-</p>\n+\n+<div class='alert__actions'>\n+  <a href='/help' class='icon-link'>\n+    {{ Icon('help') }}"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTc1MzI2",
                      "comments": {
                        "edges": [
                          {
                            "node": {
                              "id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDIxODcyNg==",
                              "bodyText": "Sounds good -- they all looked the same at first glance, but if there's lots of variability, it makes sense to not have a macro.",
                              "diffHunk": "@@ -7,6 +9,11 @@\n   Once the Task Order has been created, you will be asked to provide\n   details about the task order in the Financial Verification step.\n </p>\n-<p>\n-  <i>Learn more</i> about the JEDI Cloud Task Order and the Financial Verification process.\n-</p>\n+\n+<div class='alert__actions'>\n+  <a href='/help' class='icon-link'>\n+    {{ Icon('help') }}"
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTgzNTQw",
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
              "id": "MDExOlB1bGxSZXF1ZXN0MjE3NzgxMTYy",
              "state": "OPEN",
              "title": "Empty state for workspace reports should show existing projects",
              "body": "PT story: https://www.pivotaltracker.com/story/show/160732667\r\n\r\nWe only show the empty state for the reports page if the workspace has no projects. I also updated the reporting methods so that the projects and environments for a workspace are displayed even if there's no spend.",
              "number": 328,
              "url": "https://github.com/dod-ccpo/atst/pull/328",
              "createdAt": "2018-09-24T20:39:16Z",
              "updatedAt": "2018-09-25T13:56:20Z",
              "closedAt": null,
              "mergedAt": null,
              "labels": {
                "edges": []
              },
              "comments": {
                "edges": [
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDMzMzMxNA==",
                      "bodyText": "@patricksmithdds Good catch. Added it."
                    }
                  },
                  {
                    "node": {
                      "id": "MDEyOklzc3VlQ29tbWVudDQyNDM1MzM2OQ==",
                      "bodyText": "@dandds the month displayed in the selector should correspond to the month selected in the URL, not just be the current month. This behavior is already functioning with the dummy data."
                    }
                  }
                ]
              },
              "reviews": {
                "edges": [
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4MzU1ODcz",
                      "comments": {
                        "edges": []
                      }
                    }
                  },
                  {
                    "node": {
                      "id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU4NTQ4Mjc5",
                      "comments": {
                        "edges": []
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
        Ok(parse_prs_response(String::from(s)).unwrap())
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
        let prs = mock_api.fetch_prs(&repo).unwrap();
        assert!(prs[0].title == "dev users")
    }

}
