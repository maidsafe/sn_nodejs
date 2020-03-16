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

## Contributing

Want to contribute? Great :tada:

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](https://github.com/maidsafe/QA/blob/master/CONTRIBUTING.md).
