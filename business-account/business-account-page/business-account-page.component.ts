import { ChangeDetectionStrategy, Component } from '@angular/core';
import { CLAIM } from '@app/features/claims';
import { TableFilterService } from '@app/shared/services/table/table-filter.service';
import { TableStreamService } from '@app/shared/services/table/table-stream.service';

@Component({
  selector: 'business-account-page',
  templateUrl: './business-account-page.component.html',
  styleUrls: ['./business-account-page.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [TableStreamService, TableFilterService]
})
export class BulkTransferPageComponent {
  loadClaim = CLAIM.undefined
}