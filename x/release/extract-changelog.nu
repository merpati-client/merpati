#!/usr/bin/env nu

def main [version: string] {
  let lines = open CHANGELOG.md | lines
  let skip_index = $lines | enumerate | where item =~ $version | get index.0
  let block = $lines | skip $skip_index | take until {|line| $line == "---"}
  print ($block | str join "\n")
}
