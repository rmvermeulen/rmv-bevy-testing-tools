# set unstable
set shell := ['nu', '-c']

[private]
default:
    just --list

# run tests for all configured feature combinations
test-all *FLAGS:
    cargo fc --silent --fail-fast --pedantic nextest run {{ FLAGS }}

current_version := `open Cargo.toml | get package.version`
current_tag := `git tag --points-at HEAD`

[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
_patch-version version:
    #!/usr/bin/env nu
    if (git status -s | lines | is-not-empty) {
        print -e "Workspace is not clean!"
        exit 1
    }
    sed -i 's/version = "{{ current_version }}"/version = "{{ version }}"/' Cargo.toml
    cargo generate-lockfile --offline
    git add Cargo.toml Cargo.lock
    git commit --message 'Cargo.toml: version {{ version }}'
    print "version changed!"

[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
_push-version version:
    #!/usr/bin/env nu
    let last_commit_msg = (git show HEAD --no-patch --format="%s")
    if $last_commit_msg != 'Cargo.toml: version {{ version }}' {
        print -e "Unexpected commit message on HEAD (Did you run `just version patch`?)"
        exit 1
    }
    # try to push; if it fails, we do not tag the commit yet
    git push
    # now set the tag
    git tag 'v{{ version }}'
    # push the tag; hook already passed
    git push --no-verify --tags
    print "version change pushed!"

_publish-version version:
    #!/usr/bin/env nu
    let current_tag = git tag --points-at HEAD
    if $current_tag != "v{{ version }}" {
        print -e "Invalid tag on HEAD (Did you run `just version push`?)"
        exit 1
    }
    cargo publish
    print "version change published!"

# create a new version.
[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
[arg("action", pattern="patch|push|publish", help="Patch(=commit) Push(=push and tag) Publish")]
[script("nu")]
version version action:
    match {{ action }} {
        patch   => {
            print "patching version..."
            just _patch-version {{ version }} }
        push    => {
            print "pushing version..."
            just _push-version {{ version }}
        }
        publish => {
            print "publishing version..."
            just _publish-version {{ version }}
        }
    }
