use crate::actions::Actions;
use crate::company::Company;

#[derive(Debug)]
pub struct Updater {
    action: Actions,
    name: String,
    department: String,
}

impl Updater {
    pub fn from_str(string: &str) -> Option<Self> {
        if let Some((action, name, department)) = Updater::parse_string(string) {
            Some(Updater {
                action,
                name,
                department,
            })
        } else {
            None
        }
    }

    pub fn update(&self, company: &mut Company) -> Result<(), ()> {
        match self.action {
            Actions::Add => company.add_to_department(&self.department, &self.name),
            Actions::Remove => company.remove_from_department(&self.department, &self.name),
            Actions::Unknown => {
                println!("Unknown action received. Valid Values: 'Add', 'Remove'");
                Err(())
            }
        }
    }

    fn parse_string(string: &str) -> Option<(Actions, String, String)> {
        let collection: Vec<_> = string.split(" ").collect();

        let action = collection[0];
        let action = Actions::from_str(action);

        let conjuction_pos = collection
            .iter()
            .position(|&x| x.to_lowercase() == "to" || x.to_lowercase() == "from")
            .unwrap_or(2);

        let name = &collection[1..conjuction_pos].join(" ");
        let department = collection[conjuction_pos + 1];

        let data_tuple = (action, name.to_string(), department.to_string());

        Some(data_tuple)
    }
}
