# wifi-connection-controller
 
A tool that continuously performs Wi-Fi connections
 
# Features

This tool will ping the specified IP address at the specified interval. If it does not connect, it will make a network connection to the specified Wi-Fi access point.

# Requirement

* The language used is Rust, and the build was confirmed with the following version.
    * rustc 1.66.0
    * cargo 1.66.0
* Build and operating system confirmed
    * Linux (amd64, arm64)

# Installation
 
Execute the following command in the current directory.
 
```bash
$ cargo build --release
```

Binary file ``wifi-connection-controller`` will be generated in directory ``./target/release/``.

# Usage
 
```bash
$ ./wifi-connection-controller --help
Check if the network is connected, and if it is not, try to connect to Wi-Fi

Usage: wifi-connection-controller [OPTIONS] --iface <IFACE> --ssid <SSID> <--password <PASSWORD>|--secret <SECRET>>

Options:
  -c, --connection-ip <CONNECTION_IP>  IP address for checking network connetcion [default: 192.168.1.1]
  -i, --iface <IFACE>                  network interface for connecting Wi-Fi
  -s, --ssid <SSID>                    SSID of Wi-Fi connection
  -p, --password <PASSWORD>            password of Wi-Fi access point (Supports plain text, Base64)
      --secret <SECRET>                File name that stored the password (Supports plain text, Base64)
  -e, --exec-interval <EXEC_INTERVAL>  execution interval (sec) [default: 10]
  -o, --once                           set for execution only once
  -h, --help                           Print help information
  -V, --version                        Print version information
```

# Examples

## Simple example

```
$ sudo ./wifi-connection-controller -c 192.168.1.1 -i wlan0 -s "HG8045Q-xxxx" -p "wifi-password"
```

It is also possible to input the password in Base64.

```
$ sudo ./wifi-connection-controller -c 192.168.1.1 -i wlan0 -s "HG8045Q-xxxx" -p "d2lmaS1wYXNzd29yZA=="
```

## Example using Systemd

Please create the following service file at ``/etc/systemd/system/wifi-connection-controller.service``.

```service:/etc/systemd/system/wifi-connection-controller.service
[Unit]
Description = wifi-connection-controller

[Service]
EnvironmentFile=/etc/default/wifi-connection-controller
ExecStart=/usr/local/bin/wifi-connection-controller -c $PING_IP -i $IFACE -s $ACCSSS_POINT -p $PASSWORD -e $INTERVAL_SEC
ExecStop=/bin/kill -SIGTERM ${MAINPID}
KillSignal=SIGCONT
Restart=always
Type=simple

[Install]
WantedBy = multi-user.target
```

The environment variable file ``/etc/default/wifi-connection-controller`` is written as below.

```env:/etc/default/wifi-connection-controller
PING_IP="192.168.1.1"
IFACE="wlan0"
ACCSSS_POINT="HG8045Q-xxxx"
PASSWORD="d2lmaS1wYXNzd29yZA=="
INTERVAL_SEC=10
```

Please store the executable file ``wifi-connection-controller`` in ``/usr/local/bin``.

```
$ cp ./target/release/wifi-connection-controller /usr/local/bin/
$ sudo chown root:root /usr/local/bin/wifi-connection-controller
```

Activate the configuration and start the service.

```
$ sudo systemctl daemon-reload
$ sudo systemctl start check-wifi-connection.service
$ sudo systemctl enable check-wifi-connection.service
```
 
# Author
 
showchan33

# License
"wifi-connection-controller" is under [GPL license](https://www.gnu.org/licenses/licenses.en.html).
 
