use fluent::{FluentArgs, FluentBundle, FluentResource};
use strum_macros::EnumIter;
use unic_langid::LanguageIdentifier;

use std::borrow::Borrow;

use crate::resource::Resource;

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
        let res = FluentResource::try_new(
            Resource::new()
                .get(lang.id().to_string().borrow())
                .unwrap()
                .parse()
                .unwrap(),
        )
            .expect("Failed to parse an FTL string.");

        let locale: LanguageIdentifier = lang.id().parse().expect("Parsing failed");

        let mut bundle = FluentBundle::new(&[locale]);
        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        self.resource = bundle;
        self
    }
}

// ------ Lang ------

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, EnumIter, PartialEq)]
pub enum Lang {
    en_US,
    de_DE,
}

impl Lang {
    pub fn id(&self) -> &str {
        match self {
            Self::en_US => "en-US",
            Self::de_DE => "de-DE",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::en_US => "English (US)",
            Self::de_DE => "Deutsch (Deutschland)",
        }
    }
}

pub fn translate(i18n: &I18n, args: Option<&FluentArgs>, label: &str) -> String {
    let fluent_msg = i18n
        .resource
        .get_message(label)
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = fluent_msg.value.expect("Message has no value.");

    let value = i18n
        .resource
        .format_pattern(pattern, args, &mut errors);
    value.to_string()
}
