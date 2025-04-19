#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Node { next: None, value };

        match self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                let current_head = self.head.take().unwrap();
                new_node.next = Some(Box::new(current_head));
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop(&mut self) {
        self.head
            .take()
            .map(|head| self.head = head.next.map(|node| *node));
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count = count + 1;
            current_node = node.next.as_ref().map(|node| &**node)
        }
        count
    }
}