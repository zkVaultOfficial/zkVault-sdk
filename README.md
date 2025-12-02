# zkVault Monorepo

A comprehensive, production-grade monorepo for zkVault, featuring Solana on-chain programs, zero-knowledge circuits, TypeScript SDK, CLI tools, documentation, and integration examples.

## Structure

- `programs/zkvault-program/` - Solana Anchor program in Rust
- `circuits/` - Noir zero-knowledge circuits
  - `base/` - Base circuit implementations
  - `prover/` - Prover circuits
  - `compressor/` - Circuit compression utilities
  - `examples/` - Example circuits
- `sdk/` - TypeScript SDK
  - `core/` - Core SDK functionality
  - `solana/` - Solana-specific integrations
  - `crypto/` - Cryptographic utilities
- `cli/` - Command-line interfaces
  - `zkvault-cli/` - Main CLI tool
  - `tasks/` - CLI task definitions
- `docs/` - Docusaurus documentation site
- `examples/` - Integration examples
  - `nextjs-app/` - Next.js application example
  - `node-proof-demo/` - Node.js proof generation demo
  - `solana-integration/` - Solana integration example
- `scripts/` - Utility scripts
- `tests/` - Test suites
- `.github/workflows/` - CI/CD pipelines

## Getting Started

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build all packages:
   ```bash
   npm run build
   ```

3. Run tests:
   ```bash
   npm run test
   ```

## Documentation

See the [docs](./docs) directory for full documentation.

## License

MIT