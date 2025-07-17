# Solana Token Auction dApp

A decentralized token auction platform built on Solana using Next.js, TypeScript, and Tailwind CSS.

## Features

- 🔐 **Wallet Integration**: Connect with popular Solana wallets (Phantom, Solflare)
- 🎯 **Create Auctions**: Create token auctions with customizable parameters
- 💰 **Place Bids**: Participate in live auctions with real-time bidding
- ⏰ **Time Management**: Automatic auction expiration and status tracking
- 🏆 **End Auctions**: Auction creators can finalize completed auctions
- 📱 **Responsive Design**: Works on desktop and mobile devices

## Program Details

- **Program ID**: `7aiANAF6e8JEyPVdY7whju5cXxuBBdAbLdVgz7xtPd65`
- **Cluster**: Devnet
- **Framework**: Anchor

## Smart Contract Functions

The dApp interacts with three main program instructions:

### `createAuction`
Creates a new token auction with specified parameters.

**Parameters:**
- `endTime`: Unix timestamp for auction end
- `minBid`: Minimum bid amount in lamports

### `bid`
Places a bid on an active auction.

**Parameters:**
- `amount`: Bid amount in lamports

### `endAuction`
Finalizes an expired auction and transfers tokens to the winner.



FE repo : https://github.com/Firdausfarul/auction_ui
