pub static S: &'static str = r#"
# Einfache Dinge sind einfach.
hello-world = Hallo Welt.
hello-user = {$formal ->
  [true] Guten Tag {$userName}!
  *[other] Hallo {$userName}!
}

# Complex things are possible.
shared-photos =
  {$userName} {$photoCount ->
    [one] hat ein neues Foto zu {$userGender ->
      [male] seinem Stream hinzugefügt
      [female] ihrem Stream hinzugefügt
      *[other] seinem Stream hinzugefügt
    }
    *[other] fügte {$photoCount} neue Fotos zu {$userGender ->
      [male] seinem Stream hinzu
      [female] ihrem Stream hinzu
      *[other] seinem Stream hinzu
    }
  }.

## Closing tabs
tabs-close-button = Schließen
tabs-close-tooltip = {$tabCount ->
  [one] Schließe {$tabCount} Tab
  *[other] Schließe {$tabCount} Tabs
}
tabs-close-warning = {$formal ->
  [true] Sie sind dabei {$tabCount} Tabs zu schließen.
  Sind Sie sicher, dass Sie das wollen?
  *[other] Du bist dabei {$tabCount} Tabs zu schließen.
  Bist Du sicher, dass Du das willst?
}

## Syncing
-sync-brand-name = Firefox Konto

sync-dialog-title = {-sync-brand-name}
sync-headline-title =
  {-sync-brand-name}: Die beste Art Deine Daten immer
  bei Dir zu haben
sync-signedout-title = Verbinde Dich mit Deinem {-sync-brand-name}"#;
