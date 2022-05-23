#!/bin/env python

from malachi import Command

c = Command("""
?bible
[
	<index?: nocase(), `index`, `-index`, `show`>
	<book?: starts(`book=`)>
	<chapter?: starts(`ch=`, `chapter=`), /^[0-9]+$/>
	<verse?: starts(`verse=`)>
]
""")

inputs = [
	"?bible",
	"?bible book=genesis",
	"?bible book=genesis ch=3",
	"?bible book=genesis ch=3 verse=10",
	"?bible index",
	"not a match",
	"?bible ch=notanumber",
]

for s in inputs:
	m = c.get_matches(s)
	has_prefix = c.has_prefix(s)
	print(f"{s}\n->  {m}\n->  has prefix: {has_prefix}")

try:
	invalid = "<lmao"
	print(f"trying to compile this invalid command: {invalid}")
	_ = Command(invalid)
except Exception as e:
	print(f"error: {e}")
