# CoinGecko exchange data anomaly study

*This project is merely demonstrative, not intended to prove anything in a serious/legal context.*

## The general idea

Cryptocurrencies's Centralized EXchanges (CEXes) often boast enourmous (several billions USD) daily volumes, but due to a lax regulatory framework and the overall disinterest of the community to audit CEXes, how much of that is real and how much is faked remains in the air. There are numbers ranging from ~50% to ~95%. Faked volume can be seen with anomalous, or otherwise nonsensical statsticall pattern, some of which this codebase deals with.

## methodology

- volume 
    - do the first and last digits follow Benford's law and a random distribution respectivelly?
- spread => high spread is abnormal
- volume * spread => high spread with high volume is abnormal. Volume and spread should be inversely correlated

## TODO
- [ ] migrate from serde to rkyv
