# RH Matching Backend

[![Travis Status](https://travis-ci.org/liberodark/RH-Matching-Backend.svg)](https://travis-ci.org/liberodark/RH-Matching-Backend)
[![Code Status](https://www.codefactor.io/repository/github/liberodark/RH-Matching-Backend/badge)](https://www.codefactor.io/repository/github/liberodark/RH-Matching-Backend)

# About

Later 

## Install MongoDB (CentOS 7.x)


```
nano /etc/yum.repos.d/mongodb-org-4.2.repo

[mongodb-org-4.2]
name=MongoDB Repository
baseurl=https://repo.mongodb.org/yum/redhat/$releasever/mongodb-org/4.2/x86_64/
gpgcheck=1
enabled=1
gpgkey=https://www.mongodb.org/static/pgp/server-4.2.asc

yum install -y mongodb-org
mkdir -p /var/lib/mongo
mkdir -p /var/log/mongodb
chown -R mongod:mongod /var/lib/mongo
chown -R mongod:mongod /var/log/mongodb
systemctl start mongod
systemctl enable mongod
systemctl status mongod
```

## How to install

```
git clone https://github.com/liberodark/RH-Matching-Backend/
cd RH-Matching-Backend/*
cargo build
mv rh-matching-backend.service /etc/systemd/system/
systemctl enable rh-matching-backend.service
systemctl start rh-matching-backend.service
```
