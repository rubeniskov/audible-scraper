use url::{ParseError, Url};

#[derive(Default, Clone)]
pub struct QueryParams {
    narrator: Option<String>,
    keywords: Option<String>,
    sort: String,
    page_size: u32,
    page: u32,
}

#[allow(dead_code)]
impl QueryParams {
    pub fn new() -> Self {
        Self {
            page_size: 50,
            sort: "title-asc-rank".to_string(),
            page: 1,
            ..Default::default()
        }
    }

    pub fn keywords(mut self, keywords: &str) -> Self {
        self.keywords = Some(keywords.to_string());
        self
    }

    pub fn narrator(mut self, narrator: &str) -> Self {
        self.narrator = Some(narrator.to_string());
        self
    }

    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = sort.to_string();
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn build_url(&self) -> Result<Url, ParseError> {
        let mut base_url = Url::parse("https://www.audible.es/search?")?;

        if let Some(ref narrator) = self.narrator {
            base_url
                .query_pairs_mut()
                .append_pair("searchNarrator", narrator);
        }

        base_url
            .query_pairs_mut()
            .append_pair("sort", &self.sort)
            .append_pair("pageSize", &self.page_size.to_string())
            .append_pair("page", &self.page.to_string());

        Ok(base_url)
    }
}
