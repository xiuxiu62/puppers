pub struct Ability {
    key: String,
}

impl Ability {
    #[inline]
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
        }
    }

    #[inline]
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

pub struct AbilityList(Vec<Ability>);

impl AbilityList {
    #[inline]
    pub fn new(abilities: Vec<Ability>) -> Self {
        Self(abilities)
    }

    pub fn get(&self, key: &str) -> Option<&Ability> {
        for ability in self.0.iter() {
            if ability.key() == key {
                return Some(ability);
            }
        }

        None
    }
}

impl From<&[&str]> for AbilityList {
    fn from(keys: &[&str]) -> Self {
        let abilities = keys.into_iter().map(|key| Ability::new(key)).collect();

        Self(abilities)
    }
}
