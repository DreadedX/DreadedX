#!/bin/bash
cd tool && cargo run && cd .. && latexmk -shell-escape -lualatex
