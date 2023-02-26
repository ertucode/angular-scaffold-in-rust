import { EnumType } from "src/app/shared/modules/enum/enums";

export interface BusinessAccountListFilterModel {
    start_date: Date, 
	end_date: Date,
	name: string,
	account_number: string,
	kyc_level: EnumType["KycLevelStatus"],
	email: string,
	access_status_id: EnumType["AccessLevelStatus"],
	country_code: string,
	phone_number: string
}