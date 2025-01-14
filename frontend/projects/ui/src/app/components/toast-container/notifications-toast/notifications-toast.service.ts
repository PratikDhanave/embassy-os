import { Injectable } from '@angular/core'
import { endWith, Observable } from 'rxjs'
import { filter, map, pairwise } from 'rxjs/operators'
import { exists } from '@start9labs/shared'
import { PatchDbService } from '../../../services/patch-db/patch-db.service'

@Injectable({ providedIn: 'root' })
export class NotificationsToastService extends Observable<boolean> {
  private readonly stream$ = this.patch
    .watch$('server-info', 'unread-notification-count')
    .pipe(
      filter(exists),
      pairwise(),
      map(([prev, cur]) => cur > prev),
      endWith(false),
    )

  constructor(private readonly patch: PatchDbService) {
    super(subscriber => this.stream$.subscribe(subscriber))
  }
}
