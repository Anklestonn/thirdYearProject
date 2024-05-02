#!/bin/sh

cp download/openssl_conf www/
cp download/server server
cp download/error_file www/
cp download/exploit.sh www/
cp download/ip_addr conf/ip_serv

openssl genrsa -out conf/privkey.pem 4096
openssl req -key conf/privkey.pem -new -config www/openssl_conf -out conf/certs.pem
openssl x509 -signkey conf/privkey.pem -in conf/certs.pem -req -days 356 -out conf/certs.pem


# ./server & disown # TODO: Active that before putting in prod or the serv won't active.
