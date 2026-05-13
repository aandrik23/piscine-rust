pub mod mall;

pub use mall::*;

use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let employees: Vec<(&String, &Employee)> = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.iter())
        .collect();

    let max_salary = employees
        .iter()
        .map(|(_, employee)| employee.salary)
        .fold(0.0, f64::max);

    employees
        .into_iter()
        .filter(|(_, employee)| employee.salary == max_salary)
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employees_count: usize = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum();

    employees_count + mall.guards.len()
}

pub fn check_for_securities(
    mall: &mut Mall,
    available_guards: HashMap<String, Guard>,
) {
    let total_size: u64 = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let needed_guards = (total_size as usize + 199) / 200;

    for (name, guard) in available_guards {
        if mall.guards.len() >= needed_guards {
            break;
        }

        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                let amount = employee.salary * 0.10;

                if hours >= 10 {
                    employee.raise(amount);
                } else {
                    employee.cut(amount);
                }
            }
        }
    }
}