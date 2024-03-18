# This is our Group Project

This is a model of a botnet. It is spread through a known CVE which tarets outdated systems to gain root access. The CVE is rewritten in Rust.

Use the script located at https://gitea.ahur.ac/statzitz/remote_connection to initiate ssh connections.

# Apache setup dependencies
1.  Apache2
2.  openSSL


# VM setup dependcies
1.  qemu
2.  slirp4netns
3.  rootlesskit

# TODO:
Develop the malware at its most simple -> software that runs, establishes a connection with the server, is remotely controlled and can download+configure files from server
1.  Run CVE which gets root access
2.  Establish connection with server, test connection periodically
3.  Download necessarily files
4.  Run necessary files at in a specified configuration



# Optimisations
1.  Anti sandboxing techniques
2.  p2p mining
3.  remote node for wallet/mining 
4.  running in the background
5.  minimize the resource usage/footprint of running program(post exploitation)
