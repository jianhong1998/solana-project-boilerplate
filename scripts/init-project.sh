# Point solana cli to local
solana --version
solana config set -ul

# Install project dependencies
echo "Installing packages..."
make install

# Generate keypairs for local env
solana-keygen new -o ./temp/keypairs/fee-payer.json
solana-keygen new -o ./temp/keypairs/program-owner.json