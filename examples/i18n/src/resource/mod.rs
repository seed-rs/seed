use std::collections::HashMap;

mod german;

#[derive(Debug)]
pub struct Resources(HashMap<String, String>);

impl Default for Resources {
    fn default() -> Self {
        let ftl = String::from(
            r#"
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
sync-signedout-title = Connect with your {-sync-brand-name}"#,
        );

        let mut m = HashMap::new();
        m.insert(String::from("en-US"), ftl);
        Resources(m)
    }
}

impl Resources {
    pub fn new() -> Self {
        let mut r = Resources::default();
        r.insert("de-DE".to_string(), (german::S).to_string());
        r
    }
    pub fn insert(&mut self, k: String, v: String) -> &Self {
        self.0.insert(k, v);
        self
    }
    pub fn get(&self, k: &String) -> Option<&String> {
        self.0.get(k)
    }
}
