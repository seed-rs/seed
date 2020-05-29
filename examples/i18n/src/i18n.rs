use fluent::{FluentArgs, FluentBundle, FluentResource};
use strum_macros::{AsRefStr, EnumIter, EnumString};
use unic_langid::LanguageIdentifier;

use super::resource::Resource;

#[macro_export]
macro_rules! t {
    { $key:expr } => {
        {
            translate($key, None)
        }
     };
     { $key:expr, $args:expr } => {
        {
            translate($key, Some($args))
        }
     };
}

// ------ I18n ------

pub struct I18n {
    lang: Lang,
    resource: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        let mut i18n = Self {
            lang,
            resource: FluentBundle::default(),
        };
        i18n.set_lang(lang);
        i18n
    }

    pub const fn lang(&self) -> &Lang {
        &self.lang
    }

    pub fn set_lang(&mut self, lang: Lang) -> &Self {
        self.lang = lang;
        let ftl_res = FluentResource::try_new(
            Resource::new()
                .get(lang.as_ref())
                .expect("get language identifier")
                .parse()
                .expect("parse language identifier"),
        )
        .expect("parse FTL string");

        let locale: LanguageIdentifier = lang.as_ref().parse().expect("parse language identifier");

        let mut bundle = FluentBundle::new(&[locale]);
        bundle.add_resource(ftl_res).expect("add FTL resource");

        self.resource = bundle;
        self
    }

    pub fn translate(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let fluent_msg = self.resource.get_message(key).expect("get fluent message");
        let pattern = fluent_msg.value.expect("get value for fluent message");

        self.resource
            .format_pattern(pattern, args, &mut vec![])
            .to_string()
    }
}

// ------ Lang ------

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, EnumIter, EnumString, AsRefStr, PartialEq)]
pub enum Lang {
    #[strum(serialize = "en-US")]
    EnUS,
    #[strum(serialize = "de-DE")]
    DeDE,
}

impl Lang {
    pub fn label(&self) -> &str {
        match self {
            Self::EnUS => "English (US)",
            Self::DeDE => "Deutsch (Deutschland)",
        }
    }
}
