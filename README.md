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

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn
- A Solana wallet (Phantom, Solflare, etc.)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd solana-dapp-ui
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run dev
```

4. Open [http://localhost:3000](http://localhost:3000) in your browser

### Usage

1. **Connect Wallet**: Click "Connect Wallet" and select your preferred Solana wallet
2. **Create Auction**: Fill in the auction form with:
   - Token mint address
   - End time
   - Minimum bid amount
3. **Browse Auctions**: View all active auctions in the main list
4. **Place Bids**: Select an auction and place your bid
5. **End Auction**: As the auction creator, you can end expired auctions

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

## Project Structure

```
solana-dapp-ui/
├── app/
│   ├── globals.css
│   ├── layout.tsx
│   └── page.tsx
├── components/
│   ├── AuctionApp.tsx        # Main application component
│   ├── AuctionList.tsx       # Display list of auctions
│   ├── BidForm.tsx           # Bidding interface
│   ├── CreateAuctionForm.tsx # Auction creation form
│   ├── EndAuctionButton.tsx  # End auction functionality
│   └── WalletProvider.tsx    # Wallet connection provider
├── lib/
│   └── anchor.ts             # Anchor program configuration
└── package.json
```

## Technologies Used

- **Next.js 15**: React framework with App Router
- **TypeScript**: Type-safe JavaScript
- **Tailwind CSS**: Utility-first CSS framework
- **Solana Web3.js**: Solana blockchain interaction
- **Anchor**: Solana program framework
- **Wallet Adapter**: Solana wallet integration
- **Lucide React**: Icon library

## Development

### Available Scripts

- `npm run dev`: Start development server
- `npm run build`: Build for production
- `npm run start`: Start production server
- `npm run lint`: Run ESLint

### Environment Setup

The application is configured for Solana Devnet. To use a different cluster, modify the network configuration in `components/WalletProvider.tsx`.

FE repo : https://github.com/Firdausfarul/auction_ui
