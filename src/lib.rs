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
    pub fn search(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let text = text.to_owned();
        if text.len() < 2 {
            return result;
        }
        for city in CITIES {
            if city.contains(text.as_str()) {
                result.push(city.to_owned());
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::CitySearch;

    #[test]
    fn given_search_string_when_lengh_less_than_2_chars_then_return_empty() {
        // Arrange
        let param = "P";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert!(
            result.is_empty(),
            "search parameter less than 2 chars returns empty result"
        );
    }

    #[test]
    fn given_valid_search_substring_then_return_all_cities_with_exact_match() {
        // Arrange
        let param = "Va";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert_eq!(result.len(), 2, "Search term 'Va' returns two cities");
        assert_eq!(
            result,
            vec![String::from("Valencia"), String::from("Vancouver")],
            "search term: 'Va', result: Valencia, Vancouver"
        );
    }
}
