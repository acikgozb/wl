use std::{
    collections::HashMap,
    ffi::OsString,
    io::{self, BufRead},
    os::unix::ffi::OsStringExt,
    process::Command,
};

use crate::{
    adapter::{CARRIAGE_RETURN, Decimal, Error, LINE_FEED, Wl},
    api,
};

/// The adapter struct that implements [`Wl`] by using `nmcli`.
///
/// Since the struct can be changed in future versions, always
/// prefer to initialize it by using [`Nmcli::new`] instead.
///
/// [`Wl`]: crate::Wl
/// [`Nmcli::new`]: crate::Nmcli::new
#[derive(Clone, Default)]
pub struct Nmcli;

impl Nmcli {
    /// Creates a new `Nmcli` instance.
    ///
    /// The instance created by `new` can be reused multiple times
    /// in a given context. It can also be cloned freely.
    pub fn new() -> Self {
        Self
    }

    fn exec(&self, args: &[&[u8]]) -> Result<Vec<u8>, (io::Error, i32)> {
        let default_ecode = 1i32;
        let mut nmcli = Command::new("nmcli");
        let args = args.iter().map(|s| OsString::from_vec(s.to_vec()));
        let cmd = nmcli
            .args(args)
            .output()
            .map_err(|err| (err, default_ecode))?;

        if !cmd.status.success() {
            let nmcli_err = cmd
                .stderr
                .lines()
                .collect::<Result<String, io::Error>>()
                .map_err(|err| (err, default_ecode))?;
            let ecode = cmd.status.code().unwrap_or(default_ecode);
            return Err((io::Error::other(nmcli_err), ecode));
        }

        Ok(cmd.stdout)
    }
}

impl Wl for Nmcli {
    /// Provides the WiFi status.
    ///
    /// It returns the WiFi status that is in a **human-readable format** and is a single line.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns [`NetworkAdapterError::CannotGetWiFiStatus`] when it fails
    /// to obtain the WiFi status.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let nmcli = Nmcli::new();
    /// let status = nmcli.get_wifi_status().unwrap();
    /// io::stdout().write_all(&status).unwrap();
    /// ```
    ///
    /// [`NetworkAdapterError::CannotGetWiFiStatus`]: crate::NetworkAdapterError::CannotGetWiFiStatus
    fn get_wifi_status(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "WIFI", "g"].map(|a| a.as_bytes());
        let result = self.exec(&args).map_err(Error::CannotGetWiFiStatus)?;

