use std::fs;
use std::fs::File;
use std::fs::create_dir;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use config::LIST_CONFIG;
use list::ListField;
use convert_case::{Case, Casing};

fn replace_file_content(file_content: &String, module_name: &str) -> String {
    let replaced = file_content.replace("{{ModuleName}}", &module_name.to_case(Case::Pascal));
    let replaced = replaced.replace("{{module-name}}", &module_name.to_case(Case::Kebab));
    return replaced;
}

fn open_and_replace(path: &str, module_name: &str) -> String {
    let content = read_to_string(path).unwrap();
    let content = replace_file_content(&content, module_name);
    return content
}

fn open_and_write(base_path: &str, end_path: &Path, module_name: &str) {
    let content = open_and_replace(base_path, module_name);
    
    let mut file = File::create(end_path).unwrap();
    file.write_all(&content.as_bytes()).unwrap();
}

fn handle_root_file(path_suffix: &str, base_file: &str, module_name: &str, module_path: &Path) {
    let file_name = [module_name, path_suffix].join("-");
    let file_path = Path::join(&module_path, Path::new(&file_name));
    open_and_write(base_file, &file_path, module_name);
}

fn create_component_files(folder_path: &PathBuf, file_prefix: &str) -> ComponentFiles {
    let html = Path::join(&folder_path, [file_prefix, "component.html"].join("."));
    let html = File::create(html).unwrap();

    let scss = Path::join(&folder_path, [file_prefix, "component.scss"].join("."));
     File::create(scss).unwrap();

    let ts = Path::join(&folder_path, [file_prefix, "component.ts"].join("."));
    let ts = File::create(ts).unwrap();
    return ComponentFiles { html, ts }
}

pub mod utils;
pub mod list;
pub mod filter;
pub mod config;
pub mod parser;

struct ComponentFiles {
    pub html: File,
    pub ts: File
}

fn main() {
    let module_name = "business-account";
    let module_path = Path::new(&module_name);

    if Path::new(module_name).exists() {
        fs::remove_dir_all(module_name).unwrap();
    }

    create_dir(module_name).unwrap();

    handle_root_file("routing.module.ts", "base/routing.txt", module_name, module_path);
    handle_root_file("module.ts", "base/module.txt", module_name, module_path);
    handle_root_file("service.ts", "base/service.txt", module_name, module_path);

    let page_folder_name = [module_name, "page"].join("-");
    let page_folder_path = Path::join(&module_path, Path::new(&page_folder_name));
    create_dir(&page_folder_path).unwrap();
    let mut page_files = create_component_files(&page_folder_path, &page_folder_name);

    let page_ts_content = open_and_replace("base/pagets.txt", module_name);
    page_files.ts.write_all(&page_ts_content.as_bytes()).unwrap();

    let page_html_content = open_and_replace("base/pagehtml.txt", module_name);
    page_files.html.write_all(&page_html_content.as_bytes()).unwrap();
    
    let list_folder_name = [module_name, "list"].join("-");
    let list_folder_path = Path::join(&page_folder_path, Path::new(&list_folder_name));
    create_dir(&list_folder_path).unwrap();
    let mut list_files = create_component_files(&list_folder_path, &list_folder_name);

    let list_ts_content = open_and_replace("base/listts.txt", module_name);
    list_files.ts.write_all(&list_ts_content.as_bytes()).unwrap();

    let list_html_content = open_and_replace("base/listhtml.txt", module_name);
    let headers = ListField::create_headers(LIST_CONFIG.iter());
    let list_html_content = list_html_content.replace("{{ColumnHeaders}}", &headers);
    let bodies = ListField::create_bodies(LIST_CONFIG.iter());
    let list_html_content = list_html_content.replace("{{ColumnBody}}", &bodies);
    list_files.html.write_all(&list_html_content.as_bytes()).unwrap();

    let filter_folder_name = [module_name, "filter"].join("-");
    let filter_folder_path = Path::join(&page_folder_path, Path::new(&filter_folder_name));
    create_dir(&filter_folder_path).unwrap();
    let mut filter_files = create_component_files(&filter_folder_path, &filter_folder_name);

    let filter_config = config::get_filter_config();

    let filter_ts_content = open_and_replace("base/filterts.txt", module_name);
    let filter_ts_content = filter_ts_content.replace("{{FormFields}}", &filter::create_controls(filter_config.clone().into_iter()));
    let filter_ts_content = filter_ts_content.replace("{{Constructor}}", &filter::create_constructor(filter_config.clone().into_iter()));
    filter_files.ts.write_all(&filter_ts_content.as_bytes()).unwrap();

    let filter_html_content = open_and_replace("base/filterhtml.txt", module_name);
    let filter_html_content = filter_html_content.replace("{{FormFields}}", &filter::create_htmls(filter_config.clone().into_iter()));
    filter_files.html.write_all(&filter_html_content.as_bytes()).unwrap();
    
    let models_folder_name = "models";
    let models_folder_path = Path::join(&module_path, Path::new(&models_folder_name));
    create_dir(&models_folder_path).unwrap();  

    let list_model_name = "list.model.ts";
    let list_model_path = Path::join(&models_folder_path, Path::new(list_model_name));
    let list_model_content = open_and_replace("base/listmodel.txt", module_name);  
    let list_models = ListField::create_model_fields(LIST_CONFIG.iter());
    let list_model_content = list_model_content.replace("{{Models}}", &list_models);
    let mut list_model_file = File::create(&list_model_path).unwrap();
    list_model_file.write_all(&list_model_content.as_bytes()).unwrap();

    let filter_model_name = "filter.model.ts";
    let filter_model_path = Path::join(&models_folder_path, Path::new(filter_model_name));

    let filter_model_content = open_and_replace("base/filtermodel.txt", module_name);  
    let filter_models = filter::create_models(filter_config.clone().into_iter());
    let filter_model_content = filter_model_content.replace("{{Models}}", &filter_models);
    let mut filter_model_file = File::create(&filter_model_path).unwrap();
    filter_model_file.write_all(&filter_model_content.as_bytes()).unwrap();

    let list_config_file = read_to_string("config/list.ts").unwrap();
    parser::parse_list_file(&list_config_file);

}
