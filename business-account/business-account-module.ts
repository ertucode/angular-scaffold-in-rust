import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { BusinessAccountRoutingModule } from './business-account-routing.module';
import { BusinessAccountPageComponent } from './business-account-page/business-account-page.component';
import { BusinessAccountFilterComponent } from './business-account-page/business-account-filter/business-account-filter.component';
import { BusinessAccountListComponent } from './business-account-page/business-account-list/business-account-list.component';
import { SharedModule } from '@app/shared/shared.module';


@NgModule({
  declarations: [
    BusinessAccountPageComponent,
    BusinessAccountFilterComponent,
    BusinessAccountListComponent
  ],
  imports: [
    CommonModule,
    BusinessAccountRoutingModule,
    SharedModule
  ],
})
export class BusinessAccountModule { }