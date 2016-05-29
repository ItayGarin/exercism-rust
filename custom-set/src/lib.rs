#[derive(Debug, Clone)]
pub struct CustomSet<T> {
    pub elements: Vec<T>,
}

impl<T> CustomSet<T>
    where T: PartialEq + Clone
{
    pub fn new(elements: Vec<T>) -> CustomSet<T> {
        let mut set = CustomSet{elements: vec![]};
        for element in elements {
            set.add(element);
        }
        set
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }
    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.elements.iter().all(|element| other.contains(element))
    }
    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        !self.elements.iter().any(|element| other.contains(element))
    }
    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.elements
                       .iter()
                       .cloned()
                       .filter(|element| other.contains(element))
                       .collect())
    }
    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.elements
                       .iter()
                       .cloned()
                       .filter(|element| !other.contains(element))
                       .collect())
    }
    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.elements
                       .iter()
                       .cloned()
                       .chain(other.elements.iter().cloned())
                       .collect())
    }
}

impl<T> PartialEq for CustomSet<T>
    where T: PartialEq + Clone {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }

        self.elements.iter().all(|element| other.contains(element))
    }
}
