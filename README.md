# Gameland Appchain

A Free-to-play-to-earn(F2P2E) gaming platform with the GameFi NFT rental protocol.

Leveraging the blockchain game service market with experiential innovation

## Presentation
https://www.youtube.com/watch?v=6wg1YpB1kpk

## Website
www.gameland.network

## High-Level Concept (Abstract)
Gameland as a Gamefi platform aims to help users experience high-end games at the lowest price, provide users a rental platform with integrating experience and revenue, and lower the boundary between players and games。
This multi-dimensional enriches the blockchain game itself and brings new upgrades to the blockchain game
The initial and core Gameland product is a protocol layer that enables NFTs ERC-1155(a single transaction to represent multiple tokens at once) and EIP-2615(Non-Fungible Token with mortgage and rental functions)

## Our Vision 
The Gameland vision is to bring affordable game experience with an NFT for trustless, secure, and quick rentability in cross-platform way by gathering all the rentable items like user accounts or tools from different online gaming platforms and list them in order for people who are interested in renting one or some of them for either fun game experience or profit. 
Moreover, we want the community to benefit from this. We aim to fulfill the vision of a true DAO where each member is rewarded, to the extent that they are active participants in the community. All of the Origin products will be under the umbrella of this DAO.

Gameland architecture in future will be multi-chained as most of the different chain games are on different public chains.

## User Case
- Lenders(Lending an NFT)
* On Gameland’s platform a user can lend one or multiple NFTs. By doing so this saves multiple rental transaction costs and merges them into one single cost
* Lending implies to list it for rent
* The NFT "Lender" specifies the following:
    * The Rental Price (how much you wish to be compensated daily; this is the daily borrow price)
    * The NFT Price (this is the collateral a borrower most put up to rent)
    * Max Rental Period (maximum number of days you wish to lend out your NFT)

- Borrowers(Renting an NFT)
* The “Borrower” must specify the duration (in days) that they would like to borrow the NFT for
* This duration then gets multiplied by the lender's set rental price, the NFT price (collateral) gets added in, to arrive at the total rent price
* This total rent price gets deducted from the borrower’s balance and sent to the Gameland contract which acts as an escrow
* The NFT is then sent to the "Borrower" after successful transaction
* The NFT price (collateral) is stored in Gameland’s contract and is returned to the NFT “Borrower” only upon successful return of the NFT to the “Lender” (NFT "Lender")
* In the case that the NFT is not returned on time (rental duration has passed), the collateral can be claimed by the lender from the Gameland contract


## Build
```
$ cargo build --release
$ ./target/debug/appchain-gameland purge-chain --dev
$ ./target/debug/appchain-gameland --dev --enable-offchain-indexing true
```
