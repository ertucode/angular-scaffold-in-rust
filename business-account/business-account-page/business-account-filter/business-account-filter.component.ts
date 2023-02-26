import { ChangeDetectionStrategy, Component } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TranslatedFormControl } from '@app/core/forms/translated-form-control';
import { TableFilterService } from '@app/shared/services/table/table-filter.service';
import { TableStreamService } from '@app/shared/services/table/table-stream.service';
import { TranslatedValidators } from '@app/core/localization/forms/translated-validators';
import { BusinessAccountService } from '../../business-account.service';
import { BusinessAccountListModel } from '../../models/list.model';
import { TranslatedFormGroupFromObject } from '@app/core/forms/helper-types';
import { BusinessAccountListFilterModel } from '../../models/filter.model';

@Component({
  selector: 'business-account-filter',
  templateUrl: './business-account-filter.component.html',
  styleUrls: ['./business-account-filter.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class BusinessAccountFilterComponent {
  form = new FormGroup({
    start_date: new TranslatedFormControl<null | Date>(null),
		end_date: new FormControl(),
		name: new FormControl(),
		account_number: new FormControl(),
		kyc_level: new FormControl(),
		email: new TranslatedFormControl<string | null>(null, TranslatedValidators.email),
		access_status_id: new FormControl(),
		country_code: new FormControl(),
		phone_number: new TranslatedFormControl<string | null>(null, TranslatedValidators.exactLength(10))
  } satisfies TranslatedFormGroupFromObject<BusinessAccountListFilterModel>)

  showErrors = false

  constructor(
    private svc: BusinessAccountService,
    public ts: TableStreamService<BusinessAccountListModel>,
    public fs: TableFilterService<BusinessAccountListModel, BusinessAccountListFilterModel>,
  ) {

    TranslatedValidators.addDateOrderValidator(this.form.controls.start_date, this.form.controls.end_date)

    this.fs.initializeService(this.svc.getList)
  }

  onFormClear() {
    this.form.reset()
  }

  onFormSubmit() {
    if (this.form.valid) {
      this.fs.onFilterFormSubmit(this.form.value)
      this.showErrors = false
    } else {
      this.showErrors = true
    }
  }
}