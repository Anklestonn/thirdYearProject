


cd "$( dirname "$( readlink -f "$0" )" )" || exit


case "$1" in
    "start")
        exec rootlesskit --net=slirp4netns --copy-up=/etc bash -c "./.start.sh" &
        ;;
    "create")
        exec rootlesskit --net=slirp4netns --copy-up=/etc bash -c "./.create.sh" &
        ;;
    *)
        echo "Usage: $(basename $0) [start | create]"
        ;;
esac





