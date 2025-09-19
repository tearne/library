#!/bin/env bash
DOWNLOAD=https://github.com/vega/vl-convert/releases/download/v1.8.0/vl-convert-linux-x86.zip

FILE=${DOWNLOAD##*/}
TEMP=/tmp/vl-convert
SHARE=~/.local/share/${FILE%.*}

rm -rf $TEMP \
&& wget -P $TEMP/ $DOWNLOAD \
&& unzip -d $TEMP/inner $TEMP/$FILE \
&& unzip -d $SHARE/ $TEMP/inner/*.zip \
&& ln -s $SHARE/bin/vl-convert ~/.local/bin/vl-convert \
&& rm -rf $TEMP

type vl-convert
