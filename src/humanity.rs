pub struct Humanity {
    pub owner: User,
    pub contributors: Vec<User>
}

impl<'a> Humanity {
    pub fn get_from<B: HumanityBearer>(source: &'a str) -> Self {
        B::generate(source)
    }
}

impl ToString for Humanity {
    fn to_string(&self) -> String {
        let mut owner = format!("/* OWNER */{}\n", self.owner.to_string());
        if !self.contributors.is_empty() {
            let contributors = format!("/* CONTRIBUTORS */{}", self.contributors.iter().map(|contributor| contributor.to_string()).collect::<String>());
            owner.push_str(&*contributors);
        }
        owner.trim().to_owned()
    }
}

pub trait HumanityBearer {
    fn generate(source: &str) -> Humanity;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    #[serde(rename="html_url")]
    pub profile_url: String,
    pub location: Option<String>,
    #[serde(rename="blog")]
    pub website: Option<String>
}

impl ToString for User {
    fn to_string(&self) -> String {
        let name = match self.name.to_owned() {
            Some(name) => name,
            _          => self.login.to_string()
        };

        let profile_url = format!("GitHub Profile: {}", self.profile_url);

        let mut location = String::new();
        if let Some(location_) = self.location.to_owned() {
            location = format!("Location: {}", location_);
        }

        let mut website = String::new();
        if let Some(website_) = self.website.to_owned() {
            website = format!("Website: {}", website_);
        }

        [name, profile_url, location, website, " ".to_string()].iter().filter_map(|s| {
            if s.is_empty() {
                None
            }
            else {
                Some(format!("\n  {}", s))
            }
        }).collect()
    }
}
