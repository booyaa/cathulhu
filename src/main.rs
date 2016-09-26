extern crate hyper;
extern crate hubcaps;
extern crate dotenv;
extern crate clap;

use dotenv::dotenv;
use std::env;
use hubcaps::IssueOptions;
use clap::{Arg, App, SubCommand, AppSettings};

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


fn main() {
    dotenv().ok();

    let matches = App::new(APP_NAME)
                      .version(APP_VERSION)
                      .author(APP_AUTHOR)
                      .about(APP_DESCRIPTION)
                      .setting(AppSettings::ArgRequiredElseHelp)
                    //   .setting(AppSettings::SubcommandRequired)
                      .subcommand(SubCommand::with_name("repository")
                                      .about("Repository related commands")
                                      .arg(Arg::with_name("repository name")
                                               .help("Set repository name")
                                               .required(true)
                                               .index(1))
                                      .subcommand(SubCommand::with_name("issue")
                                                      .about("Issue subcommand")
                                                      .arg(Arg::with_name("list issue")
                                                               .short("l")
                                                               .long("list")
                                                               .help("lists open issues \
                                                                      compactly"))
                                                      .arg(Arg::with_name("close issue")
                                                               .short("r")
                                                               .long("remove")
                                                               .help("closes an issue"))))
                      .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("repository") {
        let repo_parts: Vec<_> = matches.value_of("repository name")
                                        .unwrap()
                                        .split('/')
                                        .collect();
        println!("repo name: {:?}", repo_parts);
                 ;
        if let Some(ref matches) = matches.subcommand_matches("issue") {
            if matches.is_present("list issue") {
                println!("issue ls was issued.");
                let user_agent = format!("{}/{}", APP_NAME, APP_VERSION);
                let access_token = env::var("GH_EEYORE_PAT").expect("Github Personal Access Token");
                let user_client = hyper::Client::new();
                let user_github =
                    hubcaps::Github::new(user_agent,
                                         &user_client,
                                         hubcaps::Credentials::Token(access_token.to_string()));

                let repo = user_github.repo(repo_parts[0], repo_parts[1]);//"rust-community", "team");
                let ish = repo.issues();
                let issues = ish.list(&Default::default()).unwrap();
                for issue in issues {
                    println!("{}|{}|{}", issue.number, issue.title, issue.html_url);
                }
            } else {
                println!("issue not implemented.");
            }
        } else {
            println!("please select either list or remove.");
        }
    }




    // let ish = repo.issues();
    // let issues = ish.list(&Default::default()).unwrap();
    // find only open issues and not prs
    // let filtered = issues.into_iter()
    // .filter(|x| x.title.contains("2016-09-28"))
    // .collect::<Vec<_>>();
    // println!("{:#?}\nitems: {}", filtered, filtered.len());

    // if filtered.len() == 1 {
    // println!("found it! {} - {}", filtered[0].title, filtered[0].html_url);
    // }


    // let repo = user_github.repo("booyaa", "hello-homu");

    // // find only open issues and not prs
    // let filtered = issues.into_iter()
    //                      .filter(|x| x.html_url.contains("issues"))
    //                      .collect::<Vec<_>>();
    //
    //
    // println!("{:#?}\nitems: {}", filtered, filtered.len());
    //
    // let issue = &filtered[0]; // issue 9
    // println!("{:#?}", issue);
    //
    // let issue_to_close = ish.get(9);
    // let empty_labels: Vec<String> = Vec::new();
    //
    // // the reason this won't is that the compiler can't infer what type of None the parameters are.
    // // let issue_options = IssueOptions::new("issue directed at homubot",
    // //   None,
    // //   None,
    // //   None,
    // //   empty_labels);
    // let res = issue_to_close.edit(&IssueOptions {
    //     title: "issue directed at homubot".into(),
    //     body: None,
    //     assignee: None,
    //     milestone: None,
    //     labels: empty_labels,
    // });
    //
    // println!("Issue #9 edit result: {:#?}\n", res);
}
