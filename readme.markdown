# Nametag

Generate random usernames.

```text
$ nametag
barbecuedreliance655
ace_conduit879
overdressed_leftover489
ill-considered_monopolization726
phantomsuspect347
badlevel251
precautionarywesterner537
horse-drawn_futility393
incompatiblecontingent628
vapid_protea736
```

## Usage

```text
USAGE:
    nametag [OPTIONS]

OPTIONS:
    -h, --help                        Print help information
    -l, --list <LIST_FILE_PATH>       Provide a text file with a list of words to generate username
                                      from randomly
    -m, --maximum <MAXIMUM_LENGTH>    Set maximum username length. Must be greater than 5 [default:
                                      50]
    -n, --number <NUMBER_TO_PRINT>    Set how many random usernames to output [default: 10]
    -t, --title-case                  Uses Title Case for words in generated usernames
    -v, --verbose                     Prints verbose output, including parameters as received
    -V, --version                     Print version information
```

If the `maximum` option is set less than 11, nametag will only pull one word from the word list. If it's set to 11 or higher, it will pull two words from the word list.

**Note**: This program is NOT intended to be used to create secure passwords. **Do NOT use this program to create passwords.**

## Usage examples

- `nametag` generates 10 random usernames from included word lists (see below)
- `nametag -l path/to/a/custom_wordlist.txt` generates 10 random usernames using words from provided `.txt` file, where each word is on its own line
- `nametag -n 5 -m 12` generates 5 random usernames with a maximum length of 12 characters
- `nametag -n 15 -t` generates 15 random usernames where each word is in Title Case (e.g. Axiomatic_Myalgia791 and Foolhardy_Appropriateness269)
- `nametag -n 25 > usernames.txt` writes 25 random usernames to a new file called `usernames.txt`

## Why random usernames are important

Check out [this blog post from Bitwarden about why random usernames can boost security and privacy](https://bitwarden.com/blog/whats-in-a-user-name/).

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/nametag --branch main`

## Installation with Docker

1. [Install Docker](https://docs.docker.com/get-docker)
2. Clone this repo and move into the created directory: `git clone https://github.com/sts10/nametag && cd nametag`
3. Run: `docker-compose up`

### Docker Notes

- During development, run `docker-compose up --build` when changing Rust code and adding word lists to rebuild with new code and/or dependencies.

- To change the command nametag is run with, edit the command in `docker-compose.yml`. Make sure to keep `./` before `nametag` as provided in the default. For example: `./nametag -v -l /nametag/word-lists/eff_large_wordlist.txt`

- It is recommended to add user-provided word lists into the `word-list` directory, and use either the absolute path (`/nametag/word-lists/custom_wordlist.txt`) or relative path (`./word-lists/custom_wordlist.txt`) when using them as an argument.

## On the included word lists

If no word list is provided, nametag uses one or two word lists from [the SecureDrop project](https://github.com/freedomofpress/securedrop/) to create usernames. If the maximum length allows (or is not specified), usernames will be in "[adjective](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) + [noun](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt) + number" format. If the maximum length is set below 11 characters, nametag will use "noun + number" format.

If the user provides a word list of their own, using the `-l` option, nametag will use that provided list to generate _both_ words. 

If you're looking for some different word lists, you can check out my [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists). If you're looking to _create or edit_ a word list, you might find useful another tool I built called [Tidy](https://github.com/sts10/tidy).

## Other similar projects also written in Rust

It makes sense that others have made similar programs in Rust. Given my limited experience with Rust, they may very well be better than nametag, _especially_ if you're looking for a library to use in a larger (Rust) project.

- [names](https://github.com/fnichol/names)
- [rust-petname](https://github.com/allenap/rust-petname)

## Licensing

This project is licensed under [GNU Affero General Public License v3](https://www.gnu.org/licenses/agpl-3.0.en.html). (Note that [SecureDrop is available under that same License](https://github.com/freedomofpress/securedrop/blob/develop/LICENSE).) See LICENSE.txt file for more information.
