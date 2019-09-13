# safe-nodejs

## Overview


| [MaidSafe website](https://maidsafe.net) | [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:-------:|:-------:|:-------:|

## Deploy

For electron run, `yarn neon-build-electron`, which will build for electron 6 as it stands.

Before uploading binaries you should ensure you remove `native/target` folder or that'll be collected and packaged up too. VASTLY increasing file size, needlessly.

Then `yarn upload binaries` will upload the current node version and an electron one too.

Releases are triggered on github via the presence of TRAVIS_TAG.

## License

This is dual-licensed under the Modified BSD ([LICENSE-BSD](LICENSE-BSD) https://opensource.org/licenses/BSD-3-Clause) or the MIT license ([LICENSE-MIT](LICENSE-MIT) https://opensource.org/licenses/MIT) at your option.

## Contribution

Copyrights in the SAFE Network are retained by their contributors. No copyright assignment is required to contribute to this project.
