#!/bin/bash
arguments_count=$#
cargo_addition=""

if [ "$1" == '-e' ]  || [ "$1" == '--example' ]; then
	if [ $arguments_count != 3 ]; then
		echo "should be 3 arguments, but you provide only" $arguments_count
		exit 1
	fi

	example_name=$2
	cargo_addition='--example '$example_name
  elf_file='target/thumbv7m-none-eabi/release/examples/'$example_name
  bin_filepath=$3
else
	if [ $arguments_count != 1 ]; then
		echo "should be 1 argument, but you provide only" $arguments_count
		exit 1
	fi

	elf_file=target/thumbv7m-none-eabi/release/bare-metal
	bin_filepath=$1
fi

echo "==="
echo "building file '"$elf_file"' to '"$bin_filepath"' and uploading it to device"
echo "==="


cargo build --release $cargo_addition
if [ $? != 0 ]; then
  echo "building failed"
  exit
fi

arm-none-eabi-objcopy -O binary $elf_file $bin_filepath
if [ $? != 0 ]; then
  echo "binary file creation failed"
  exit
fi

st-flash write $bin_filepath 0x08000000