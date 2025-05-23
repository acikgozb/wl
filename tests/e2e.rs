#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{self},
        process::Command,
    };

    const BINARY: &str = "./target/debug/wl";

    #[test]
    fn wl_should_show_wifi_status() -> io::Result<()> {
        let program = fs::canonicalize(BINARY)?;

        for subcommand in ["s", "t"] {
            let cmd = Command::new(&program).arg(subcommand).output()?;

            assert!(cmd.status.success());
            assert!(!cmd.stdout.is_empty());
            assert!(cmd.stderr.is_empty());
        }

        Ok(())
    }

    #[test]
    fn wl_should_show_known_networks() -> io::Result<()> {
        let program = fs::canonicalize(BINARY)?;

        let ls_cmd = Command::new(&program).arg("ls").output()?;

        assert!(ls_cmd.status.success());
        assert!(!ls_cmd.stdout.is_empty());
        assert!(ls_cmd.stderr.is_empty());

        let ls_active_cmd = Command::new(&program).args(["ls", "--active"]).output()?;

        assert!(ls_active_cmd.status.success());
        assert!(!ls_active_cmd.stdout.is_empty());
        assert!(ls_active_cmd.stderr.is_empty());

        assert!(ls_cmd.stdout.len() > ls_active_cmd.stdout.len());

        Ok(())
    }

    #[test]
    fn disconnect_should_fail_with_diagnostics() -> io::Result<()> {
        let program = fs::canonicalize(BINARY)?;

        let disconnect_cmd = Command::new(&program)
            .args(["d", "-i", "invalid-ssid"])
            .output()?;

        assert!(!disconnect_cmd.status.success());
        assert!(disconnect_cmd.stdout.is_empty());
        assert!(!disconnect_cmd.stderr.is_empty());

        Ok(())
    }

    #[test]
    fn scan_should_fail_with_diagnostics() -> io::Result<()> {
        let program = fs::canonicalize(BINARY)?;

        let invalid_args = [
            ["-s", "101"],
            ["-g", "INVALID_COLUMN"],
            ["-c", "INVALID_COLUMN"],
        ];

        for invalid_args in invalid_args {
            let scan_cmd = Command::new(&program)
                .arg("sc")
                .args(invalid_args)
                .output()?;

            assert!(!scan_cmd.status.success());
            assert!(scan_cmd.stdout.is_empty());
            assert!(!scan_cmd.stderr.is_empty());
        }

        Ok(())
    }
}
