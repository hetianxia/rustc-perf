pub mod next_commit {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Commit {
        pub sha: String,
        pub include: Option<String>,
        pub exclude: Option<String>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        pub commit: Option<Commit>,
    }
}
