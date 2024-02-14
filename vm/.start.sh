
option="-display gtk \
    -usbdevice tablet \
    -daemonize"

ip link add br0 type bridge
ip link set dev tap0 master br0
ip address delete 10.0.2.100/24 dev tap0
ip address add 10.0.2.100/24 dev br0
ip link set dev br0 up
ip route add default via 10.0.2.0 dev br0
if cat /etc/qemu/bridge.conf | grep -q "br0"
then
    echo "br0 already authorised by qemu"
else
    echo "allow br0" | tee -a /etc/qemu/bridge.conf
    echo "entry added to /etc/qemu/bridge.conf"
fi

#vm 1
qemu-system-x86_64 \
    -enable-kvm \
    -cpu host \
    -smp "4" \
    -netdev bridge,id=br0,br=br0 \
    -device virtio-net,netdev=br0,mac="AA:BB:CC:DD:EE:00" \
    -m "2G" \
    -drive file="server.cow",index=2,id=maindrive,media=disk \
    $option

#vm 2
qemu-system-x86_64 \
    -enable-kvm \
    -cpu host \
    -smp "4" \
    -netdev bridge,id=br0,br=br0 \
    -device virtio-net,netdev=br0,mac="AA:BB:CC:DD:EE:01" \
    -m "2G" \
    -drive file="client.cow",index=2,id=maindrive,media=disk \
    $option


