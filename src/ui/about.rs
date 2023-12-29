use relm4::prelude::*;
use gtk::prelude::*;

use anime_launcher_sdk::VERSION as SDK_VERSION;
use anime_launcher_sdk::anime_game_core::VERSION as CORE_VERSION;

use crate::*;

lazy_static::lazy_static! {
    pub static ref APP_VERSION: String = if crate::APP_DEBUG && !crate::APP_VERSION.contains('-') {
        format!("{}-dev", crate::APP_VERSION)
    } else {
        crate::APP_VERSION.to_string()
    };
}

#[derive(Debug)]
pub struct AboutDialog {
    visible: bool
}

#[derive(Debug)]
pub enum AboutDialogMsg {
    Show,
    Hide
}

#[relm4::component(pub)]
impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = AboutDialogMsg;
    type Output = ();

    view! {
        dialog = adw::AboutWindow {
            set_application_name: "Honkers Launcher",
            set_application_icon: APP_ID,

            set_website: "https://github.com/an-anime-team/honkers-launcher",
            set_issue_url: "https://github.com/an-anime-team/honkers-launcher/issues",

            set_license_type: gtk::License::Gpl30,
            set_version: &APP_VERSION,

            set_developers: &[
                "Nikita Podvirnyy https://github.com/krypt0nn"
            ],

            add_credit_section: (Some("Patch credits"), &[
                "@mkrsym1 https://codeberg.org/mkrsym1",
                "@EternalStudent https://github.com/EternalStudentDesuKa"
            ]),

            add_credit_section: (Some("An Anime Team"), &[
                "Nikita Podvirnyy https://github.com/krypt0nn",
                "Marie Piontek https://github.com/Mar0xy",
                "Luna Neff  https://github.com/lunaneff",
                "Renaud Lepage https://github.com/cybik",
                "Soham Nandy https://github.com/natimerry",
                "@mkrsym1 https://github.com/mkrsym1"
            ]),

            set_artists: &[
                "Lê Thanh Trực https://www.pinterest.com/pin/140806225786287"
            ],

            set_translator_credits: &[
                "Русский, English — Nikita Podvirnyy https://github.com/krypt0nn",
                "Deutsch — Marie Piontek https://github.com/Mar0xy",
                "Deutsch — @caem",
                "Français — @zeGolem https://github.com/zeGolem",
                "Español — Lautaro Garavano https://github.com/Rattlehead15",
                "Português — @kafushy",
                "Italiano — @QuazarOmega https://github.com/quazar-omega",
                "Türkçe — @Kaozix https://github.com/Kaozix1776",
                "Türkçe — Kayra Nachfolger https://github.com/kayranachfolger",
                "Polski — Dominik Opyd https://github.com/oritwoen",
                // Hungarian?
                "Svenska — Jakob Fridesjö https://github.com/jakobfridesjo",
                "Nederlands — @everlyy https://github.com/everlyy",
                "简体中文 — Caibin Chen https://github.com/tigersoldier",
                "日本語 — @zozonteq https://github.com/zozonteq",
                "한국어 — @project-dy https://github.com/project-dy",
                "Indonesia — @yumekarisu https://github.com/yumekarisu",
                "Tiếng Việt — Nguyễn Hữu Chánh https://github.com/Chanhnh"
            ].join("\n"),

            set_debug_info: &[
                format!("Anime Launcher SDK: {SDK_VERSION}"),
                format!("Anime Game Core: {CORE_VERSION}"),
                String::new(),
                format!("gtk: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
                format!("libadwaita: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
                format!("pango: {}", gtk::pango::version_string()),
                format!("cairo: {}", gtk::cairo::version_string())
            ].join("\n"),

            set_release_notes_version: &APP_VERSION,
            set_release_notes: &[
                "<p>Added</p>",

                "<ul>",
                    "<li>Added Vietnamese</li>",
                    "<li>Added Korean</li>",
                    "<li>Added Dutch</li>",
                    "<li>Added new `Concerning` patch status</li>",
                    "<li>Made free space checks resolve symlinks</li>",
                    "<li>Added `UpdatingPermissions` installation step</li>",
                    "<li>Downloaders now will skip finished files and truncate them if needed</li>",
                    "<li>Added new fix for the API responses</li>",
                    "<li>Added special tooltips for concerning patch status</li>",
                "</ul>",

                "<p>Fixed</p>",

                "<ul>",
                    "<li>Fixed \"Kill game process\" button</li>",
                "</ul>",

                "<p>Changed</p>",

                "<ul>",
                    "<li>Updated development libraries versions</li>",
                    "<li>Updated Turkish</li>",
                    "<li>Updated German</li>",
                    "<li>Updated Polish</li>",
                    "<li>Updated Chinese</li>",
                "</ul>",
            ].join("\n"),

            set_modal: true,
            set_hide_on_close: true,

            #[watch]
            set_visible: model.visible,

            connect_close_request[sender] => move |_| {
                sender.input(AboutDialogMsg::Hide);

                gtk::glib::Propagation::Proceed
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing about dialog");

        let model = Self {
            visible: false
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AboutDialogMsg::Show => {
                self.visible = true;
            }

            AboutDialogMsg::Hide => {
                self.visible = false;
            }
        }
    }
}
