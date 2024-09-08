pub mod config {
    use config::Config;
    use dotenv::dotenv;
    use serde::Deserialize;
    use std::{collections::HashMap, fs};

    #[derive(Debug, Deserialize)]
    pub struct AppConfig {
        pub templates: TemplatesConfig,
        pub repositories: RepositoriesConfig,
        pub datasets: DatasetsConfig,
        #[serde(default)]
        pub project: Option<ProjectConfig>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TemplatesConfig {
        pub path: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct RepositoriesConfig {
        pub default: String,
        pub options: HashMap<String, RepositoryOption>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RepositoryOption {
        pub url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct DatasetsConfig {
        pub supported_formats: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ProjectConfig {
        pub name: String,
        pub description: String,
        pub repository: String,
        pub license: String,
        pub author: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct EnvConfig {
        pub openai_api_key: String,
        pub gemini_api_key: String,
        pub huggingface_key: String,
        pub github_token: String,
        pub port: u16,
        pub home_app_dir: String,
        pub url_whitelist: String,
        pub logging_level: String,
    }

    pub fn load_configuration(project: Option<&str>) -> Result<(EnvConfig, AppConfig), Box<dyn std::error::Error>> {
        // Load environment variables from .env file
        dotenv().ok();
        let env_config = Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()?
            .try_deserialize::<EnvConfig>()?;
        
        // Load application configuration from config.yaml
        let config_content = fs::read_to_string("config.yaml")?;
        let mut app_config: AppConfig = serde_yml::from_str(&config_content)?;

        // Optionally load project-specific configuration
        if let Some(project_name) = project {
            let project_config_path = format!("src/projects/{}/config.project.yml", project_name);
            if let Ok(project_config_content) = fs::read_to_string(&project_config_path) {
                let project_config: ProjectConfig = serde_yml::from_str(&project_config_content)?;
                app_config.project = Some(project_config);
            }
        }

        Ok((env_config, app_config))
    }
}
