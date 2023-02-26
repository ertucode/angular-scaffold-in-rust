
use crate::utils::{self, new_line, new_line_tab};

pub trait FormField {
    fn model(&self) -> String;
    fn control(&self) -> String;
    fn constructor(&self) -> String;
    fn html(&self) -> String;
}

pub struct SimpleField<'a> {
    pub data_field: &'a str,
    pub label: &'a str,
    pub data_type: SimpleType,
    pub validator: Option<FormValidator<'a>>
}

impl FormField for SimpleField<'_> {
    fn constructor(&self) -> String {
        String::from("")
    }

    fn control(&self) -> String {
        match &self.validator {
            None => format!("{}: new FormControl()", self.data_field),
            Some(vld) => format!("{}: new TranslatedFormControl<{} | null>(null, {})", self.data_field, self.data_type.get_type(), vld.get_fn())
        }
    }

    fn html(&self) -> String {
        utils::wrap_field(
            &format!("{}{}{}{}", 
                utils::html_label(self.label, self.data_field),
                new_line_tab(5),
                self.data_type.get_html(self.data_field),
                match &self.validator {
                    Some(_) => format!("{}{}", new_line_tab(5), utils::form_error(self.data_field)),
                    None => String::from("")
                } 
            )
        )

    }

    fn model(&self) -> String {
        format!("{}: {}", self.data_field, self.data_type.get_type())
    }
}

pub enum SimpleType {
    String,
    Number(NumberType)
}

impl SimpleType {
    pub fn get_type(&self) -> String {
        match &self {
            SimpleType::String => String::from("string"),
            SimpleType::Number(_) => String::from("number")
        }
    }

    pub fn get_html(&self, field: &str) -> String {
        match &self {
            SimpleType::String => format!("<input id=\"{f}\" pInputText formControlName=\"{f}\">", f=field),
            SimpleType::Number(num) => num.get_html(field)
        }
    }
}

pub enum NumberType {
    Number,
    Decimal
}

impl NumberType {
    pub fn get_html(&self, field: &str) -> String {
        match &self {
            NumberType::Decimal => format!("<input type=\"text\" decimalInput id=\"{f}\" pInputText formControlName=\"{f}\">", f=field),
            NumberType::Number => format!("<p-inputNumber inputId=\"{f}\" [min]=\"0\" [maxFractionDigits]=\"2\" [useGrouping]=\"false\" formControlName=\"{f}\"></p-inputNumber>", f=field)
        }
    }
}

pub struct FormValidator<'a> {
    pub method: &'a str,
    pub param: Option<&'a str>
}

impl FormValidator<'_> {
    pub fn get_fn(&self) -> String {
        match self.param {
            None => format!("TranslatedValidators.{}", self.method),
            Some(param) => format!("TranslatedValidators.{}({})", self.method, param)
        }
    }
}

pub struct DateRangeField<'a> {
    pub start_field: &'a str,
    pub end_field: &'a str,
    pub label: &'a str,
}

impl FormField for DateRangeField<'_> {
    fn constructor(&self) -> String {
        format!("TranslatedValidators.addDateOrderValidator(this.form.controls.{}, this.form.controls.{})", self.start_field, self.end_field)
    }

    fn control(&self) -> String {
        format!("{}: new TranslatedFormControl<null | Date>(null),{}\t\t{}: new FormControl()", self.start_field, new_line(), self.end_field)
    }

    fn html(&self) -> String {
        utils::wrap_field_span(
            &format!("{}{}{}{}{}", 
                utils::html_label(self.label, "date"),
                new_line_tab(5),
                format!("<date-range inputId=\"date\" [startControl]=\"form.controls.{}\" [endControl]=\"form.controls.{}\"></date-range>", self.start_field, self.end_field),
                new_line_tab(5),
                utils::form_error(self.start_field)
            ),
            2
        )

    }

    fn model(&self) -> String {
        format!("{}: Date, {}\t{}: Date", self.start_field, new_line(), self.end_field)
    }
}

pub struct EnumField<'a> {
    pub data_field: &'a str,
    pub label: &'a str,
    pub enum_name: &'a str,
    pub multi: bool,
    pub validator: Option<FormValidator<'a>>
}

impl EnumField<'_> {
    pub fn get_type(&self) -> String {
        format!("EnumType[\"{}\"]", self.enum_name)
    }

    pub fn single_enum_html(&self) -> String {
        format!("<p-dropdown inputId=\"{f}\"
                        [options]=\"('{e}' | getEnumOptions | translateEnumOptions$ | async) || []\"
                        [showClear]=\"true\" optionLabel=\"label\" optionValue=\"value\" formControlName=\"{f}\"
                        [placeholder]=\"t('{l}Placeholder')\">
                    </p-dropdown>",f=self.data_field, e=self.enum_name, l=self.label)
    }

    pub fn multi_enum_html(&self) -> String {
        format!("<p-multiSelect inputId=\"{f}\"
                        [options]=\"('{e}' | getEnumOptions | translateEnumOptions$ | async)!\"
                        [showClear]=\"true\" optionLabel=\"label\" optionValue=\"value\"
                        [placeholder]=\"t('{l}Placeholder')\" formControlName=\"{f}\">
                    </p-multiSelect>", f=self.data_field, e=self.enum_name, l=self.label)
    }

    pub fn input_html(&self) -> String {
        match &self.multi {
            false => self.single_enum_html(),
            true => self.multi_enum_html()
        }
    }
}

impl FormField for EnumField<'_> {
    fn constructor(&self) -> String {
        String::from("")
    }

    fn control(&self) -> String {    
        match &self.validator {
            None => format!("{}: new FormControl()", self.data_field),
            Some(vld) => format!("{}: new TranslatedFormControl<{} | null>(null, {})", self.data_field, self.get_type(), vld.get_fn())
        }
    }

    fn html(&self) -> String {
        utils::wrap_field(
            &format!("{}{}{}{}{}", 
                utils::html_label(self.label, self.data_field),
                new_line_tab(5),
                self.input_html(),
                new_line_tab(5),
                utils::form_error(&self.data_field)
            )
        )

    }

    fn model(&self) -> String {
        format!("{}: {}", self.data_field, self.get_type())
    }
}



pub fn create_constructor<'a, I>(vals: I) -> String
where I: Iterator<Item = &'a dyn FormField>
{
    vals.map(|val| val.constructor()).filter(|v| v!="").collect::<Vec<String>>().join(new_line_tab(1).to_owned().as_str())
}

pub fn create_controls<'a, I>(vals: I) -> String
where I: Iterator<Item = &'a dyn FormField>
{
    vals.map(|val| val.control()).collect::<Vec<String>>().join((format!(",{}", new_line_tab(2))).to_owned().as_str())
}

pub fn create_models<'a, I>(vals: I) -> String
where I: Iterator<Item = &'a dyn FormField>
{
    vals.map(|val| val.model()).collect::<Vec<String>>().join((format!(",{}", new_line_tab(1))).to_owned().as_str())
}

pub fn create_htmls<'a, I>(vals: I) -> String
where I: Iterator<Item = &'a dyn FormField>
{
    vals.map(|val| val.html()).collect::<Vec<String>>().join(new_line_tab(4).to_owned().as_str())
}