pub mod cgi;
pub mod errors;
pub mod http;

use errors::WebringError;
use url::Url;

pub struct Webring {
    sites: Vec<Url>,
}

impl Webring {
    pub fn new(list: &str) -> Self {
        let sites = list
            .lines()
            .map(Url::parse)
            .filter_map(|x| x.ok())
            .collect();
        Self { sites }
    }

    pub fn load_remote(url: &str) -> Result<Self, WebringError> {
        let list = reqwest::blocking::get(url)?.text()?;
        Ok(Self::new(&list))
    }

    pub fn index_of(&self, site: &Url) -> Option<usize> {
        self.sites.iter().position(|x| x == site)
    }

    pub fn before(&self, site: &Url) -> Option<&Url> {
        let i = self.index_of(site)?;
        if i == 0 {
            self.sites.last()
        } else {
            self.sites.get(i - 1)
        }
    }

    pub fn after(&self, site: &Url) -> Option<&Url> {
        let i = self.index_of(site)?;
        if i == self.sites.len() - 1 {
            self.sites.first()
        } else {
            self.sites.get(i + 1)
        }
    }

    pub fn random(&self, site: &Url) -> Option<&Url> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        self.sites.iter().filter(|&x| x != site).choose(&mut rng)
    }
}
