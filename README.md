# `wl`

![version](https://img.shields.io/badge/version-0.1.0-red) ![release](https://img.shields.io/badge/release-stable-89e051)

A simple `nmcli` wrapper that is designed for host WiFi management.

## Table of Contents

<!--toc:start-->
  - [Installation](#installation)
  - [Usage](#usage)
    - [`wl status`](#wl-status)
    - [`wl toggle`](#wl-toggle)
    - [`wl list-networks`](#wl-list-networks)
    - [`wl scan`](#wl-scan)
    - [`wl connect`](#wl-connect)
      - [SSID](#connect-ssid)
      - [Force password](#force-password)
    - [`wl disconnect`](#wl-disconnect)
      - [SSID](#disconnect-ssid)
      - [Forget a network](#forget-a-network)
  - [<a id='license'></a> LICENSE](#license)
<!--toc:end-->

## <a id='installation'></a> Installation

`wl` can be installed via source.
This requires having `cargo` installed on the host.

```bash
# Clone the repository.
git clone git@github.com:acikgozb/wl.git ./wl

# Install via `cargo`
cd ./wl
cargo build --release --locked 

# Put the binary under $PATH.
# In here, it is assumed that ~/.local/bin is on $PATH.
cp ./target/release/wl ~/.local/bin/wl

# Validate the $PATH lookup before using wl.
which wl

# Now you can start using wl.
wl --version
```

## <a id='usage'></a> Usage

`wl` provides the subcommands below:

- `status`
- `toggle`
- `list-networks`
- `scan`
- `connect`
- `disconnect`

To understand more about the interface, please refer to `help`:

```bash
wl -h
```

### <a id='wl-status'></a> `wl status`

Use `status` to get information about the current status of WiFi.

```bash
$ wl status
# wifi: enabled
# connected networks: SSID1/Dev1, SSID2/Dev2, ... SSIDN/DevN, lo/lo
```

### <a id='wl-toggle'></a> `wl toggle`

Use `toggle` to toggle WiFi.

```bash
# Prints the latest WiFi status.
$ wl toggle
# wifi: disabled
```

### <a id='wl-list-networks'></a> `wl list-networks`

Use `list-networks` to see the known network list on the host.

By default, `list-networks` prints the entire list.

```bash
$ wl list-networks
# NAME               UUID                                  TYPE      DEVICE
# SSID1              xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  wifi      wlan0
# lo                 xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  loopback  lo
# SSID2              xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  wifi      --
# SSID3              xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  wifi      --
# SSID4              xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  wifi      --
```

Use `--show-active` to see the active networks:

```bash
$ wl list-networks -a
# NAME               UUID                                  TYPE      DEVICE
# SSID1              xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  wifi      wlan0
# lo                 xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  loopback  lo
```

Use `--show-ssid` to only see the SSIDs:

```bash
$ wl list-networks -i
# NAME
# SSID1
# lo
```

### <a id='wl-scan'></a> `wl scan`

Use `scan` to see the available SSIDs to connect.

The output of `scan` can be changed just like `nmcli`:

- By default, the list of SSIDs is printed as a table.
- This table can be filtered by its columns by using `-c|--columns COL1,COL2`.
- A terse output can be printed by using `-g|--get-values COL1,COL2`.
- The values of `COL1,COL2,...,COLN` are the same as the ones that are used for `nmcli`.
- If both `--get-values` and `--columns` is provided, `--columns` takes precedence.

```bash
# The default output, pretty format (table).
# Only the headers are shown here for simplicity.
$ wl scan
# IN-USE  BSSID  SSID  MODE  CHAN  RATE  SIGNAL  BARS  SECURITY

# The default output can be filtered by its columns.
$ wl scan --columns SSID,SIGNAL
# SSID    SIGNAL
# SSID1   55

# Terse output for scripting purposes.
$ wl scan --get-values SSID,SIGNAL
# SSID1:55

# If both options are provided, pretty format (table) is printed.
$ wl scan --columns SSID,SIGNAL --get-values SSID,SIGNAL
# SSID    SIGNAL
# SSID1   55
```

In addition to the output format, `scan` can filter the available SSIDs by their signal strength.

```bash
# An example list of available SSIDs.
$ wl scan --columns SSID,SIGNAL
# SSID    SIGNAL
# SSID1   55
# SSID2   65

# Use --min-strength to specify the limit.
$ wl scan --min-strength 60
# SSID    SIGNAL
# SSID2   65

# SIGNAL column does not have to exist to be able to filter
# by signal strength.
$ wl scan --get-values SSID --min-strength 60
# SSID2
```

### <a id='wl-connect'></a> `wl connect`

Use `connect` to connect to an SSID. The flow changes based on the arguments.

#### <a id='connect-ssid'></a> SSID

`connect` can become interactive if an SSID is not provided:

```bash
# Without an SSID, `connect` goes into the interactive mode
# to obtain one.
# It shows the available networks (essentially a `scan`).
$ wl connect
# (0) SSID1 (sig: 55)
# (1) SSID2 (sig: 65)
# Select the SSID to connect: 0|1

# Assume that SSID1 is selected.
# The flow above is equivalent to calling `connect` like below.
$ wl connect -i SSID1
```

#### <a id='force-password'></a> Force password 

`connect` can also become interactive if password is explicitly requested by the user.

If the password is not explicitly requested, then `connect` goes into the interactive mode based on whether the given SSID is known or not.

```bash
# Assume that SSID1 is a known network, and SSID2 is not.

# This connect attempt is interactive (new network).
$ wl connect -i SSID2
# Enter password for SSID2:

# This connect attempt is non-interactive (known network).
$ wl connect -i SSID1

# This connect attempt is interactive (forced).
$ wl connect -i SSID1 --force-passwd
# Enter password for SSID1:
```

### <a id='wl-disconnect'></a> `wl disconnect`

Use `disconnect` to disconnect from an SSID. The flow changes based on the arguments.

#### <a id='disconnect-ssid'></a> SSID

`disconnect` can become interactive if an SSID is not provided:

```bash
# Without an SSID, `disconnect` goes into the interactive mode
# to obtain one.
# It shows the active networks (essentially a `list-networks --active`).
$ wl disconnect
# (0) SSID1
# (1) SSID2
# Select the SSID to disconnect: 0|1

# Assume that SSID1 is selected.
# The flow above is equivalent to calling `disconnect` like below.
$ wl disconnect -i SSID1
```

#### <a id='forget-a-network'></a> Forget a network

`disconnect` can also be used to delete a network from the known network list.

```bash
# Assume that SSID1 and SSID2 are both on the known network list.

# Disconnect from SSID1.
$ wl disconnect -i SSID1

# SSID1 is still on the list.
$ wl ls -i | grep -q SSID1 
$ echo $? # 0

# Disconnect from SSID2 and forget it.
$ wl disconnect -i SSID2 -f

# SSID2 is not on the list anymore.
$ wl ls -i | grep -q SSID2
$ echo $? # 1
```

## <a id='license'></a> LICENSE

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose between one of them if you use this work.

`SPDX-License-Identifier: Apache-2.0 OR GPL-2.0-or-later`
