const MIN_SEARCH_LEN: usize = 2;

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
        let text = Self::convert_to_lowercase(text);
        if Self::search_term_is_asterisk(&text) {
            for c in CITIES {
                result.push(c.to_owned())
            }

            return result;
        }
        if !Self::valid_length(&text) {
            return result;
        }
        for city in CITIES {
            if Self::substring_match(city, &text) {
                result.push(city.to_owned());
            }
        }

        return result;
    }

    fn convert_to_lowercase(text: &str) -> String {
        text.to_owned().to_lowercase()
    }

    fn valid_length(text: &String) -> bool {
        text.len() >= MIN_SEARCH_LEN
    }

    fn search_term_is_asterisk(text: &str) -> bool {
        if text == "*" {
            return true
        }

        return false
    }

    fn substring_match(city: &str, substr: &String) -> bool {
        let lower = Self::convert_to_lowercase(city);
        if lower.contains(substr.as_str()) {
            return true
        }

        return false
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

    #[test]
    fn given_valid_search_text_when_search_then_result_is_case_insensitive() {
        // Arrange
        let param = "va";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert_eq!(result.len(), 2, "Search term 'va' returns two cities");
        assert_eq!(
            result,
            vec![String::from("Valencia"), String::from("Vancouver")],
            "search term: 'Va', result: Valencia, Vancouver"
        );
    }

    #[test]
    fn given_valid_search_text_when_substring_match_middle_of_word_then_is_valid() {
        // Arrange
        let param = "ape";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert_eq!(result.len(), 1, "Search term 'ape' returns one city");
        assert_eq!(
            result,
            vec![String::from("Budapest")],
            "search term: 'ape', matches: Budapest"
        );
    }

    #[test]
    fn given_asterisk_search_text_then_return_all_cities() {
        // Arrange
        let param = "*";
        let search = CitySearch {};

        // Act
        let result = search.search(param);

        // Assert
        assert_eq!(
            result,
            vec![
                String::from("Paris"),
                String::from("Budapest"),
                String::from("Skopje"),
                String::from("Rotterdam"),
                String::from("Valencia"),
                String::from("Vancouver"),
                String::from("Amsterdam"),
                String::from("Vienna"),
                String::from("Sydney"),
                String::from("New York City"),
                String::from("London"),
                String::from("Bangkok"),
                String::from("Hong Kong"),
                String::from("Dubai"),
                String::from("Rome"),
                String::from("Istanbul"),
            ],
            "search term: '*', matches all cities"
        );
    }
}
