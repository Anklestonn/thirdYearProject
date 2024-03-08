# Notice how to start vms.

Please, first, execute `./start_with_qemu.sh -h` to know what are the existing commands.

Second, `./start_with_qemu.sh install /ABSOLUTE/PATH/TO/ISO` to create two vms using the renseugned image disk.

Finally, use `./start_with_qemu.sh internet` or `./start_with_qemu.sh sandbox` to open the relevant network mode.

You should see on your prompt the user be changed to `root`

After that, you just have to do `./launch-vm` to launch all vms on the specific network mode.
