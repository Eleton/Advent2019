#!/bin/bash
# ./gen.sh <DAY> <LANGUAGE> <YEAR>
source .envrc

DAY=$(printf "%02d\n" $1)

if [ ! -z "$2" ];
then
  LANGUAGE=$2
fi

if [ ! -z "$3" ];
then
  if [ $3 -ge 2015 ];
  then
    YEAR=$3
  else
    echo "Provide a proper year"
  fi
fi



case $LANGUAGE in
  "rust")
    PRE_DIR="$YEAR/rs${DAY}"
    DIR="$YEAR/${DAY}rs"

    echo "Does $DIR/src/main.rs exist?"
    if [ -e "$DIR" ]; then
      read -p "$DIR already exists. Overwrite? Y/n" DECISION
      if [[ "$DECISION" =~ ^([nN][oO]|[nN])$ ]] ;;
      fi
    fi
    echo "Generating Rust-project in $PRE_DIR and renaming to $DIR"
    cargo new $PRE_DIR
    mv $PRE_DIR $DIR/

    URL="https://adventofcode.com/${YEAR}/day/$1/input"
    curl --header "Cookie: session=${COOKIE}" $URL --output "$DIR/signal.txt"

    touch "$DIR/example.txt"
    cp languages/rust/main.rs $DIR/src/main.rs
    cp languages/rust/alpha.rs $DIR/src/alpha.rs
    cp languages/rust/beta.rs $DIR/src/beta.rs
  ;;
  "go")
    DIR="$YEAR/${DAY}go"
    echo "Generating Go-project in $DIR"

    mkdir $DIR
    touch "$DIR/example.txt"

    cp languages/go/main.go $DIR/main.go
    cp languages/go/alpha.go $DIR/alpha.go
    cp languages/go/beta.go $DIR/beta.go
    URL="https://adventofcode.com/${YEAR}/day/$1/input"
    curl --header "Cookie: session=${COOKIE}" $URL --output "$DIR/signal.txt"
  ;;
  "js")
    DIR="$YEAR/${DAY}js"
    echo "Generating Node-project in $DIR"

    mkdir $DIR
    touch "$DIR/example.txt"

    cp languages/js/index.js $DIR/index.js
    cp languages/js/alpha.js $DIR/alpha.js
    cp languages/js/beta.js $DIR/beta.js
    URL="https://adventofcode.com/${YEAR}/day/$1/input"
    curl --header "Cookie: session=${COOKIE}" $URL --output "$DIR/signal.txt"
  ;;
  *)
    echo "Supported languages are rust, go and js"
  ;;
esac

