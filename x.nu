#!/usr/bin/env nu

let bin_name = "merpati"
let version  = "v0.1.0"

def main [] {}

def "main ci build" [target: string] {
  build $target
  archive-tar $target

  print $"artifact-path=(archive-path $target)"
  print $"artifact-name=(archive-name $target)"
}

def "main ci changelog" [] {
  let changelog = (get-changelog)

  print $"version=($changelog.version)"
  print $"body<<EOF"
  print $"($changelog.body)"
  print $"EOF"
}

def "main changelog prepare" [] {
  let version = (merpati-version)
  let changelog = open CHANGELOG.md | to text

  if ($changelog | find $version | is-not-empty) {
    print $"Changelog version ($version) already exist"
    exit 1
  }

  let date = (date now | format date "%Y-%m-%d")

  $"\n## ($version) [($date)]\n\nTODO\n\n---"
  | append (open CHANGELOG.md)
  | save -f CHANGELOG.md
}

def build [target: string] {
  cargo build --profile release --target $target
}

def merpati-version [] {
  "v" + (open Cargo.toml | get workspace.package.version)
}

def archive-name [target: string] {
  $"($bin_name)-($version)-($target).tar.gz"
}

def archive-tar [target: string] {
  let archive_name = archive-name $target
  let binary_path  = $"target/($target)/release/($bin_name)"
  let archive_dir  = $"target/($target)/archive"
  let archive_path = $"target/($archive_name)"

  rm -rf $archive_dir
  rm -rf $archive_path

  install -Dm755 $binary_path -t $"($archive_dir)/bin"
  tar czf $archive_path -C $archive_dir .

  rm -rf $archive_dir
}

def archive-path [target: string] {
  let archive_name = archive-name $target
  let archive_path = $"target/($archive_name)"

  if ($archive_path | path exists) {
    $archive_path
  } else {
    exit 1
  }
}

def get-changelog [] {
  let vers = (merpati-version)
  let lines = open CHANGELOG.md | lines
  let skip_index = $lines | enumerate | where item =~ $version | get index.0
  let body = $lines | skip $skip_index | take until {|line| $line == "---"} | str join "\n"

  {
    version: $vers,
    body: $body,
  }
}
