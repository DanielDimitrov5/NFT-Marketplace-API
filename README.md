# NFT Marketplace REST API

## Overview
The NFT-Marketplace REST API is built with Rust. It is designed to provide access to the NFT-Marketplace via database indexed by the [NFT-Marketplace-Indexer](https://github.com/DanielDimitrov5/NFT-Marketplace-Indexer-Rust).

[NFT-Marketplace](https://github.com/DanielDimitrov5/NFT-Marketplace-Project)

## How to Run
You can compile and run the server locally by cloning the repository and using cargo run. 
```sh
git clone https://github.com/DanielDimitrov5/NFT-Marketplace-API
cd <repository_directory>
cargo run
```
The server will start on http://localhost:8000.

## API Endpoints
The API provides several GET endpoints for fetching data from the NFT-Marketplace smart contract.

* GET `/`
    * Returns information about the API itself.
    
* GET `/collections`
    * Returns all NFT collections.
    
* GET `/items`
    * Returns all items.

* GET `/offers`
    * Returns all offers.
    
## Acknowledgements

This project is powered by several open-source projects, notably:
- [rust](https://www.rust-lang.org/)
- [mongodb](https://www.mongodb.com/)
- [rocket](https://rocket.rs/)

## Contact

If you have any questions or comments, please feel free to contact me.

## License
Distributed under the [MIT License](./LICENSE). See `LICENSE` for more information.