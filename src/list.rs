pub struct ListField<'a> {
    pub data_field: &'a str,
    pub sort_field: Option<&'a str>,
    pub display_field: Option<Display<'a>>,
    pub ts_type: TsType<'a>,
}

impl ListField<'_> {

    pub const fn only_field<'a>(data_field: &'a str, ts_type: TsType<'a>) -> ListField<'a> {
        ListField { data_field, sort_field: None, display_field: None, ts_type }
    }

    fn create_header(&self) -> String {

        if let None = self.display_field {
            return String::from("")
        }

        let start= "<th ";
        let sort = match self.sort_field {
            None => "".to_owned(),
            Some(field) => format!("pSortableColumn=\"{}\" sortableColumnHeader ", field)
        };

        let display = match self.sort_field {
            None => match &self.display_field {
                None => "".to_owned(),
                Some(ds) => match ds {
                    Display::Tr(tr) => format!(">{{t('{}')}}", tr),
                    Display::No(no) => format!(">{{{}}}", no)
                } 
            },
            Some(_) => match &self.display_field {
                None => "".to_owned(),
                Some(ds) => match ds {
                    Display::Tr(tr) => format!("[label]=\"t('{}')\">", tr),
                    Display::No(no) => format!("label=\"{}\">", no)
                } 
            }
        };

        let end = "</th>";

        return start.to_owned() + &sort + &display + end
    }

    pub fn create_headers<'a, I>(vals: I) -> String
    where I: Iterator<Item = &'a ListField<'a>>
    {
        vals.map(|val| val.create_header()).filter(|v| v != "").collect::<Vec<String>>().join(((0xA as char).to_string() + "\t\t\t\t").to_owned().as_str())
    }

    pub fn create_body(&self) -> String {
        match self.display_field {
            None => "".to_owned(),
            Some(_) => match &self.ts_type {
                TsType::DateString => format!("<td>{{{{item.{} | localizeDate}}}}</td>", self.data_field),
                TsType::TsEnum(ts_enum) => if ts_enum.severity {
                    format!("<td [style.text-align]=\"'center'\">
                    <p-badge [value]=\"enum.t(item.{field} ,'{enum_name}')\"
                        [severity]=\"item.{field} | getEnumSeverity : '{enum_name}'\"></p-badge>
                    </td>", field=self.data_field, enum_name=ts_enum.name)
                } else {
                    format!("<td>{{{{enum.t(item.{}, '{}')}}}}</td>", self.data_field, ts_enum.name)
                },
                _ => format!("<td>{{{{item.{}}}}}</td>", self.data_field)
            }
        }
    }

    pub fn create_bodies<'a, I>(vals: I) -> String
    where I: Iterator<Item = &'a ListField<'a>>
    {
        vals.map(|val| val.create_body()).filter(|v| v != "").collect::<Vec<String>>().join(((0xA as char).to_string() + "\t\t\t\t").to_owned().as_str())
    }

    pub fn create_model_field(&self) -> String {
        format!("{}: {}", self.data_field, self.ts_type.get_representation())
    }

    pub fn create_model_fields<'a, I>(vals: I) -> String
    where I: Iterator<Item = &'a ListField<'a>>
    {
        vals.map(|val| val.create_model_field()).filter(|v| v != "").collect::<Vec<String>>().join(format!(";{}\t", (0xA as char).to_string()).to_owned().as_str())
    }
}

pub enum TsType<'a> {
    String,
    Number,
    DateString,
    TsEnum(TsEnum<'a>)
}

impl TsType<'_> {
    pub const fn default() -> TsType<'static> {
        TsType::String
    }

    pub fn get_representation(&self) -> String {
        match &self {
            TsType::String => String::from("string"),
            TsType::Number => String::from("number"),
            TsType::DateString => String::from("string | Date"),
            TsType::TsEnum(ts_enum) => format!("EnumType[\"{}\"]", ts_enum.name)
        }
    }
}

pub struct TsEnum<'a> {
    pub name: &'a str,
    pub severity: bool
}

impl TsEnum<'_> {
    pub const fn no_severity(name: &str) -> TsEnum {
        TsEnum {
            name,
            severity: false
        }
    }
}

pub enum Display<'a> {
    Tr(&'a str),
    No(&'a str)
}
