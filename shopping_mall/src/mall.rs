#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: Vec<guard::Guard>,
    pub floors: Vec<floor::Floor>,
}

impl Mall {
    #[allow(dead_code)]
    pub fn new(name: &str, guards: Vec<guard::Guard>, floors: Vec<floor::Floor>) -> Mall {
        Mall {
            name: name.to_string(),
            guards,
            floors,
        }
    }

    #[allow(dead_code)]
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    #[allow(dead_code)]
    pub fn hire_guard(&mut self, guard: guard::Guard) {
        self.guards.push(guard);
    }

    #[allow(dead_code)]
    pub fn fire_guard(&mut self, name: String) {
        self.guards.retain(|x| x.name != name);
    }
}

pub mod guard {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Guard {
        pub name: String,
        pub age: u8,
        pub years_experience: u8,
    }

    impl Guard {
        #[allow(dead_code)]
        pub fn new(name: &str, age: u8, years_experience: u8) -> Guard {
            Guard {
                name: name.to_string(),
                age,
                years_experience,
            }
        }
    }
}

pub mod floor {
    pub mod store {
        pub mod employee {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Employee {
                pub name: String,
                pub age: u8,
                pub working_hours: (u8, u8),
                pub salary: f64,
            }

            impl Employee {
                #[allow(dead_code)]
                pub fn new(
                    name: &str,
                    age: u8,
                    entry_hour: u8,
                    exit_hour: u8,
                    salary: f64,
                ) -> Employee {
                    Employee {
                        name: name.to_string(),
                        age,
                        working_hours: (entry_hour, exit_hour),
                        salary,
                    }
                }

                #[allow(dead_code)]
                pub fn birthday(&mut self) {
                    self.age += 1;
                }

                #[allow(dead_code)]
                pub fn change_workload(&mut self, entry_hour: u8, exit_hour: u8) {
                    self.working_hours = (entry_hour, exit_hour);
                }

                #[allow(dead_code)]
                pub fn raise(&mut self, amount: f64) {
                    self.salary += amount;
                }

                #[allow(dead_code)]
                pub fn cut(&mut self, amount: f64) {
                    self.salary -= amount;
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Store {
            pub name: String,
            pub square_meters: u64,
            pub employees: Vec<employee::Employee>,
        }

        impl Store {
            #[allow(dead_code)]
            pub fn new(name: &str, space: u64, employees: Vec<employee::Employee>) -> Store {
                Store {
                    name: name.to_string(),
                    square_meters: space,
                    employees,
                }
            }

            #[allow(dead_code)]
            pub fn hire_employee(&mut self, employee: employee::Employee) {
                self.employees.push(employee);
            }

            #[allow(dead_code)]
            pub fn fire_employee(&mut self, employee_name: &str) {
                self.employees.retain(|x| x.name != employee_name);
            }

            #[allow(dead_code)]
            pub fn expand(&mut self, square_meters: u64) {
                self.square_meters += square_meters;
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Floor {
        pub name: String,
        pub stores: Vec<store::Store>,
        pub size_limit: u64,
    }

    impl Floor {
        #[allow(dead_code)]
        pub fn new(name: &str, stores: Vec<store::Store>, store_limit: u64) -> Floor {
            Floor {
                name: name.to_string(),
                stores,
                size_limit: store_limit,
            }
        }

        #[allow(dead_code)]
        pub fn change_store(&mut self, store: &str, new_store: store::Store) {
            if let Some(pos) = self.stores.iter().position(|x| x.name == store) {
                self.stores[pos] = new_store;
            }
        }

        #[allow(dead_code)]
        pub fn add_store(&mut self, new_store: store::Store) {
            let current_size: u64 = self.stores.iter().map(|s| s.square_meters).sum();
            if current_size + new_store.square_meters <= self.size_limit {
                self.stores.push(new_store);
            }
        }

        #[allow(dead_code)]
        pub fn remove_store(&mut self, store_name: String) {
            self.stores.retain(|x| x.name != store_name);
        }
    }
}

// Public API exports
pub use guard::Guard;
pub use floor::Floor;
pub use floor::store::Store;
pub use floor::store::employee::Employee;

// Management functions
pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores)
        .max_by_key(|store| store.square_meters)
        .expect("Mall has no stores")
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let all_employees: Vec<_> = mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores.into_iter().flat_map(|store| store.employees))
        .collect();

    let max_salary = all_employees.iter()
        .map(|e| e.salary)
        .fold(f64::NEG_INFINITY, f64::max);

    all_employees.into_iter()
        .filter(|e| (e.salary - max_salary).abs() < f64::EPSILON)
        .collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let employees_count: usize = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.employees.len())
        .sum();
    employees_count + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let total_size: u64 = mall.floors.iter().map(|f| f.size_limit).sum();
    let required_guards = (total_size + 199) / 200;
    let current_guards = mall.guards.len() as u64;
    let needed = required_guards.saturating_sub(current_guards) as usize;

    mall.guards.extend(guards.into_iter().take(needed));
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let (entry, exit) = employee.working_hours;
                let duration = if exit >= entry {
                    exit - entry
                } else {
                    (exit + 24) - entry
                };

                // Apply multiplication and round to 4 decimal places
                employee.salary = if duration > 10 {
                    (employee.salary * 1.1 * 10000.0).round() / 10000.0
                } else {
                    (employee.salary * 0.9 * 10000.0).round() / 10000.0
                };
            }
        }
    }
}