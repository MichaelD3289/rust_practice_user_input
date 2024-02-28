use std::collections::HashMap;

#[derive(Debug)]
pub struct Company {
    name: String,
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new(name: &str) -> Self {
        Company {
            name: String::from(name),
            departments: HashMap::new(),
        }
    }

    pub fn add_to_department(&mut self, department: &str, employee: &str) -> Result<(), ()> {
        if let Some(employees) = self.departments.get_mut(department) {
            employees.push(employee.to_string());
            return Ok(());
        }

        if let None = self
            .departments
            .insert(department.to_string(), vec![employee.to_string()])
        {
            return Ok(());
        }

        Err(())
    }

    pub fn remove_from_department(&mut self, department: &str, employee: &str) -> Result<(), ()> {
        if let Some(employees) = self.departments.get_mut(department) {
            if let Some(index) = employees.iter().position(|n| *n == employee.to_string()) {
                if index < employees.len() {
                    employees.remove(index);
                    return Ok(());
                }
            }
        }

        Err(())
    }

    pub fn list_departments(&self) {
        println!("\nCompany: {}", self.name);
        for (key, value) in self.departments.iter() {
            println!("\tDepartment: '{key}'");
            for name in value {
                println!("\t\tEmployee: '{name}'");
            }
        }
    }
}
