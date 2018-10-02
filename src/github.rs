use reqwest;
use serde_json;
use std::error::Error;
use std::{convert, fmt};

use types::ConfigRepo;
use types::{Config, GitHubError, GitHubGraphQLResponse, GitHubRepo};

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
    fn fetch_repo(&self, config: Config, repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError>;
}

fn parse_repo_response(repo_response: String) -> Result<GitHubRepo, serde_json::Error> {
    let resp: GitHubGraphQLResponse = serde_json::from_str(&repo_response)?;
    Ok(resp.data.repository)
}

pub struct GitHubGraphqlAPI {}
pub struct GitHubMockAPI {}

impl GithubAPI for GitHubGraphqlAPI {
    fn fetch_repo(&self, config: Config, repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError> {
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
        let mut response = reqwest::Client::new()
            .post("https://api.github.com/graphql")
            .bearer_auth(config.token)
            .body(body)
            .send()?;
        let response_body = response.text()?;
        let repository = parse_repo_response(response_body)?;
        Ok(repository)
    }
}

impl GithubAPI for GitHubMockAPI {
    fn fetch_repo(&self, _config: Config, _repo: &ConfigRepo) -> Result<GitHubRepo, GitHubError> {
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
							"id": "MDExOlB1bGxSZXF1ZXN0MjE4MTEyNTg3",
							"state": "OPEN",
							"title": "Help Content",
							"body": "Begin adding content for the help document:\r\nhttps://docs.google.com/document/d/1Y6RbHd0YMwDpxxowP07MvJvkfvxhsAgmKiYNjoVm9bo/edit#/\r\n\r\nContent is not final. Some work still needs to get done to handle overflowing tables.\r\n\r\n![screencapture-localhost-8000-help-2018-09-25-16_29_33](https://user-images.githubusercontent.com/38014252/46041275-5d942f80-c0e0-11e8-84eb-aa078ea26007.png)\r\n",
							"number": 340,
							"url": "https://github.com/dod-ccpo/atst/pull/340",
							"createdAt": "2018-09-25T20:41:46Z",
							"updatedAt": "2018-09-28T14:23:29Z",
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
									}
								]
							}
						}
					},
					{
						"node": {
							"id": "MDExOlB1bGxSZXF1ZXN0MjE4MzU5NDU4",
							"state": "OPEN",
							"title": "Add more tests for workspaces routes",
							"body": "## Description\r\nThere are a numbers of workspaces routes that aren't being tested. This adds a few more and updates two forms to use `FlaskForm` instead of `Form`. It also swaps the `Required()` WTForms validator for `DataRequired()` since `Required()` will soon be deprecated. This PR also add an alert on the edit projects page to let the user know that it is not fully functional.\r\n\r\nNote that there are a few warnings when running tests, but I don't think those warnings are a result of this PR.\r\n\r\n## Screenshots\r\n![screen shot 2018-09-26 at 11 22 14 am](https://user-images.githubusercontent.com/42577527/46090432-b1555600-c17e-11e8-9294-6328915054db.png)\r\n",
							"number": 343,
							"url": "https://github.com/dod-ccpo/atst/pull/343",
							"createdAt": "2018-09-26T15:24:42Z",
							"updatedAt": "2018-09-28T14:07:15Z",
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
								"edges": [
									{
										"node": {
											"id": "MDEyOklzc3VlQ29tbWVudDQyNTQ0Njk1Nw==",
											"bodyText": "@patricksmithdds I rebased and fixed conflicts here when you have time to give it another look after the demo.",
											"author": {
												"login": "dandds"
											},
											"createdAt": "2018-09-28T14:07:15Z",
											"updatedAt": "2018-09-28T14:07:15Z"
										}
									}
								]
							},
							"reviews": {
								"edges": [
									{
										"node": {
											"id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU5MTA1Njkw",
											"author": {
												"login": "patricksmithdds"
											},
											"createdAt": "2018-09-26T17:11:46Z",
											"updatedAt": "2018-09-26T17:14:30Z",
											"bodyText": "Thanks for adding tests and getting rid of those deprecation warnings! I think we should be using InputRequired instead of DataRequired, but otherwise this looks great.",
											"comments": {
												"edges": [
													{
														"node": {
															"author": {
																"login": "patricksmithdds"
															},
															"id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDY0OTE4OQ==",
															"bodyText": "I think we should be using InputRequired instead of DataRequired, for the reasons specified in this SO answer: https://stackoverflow.com/a/23984389.",
															"diffHunk": "@@ -2,7 +2,7 @@\n import pendulum\n from wtforms.fields.html5 import DateField, EmailField\n from wtforms.fields import StringField, FileField\n-from wtforms.validators import InputRequired, Required, Email, Regexp\n+from wtforms.validators import InputRequired, DataRequired, Email, Regexp",
															"createdAt": "2018-09-26T17:11:46Z",
															"updatedAt": "2018-09-28T14:06:24Z"
														}
													}
												]
											}
										}
									},
									{
										"node": {
											"id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU5MTA4MzI0",
											"author": {
												"login": "montana-mil"
											},
											"createdAt": "2018-09-26T17:18:34Z",
											"updatedAt": "2018-09-26T17:18:34Z",
											"bodyText": "",
											"comments": {
												"edges": [
													{
														"node": {
															"author": {
																"login": "montana-mil"
															},
															"id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDY1MTE5Nw==",
															"bodyText": "Do you think there are any places where we should use DataRequired instead?",
															"diffHunk": "@@ -2,7 +2,7 @@\n import pendulum\n from wtforms.fields.html5 import DateField, EmailField\n from wtforms.fields import StringField, FileField\n-from wtforms.validators import InputRequired, Required, Email, Regexp\n+from wtforms.validators import InputRequired, DataRequired, Email, Regexp",
															"createdAt": "2018-09-26T17:18:34Z",
															"updatedAt": "2018-09-28T14:06:24Z"
														}
													}
												]
											}
										}
									},
									{
										"node": {
											"id": "MDE3OlB1bGxSZXF1ZXN0UmV2aWV3MTU5MTE0NTM5",
											"author": {
												"login": "patricksmithdds"
											},
											"createdAt": "2018-09-26T17:33:35Z",
											"updatedAt": "2018-09-26T17:33:35Z",
											"bodyText": "",
											"comments": {
												"edges": [
													{
														"node": {
															"author": {
																"login": "patricksmithdds"
															},
															"id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDIyMDY1NjAxNw==",
															"bodyText": "No, I think we should just use InputRequired everywhere.",
															"diffHunk": "@@ -2,7 +2,7 @@\n import pendulum\n from wtforms.fields.html5 import DateField, EmailField\n from wtforms.fields import StringField, FileField\n-from wtforms.validators import InputRequired, Required, Email, Regexp\n+from wtforms.validators import InputRequired, DataRequired, Email, Regexp",
															"createdAt": "2018-09-26T17:33:35Z",
															"updatedAt": "2018-09-28T14:06:24Z"
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
							"id": "MDExOlB1bGxSZXF1ZXN0MjE4Njc1NjA1",
							"state": "OPEN",
							"title": "put workspace role definitions in one place",
							"body": "PT story: https://www.pivotaltracker.com/story/show/160745296\r\n\r\nThis puts all of the workspace and atat `Role` definitions in a constant in `atst.domain.roles`.\r\n\r\n**notes**\r\n- I thought about putting the definitions in the role model file but decided it was a domain concern.\r\n- I also thought about having the form data pull from the database instead of the constant, in case we update the constant but forget to reseed the database. Maybe that would be better?\r\n- It might be useful to have an attribute on the `roles` table to define what kind of role it is. \"ccpo\" and \"default\" are not available as workspace roles. That exclusion is hard-coded at the moment.",
							"number": 347,
							"url": "https://github.com/dod-ccpo/atst/pull/347",
							"createdAt": "2018-09-27T14:32:19Z",
							"updatedAt": "2018-09-27T14:42:09Z",
							"closedAt": null,
							"mergedAt": null,
							"mergeable": "MERGEABLE",
							"author": {
								"login": "dandds"
							},
							"labels": {
								"edges": []
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
							"id": "MDExOlB1bGxSZXF1ZXN0MjE4NzM2MDg3",
							"state": "OPEN",
							"title": "IE Budget Report page fixes",
							"body": "Fixes a few things:\r\n\r\nhttps://www.pivotaltracker.com/story/show/160794811\r\nFixes box layout at the top of the budget page, so they split 50/50 correctly\r\n\r\nhttps://www.pivotaltracker.com/story/show/160794046\r\nFixes a problem with all layouts that prevented the global layout container from shrinking to fit the window. This was just most apparent on this screen, but was problem everywhere.\r\n\r\nAlso fixes another problem related to this, where the screen metrics were being measured too early, and rendering the chart wrong. This adds an event handler that runs the screen metrics measurement once again when page loads.\r\n\r\nhttps://www.pivotaltracker.com/story/show/160795011\r\nAdds a missing fallback meter bar\r\n\r\nhttps://www.pivotaltracker.com/story/show/160794726\r\nRestricts text width in EmptyState block, which was flowing off screen in IE",
							"number": 348,
							"url": "https://github.com/dod-ccpo/atst/pull/348",
							"createdAt": "2018-09-27T17:47:53Z",
							"updatedAt": "2018-09-27T18:10:47Z",
							"closedAt": null,
							"mergedAt": null,
							"mergeable": "MERGEABLE",
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
								"edges": []
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
							"updatedAt": "2018-09-27T19:58:42Z",
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
											"id": "MDEyOklzc3VlQ29tbWVudDQyNTIyMzE2NA==",
											"bodyText": "Also noting here that if you're running a different version, pipenv install --python 3.6.6 --dev should reinstall your dependencies correctly after you install 3.6.6.\nWe should probably also change script/update and script/setup to specify the version number.",
											"author": {
												"login": "dandds"
											},
											"createdAt": "2018-09-27T19:58:42Z",
											"updatedAt": "2018-09-27T19:58:42Z"
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
							"id": "MDExOlB1bGxSZXF1ZXN0MjE5MDUxNDYx",
							"state": "OPEN",
							"title": "hide PDF link when PDF download is not available",
							"body": "PT Story: https://www.pivotaltracker.com/n/projects/2160940/stories/160757166\r\n\r\nWe should only show the link to the task order PDF if it's definitely available. Thanks to Leigh for the help.\r\n\r\n![screen shot 2018-09-28 at 2 33 12 pm](https://user-images.githubusercontent.com/38955503/46226818-79d7dc80-c32b-11e8-80a4-8f076f6b68aa.png)\r\n",
							"number": 352,
							"url": "https://github.com/dod-ccpo/atst/pull/352",
							"createdAt": "2018-09-28T18:34:06Z",
							"updatedAt": "2018-09-28T18:34:19Z",
							"closedAt": null,
							"mergedAt": null,
							"mergeable": "MERGEABLE",
							"author": {
								"login": "dandds"
							},
							"labels": {
								"edges": []
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
							"id": "MDExOlB1bGxSZXF1ZXN0MjE5MDU1NDEx",
							"state": "OPEN",
							"title": "UII IDs are optional for financial verification",
							"body": "PT Story: https://www.pivotaltracker.com/story/show/160796174\r\n\r\nUII IDs are not required, per the PT story. There are no other validations on the field, so we can just remove the validators.",
							"number": 353,
							"url": "https://github.com/dod-ccpo/atst/pull/353",
							"createdAt": "2018-09-28T18:50:54Z",
							"updatedAt": "2018-09-28T18:51:02Z",
							"closedAt": null,
							"mergedAt": null,
							"mergeable": "MERGEABLE",
							"author": {
								"login": "dandds"
							},
							"labels": {
								"edges": []
							},
							"comments": {
								"edges": []
							},
							"reviews": {
								"edges": []
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
        let _repo = mock_api.fetch_repo(config, &repo).unwrap();
        assert!(true)
    }
}
