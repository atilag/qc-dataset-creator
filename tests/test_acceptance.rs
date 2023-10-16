extern crate qc_dataset_creator;
use qc_dataset_creator::*;

#[cfg(test)]
mod acceptance_tests {
    use super::*;

    #[test]
    fn test_fetch_arxiv_papers() {
        // Test if the function correctly fetches Quantum Computing papers from arXiv
        let papers = fetch_arxiv_papers();
        assert!(papers.len() > 0, "Failed to fetch papers from arXiv");
    }

    #[test]
    fn test_dataset_creation() {
        // Test if the dataset is correctly created from the fetched papers
        let dataset = create_dataset();
        assert!(dataset.len() > 0, "Failed to create dataset from papers");
    }

    #[test]
    fn test_data_cleaning() {
        // Test if the data cleaning function works as expected
        let raw_data = "Some raw data with inconsistencies"; // Sample raw data
        let cleaned_data = clean_data(raw_data);
        assert_ne!(
            raw_data, cleaned_data,
            "Data cleaning did not modify the raw data"
        );
    }

    #[test]
    fn test_data_export() {
        // Test if the dataset is correctly exported to a desired format (e.g., CSV)
        let dataset = create_dataset();
        let export_path = "path/to/exported/file.csv";
        export_dataset_to_csv(dataset, export_path);

        // Check if the exported file exists
        assert!(
            fs::metadata(export_path).is_ok(),
            "Failed to export dataset to CSV"
        );
    }
}
