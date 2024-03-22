# Changelog

All changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Do not manually edit this file. It will be automatically updated when a new release is published.

## 0.3.0
_22 March 2024_

* Add an example for offset_within_superbox ([#6](https://github.com/scouten-adobe/jumbf-rs/pull/6))
* (MINOR) DataBox: Add new function offset_within_superbox ([#5](https://github.com/scouten-adobe/jumbf-rs/pull/5))

## 0.2.2
_13 March 2024_

* Update to reflect 2023 version of JUMBF standard

## 0.2.1
_13 March 2024_

* Fix incorrect changelog link

## 0.2.0
_13 March 2024_

* Add ability to limit recursion when parsing superboxes ([#3](https://github.com/scouten-adobe/jumbf-rs/pull/3))
* (MINOR) Change `SuperBox::from_box` to `SuperBox::from_data_box` ([#4](https://github.com/scouten-adobe/jumbf-rs/pull/4))
* Add more examples to readme
* (MINOR) Rename `Box` to `DataBox` ([#1](https://github.com/scouten-adobe/jumbf-rs/pull/1))

## 0.1.0
_12 March 2024_

* First public release
