# set unstable
set shell := ['nu', '-c']

[private]
default:
    just --list

test-all *FLAGS:
    cargo fc --silent --fail-fast --pedantic nextest run {{ FLAGS }}

current_version := `open Cargo.toml | get package.version`
current_tag := `git tag --points-at HEAD`

[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
_patch-version version:
    sed -i 's/version = "{{ current_version }}"/version = "{{ version }}"/' Cargo.toml
    cargo generate-lockfile --offline
    git add Cargo.toml Cargo.lock
    git commit --message 'Cargo.toml: version {{ version }}'
    print "version changed!"

[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
_push-version version: (_patch-version version)
    # TODO: check if HEAD is version-upgrade commit
    # try to push; if it fails, we do not tag the commit yet
    git push
    # push just the tag; hook already passed
    git tag '{{ version }}'; git push --no-verify --tags
    print "version change pushed!"

_publish-version version: (_push-version version)
    #!/usr/bin/env nu
    if ( {{ current_tag }} | is-empty ) {
        print -e "HEAD has no tag?!"
        exit 1
    }
    cargo publish
    print "version change published!"

# patch, push, or publish, a new version
[arg("action", pattern="patch|push|publish")]
[arg("version", pattern="\\d+\\.\\d+\\.\\d+(-\\w+)?")]
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
