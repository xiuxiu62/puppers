#[derive(Debug, PartialEq)]
pub struct Role {
    name: String,
    short_name: String,
    lcu_name: String,
    ugg_name: String,
    mobalytics_name: String,
}

impl Role {
    pub fn new(
        name: String,
        short_name: String,
        lcu_name: String,
        ugg_name: String,
        mobalytics_name: String,
    ) -> Self {
        Self {
            name,
            short_name,
            lcu_name,
            ugg_name,
            mobalytics_name,
        }
    }
}

#[derive(Debug)]
pub struct RoleList(Vec<Role>);

impl RoleList {
    pub fn new(inner: Vec<Role>) -> Self {
        Self(inner)
    }

    // Gets a Role, checking against all naming conventions
    pub fn get(&self, name: &str) -> Option<&Role> {
        macro_rules! check_role {
            ($f:expr) => {
                let role = $f(name);
                if role.is_some() {
                    return role;
                }
            };

            ($($f:expr),*) => {
                $(check_role!($f));*
            };
        }

        check_role!(
            |name| self.get_by_name(name),
            |name| self.get_by_short_name(name),
            |name| self.get_by_lcu_name(name),
            |name| self.get_by_ugg_name(name),
            |name| self.get_by_mobalytics_name(name)
        );

        None
    }

    // Moves a role to the front, if it exists in RoleList
    pub fn move_to_front(&mut self, role: &Role) {
        for (i, query_role) in self.0.iter().enumerate() {
            if role == query_role {
                let role = self.0.remove(i);
                self.0.insert(0, role);

                return;
            };
        }
    }

    fn get_by_name(&self, name: &str) -> Option<&Role> {
        self.0.iter().find(|role| role.name == name)
    }

    fn get_by_short_name(&self, short_name: &str) -> Option<&Role> {
        self.0.iter().find(|role| role.short_name == short_name)
    }

    fn get_by_lcu_name(&self, lcu_name: &str) -> Option<&Role> {
        self.0.iter().find(|role| role.lcu_name == lcu_name)
    }

    fn get_by_ugg_name(&self, ugg_name: &str) -> Option<&Role> {
        self.0.iter().find(|role| role.ugg_name == ugg_name)
    }

    fn get_by_mobalytics_name(&self, mobalytics_name: &str) -> Option<&Role> {
        self.0
            .iter()
            .find(|role| role.mobalytics_name == mobalytics_name)
    }
}
