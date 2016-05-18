use std::iter::Iterator;
use std::collections::BTreeMap;

pub struct School {
    grades_map: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School{grades_map: BTreeMap::new()}
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades_map
            .keys()
            .map(|&grade| grade)
            .collect()
    }

    pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
        self.grades_map.get(&grade)
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        let mut grade = self.grades_map.entry(grade).or_insert(vec!());
        grade.push(name.to_string());
        grade.sort();
    }
}
