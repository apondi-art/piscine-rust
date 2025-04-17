#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
      WorkEnvironment{grade : None}
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker{
            name,
            role,
            next : self.grade.take()
        });
        self.grade = Some(new_worker);

       
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take(){
            self.grade = worker.next;
            Some(worker.name)
        }else{
            None
        }
    
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
       self.grade
       .as_ref()
       .map(|worker|(worker.name.clone(), worker.role.clone()))
       
    }
}