pub struct Ability<'a> {
    key: &'a str,
}

impl<'a> Ability<'a> {
    #[inline]
    pub fn new(key: &'a str) -> Self {
        Self { key }
    }

    #[inline]
    pub fn key(&self) -> &str {
        self.key
    }
}

pub struct AbilityList<'a>(Vec<Ability<'a>>);

impl<'a> AbilityList<'a> {
    #[inline]
    pub fn new(abilities: Vec<Ability<'a>>) -> Self {
        Self(abilities)
    }

    pub fn get(&self, key: &str) -> Option<&Ability<'a>> {
        for ability in self.0.iter() {
            if ability.key() == key {
                return Some(ability);
            }
        }

        None
    }
}

impl<'a> From<&[&'a str]> for AbilityList<'a> {
    fn from(keys: &[&'a str]) -> Self {
        let abilities = keys.into_iter().map(|key| Ability::new(key)).collect();

        Self(abilities)
    }
}

impl<'a> Into<Vec<&'a str>> for AbilityList<'a> {
    fn into(self) -> Vec<&'a str> {
        self.0.iter().map(|ability| ability.key).collect()
    }
}
