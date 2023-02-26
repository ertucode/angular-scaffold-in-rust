import { EnumType } from "src/app/shared/modules/enum/enums";

export interface BusinessAccountListModel {
    id: string;
	start_time: string | Date;
	end_time: string | Date;
	success: number;
	fail: number;
	created_date_utc: string | Date;
	request_owner_system_user_id: number;
	request_owner_account_id: number;
	transaction_type_code: EnumType["TransactionType"]
}