# changelg
Generate changelog reports from Git repositories

[![Build Status](https://travis-ci.org/imjacobclark/changelg.svg)](https://travis-ci.org/imjacobclark/changelg)

### Usage

```shell
$ changelg <SHA_FROM> <SHA_TO>
```

Example:

```shell
$ changelg 377d686351969f27f288dec2fb09d0d5431fcde1 8c83317ae4d9a9f79a34321438843f023a94c9e8
#changelog Allow commit to commit #changelog stdout by Jacob Clark <jacob.jh.clark@googlemail.com>
```

### Building

```shell
$ git clone git@github.com:imjacobclark/changelg.git && cd changelg
$ cargo run 377d686351969f27f288dec2fb09d0d5431fcde1 8c83317ae4d9a9f79a34321438843f023a94c9e8
#changelog Allow commit to commit #changelog stdout by Jacob Clark <jacob.jh.clark@googlemail.com>
```

