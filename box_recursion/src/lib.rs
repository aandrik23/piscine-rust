#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        };

        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(worker) => {
                self.grade = worker.next;
                Some(worker.name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        match &self.grade {
            Some(worker) => Some((worker.name.clone(), worker.role.clone())),
            None => None,
        }
    }
}