#!/bin/sh

# start two virtual machines, the objectifs is three modes:
#
# Start in instalation mode (Use another image to boot on)
# Start in maintenance mode (internet)
# Start in sandbox mode. (no internet)


usage="Usage: $(basename $0) [ install absolute-path-iso | internet | sandbox ]

Lanch two vms on the same network.

OPTIONS
    install absolute-path-iso   launch the two vm in install mode. take as parametter a boot iso file.
    internet                    Launch the vm connected to the external network via nat.
    sandbox                     Launch the vm in sandbox mode, the vms won't be connected to the external network.

Depends on qemu, rootlesskit,  and have to be run on linux."

cd "$( dirname "$( readlink -f "$0" )" )" || exit


case "$1" in
    "sandbox")
        unshare --user --map-root-user --net --mount
        ;;
    "internet")
        rootlesskit --net=slirp4netns --copy-up=/etc bash
        ;;
    "install")
	    if [ $# -eq 2 ]
	    then
            echo "$1"
            exec rootlesskit --net=slirp4netns --copy-up=/etc bash -c "./.create.sh $2" &
        else
            echo "$usage"
        fi
        ;;
    *)
        echo "$usage"
        ;;
esac





