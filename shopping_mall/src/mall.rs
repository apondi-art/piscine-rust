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
            guards: guards,
            floors: floors,
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
                age: age,
                years_experience: years_experience,
            }
        }
    }
}

pub mod floor {

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
                stores: stores,
                size_limit: store_limit,
            }
        }

        #[allow(dead_code)]
        pub fn change_store(&mut self, store: &str, new_store: store::Store) {
            let pos = self.stores.iter().position(|x| x.name == store).unwrap();
            self.stores[pos] = new_store;
        }

        #[allow(dead_code)]
        pub fn add_store(&mut self, new_store: store::Store) {
            let mut current_floor_size = 0;

            for store in self.stores.iter() {
                current_floor_size += store.square_meters;
            }

            if self.size_limit < current_floor_size + new_store.square_meters {
                self.stores.push(new_store);
            }
        }

        #[allow(dead_code)]
        pub fn remove_store(&mut self, store_name: String) {
            self.stores.retain(|x| x.name != store_name);
        }
    }

    pub mod store {

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
                    employees: employees,
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
                        age: age,
                        working_hours: (entry_hour, exit_hour),
                        salary: salary,
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
                    self.salary = self.salary - amount;
                }
            }
        }
    }
}

pub fn biggest_store(mall: Mall) -> floor::store::Store {
    mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores)
        .max_by_key(|store| store.square_meters)
        .expect("Mall has no stores")
}

pub fn highest_paid_employee(mall: Mall) -> Vec<floor::store::employee::Employee> {
    let all_employees: Vec<_> = mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores.into_iter().flat_map(|store| store.employees))
        .collect();
    
    let max_salary = all_employees.iter()
        .map(|e| e.salary)
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));
    
    all_employees.into_iter()
        .filter(|e| (e.salary - max_salary).abs() < f64::EPSILON)
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employees_count: usize = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.employees.len())
        .sum();
    employees_count + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<guard::Guard>) {
    let total_floor_size: u64 = mall.floors.iter().map(|floor| floor.size_limit).sum();
    let required_guards = (total_floor_size + 199) / 200;
    let current_guards = mall.guards.len() as u64;
    let needed = required_guards.saturating_sub(current_guards) as usize;
    
    let guards_to_add: Vec<_> = guards.into_iter().take(needed).collect();
    mall.guards.extend(guards_to_add);
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
                
                if duration > 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
            }
        }
    }
}




pub use guard::Guard;
pub use floor::Floor;
pub use floor::store::Store;
pub use floor::store::employee::Employee;