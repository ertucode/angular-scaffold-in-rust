pub fn concat_str(str1: &str, str2: &str) -> String {
    [str1, str2].join("")
}

pub fn get_initials(module_name: &str) -> String {
    module_name.split("-").map(|s| s.chars().nth(0).unwrap()).collect()
}

pub fn new_line() -> String {
    (0xA as char).to_string()
}

pub fn new_line_tab(tabs: usize) -> String {
    format!("{}{}", new_line(), "\t".repeat(tabs).as_str())
}

pub fn wrap_field(field: &str) -> String {
    format!("<div class=\"form-input-field\">{}{}{}</div>", new_line_tab(5), field, new_line_tab(4))
} 

pub fn wrap_field_span(field: &str, span: i32) -> String {
    format!("<div class=\"span-{} form-input-field\">{}{}{}</div>", span, new_line_tab(5), field, new_line_tab(4))
} 

pub fn html_label(display: &str, field: &str) -> String {
    format!("<label for=\"{}\">{{{{t('{}')}}}}</label>", field, display)
}

pub fn form_error(field: &str) -> String {
    format!("<form-error class=\"form-error-message\" [showErrors]=\"showErrors\" [control]=\"form.controls.{}\"></form-error>", field)
}