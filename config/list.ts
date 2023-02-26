

export type BusinessAccountListModel = {
    id: string;
    tenant_id: string;
    parent_business_account_id: null;
    name: string;
    account_number: string;
    business_type: EnumType["BusinessType"];
    alias: string;
    kyc_level: EnumType["KycLevelStatusString"];
    owner_user_id: string;
    contact_address_contact_first_name: string;
    contact_address_contact_last_name: string;
    contact_address_contact_email: string;
    contact_address_contact_phone: string;
    contact_address_address_line1: string;
    contact_address_address_line2: string;
    contact_address_zip_postal_code: string;
    contact_address_state_province_code: string;
    contact_address_country_code: string;
    tax_office: string;
    tax_number: string;
    access_level_status_id: EnumType["AccessLevelStatus"];
    updated_date_utc: null | string;
    created_date_utc: string;
}