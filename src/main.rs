use anyhow::Result;
use arxiv::{Arxiv, ArxivQueryBuilder};

async fn fetch_quantum_computing_papers_from_arxiv(number_of_papers: usize) -> Result<Vec<Arxiv>> {
    let query = ArxivQueryBuilder::new()
        .search_query("quantum computing")
        .start(0)
        .max_results(number_of_papers as i32)
        .sort_by("submittedDate")
        .sort_order("descending")
        .build();
    arxiv::fetch_arxivs(query).await
}

async fn persist_arxiv_papers(arxivs: Vec<Arxiv>) -> Result<()> {
    for arxiv in arxivs {
        arxiv.fetch_pdf(&arxiv.title).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let papers = fetch_quantum_computing_papers_from_arxiv(5).await?;
    persist_arxiv_papers(papers).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_fetch_5_arxiv_papers() -> Result<()> {
        // Test if the function correctly fetches Quantum Computing papers from arXiv
        let papers = fetch_quantum_computing_papers_from_arxiv(5).await?;
        assert!(papers.len() == 5, "Failed to fetch papers from arXiv");
        Ok(())
    }

    #[tokio::test]
    async fn test_persists_arxiv_papers() -> Result<()> {
        // Test if the function correctly persists Quantum Computing papers from arXiv
        let papers = fetch_quantum_computing_papers_from_arxiv(5).await?;
        let result = persist_arxiv_papers(papers).await?;
        assert!(result == (), "Failed to persist papers from arXiv");
        Ok(())
    }
}
