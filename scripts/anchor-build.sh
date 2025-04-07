cd ./anchor
anchor build

cp ./target/deploy/solbay.so ./tests/fixtures

# Copy anchor src & target to client
cd ..

rm -rf ./client/src/contract
mkdir -p ./client/src/contract/src
mkdir -p ./client/src/contract/target

cp -r ./anchor/target/idl ./anchor/target/types ./client/src/contract/target
cp -r ./anchor/src ./client/src/contract