        Ok(result
            .split(|a| a == &LINE_FEED)
            .flat_map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .copied()
            .collect())
    }

    /// Toggles the WiFi status.
    ///
    /// It returns the updated WiFi status in a **human-readable format** within a single line.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method can return either a [`NetworkAdapterError::CannotGetWiFiStatus`] or a [`NetworkAdapterError::CannotToggleWiFi`] when it fails to toggle WiFi.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let nmcli = Nmcli::new();
    /// let updated_status = nmcli.toggle_wifi().unwrap();
    /// io::stdout().write_all(&updated_status).unwrap();
    /// ```
    ///
    /// [`NetworkAdapterError::CannotGetWiFiStatus`]: crate::NetworkAdapterError::CannotGetWiFiStatus
    /// [`NetworkAdapterError::CannotToggleWiFi`]: crate::NetworkAdapterError::CannotToggleWiFi
    fn toggle_wifi(&self) -> Result<Vec<u8>, Error> {
        let cloned_process = self.clone();
        let prev_status = cloned_process.get_wifi_status()?;

        let mut args = ["radio", "wifi", ""];

        let new_status = if &prev_status[..] == b"enabled" {
            args[2] = "off";
            b"disabled".to_vec()
        } else {
            args[2] = "on";
            b"enabled".to_vec()
        };

        let _ = self
            .exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotToggleWiFi)?;

        Ok(new_status)
    }

    /// Provides a list of SSID-Device pairs.
    ///
    /// The output is in a **terse format**.
    /// To parse the output, [`get_field_separator`] can be used to split each element to get the SSID and the device.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns a [`NetworkAdapterError::CannotGetActiveConnections`] when it fails to retrieve the pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let nmcli = Nmcli::new();
    /// let pairs = nmcli.get_active_ssid_dev_pairs().unwrap();
    /// io::stdout().write_all(&pairs).unwrap();
    /// ```
    ///
    /// [`get_field_separator`]: crate::Wl::get_field_separator
    /// [`NetworkAdapterError::CannotGetActiveConnections`]: crate::NetworkAdapterError::CannotGetActiveConnections
    fn get_active_ssid_dev_pairs(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "NAME,DEVICE", "connection", "show", "--active"];

        self.exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotGetActiveConnections)
    }

    /// Provides a list of known networks on the host.
    ///
    /// The output is in a **human-readable format**.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns a [`NetworkAdapterError::CannotListNetworks`] when it fails to retrieve the pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let nmcli = Nmcli::new();
    ///
    /// // By default, it returns the whole list:
    /// let net_list = nmcli.list_networks(false, false).unwrap();
    ///
    /// // `show_active` can be used to show the active connections on the host.
    /// let net_list = nmcli.list_networks(true, false).unwrap();
    ///
    /// // `show_ssid` can be used to only show the SSIDs of the known networks.
    /// let net_list = nmcli.list_networks(false, true).unwrap();
    ///
    /// // Both args can be used to only show the SSIDs of active connections in the known networks.
    /// let net_list = nmcli.list_networks(true, true).unwrap();
    ///
    ///
    /// io::stdout().write_all(&net_list).unwrap();
    /// ```
    ///
    /// [`NetworkAdapterError::CannotListNetworks`]: crate::NetworkAdapterError::CannotListNetworks    
    fn list_networks(&self, show_active: bool, show_ssid: bool) -> Result<Vec<u8>, Error> {
        let mut args = ["", "", "connection", "show", ""];

        if show_ssid {
            args[0] = "--fields";
            args[1] = "NAME";
        }

        if show_active {
            args[4] = "--active";
        }

        let args: Vec<&[u8]> = args
            .into_iter()
            .filter(|a| !a.is_empty())
            .map(|a| a.as_bytes())
            .collect();

        self.exec(&args).map_err(Error::CannotListNetworks)
    }

    /// Provides a list of active SSIDs on the host.
    ///
    /// The output is in a **terse format** and may contain multiple lines.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns a [`NetworkAdapterError::CannotGetSSIDStatus`] when it fails to retrieve the SSIDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let nmcli = Nmcli::new();
    /// let active_ssids = nmcli.get_active_ssids().unwrap();
    /// io::stdout().write_all(&active_ssids).unwrap();
    /// ```
    /// [`NetworkAdapterError::CannotGetSSIDStatus`]: crate::NetworkAdapterError::CannotGetSSIDStatus
    fn get_active_ssids(&self) -> Result<Vec<u8>, Error> {
        let args = ["-g", "NAME", "connection", "show", "--active"];

        self.exec(&args.map(|a| a.as_bytes()))
            .map_err(Error::CannotGetSSIDStatus)
    }

    /// Disconnects from the given SSID.
    ///
    /// The output is in a **human-readable format**.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns a [`NetworkAdapterError::CannotDisconnect`] when it fails to disconnect.
    ///
    /// # Examples
    ///
    /// Without `forget`, disconnect just disconnects from the SSID.
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "SSID";
    /// let forget = false;
    ///
    /// let nmcli = Nmcli::new();
    /// let result = nmcli.disconnect(ssid.as_bytes(), forget);
    /// match result {
    ///     Ok(result) => io::stdout().write_all(&result).unwrap(),
    ///     Err(err) => eprintln!("err during disconnect: {}", err),
    /// }
    /// ```
    ///
    /// Set `forget` to delete the SSID after disconnecting.
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "SSID";
    /// let forget = true;
    ///
    /// let nmcli = Nmcli::new();
    /// let result = nmcli.disconnect(ssid.as_bytes(), forget);
    ///
    /// match result {
    ///     Ok(result) => io::stdout().write_all(&result).unwrap(),
    ///     Err(err) => eprintln!("err during disconnect: {}", err),
    /// }
    /// ```
    ///
    /// [`NetworkAdapterError::CannotDisconnect`]: crate::NetworkAdapterError::CannotDisconnect
    fn disconnect(&self, ssid: &[u8], forget: bool) -> Result<Vec<u8>, Error> {
        let mut args = [
            "connection",
            if forget { "delete" } else { "down" },
            "id",
            "",
        ]
        .map(|a| a.as_bytes());
        args[3] = ssid;

        self.exec(&args).map_err(Error::CannotDisconnect)
    }

    /// Scan the available SSIDs to connect.
    ///
    /// The output depends on the [`ScanArgs`]:
    ///
    /// - If `columns` is used, then the output is in a **human-readable format** and may contain multiple lines.
    /// - If `get_values` is used, then the output is in a **terse format** and may contain multiple lines. In this case, each line element contains FIELDS that are separated by [`get_field_separator`].
    /// - Using `columns` overrides `get_values`.
    ///
    /// `columns` and `get_values` both use the same values as `nmcli -f FIELDS` and `nmcli -g FIELDS` respectively, which are comma separated values of column names.
    ///
    /// If `re-scan` is set, then `scan` refreshes the underlying cache of available networks.
    ///
    /// If `min_strength` is provided, the `scan` filters the list by the given signal strength.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns [`NetworkAdapterError::CannotScanWiFi`] if it fails to scan the available networks.
    ///
    /// # Examples
    ///
    /// Use `columns` to get the scan result in a human-readable format.
    /// ```
    /// use wl::{Nmcli,Wl, api::ScanArgs};
    /// use std::io::{self, Write};
    ///
    /// let args = ScanArgs {
    ///     min_strength: 0,
    ///     re_scan: false,
    ///     get_values: None,
    ///     columns: Some(String::from("SSID,SIGNAL")),
    /// };
    ///
    /// let nmcli = Nmcli::new();
    /// let scan_result = nmcli.scan(&args).unwrap();
    /// io::stdout().write_all(&scan_result).unwrap();
    /// ```
    ///
    /// Use `get_values` to get the scan result in a terse format.
    /// ```
    /// use wl::{Nmcli,Wl, api::ScanArgs};
    /// use std::io::{self, Write};
    ///
    /// let args = ScanArgs {
    ///     min_strength: 0,
    ///     re_scan: false,
    ///     columns: None,
    ///     get_values: Some(String::from("SSID,SIGNAL")),
    /// };
    ///
    /// let nmcli = Nmcli::new();
    /// let scan_result = nmcli.scan(&args).unwrap();
    /// io::stdout().write_all(&scan_result).unwrap();
    /// ```
    ///
    /// Use `min_strength` to filter the scan list by SIGNAL.
    /// ```
    /// use wl::{Nmcli,Wl, api::ScanArgs};
    /// use std::io::{self, Write};
    ///
    /// let args = ScanArgs {
    ///     min_strength: 60,
    ///     re_scan: false,
    ///     columns: None,
    ///     get_values: Some(String::from("SSID,SIGNAL")),
    /// };
    ///
    /// let nmcli = Nmcli::new();
    /// let scan_result = nmcli.scan(&args).unwrap();
    /// io::stdout().write_all(&scan_result).unwrap();
    /// ```
    ///
    /// Use `re_scan` to refresh the scan cache.
    /// ```
    /// use wl::{Nmcli,Wl, api::ScanArgs};
    /// use std::io::{self, Write};
    ///
    /// let args = ScanArgs {
    ///     min_strength: 0,
    ///     re_scan: true,
    ///     columns: None,
    ///     get_values: Some(String::from("SSID,SIGNAL")),
    /// };
    ///
    /// let nmcli = Nmcli::new();
    /// let scan_result = nmcli.scan(&args).unwrap();
    /// io::stdout().write_all(&scan_result).unwrap();
    /// ```
    ///
    /// [`NetworkAdapterError::CannotScanWiFi`]: crate::NetworkAdapterError::CannotScanWiFi
    /// [`ScanArgs`]: crate::api::ScanArgs
    /// [`get_field_separator`]: crate::Nmcli::get_field_separator
    fn scan(&self, args: &api::ScanArgs) -> Result<Vec<u8>, Error> {
        let mut nmcli_args = ["", "", "d", "wifi", "list", "", ""];

        let nmcli_global_args = match (&args.columns, &args.get_values) {
            (None, None) => ["", ""],
            (None, Some(values)) => ["-g", values],
            (Some(columns), None) => ["-f", columns],
            (Some(columns), Some(_)) => ["-f", columns],
        };
        nmcli_args[0..2].copy_from_slice(&nmcli_global_args);

        if args.re_scan {
            nmcli_args[5..].copy_from_slice(&["--rescan", "yes"]);
        }

        let nmcli_args: Vec<&[u8]> = nmcli_args
            .into_iter()
            .filter(|a| !a.is_empty())
            .map(|a| a.as_bytes())
            .collect();

        let scan_result = self.exec(&nmcli_args).map_err(Error::CannotScanWiFi)?;

        let cloned_process = self.clone();
        let nmcli_args = ["-g", "SIGNAL", "d", "wifi", "list"];

        let signal_result = cloned_process
            .exec(&nmcli_args.map(|a| a.as_bytes()))
            .map_err(Error::CannotScanWiFi)?;
        let signal_lines = signal_result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .enumerate();

        let mut valid_signals = HashMap::new();

        for (idx, signal) in signal_lines {
            let signal = Decimal::from(signal).inner();

            if signal >= args.min_strength {
                valid_signals.insert(idx + 1, signal);
            }
        }

        let filtered_scan = scan_result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .enumerate()
            .filter_map(|(idx, l)| {
                if idx == 0 || valid_signals.contains_key(&idx) {
                    Some([l, &[LINE_FEED]].concat())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<u8>>();

        Ok(filtered_scan)
    }

    /// Checks whether the given SSID is a known one or not.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns [`NetworkAdapterError::CannotGetSSIDStatus`] if it fails to check the SSID.
    ///
    /// # Examples
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "SSID";
    /// let nmcli = Nmcli::new();
    ///
    /// let is_known_ssid = nmcli.is_known_ssid(ssid.as_bytes()).unwrap();
    /// match is_known_ssid {
    ///     true => println!("{} is a known SSID!", ssid),
    ///     false => println!("{} is not a known SSID!", ssid),
    /// };
    /// ```
    ///
    /// [`NetworkAdapterError::CannotGetSSIDStatus`]: crate::NetworkAdapterError::CannotGetSSIDStatus
    fn is_known_ssid(&self, ssid: &[u8]) -> Result<bool, Error> {
        let args = ["-g", "NAME", "connection", "show"].map(|a| a.as_bytes());

        let result = self.exec(&args).map_err(Error::CannotGetSSIDStatus)?;
        let exists = result
            .split(|b| b == &LINE_FEED)
            .map(|l| l.strip_suffix(&[CARRIAGE_RETURN]).unwrap_or(l))
            .any(|l| ssid == l);

        Ok(exists)
    }

    /// Connects to the given SSID.
    ///
    /// The output is in a **human-readable format** and may contain multiple lines.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Errors
    ///
    /// This method returns [`NetworkAdapterError::CannotConnect`] if it fails to connect to the the SSID.
    ///
    /// # Examples
    ///
    /// To establish a new connection, provide both `ssid` and `passwd`.
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "SSID";
    /// let passwd = Some("PASS".as_bytes());
    /// let is_known_ssid = false;
    ///
    /// let nmcli = Nmcli::new();
    /// let connect_result = nmcli.connect(ssid.as_bytes(), passwd, is_known_ssid);
    ///
    /// match connect_result {
    ///     Ok(res) => io::stdout().write_all(&res).unwrap(),
    ///     Err(err) => eprintln!("{}", err),
    /// };
    /// ```
    ///
    /// To re-use a connection from the known network list, only provide `ssid`.
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "Known-SSID";
    /// let passwd = None;
    /// let is_known_ssid = true;
    ///
    /// let nmcli = Nmcli::new();
    /// let connect_result = nmcli.connect(ssid.as_bytes(), passwd, is_known_ssid);
    ///
    /// match connect_result {
    ///     Ok(res) => io::stdout().write_all(&res).unwrap(),
    ///     Err(err) => eprintln!("{}", err),
    /// };
    /// ```
    ///
    /// To "update" the connection from the known network list, provide all the arguments.
    ///
    /// ```
    /// use wl::{Nmcli,Wl};
    /// use std::io::{self, Write};
    ///
    /// let ssid = "Known-SSID";
    /// let passwd = Some("NEW_PASS".as_bytes());
    /// let is_known_ssid = true;
    ///
    /// let nmcli = Nmcli::new();
    /// let connect_result = nmcli.connect(ssid.as_bytes(), passwd, is_known_ssid);
    ///
    /// match connect_result {
    ///     Ok(res) => io::stdout().write_all(&res).unwrap(),
    ///     Err(err) => eprintln!("{}", err),
    /// };
    /// ```
    /// [`NetworkAdapterError::CannotConnect`]: crate::NetworkAdapterError::CannotConnect
    fn connect(
        &self,
        ssid: &[u8],
        passwd: Option<&[u8]>,
        is_known_ssid: bool,
    ) -> Result<Vec<u8>, Error> {
        if is_known_ssid && passwd.is_some() {
            self.disconnect(ssid, true)?;
        }

        let args = if let Some(passwd) = passwd {
            let mut args = ["d", "wifi", "connect", "", "password", ""].map(|a| a.as_bytes());
            args[3] = ssid;
            args[5] = passwd;

            args.to_vec()
        } else {
            let mut args = ["connection", "up", "id", ""].map(|a| a.as_bytes());
            args[3] = ssid;

            args.to_vec()
        };

        self.exec(&args).map_err(Error::CannotConnect)
    }

    fn get_field_separator(&self) -> u8 {
        b':'
    }
}
