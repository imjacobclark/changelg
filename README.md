# changelg
Generate changelog reports from Git repositories

[![Build Status](https://travis-ci.org/imjacobclark/changelg.svg)](https://travis-ci.org/imjacobclark/changelg)

### About

Currently under active development, changelg will walk your Git revision history and pull out commit messages tagged with `#changelog` into stdout.

changelg accepts two revision indetifiers in the form of commit and tag hashes as well as tag names and any other possible commit identifier.  

## Plans

- Allow autimatic submission of generated changelogs to services such as GitHub and Bitbucket
- Export changelogs to mulitple formats such as HTML, CSV and PDF

### Usage

```shell
$ changelg <FROM> <TO>
```

Example:

Where `1.0` & `2.0` are simply tag names. 

```shell
$ changelg 1.0 2.0
#changelog Allow commit to commit #changelog stdout by Jacob Clark <jacob.jh.clark@googlemail.com>
```

### Building

```shell
$ git clone git@github.com:imjacobclark/changelg.git && cd changelg
$ cargo run 377d686351969f27f288dec2fb09d0d5431fcde1 8c83317ae4d9a9f79a34321438843f023a94c9e8
#changelog Allow commit to commit #changelog stdout by Jacob Clark <jacob.jh.clark@googlemail.com>
```

