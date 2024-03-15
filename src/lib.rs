const CITIES: [&str; 16] = [
    "Paris",
    "Budapest",
    "Skopje",
    "Rotterdam",
    "Valencia",
    "Vancouver",
    "Amsterdam",
    "Vienna",
    "Sydney",
    "New York City",
    "London",
    "Bangkok",
    "Hong Kong",
    "Dubai",
    "Rome",
    "Istanbul",
];

#[derive(Debug)]
pub struct CitySearch {}

impl CitySearch {
    pub fn search(&self, param: &str) -> Vec<String> {
        return Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::CitySearch;

    #[test]
    fn given_search_string_when_lengh_less_than_2_chars_then_return_empty() {
        // Arrange
        let param = "Pa";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert!(result.is_empty(), "search parameter less than 2 chars returns empty result");
    }
}
