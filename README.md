### README.md
# Data Garden: grooming your datasets with ❤️

Data Garden is a tool for processing datasets using different templates. It supports various dataset formats such as JSON, JSONL, CSV, and Parquet. The tool allows users to preprocess, process, and postprocess datasets, and then either save the results locally or push them to a Git repository, including Hugging Face repositories. Data Garden can be run as a CLI tool or as a server with a web-based UI.

## Features

- Support for multiple dataset formats: JSON, JSONL, CSV, and Parquet.
- Template-based dataset validation and processing.
- Preprocessing and postprocessing steps for enhanced data manipulation.
- Save results locally or push to any Git repository, with specific support for Hugging Face repositories.
- Multi-threaded processing for improved performance.
- Configurable via YAML/TOML configuration files.
- Error logging to prevent data loss.
- Web UI for managing dataset processing through a browser.

## Requirements

- Rust (latest stable version)
- Git
- Dependencies listed in `Cargo.toml`

## Project Structure

```
data_garden/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── local_fs.rs
│   ├── git_repo.rs
│   ├── huggingface_repo.rs
│   ├── templates.rs
│   ├── row_process.rs
│   ├── webui_server.rs
├── config.yaml
├── projects/
│   ├── output/
│   ├── config/
│   ├── input/
├── webui/
├── hooks/
│   ├── example_hooks/
│   ├── user_defined/
├── services/
│   ├── user_defined/
├── templates/
│   ├── user_defined/
└── libs/
    ├── local_fs/
    ├── git_repo/
    ├── huggingface_repo/
    ├── templates/
    ├── row_process/
    ├── webui_server/
    ├── cli/
```

## Configuration

The main configuration file is located in the root directory (`config.yaml` or `config.toml`). This file contains global settings for the project, including logging levels, server configurations, and repository options.

Example `config.yaml`:

```yaml
logging:
  level: "info"

server:
  port: 8080

templates:
  path: "templates"

repositories:
  default: "huggingface"
  options:
    huggingface:
      url: "https://huggingface.co"
    custom:
      url: "https://custom.git.repo"
```

## Running Data Garden

### As a CLI Tool

To process a dataset or start the server, use the following commands:

```bash
cargo run -- --process /path/to/dataset
cargo run -- --server
```

### As a Server with Web UI

Start the server using the CLI:

```bash
cargo run -- --server
```

Then, open your browser and navigate to `http://localhost:8080`.

## Error Handling

Data Garden logs errors to prevent data loss. Logs are excluded from version control.

## Contributing

Contributions are welcome! Please read the [contributing guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Contact

For any questions, please contact the maintainers at [israellaguan@gmail.com](mailto:israellaguan@gmail.com).
