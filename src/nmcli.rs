use std::{
    fmt,
    io::{BufRead, Error},
    process::Command,
};

pub enum WiFiStatus {
    Enabled,
    Disabled,
}

impl fmt::Display for WiFiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiFiStatus::Enabled => write!(f, "enabled"),
            WiFiStatus::Disabled => write!(f, "disabled"),
        }
    }
}

struct Nmcli<'a> {
    global_args: Vec<&'a str>,
    subcommands: Vec<&'a str>,
    subcommand_args: Vec<&'a str>,
}

impl<'a> Nmcli<'a> {
    fn new() -> Self {
        Self {
            global_args: vec![],
            subcommands: vec![],
            subcommand_args: vec![],
        }
    }

    fn with_global_args(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.global_args = args;
        self
    }

    fn with_subcommands(&mut self, cmds: Vec<&'a str>) -> &mut Self {
        self.subcommands = cmds;
        self
    }

    fn with_subcommand_args(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.subcommand_args = args;
        self
    }

    fn exec(self) -> Result<Vec<u8>, Error> {
        let mut nmcli = Command::new("nmcli");
        let cmd = nmcli
            .args(self.global_args)
            .args(self.subcommands)
            .args(self.subcommand_args)
            .output()?;

        if !cmd.status.success() {
            let nmcli_err = cmd.stderr.lines().collect::<Result<String, Error>>()?;
            return Err(Error::other(nmcli_err));
        }

        Ok(cmd.stdout)
    }
}

pub fn show_active_connections() -> Result<Vec<String>, Error> {
    let mut nmcli = Nmcli::new();
    nmcli
        .with_global_args(vec!["-g", "NAME,DEVICE"])
        .with_subcommands(vec!["connection", "show"])
        .with_subcommand_args(vec!["--active"]);

    let result = nmcli.exec()?;

    let active_conn_device_pairs = result.lines().collect::<Result<Vec<String>, Error>>()?;

    Ok(active_conn_device_pairs)
}


    }

        .lines()
pub fn get_wifi_status() -> Result<WiFiStatus, Error> {
    let mut nmcli = Nmcli::new();
    nmcli
        .with_global_args(vec!["-g", "WIFI"])
        .with_subcommands(vec!["g"]);

    let result = nmcli.exec()?;

    let status = result.lines().take(1).collect::<Result<String, Error>>()?;

    Ok(if status == "enabled" {
        WiFiStatus::Enabled
    } else {
        WiFiStatus::Disabled
    })
}

pub fn toggle_wifi(old_status: WiFiStatus) -> Result<WiFiStatus, Error> {
    let mut nmcli = Nmcli::new();
    nmcli.with_subcommands(vec!["radio", "wifi"]);

    let new_status = match old_status {
        WiFiStatus::Enabled => {
            nmcli.with_subcommand_args(vec!["off"]);
            WiFiStatus::Disabled
        }
        WiFiStatus::Disabled => {
            nmcli.with_subcommand_args(vec!["on"]);
            WiFiStatus::Enabled
        }
    };

    let _ = nmcli.exec()?;

    Ok(new_status)
}
