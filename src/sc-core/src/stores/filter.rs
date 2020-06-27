pub trait KeyFilter<V: ?Sized> {

    fn filter(&self,value: &V) -> bool;
}


impl KeyFilter<str> for str {

    fn filter(&self, value: &str) -> bool {
        value.contains(self)
    }

}

impl KeyFilter<str> for Vec<String> {

    fn filter(&self, value: &str) -> bool {
        self.iter()
            .filter(|key| key.filter(value))
            .count() == 0
    }
}

#[cfg(test)]
mod tests {

    use super::KeyFilter;

    #[test]
    fn test_str() {

        let value = "quick brown";
        assert!("quick".filter(value));
        assert!(vec!["q".to_owned(),"b".to_owned()].filter(value));
    }

}