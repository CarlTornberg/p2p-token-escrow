### Peer2Peer Token Escrow
### Project Name: p2p-token-escrow 
### Devnet Program ID: ChDn1PMciMVbLKoQXe2qHo6HtLCikFsDaKC7fJgfaPNc
### Value Proposition
Trading involving exchanging goods between two entities. 
For the trade to happen, both parties must agree of the ask and offer before a trade can take part.
Moreover, a trade is commonly of two different types, the maker asks for something, giving something back in exchange for their item.
A maker should be able to create a make at any point in time, and the taker to accept at any point in time, async.

### Product-Market Fit
Trade's between two entities should be atomic, meaning that both parties should agree to the trade for it to take place.
This requires the maker to create a trade offer, which a taker has the posibility to accept or ignore.
If the taker agrees to the trade, both the maker and the taker shall receive their assumed product, and if any party for some reason changes their mind or cancles the trade, the transaction shall be terminated and any transfers reversed.

### Target User Profiles
* The maker: A user who want to trade X amount of token A, in exhange for Y amount of token B.
* The taker: A user who accepts to trade Y amount of their token B, in exchange for X amount of token A.

### User Story
* As a maker, when I create an escrow, an escrow is created in my authority.
* As a maker, when I create an escrow, my tokens I trade shall be stored in a token vault owned by the escrow.
* As a maker, when I create mulple escrows, the seed I provide distingishes each escrow from the oneother.
* As a taker, when I accept an escrow, my tokens are transferred to the maker, and the tokens in the escrow token vault are transferred to me.
* As a maker, when I have created an escrow, I can close the escrow and get my tokens refunded.

![Counter sequence diagram](./p2p-escrow.drawio.svg)

### How To Run:
1. Clone and enter the repo
```
git clone https://github.com/CarlTornberg/p2p-token-escrow.git
cd p2p-token-escrow
```
2. Install dependencies
```
yarn install
```

3. Run/test
```
anchor test
```

### Examples
* Make: https://solscan.io/tx/raDjPRL7rk3jc5mq4rYhYSBchYssPd7zBm1k8nnJms6eoJJiEa4NWD1pfrqTFcKBsvSiGa6hKCwTAUjK65ZbFzR?cluster=devnet
* Take: https://solscan.io/tx/53Jca5u5rfESQkbQxiyxcqNxL15RKBpMWsTmoXVatb3cFpWNbmSwFpSn5gTYKoZe61YpMNGsajt2HtQJk93HHp64?cluster=devnet
