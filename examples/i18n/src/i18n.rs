use fluent::{FluentArgs, FluentBundle, FluentResource};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};
use unic_langid::LanguageIdentifier;

// ------ I18n ------

pub struct I18n {
    lang: Lang,
    ftl_bundle: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        Self {
            lang,
            ftl_bundle: lang.create_ftl_bundle(),
        }
    }

    pub const fn lang(&self) -> &Lang {
        &self.lang
    }

    pub fn set_lang(&mut self, lang: Lang) -> &Self {
        self.lang = lang;
        self.ftl_bundle = lang.create_ftl_bundle();
        self
    }

    pub fn translate(&self, key: impl AsRef<str>, args: Option<&FluentArgs>) -> String {
        let mmessage = self
            .ftl_bundle
            .get_message(key.as_ref())
            .expect("get fluent message");

        let pattern = mmessage.value.expect("get value for fluent message");

        self.ftl_bundle
            .format_pattern(pattern, args, &mut vec![])
            .to_string()
    }
}

// ------ Lang ------

#[derive(Debug, Copy, Clone, Display, EnumIter, EnumString, AsRefStr, Eq, PartialEq)]
pub enum Lang {
    #[strum(serialize = "en-US")]
    EnUS,
    #[strum(serialize = "de-DE")]
    DeDE,
}

impl Lang {
    pub const fn label(self) -> &'static str {
        match self {
            Self::EnUS => "English (US)",
            Self::DeDE => "Deutsch (Deutschland)",
        }
    }

    pub const fn ftl_messages(self) -> &'static str {
        macro_rules! include_ftl_messages {
            ( $lang_id:literal ) => {
                include_str!(concat!("../ftl_messages/", $lang_id, ".ftl"))
            };
        }
        match self {
            Self::EnUS => include_ftl_messages!("en-US"),
            Self::DeDE => include_ftl_messages!("de-DE"),
        }
    }

    pub fn to_language_identifier(self) -> LanguageIdentifier {
        self.as_ref()
            .parse()
            .expect("parse Lang to LanguageIdentifier")
    }

    pub fn create_ftl_bundle(self) -> FluentBundle<FluentResource> {
        let ftl_resource =
            FluentResource::try_new(self.ftl_messages().to_owned()).expect("parse FTL messages");

        let mut bundle = FluentBundle::new(&[self.to_language_identifier()]);
        bundle.add_resource(ftl_resource).expect("add FTL resource");
        bundle
    }
}

// ------ create_t ------

/// Convenience macro to improve readability of `view`s with many translations.
///
/// # Example
///
///```rust,no_run
/// fn view(model: &Model) -> Node<Msg> {
///    let args_male_sg = fluent_args![
///      "userName" => "Stephan",
///      "userGender" => "male",
///    ];
///
///    create_t!(model.i18n);
///    div![
///        p![t!("hello-world")],
///        p![t!("hello-user", args_male_sg)],
///    ]
/// }
///```
#[macro_export]
macro_rules! create_t {
    ( $i18n:expr ) => {
        // This replaces $d with $ in the inner macro.
        seed::with_dollar_sign! {
            ($d:tt) => {
                macro_rules! t {
                    { $d key:expr } => {
                        {
                            $i18n.translate($d key, None)
                        }
                    };
                    { $d key:expr, $d args:expr } => {
                        {
                            $i18n.translate($d key, Some(&$d args))
                        }
                    };
                }
            }
        }
    };
}
