# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.11.1](https://github.com/maidsafe/safe-nodejs/compare/v0.1.3...v0.11.1) (2020-03-30)


### Features

* **authd:** add bindings for authd install and uninstall APIs ([dd71ccb](https://github.com/maidsafe/safe-nodejs/commit/dd71ccba74960a540813b62535c63a75c7c7db3f))
* **authd:** add nodejs bindings for SafeAuthdClient start/stop/restart APIs ([5c0eb1a](https://github.com/maidsafe/safe-nodejs/commit/5c0eb1af94ea374bd42570b6e6df3499705c5e94))
* **authd:** implement binding for authd status API ([b57baa1](https://github.com/maidsafe/safe-nodejs/commit/b57baa19bb9558663259b3af176d274184576418))
* **authd:** implementation of nodejs bindings for SafeAuthdClient API ([870ab7a](https://github.com/maidsafe/safe-nodejs/commit/870ab7a5c37ab0868f25f7e079b5e93fe6f45bd7))
* **authd:** implementation of the subscribe JS binding to receive auth reqs notifications from authd ([dd70cb7](https://github.com/maidsafe/safe-nodejs/commit/dd70cb75c049c4598cd411a925a6ca6055aadb0b))
* **electron:** Build for electron 7 ([653f5ec](https://github.com/maidsafe/safe-nodejs/commit/653f5ec94e057bbaf2c4b6fa5dc624d25e3cb8f1))
* **Electron:** Update to 6.0.11 ([7581d8b](https://github.com/maidsafe/safe-nodejs/commit/7581d8b0ac341a2948ebf283cefcced38dae03ef))
* **Electron:** Update to building 6.0.12 ([2f26c7e](https://github.com/maidsafe/safe-nodejs/commit/2f26c7e9414207d53f020c0b9809c3ed113b755c))
* **files:** add binding for files add APIs, plus support Buffer and ArrayBuffer for args receiving file bytes ([d71c93a](https://github.com/maidsafe/safe-nodejs/commit/d71c93acabe5ba1bea109a4285be153daadfa19b))
* **files:** implementation of FilesContainer and PublishedImmutableData API bindings ([c7e1a71](https://github.com/maidsafe/safe-nodejs/commit/c7e1a71a0b33e6236b26430300f220f940e36279))
* **files:** minor changes to files api binding to upgrade safe-api ([d4c86de](https://github.com/maidsafe/safe-nodejs/commit/d4c86de985fff1008ea86ee5ae38f96614e2caf1))
* **inspect:** implementation of binding for inspect API ([edd4e64](https://github.com/maidsafe/safe-nodejs/commit/edd4e64a535d8f13055e12502934634b7474c77b))
* **lib:** adapt fetch binding to support optional range arg, and adapt files_container_create binding to support creation of empty containers ([e8c256a](https://github.com/maidsafe/safe-nodejs/commit/e8c256a4ab088491f389a75d40d1f390bade6ac3))
* **lib:** add files_container_remove_path binding ([7e77dde](https://github.com/maidsafe/safe-nodejs/commit/7e77ddee89c168194e210555c8a4f39f2b02fd5f))
* **nrs:** implementation of the NRS API bindings ([b77f0bc](https://github.com/maidsafe/safe-nodejs/commit/b77f0bc327e6ee39d2f36ff008a9ced34e1ce33e))
* **packaging:** Enable packaging and downloading of prepackaged versions ([f423f9e](https://github.com/maidsafe/safe-nodejs/commit/f423f9e0e04ea254ec99ee882378e82e38b03a9d))
* **packaging:** Setup basic packaging ([efb4e54](https://github.com/maidsafe/safe-nodejs/commit/efb4e54655cc88488b260a27f600dec68b1f1e3d))
* **wallet:** implementation of Keys and Wallet bindings ([0ae1a90](https://github.com/maidsafe/safe-nodejs/commit/0ae1a9039d296332a3900573294cae79906d268c))
* **win:** Windows build and packaging. ([0786629](https://github.com/maidsafe/safe-nodejs/commit/0786629333b676a5d2cf792a98711ca10c1eceb5))
* **Windows:** Use temp fork of neon to fix windows builds ([87dcf1f](https://github.com/maidsafe/safe-nodejs/commit/87dcf1ff4772dc132a1b110ab5ef1143df5c8ab4))
* **xorurl:** implement JS binding for XorUrlEncoder ([d6e9375](https://github.com/maidsafe/safe-nodejs/commit/d6e9375809f4678c634e883ba9a4fd49e9978b60))


### Bug Fixes

* **api:** allow optional arguments to be passed as null ([fef8f59](https://github.com/maidsafe/safe-nodejs/commit/fef8f59ecd7cc7ce499b8b0d97fe4e1932578874))
* **ci:** Cleanup between various electron releases. ([ae6baa2](https://github.com/maidsafe/safe-nodejs/commit/ae6baa2bf0c84f1683e85622011e79b758e75e1a))
* **ci:** remove native/target folder before creating electron package in CI ([4589f16](https://github.com/maidsafe/safe-nodejs/commit/4589f16e6ac02db073d63fafe49b3efa1983dd41))
* **ci:** Use travis-wait-enhanced in postinstall ([6a6106c](https://github.com/maidsafe/safe-nodejs/commit/6a6106c5c092c5d6b649ded3645bafdaf832fb3f))
* **Travis:** Use project specific env var for full build. ([6497139](https://github.com/maidsafe/safe-nodejs/commit/6497139f7f4fcdd21a9fb6137736fbf1488cc620))
* postinstall script module ([aec5a7f](https://github.com/maidsafe/safe-nodejs/commit/aec5a7fd411b0052c0e42256bd7cf21ec5f0abcc))
* **ci:** Update release process to improve module publishing ([b8ba3b5](https://github.com/maidsafe/safe-nodejs/commit/b8ba3b5682074d61aa75cf9460fff6d7a155ee04))
* windows rust host ([39bab5f](https://github.com/maidsafe/safe-nodejs/commit/39bab5ff72045300cd7259dd44b068ece4737ec1))

### [0.11.0](https://github.com/maidsafe/safe-nodejs/compare/v0.10.0...v0.11.1) (2020-03-30)

### Features

* **safe-api** Update safe-api to 0.11.0

### [0.10.0](https://github.com/maidsafe/safe-nodejs/compare/v0.9.0...v0.10.0) (2020-03-23)


### Features

* **lib:** adapt fetch binding to support optional range arg, and adapt files_container_create binding to support creation of empty containers ([01ca8e9](https://github.com/maidsafe/safe-nodejs/commit/01ca8e95aa5bbf77774067c5d2e0e6d496fd89ed))
* **lib:** add files_container_remove_path binding ([2f6d1cb](https://github.com/maidsafe/safe-nodejs/commit/2f6d1cb496f220895547113a8594007c4e06a9e8))

### [0.9.0](https://github.com/maidsafe/safe-nodejs/compare/v0.8.1...v0.9.0) (2020-03-03)


### Features

* **api:** upgrade safe-api to v0.9.0 ([dc7aedc](https://github.com/maidsafe/safe-nodejs/commit/dc7aedcf9207e042fb2466efa090fe6ce906d54e))


### [0.8.1](https://github.com/maidsafe/safe-nodejs/compare/v0.8.0...v0.8.1) (2020-02-07)


### Features

* **electron** Update target electron versions to be 7/8 ([725d5b4](https://github.com/maidsafe/safe-nodejs/commit/725d5b49469aa1e6936dcd5f9b03c3ee9ac76e4c))

### [0.8.0](https://github.com/maidsafe/safe-nodejs/compare/v0.7.0...v0.8.0) (2020-01-30)


## [0.7.0](https://github.com/maidsafe/safe-nodejs/compare/v0.6.0...v0.7.0) (2020-01-21)


### Features

* **files:** minor changes to files api binding to upgrade safe-api ([b955c7a](https://github.com/maidsafe/safe-nodejs/commit/b955c7af75be10483d5cba1da74de5fd6a9460bd))

## [0.6.0](https://github.com/maidsafe/safe-nodejs/compare/v0.5.1...v0.6.0) (2019-12-03)


### Features

* **authd:** add bindings for authd install and uninstall APIs ([49b704c](https://github.com/maidsafe/safe-nodejs/commit/49b704cf5b4b3341d86e0eec3d4a511ce57bcb34))

### [0.5.1](https://github.com/maidsafe/safe-nodejs/compare/v0.4.0...v0.5.1) (2019-11-12)


### Bug Fixes

* **ci:** Cleanup between various electron releases. ([62dcc3a](https://github.com/maidsafe/safe-nodejs/commit/62dcc3a))

## [0.5.0](https://github.com/maidsafe/safe-nodejs/compare/v0.3.2...v0.5.0) (2019-11-11)


### Features

* **authd:** add nodejs bindings for SafeAuthdClient start/stop/restart APIs ([1dd4526](https://github.com/maidsafe/safe-nodejs/commit/1dd4526777a01922679c1c439738c0081e716355))
* **authd:** implement binding for authd status API ([434e3c9](https://github.com/maidsafe/safe-nodejs/commit/434e3c9690b532502d8cf6febd9c89c70dffd656))
* **authd:** implementation of nodejs bindings for SafeAuthdClient API ([4006c21](https://github.com/maidsafe/safe-nodejs/commit/4006c2139fd86864f475f2a5cb98c7e2c6a79705))
* **authd:** implementation of the subscribe JS binding to receive auth reqs notifications from authd ([640f0a1](https://github.com/maidsafe/safe-nodejs/commit/640f0a1ce667f1c9fd3451a563f8d1c63d270663))
* **electron:** Build for electron 7 ([acc8de8](https://github.com/maidsafe/safe-nodejs/commit/acc8de8a78da747badf3672027e2d6ca339cb8ed))

## [0.4.0](https://github.com/maidsafe/safe-nodejs/compare/v0.3.2...v0.4.0) (2019-10-29)


### ⚠ BREAKING CHANGES

* **electron:** Stop doing builds for electron 6 versions

### Features

* **electron:** Build for electron 7 ([351f1e7](https://github.com/maidsafe/safe-nodejs/commit/351f1e7))

### [0.3.2](https://github.com/maidsafe/safe-nodejs/compare/v0.3.1...v0.3.2) (2019-10-22)


### Features

* **package:** Upgrade neon to 0.3.2

### Bug Fixes

* **api:** allow optional arguments to be passed as null ([a1f6640](https://github.com/maidsafe/safe-nodejs/commit/a1f66400c79e59337c208dd5eeb3c933adb8e4a4))
* **Travis:** Use project specific env var for full build. ([fd81dad](https://github.com/maidsafe/safe-nodejs/commit/fd81dad36a823fe5b028f45056f50e704087ef3a))

### [0.3.1](https://github.com/maidsafe/safe-nodejs/compare/v0.3.0...v0.3.1) (2019-10-16)

### Features

* **package:** Upgrade safe-api to 0.5.0

## 0.3.0 (2019-10-16)


### Features

* **Electron:** Update to 6.0.11 ([0ead232](https://github.com/maidsafe/safe-nodejs/commit/0ead232aef21fab006b866322cf691b108e9a82b))
* **Electron:** Update to building 6.0.12 ([2d7c6c2](https://github.com/maidsafe/safe-nodejs/commit/2d7c6c263fb59b8e30f06a0b24f508a5f5b86ad8))
* **inspect:** implementation of binding for inspect API ([8f8e1c0](https://github.com/maidsafe/safe-nodejs/commit/8f8e1c01a841d6a79a2a829120f697cd134a30a4))
* **wallet:** implementation of Keys and Wallet bindings ([a542e8d](https://github.com/maidsafe/safe-nodejs/commit/a542e8d0acf4da300f0a701c735f7d464ba53b8f))


### Bug Fixes

* postinstall script module ([d754cbc](https://github.com/maidsafe/safe-nodejs/commit/d754cbc18828adcdd39145f6f4f4ff71b41fd0f3))


### 0.2.4 (2019-09-23)


### Bug Fixes

* **ci:** remove native/target folder before creating electron package in CI ([951cb43](https://github.com/maidsafe/safe-nodejs/commit/951cb43))

### 0.2.3 (2019-09-23)


### Bug Fixes

* windows rust host ([590938c](https://github.com/maidsafe/safe-nodejs/commit/590938c))
* **ci:** Update release process to improve module publishing ([884e335](https://github.com/maidsafe/safe-nodejs/commit/884e335))
* **ci:** Use travis-wait-enhanced in postinstall ([7c5ad5c](https://github.com/maidsafe/safe-nodejs/commit/7c5ad5c))


### Features

* **fetch:** implementation of first set of bindings, auth, connect, and fetch functions ([ca40325](https://github.com/maidsafe/safe-nodejs/commit/ca40325))
* **files:** add binding for files add APIs, plus support Buffer and ArrayBuffer for args receiving file bytes ([70812bd](https://github.com/maidsafe/safe-nodejs/commit/70812bd))
* **files:** implementation of FilesContainer and PublishedImmutableData API bindings ([e802a56](https://github.com/maidsafe/safe-nodejs/commit/e802a56))
* **nrs:** implementation of the NRS API bindings ([1270380](https://github.com/maidsafe/safe-nodejs/commit/1270380))
* **packaging:** Enable packaging and downloading of prepackaged versions ([9a1cd02](https://github.com/maidsafe/safe-nodejs/commit/9a1cd02))
* **packaging:** Setup basic packaging ([89c9010](https://github.com/maidsafe/safe-nodejs/commit/89c9010))
* **win:** Windows build and packaging. ([9536311](https://github.com/maidsafe/safe-nodejs/commit/9536311))
* **Windows:** Use temp fork of neon to fix windows builds ([9748c40](https://github.com/maidsafe/safe-nodejs/commit/9748c40))
* **xorurl:** implement JS binding for XorUrlEncoder ([ab40d5f](https://github.com/maidsafe/safe-nodejs/commit/ab40d5f))

### [0.2.2](https://github.com/maidsafe/safe-nodejs/compare/v0.2.1...v0.2.2) (2019-09-23)


### Bug Fixes

* **ci:** Update release process to improve module publishing ([aa33365](https://github.com/maidsafe/safe-nodejs/commit/aa33365))
* **ci:** Use travis-wait-enhanced in postinstall ([b9de9f8](https://github.com/maidsafe/safe-nodejs/commit/b9de9f8))

### [0.2.1](https://github.com/maidsafe/safe-nodejs/compare/v0.1.3...v0.2.1) (2019-09-23)


### Bug Fixes

* windows rust host ([590938c](https://github.com/maidsafe/safe-nodejs/commit/590938c))


### Features

* **files:** add binding for files add APIs, plus support Buffer and ArrayBuffer for args receiving file bytes ([70812bd](https://github.com/maidsafe/safe-nodejs/commit/70812bd))
* **files:** implementation of FilesContainer and PublishedImmutableData API bindings ([e802a56](https://github.com/maidsafe/safe-nodejs/commit/e802a56))
* **nrs:** implementation of the NRS API bindings ([1270380](https://github.com/maidsafe/safe-nodejs/commit/1270380))
* **packaging:** Enable packaging and downloading of prepackaged versions ([9a1cd02](https://github.com/maidsafe/safe-nodejs/commit/9a1cd02))
* **packaging:** Setup basic packaging ([89c9010](https://github.com/maidsafe/safe-nodejs/commit/89c9010))
* **win:** Windows build and packaging. ([9536311](https://github.com/maidsafe/safe-nodejs/commit/9536311))
* **Windows:** Use temp fork of neon to fix windows builds ([9748c40](https://github.com/maidsafe/safe-nodejs/commit/9748c40))
* **xorurl:** implement JS binding for XorUrlEncoder ([ab40d5f](https://github.com/maidsafe/safe-nodejs/commit/ab40d5f))

## [0.2.0]
- Initial pre-packaged release.
## [0.0.1]
- Initial implementation
