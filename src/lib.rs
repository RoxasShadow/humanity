#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::env;
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::fs::File;
use std::path::Path;

use hyper::Client;
use hyper::header::{Connection, UserAgent};

use serde_json::error::Result as JSONResult;
use serde::de::Deserialize;

struct Humanity {
    owner: User,
    contributors: Vec<User>
}

impl<'a> Humanity {
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![1] // TODO
    }

    fn from<B: HumanityBearer>(source: &'a str, path: &Path) -> IOResult<()> {
        let mut file = try!(File::create(path));
        let contents = B::generate(source);
        try!(file.write_all(&*contents.as_bytes()));
        Ok(())
    }
}

trait HumanityBearer {
    fn generate(source: &str) -> Humanity;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    login: String,
    name: Option<String>,
    #[serde(rename="html_url")]
    profile_url: String,
    location: Option<String>,
    #[serde(rename="blog")]
    website: Option<String>
}

impl ToString for User {
    fn to_string(&self) -> String {
        let mut location = String::new();
        if let Some(location_) = self.location.to_owned() {
            location = format!("\nLocation: {}", location_);
        }

        let mut website = String::new();
        if let Some(website_) = self.website.to_owned() {
            website = format!("\nWebsite: {}", website_);
        }

        let name = match self.name.to_owned() {
            Some(name) => name,
            _          => self.login.to_owned()
        };

        format!("{}\n{}{}{}",
                name,
                format!("GitHub Profile: {}\n", self.profile_url),
                location, website)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHubContributor {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubOwner {
    pub owner: GitHubContributor
}

struct GitHub;
impl GitHub {
   pub fn fetch_contributors(repo: &str) -> Vec<User> {
        let url = format!("https://api.github.com/repos/{}/contributors", repo);
        let github_contributors: Vec<GitHubContributor> = Self::http_request(url).unwrap();
        github_contributors.into_iter().map(|github_contributor| Self::fetch_user(github_contributor)).collect()
    }

    pub fn fetch_owner(repo: &str) -> User {
        let url = format!("https://api.github.com/repos/{}", repo);
        let github_owner: GitHubOwner = Self::http_request(url).unwrap();
        Self::fetch_user(github_owner.owner)
    }

    pub fn fetch_user(github_contributor: GitHubContributor) -> User {
        Self::http_request(github_contributor.url).unwrap()
    }

    pub fn http_request<T>(url: String) -> JSONResult<T> where T: Deserialize {
        let url = match env::var("GITHUB_ACCESS_TOKEN") {
            Ok(access_token) => format!("{}?access_token={}", url, access_token),
            Err(_)           => url
        };

        let mut body = String::new();
        Client::new()
            .get(&*url)
            .header(Connection::close())
            .header(UserAgent("Humanity".to_string()))
            .send().unwrap()
            .read_to_string(&mut body).unwrap();
        serde_json::from_str(&*body)
    }
}

impl HumanityBearer for GitHub {
    fn generate(repo: &str) -> Humanity {
        Humanity {
            owner:        GitHub::fetch_owner(repo),
            contributors: GitHub::fetch_contributors(repo)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Humanity;
    use super::GitHub;

    #[test]
    fn test_fetch_contributors() {
        let contributors = GitHub::fetch_contributors("RoxasShadow/manageiq.org");
        assert_eq!(contributors[0].clone().name.unwrap(), "Garrett LeSage".to_string());
    }

    #[test]
    fn test_fetch_owner() {
        let owner = GitHub::fetch_owner("RoxasShadow/manageiq.org");
        assert_eq!(owner.name.unwrap(), "Giovanni Capuano".to_string());
    }

    #[test]
    fn test_from() {
        Humanity::from::<GitHub>("RoxasShadow/manageiq.org", &Path::new("a.txt"));
    }
}
