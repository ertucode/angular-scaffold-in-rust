import { Injectable } from '@angular/core';
import { DataService } from '@app/core/data/data.service';
import { emptyListResponsePayload, ServerListResponsePayload } from '@app/core/data/serverResponse';
import { MakeOptionalNested } from '@app/core/util-types/typescript-util';
import { CLAIM, ClaimsService } from '@app/features/claims';
import { BusinessAccountListFilterModel } from './models/filter.model';
import { BusinessAccountListModel } from './models/list.model';

@Injectable({
  providedIn: 'root'
})
export class BusinessAccountService {

  constructor(private ds: DataService, private cls: ClaimsService) { }

  getList = (model: MakeOptionalNested<BusinessAccountListFilterModel>) => {
    return this.cls.performClaimedAction(CLAIM.BULK.LOAD, this.ds.searchList<BusinessAccountListModel, BusinessAccountListFilterModel>(undefined, model), emptyListResponsePayload)
  }

}