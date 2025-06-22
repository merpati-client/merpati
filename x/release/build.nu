#!/usr/bin/env nu

use ./changelog.nu

let name    = "merpati"
let version = changelog app-version

def build [target: string] {
  print $"[info] compiling binary for target: ($target)"
  cargo build --profile release --target $target
  print $"[info] finished compiling binary for target: ($target)"
}

def archive-tar [target: string] {
  let archive_name = $"($name)-($version)-($target).tar.gz"

  let binary_path  = $"target/($target)/release/($name)"
  let archive_dir  = $"target/($target)/archive"
  let archive_path = $"target/($archive_name)"

  print $"[info] creating archive: ($archive_name)"

  rm -rf $archive_dir

  install -Dm755 $binary_path -t $"($archive_dir)/bin"
  tar czvf $archive_path -C $archive_dir .

  rm -rf $archive_dir

  print $"[info] finished creating archive: ($archive_path)"
}

def archive-path [target: string] {
  let archive_name = $"($name)-($version)-($target).tar.gz"
  let archive_path = $"target/($archive_name)"

  if ($archive_path | path exists) {
    $archive_path
  } else {
    exit 1
  }
}

def archive-name [target: string] {
  $"($name)-($version)-($target).tar.gz"
}

def main [cmd: string, target: string] {
  match $cmd {
    "build" => { build $target; archive-tar $target },
    "archive-path" => { print (archive-path $target) },
    "archive-name" => { print (archive-name $target) },
    _ => { print "available commands: build, archive-path, archive-name" }
  }
}
