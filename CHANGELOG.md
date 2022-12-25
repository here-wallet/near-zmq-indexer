# Changelog

## 0.1.16

* Upgrade Indexer Framework to be based on [nearcore 1.30.0-patch](https://github.com/near/nearcore/commit/267e36e39fb5bb29c1df23c73afbcaa750ce96b1)

## 0.1.15

* Upgrade Indexer Framework to be based on [nearcore 1.30.0-rc.2 release](https://github.com/near/nearcore/releases/tag/1.30.0-rc.2)

## 0.1.14

* Upgrade Indexer Framework to be based on [nearcore 1.29.0 release](https://github.com/near/nearcore/releases/tag/1.29.0)

## 0.1.13

* Upgrade `nearcore` to 1.29.0-rc.3

## 0.1.12

* Upgrade `nearcore` to 1.29.0-rc.2

## 0.1.11

* Upgrade `nearcore` to 1.29.0-rc.1

## 0.1.10

* Upgrade `nearcore` to 1.28.0

## 0.1.9

* Upgrade `nearcore` to 1.28.0-rc.1

## 0.1.8

* Upgrade `nearcore` to 1.27.0

## 0.1.7

* Upgrade `nearcore` to 1.27.0-rc.5

## 0.1.6

* Upgrade `nearcore` to 1.27.0-rc.4

## 0.1.5

* Upgrade `nearcore` to 1.27.0-rc.2

## 0.1.4

* Upgrade `nearcore` to 1.27.0-rc.1

## 0.1.3

* Upgrade `nearcore` to 1.26.0

## 0.1.2

* Fix: Calculation time to catch up with the network
* Upgrade `nearcore` to 1.26.0-rc.1

## 0.1.1

* Minor fix: avoid division by zero in stats printer function

## 0.1.0

* Make info logs easy to reason about (ref https://github.com/near/near-lake/issues/11)
* Optional `--endpoint` parameter to store the data to custom S3 compatible storage

## 0.1.0-rc.0

A first try in releasing the alpha version of NEAR Lake

* Runs NEAR Indexer and stores data to AWS S3
* Depends on `nearcore` commit that is not included in release yet https://github.com/near/nearcore/pull/6255
