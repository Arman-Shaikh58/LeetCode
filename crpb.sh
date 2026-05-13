#!/bin/bash

# this script is written by me so basically the reason was that
# every time i create a new folder of the leetcode problem i was repeatedly writing the same instruction to do the same job
# im tring not to make this script from the chatGPT and create souly just taking guidance and commands from the chatGPT

if [ $# -gt 1 ]; then
  echo "To many arguments skipping arguments after -> $1"
fi

problem_no=$1

initial_folder_name="problem_"

folder_name="$initial_folder_name$problem_no"

cd problems

if [ -d $folder_name ]; then
  echo "Error: Folder already exists!"
else
  cargo init $folder_name
  cd $folder_name
  nvim .
fi
