# Geodate HT16K33

Print the time in centidays and dimidays on a seven-segment display driven by
a HT16K33 controller on the i2c bus of a computer.

This project currently assumes that you are using Raspberry Pi with the display
accessible at `/dev/i2c-1`.

The following command should show you the display at the address 70:

    $ i2cdetect -y -a 1
         0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    00: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    70: 70 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --

If that's not the case you might need to enable i2c with `raspi-config`.


## Installation

    $ curl -sSL https://sh.rustup.rs | sh
    $ git clone https://github.com/vinc/geodate-ht16k33
    $ cd geodate-ht16k33
    $ cargo install --path .


## Usage

The program requires your longitude to compute the local time and accepts an
additional argument to control the brightness of the display between 0 and 15:

    $ geodate-ht16k33 -1.8262 3


## Service

Add the following content to `/etc/systemd/system/geodate-ht16k33.service`

    [Unit]
    Description=Geodate HT16K33

    [Service]
    ExecStart=/home/pi/.cargo/bin/geodate-ht16k33 -1.8262 3

    [Install]
    WantedBy=multi-user.target

Then start and enable the service with:

    $ sudo systemctl start geodate-ht16k33.service
    $ sudo systemctl enable geodate-ht16k33.service


## License

This program is released under the MIT license.
