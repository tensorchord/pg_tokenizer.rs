#!/usr/bin/env bash
set -xeu

printf "SEMVER = ${SEMVER}\n"
printf "VERSION = ${VERSION}\n"
printf "ARCH = ${ARCH}\n"
printf "PLATFORM = ${PLATFORM}\n"

cargo build --lib --features pg$VERSION --release
cargo pgrx schema --features pg$VERSION --out ./target/schema.sql

mkdir -p ./build/zip
[[ -d ./sql/upgrade ]] && cp -a ./sql/upgrade/. ./build/zip/
cp ./target/schema.sql ./build/zip/pg_tokenizer--$SEMVER.sql
sed -e "s/@CARGO_VERSION@/$SEMVER/g" <./pg_tokenizer.control >./build/zip/pg_tokenizer.control
cp ./target/release/libpg_tokenizer.so ./build/zip/pg_tokenizer.so
zip ./build/postgresql-${VERSION}-pg-tokenizer_${SEMVER}_${ARCH}-linux-gnu.zip -j ./build/zip/*

mkdir -p ./build/deb
mkdir -p ./build/deb/DEBIAN
mkdir -p ./build/deb/usr/share/postgresql/$VERSION/extension/
mkdir -p ./build/deb/usr/lib/postgresql/$VERSION/lib/
for file in $(ls ./build/zip/*.sql | xargs -n 1 basename); do
    cp ./build/zip/$file ./build/deb/usr/share/postgresql/$VERSION/extension/$file
done
for file in $(ls ./build/zip/*.control | xargs -n 1 basename); do
    cp ./build/zip/$file ./build/deb/usr/share/postgresql/$VERSION/extension/$file
done
for file in $(ls ./build/zip/*.so | xargs -n 1 basename); do
    cp ./build/zip/$file ./build/deb/usr/lib/postgresql/$VERSION/lib/$file
done
echo "Package: postgresql-${VERSION}-pg-tokenizer
Version: ${SEMVER}-1
Section: database
Priority: optional
Architecture: ${PLATFORM}
Maintainer: Tensorchord <support@tensorchord.ai>
Description: Tokenizer plugin for PostgreSQL
Homepage: https://vectorchord.ai/
License: Apache-2.0" \
    >./build/deb/DEBIAN/control
(cd ./build/deb && md5sum usr/share/postgresql/$VERSION/extension/* usr/lib/postgresql/$VERSION/lib/*) >./build/deb/DEBIAN/md5sums
dpkg-deb --root-owner-group -Zxz --build ./build/deb/ ./build/postgresql-${VERSION}-pg-tokenizer_${SEMVER}-1_${PLATFORM}.deb

ls ./build
