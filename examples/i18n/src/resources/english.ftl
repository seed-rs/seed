# Simple things are simple.
hello-world = Hello World.
hello-user = Hello, {$userName}!

# Complex things are possible.
shared-photos =
  {$userName} {$photoCount ->
    [one] added a new photo
    *[other] added {$photoCount} new photos
  } to {$userGender ->
    [male] his stream
    [female] her stream
    *[other] their stream
  }.

## Closing tabs
tabs-close-button = Close
tabs-close-tooltip = {$tabCount ->
  [one] Close {$tabCount} tab
  *[other] Close {$tabCount} tabs
}
tabs-close-warning =
  You are about to close {$tabCount} tabs.
  Are you sure you want to continue?

## Syncing
-sync-brand-name = Firefox Account

sync-dialog-title = {-sync-brand-name}
sync-headline-title =
  {-sync-brand-name}: The best way to bring
  your data always with you
sync-signedout-title = Connect with your {-sync-brand-name}
