# Note: replace /dev/sdX with the device name of your SD card (e.g. /dev/mmcblk0, /dev/sdg1 ...)
wget http://people.canonical.com/~platform/snappy/raspberrypi2/ubuntu-15.04-snappy-armhf-rpi2.img.xz
unxz -c ubuntu-15.04-snappy-armhf-rpi2.img.xz | sudo dd of=$1 bs=32M
sync