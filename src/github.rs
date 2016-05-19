extern crate serde_json;

use std::env;
use std::io::prelude::*;

use hyper::Client;
use hyper::header::{Connection, UserAgent};

use serde_json::error::Result as JSONResult;
use serde::de::Deserialize;

use humanity::{Humanity, HumanityBearer, User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitHubContributor {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubOwner {
    pub owner: GitHubContributor
}

pub struct GitHub;
impl GitHub {
   pub fn fetch_contributors(repo: &str) -> Vec<User> {
        let url = format!("https://api.github.com/repos/{}/contributors", repo);
        let github_contributors: Vec<GitHubContributor> = Self::http_request(url).unwrap();
        github_contributors.into_iter().map(Self::fetch_user).collect()
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
        let owner = GitHub::fetch_owner(repo);
        Humanity {
            owner:        owner.clone(),
            contributors: GitHub::fetch_contributors(repo).into_iter().filter(|contributor| contributor.login != owner.login).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GitHub;
    use humanity::Humanity;

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
    fn test_get_from_github() {
       let humanity = Humanity::get_from::<GitHub>("RoxasShadow/manageiq.org");
       assert!(!humanity.to_string().is_empty());
    }
}
