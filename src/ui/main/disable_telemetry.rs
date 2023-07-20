use std::process::Command;

use relm4::prelude::*;

use crate::*;
use crate::i18n::*;

use super::{App, AppMsg};

pub fn disable_telemetry(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    std::thread::spawn(move || {
        let telemetry = TELEMETRY_SERVERS
            .iter()
            .map(|server| format!("echo '0.0.0.0 {server}' >> /etc/hosts"))
            .collect::<Vec<String>>()
            .join(" ; ");

        let output = Command::new("pkexec")
            .arg("bash")
            .arg("-c")
            .arg(format!("echo '' >> /etc/hosts ; {telemetry} ; echo '' >> /etc/hosts"))
            .spawn();

        match output.and_then(|child| child.wait_with_output()) {
            Ok(output) => if !output.status.success() {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr("telemetry-servers-disabling-error"),
                    description: None // stdout/err is empty
                });
            }

            Err(err) => {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr("telemetry-servers-disabling-error"),
                    description: Some(err.to_string())
                });
            }
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true
        });
    });
}
