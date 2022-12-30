fn equals_ignore_case(a: &str, b: &str) -> bool {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    a == b
}

pub struct Webring {
    sites: Vec<String>,
}

impl Webring {
    pub fn new(list: &str) -> Self {
        let sites = list.lines().map(str::to_owned).collect();
        Self { sites }
    }

    pub fn index_of(&self, site: &str) -> Option<usize> {
        self.sites.iter().position(|x| equals_ignore_case(x, site))
    }

    pub fn before(&self, site: &str) -> Option<&String> {
        let i = self.index_of(site)?;
        if i == 0 {
            self.sites.last()
        } else {
            self.sites.get(i - 1)
        }
    }

    pub fn after(&self, site: &str) -> Option<&String> {
        let i = self.index_of(site)?;
        if i == self.sites.len() - 1 {
            self.sites.first()
        } else {
            self.sites.get(i + 1)
        }
    }

    pub fn random(&self, site: &str) -> Option<&String> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        self.sites
            .iter()
            .filter(|x| !equals_ignore_case(x, site))
            .choose(&mut rng)
    }
}
