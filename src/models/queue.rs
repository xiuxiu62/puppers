use super::role::RoleList;

pub struct Queue {
    lcu_name: String,
    ugg_name: String,
    mobalytics_name: String,
    rank: String,
    roles: RoleList,
}

impl Queue {
    pub fn new(
        lcu_name: String,
        ugg_name: String,
        mobalytics_name: String,
        rank: String,
        roles: RoleList,
    ) -> Self {
        Self {
            lcu_name,
            ugg_name,
            mobalytics_name,
            rank,
            roles,
        }
    }

    pub fn roles(&self) -> &RoleList {
        &self.roles
    }
}

struct QueueList(Vec<Queue>);

impl QueueList {
    pub fn new(inner: Vec<Queue>) -> Self {
        Self(inner)
    }

    pub fn get_default(&self) -> Option<&Queue> {
        self.get_by_lcu_name("Summoner's Rift")
    }

    pub fn get_by_lcu_name(&self, name: &str) -> Option<&Queue> {
        self.0.iter().find(|queue| queue.lcu_name == name)
    }

    pub fn get_by_ugg_name(&self, name: &str) -> Option<&Queue> {
        self.0.iter().find(|queue| queue.ugg_name == name)
    }
}
