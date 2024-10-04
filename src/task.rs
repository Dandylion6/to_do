pub enum Status {
    Planned = 0,
    Doing = 1,
    Done = 2,
    Cancelled = 3,
}

impl Status {
    pub fn from_u8(u8: u8) -> Self {
        match u8 {
            0 => return Status::Planned,
            1 => return Status::Doing,
            2 => return Status::Done,
            3 => return Status::Cancelled,
            _ => return Status::Planned,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Status::Planned => return "0",
            Status::Doing => return "1",
            Status::Done => return "2",
            Status::Cancelled => return "3",
        }
    }
}

impl Copy for Status {}
impl Clone for Status {
    fn clone(&self) -> Status {
        *self
    }
}
impl IntoIterator for Status {
    type Item = Status;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let statuses: Vec<Status> = vec![
            Status::Planned,
            Status::Doing,
            Status::Done,
            Status::Cancelled,
        ];
        return statuses.into_iter();
    }
}

pub struct Task {
    list_name: String,
    description: String,
    status: Status,
    is_deleted: bool,
}

impl Task {
    pub fn new(description: &str, list_name: &str) -> Self {
        return Self {
            list_name: list_name.to_string(),
            description: description.to_string(),
            status: Status::Planned,
            is_deleted: false,
        };
    }

    pub fn get_list_name(&self) -> &str {
        return &self.list_name;
    }

    pub fn get_description(&self) -> &str {
        return self.description.as_str();
    }

    pub fn get_status(&self) -> Status {
        return self.status;
    }

    pub fn is_deleted(&self) -> bool {
        return self.is_deleted;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn mark_for_deletion(&mut self) {
        self.is_deleted = true;
    }
}
