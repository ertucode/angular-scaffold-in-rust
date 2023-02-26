import { ChangeDetectionStrategy, Component } from '@angular/core';
import { TableStreamService } from '@app/shared/services/table/table-stream.service';
import { BusinessAccountListModel } from '../../models/list.model';

@Component({
  selector: 'business-account-list',
  templateUrl: './business-account-list.component.html',
  styleUrls: ['./business-account-list.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class BusinessAccountListComponent {
  constructor(
    public ts: TableStreamService<BusinessAccountListModel>,
  ) { }


}