# ForgeGuardian
A simple binary blob test synthesiser and mutator. 

## Installation

Download the most recent release, or ForgeGuardian can be compiled from source with `cargo build --release`.

## Usage
#### Generating Test Binaries
Use `./ForgeGuardian config.toml` to run ForgeGuardian. Test binaries will be built in the same directory as `config.toml`. 

#### Writing the Config
The configuration file should take the following form:
```toml
[tool]
cmd = "xz"
ext = ".xz"

[tests]

[tests.good_1]
extension = ".txt"
data = "This is a test file!"
encoding = "null"

[tests.good_2]
extension = ".jpg"
data = "/9j/4AAQSkZJRgABAQAAAQABAAD/2w..."
encoding = "b64"
transform = "--compress [path]"

[tests.bad_1]
extension = ".txt"
data = "This is a test file!"
encoding = "null"
transform = "--compress [path]"
hex_edit = {start = 0x10, end = 0x20, data = ""}

[tests.bad_2]
extension = ".jpg"
data = "/9j/4AAQSkZJRgABAQAAAQABAAD/2w..."
encoding = "b64"
transform = "--compress [path]"
hex_edit = {start = 0x10, end = 0x20, data = "46000101 00000100 010000FF DB008400 09060713 13121513 13131616 15171818"}
```
The path to the tool specified may be a fully qualified path, or where the tool is on the PATH of the machine, the tool may be specified by name as above.
Tests may take any name. All fields are required except `transform` and `hex_edit` which may be included where needed. The example above would result in the following files:
- `good_1.txt`: A text file containing "This is a test file!"
- `good_1.txt.xz:` `good_1.txt` compressed with `xz` (built with `xz --compress good_1.txt`)
- `good_2.jpg`: (Where the full base64 data is included) a JPEG image
- `good_2.jpg.xz`: `good_2.jpg` compressed with `xz` (built with `xz --compress good_2.jpg`)
- `bad_1.txt`: A text file containing "This is a test file!"
- `bad_1.txt.xz`: `bad_1.txt` compressed with `xz` (built with `xz --compress bad_1.txt`). Hex 0x10 to 0x20 is deleted.
- `bad_2.jpg`: (Where the full base64 data is included) a JPEG image
- `bad_2.jpg.xz`: `bad_2.jpg` compressed with `xz` (built with `xz --compress good_2.jpg`). Hex 0x10 to 0x20 is replaced with the specified values.

## Roadmap
- [x] Minimum Viable Product
- [ ] Delete base files created by `create.rs`
- [ ] Options for tool extensions - ForgeGuardian will only handle compression-style extensions gracefully at present
- [ ] Threading for efficiency
- [ ] Fetch data from online source

## Contributing

Suggestions, bug reports, and changes are welcome.

## FAQ's
#### What is This?
ForgeGuardian is a simple open-source tool to help developers test software. It builds files, transforms them with a CLI tool, and alters their hex according to a human friendly config file, `config.toml`.
#### Who should use ForgeGuardian?
ForgeGuardian is not a complex test suite, nor was it intended to be. ForgeGuardian is intended for developers who do not need complex, fully fledged, testing suites or do not want to spend time configuring such solutions.
#### Why should I use ForgeGuardian?
Recently the open source community was taken aback when an almost-successful attack on the `xz` utils was launched by Jia Tan. Jia used test binaries distributed with `xz` which contained at least one obfuscated backdoor. ForgeGuardian aims to reduce the attack surface of such projects by defining how all test files should be built in a human-readable format. Many of the original test files distributed with `xz` were binaries created by hand in a hex editor meaning other developers were not able to easily verify the behaviour of the files. By defining how files should be built, developers should gain more visibility over test binaries associated with a project. It is hoped this goes some way to reducing the attack surface of open source projects.
#### When should I use ForgeGuardian?
When testing a project during development requires a set of files built in a consistent manner with specific portions of the file modified in some way. An example would be the test files distributed with the `xz` project allowing developers to verify the behaviour of their changes before creating PRs.
#### How should I use ForgeGuardian?
- **Developers** should define test files and binaries in the `config.toml` and make this public with the associated repo.
- **Contributors** should use a project's `config.toml` to build test files before running any other miscellaneous tests.