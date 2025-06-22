#!/usr/bin/env nu

def app-version [] {
  "v" + (open Cargo.toml | get workspace.package.version)
}

def get-changelog [version: string] {
  let lines = open CHANGELOG.md | lines
  let skip_index = $lines | enumerate | where item =~ $version | get index.0
  let block = $lines | skip $skip_index | take until {|line| $line == "---"}
  print ($block | str join "\n")
}

def new-changelog [version: string] {
  let changelog = open CHANGELOG.md | to text

  if ($changelog | find $version | is-not-empty) {
    print $"Changelog version ($version) already exist"
    exit 1
  }

  let date = (date now | format date "%Y-%m-%d")

  $"\n## ($version) - ($date)\n\nTODO\n\n---"
  | append (open CHANGELOG.md)
  | save -f CHANGELOG.md
}

def main [cmd: string, version?: string] {
  let version = $version | default (do { app-version })

  match $cmd {
    "get" => { get-changelog $version },
    "new" => { new-changelog $version },
    _ => { print "Unknown command" }
  }
}
