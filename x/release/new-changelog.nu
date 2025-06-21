#!/usr/bin/env nu

def main [] {
  let date = (date now | format date "%Y-%m-%d")

  $"\n## vX.X.X - ($date )\n\nTODO\n\n---"
  | append (open CHANGELOG.md)
  | save -f CHANGELOG.md
}
