# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-02-03

### Added
- Initial release of the unofficial BlindPay Rust SDK
- Core client implementation with authentication
- **Available resources** (rails, bank details, SWIFT codes)
- **Receivers resource** (create, list, get, delete, limits)
- **Bank Accounts resource** - All payment rails:
  - PIX (Brazil)
  - ACH (United States)
  - Wire (United States)
  - RTP (Real-Time Payments, United States)
  - SPEI (Mexico)
  - Argentina Transfers (CVU/CBU/ALIAS)
  - Colombia ACH
  - International SWIFT
- **Payouts resource** (create, list, get, track)
  - Stellar payouts
  - EVM payouts
  - Solana payouts
- **Payins resource** (create, list, get, track)
- **Quotes resource**
  - Payout quotes with contract information
  - Payin quotes
  - FX rate calculations
- **Partner Fees resource** (create, list, get, delete)
- **Blockchain Wallets resource** (create, list, get, delete, sign messages)
- **Offramp Wallets resource** (create, list, get)
- **Virtual Accounts resource** (create, list, get, update)
- **Instances resource**
  - Get members
  - Update instance
  - Delete instance
  - Manage member roles
- **API Keys resource** (create, list, get, delete)
- **Webhooks resource**
  - Create, list, delete endpoints
  - Get endpoint secrets
  - Portal access URLs
- **Terms of Service resource** (initiate acceptance flow)
- Comprehensive type definitions for all API entities
- Error handling with custom error types
- Async/await support with tokio
- Full documentation and examples
- Integration tests

### Features
- Type-safe API with Rust's type system
- Idiomatic Rust design patterns
- Comprehensive error handling
- Easy-to-use builder patterns
- Full async/await support
- Support for all payment rails and methods
- Complete webhook event types
- All instance management operations

[Unreleased]: https://github.com/Emengkeng/blindpay-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Emengkeng/blindpay-rust/releases/tag/v0.1.0
