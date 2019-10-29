# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

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
