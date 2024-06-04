pub fn process_dataset(dataset_path: &str) -> Result<(), CliError> {
    let dataset = local_fs::read_file(dataset_path).map_err(CliError::IoError)?;
    let template = templates::get_template("default_template").map_err(CliError::ParseError)?;
    if !templates::validate_dataset(&template, &dataset) {
        return Err(CliError::ParseError("Invalid dataset format".to_string()));
    }
    let processed_dataset =
        row_process::process_dataset(&dataset, &template).map_err(CliError::ParseError)?;
    local_fs::write_file("output.parquet", &processed_dataset).map_err(CliError::IoError)?;
    Ok(())
}
