use crate::list::{ListField, Display, TsType, TsEnum};
use crate::filter::{FormField, SimpleType, FormValidator, DateRangeField, SimpleField, EnumField};

pub static LIST_CONFIG: [ListField; 9] = 
    [
        ListField {data_field: "id", sort_field: Some("Id"), ts_type: TsType::String, display_field: Some(Display::No("Id"))},
        ListField {data_field: "start_time", sort_field: Some("StartTime"), ts_type: TsType::DateString, display_field: Some(Display::Tr("bulk.processStartDate"))},
        ListField {data_field: "end_time", sort_field: Some("EndTime"), ts_type: TsType::DateString, display_field:  Some(Display::Tr("bulk.processEndDate"))},
        ListField {data_field: "success", sort_field: Some("Success"), ts_type: TsType::Number, display_field: Some(Display::Tr("bulk.successfulProcessCount"))},
        ListField {data_field: "fail", sort_field: Some("Fail"), ts_type: TsType::Number, display_field: Some(Display::Tr("bulk.failedProcessCount"))},
        ListField {data_field: "created_date_utc", sort_field: Some("CreatedDateUtc"), ts_type: TsType::DateString, display_field: Some(Display::Tr("createdDate"))},
        ListField::only_field("request_owner_system_user_id", TsType::Number),
        ListField::only_field("request_owner_account_id", TsType::Number),
        ListField::only_field("transaction_type_code", TsType::TsEnum(TsEnum::no_severity("TransactionType"))),
    ];

    pub fn get_filter_config() -> Vec::<&'static dyn FormField> {
        vec![
            &DateRangeField {start_field: "start_date", end_field: "end_date", label: "dateRange"},
            &SimpleField {data_field: "name", label: "name", data_type: SimpleType::String, validator: None},
            &SimpleField {data_field: "account_number", label: "accountNumber", data_type: SimpleType::String, validator: None},
            &EnumField {data_field: "kyc_level", enum_name: "KycLevelStatus", label: "kycLevel", multi: false, validator: None},
            &SimpleField {data_field: "email", label: "email", data_type: SimpleType::String, validator: Some(FormValidator {method: "email", param: None})},
            &EnumField {data_field: "access_status_id", enum_name: "AccessLevelStatus", label: "accessLevelStatus", multi: false, validator: None},
            &SimpleField {data_field: "country_code", label: "countryCode", data_type: SimpleType::String, validator: None},
            &SimpleField {data_field: "phone_number", label: "phoneNumber", data_type: SimpleType::String, validator: Some(FormValidator { method: "exactLength", param: Some("10") })},
        ]
    } 