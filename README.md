# Stellar Weather DApp

**Stellar Weather DApp** - Blockchain-Based Decentralized Weather Favorites Management System

## Project Description

Stellar Weather Cities DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure and transparent platform for managing a list of favorite cities for weather tracking directly on-chain.

Instead of storing weather data, the system stores user-selected cities while real-time weather information is retrieved from a free external API such as Open-Meteo. Each city is uniquely identified and stored in the contract instance storage, ensuring persistence and reliability.

## Project Vision

Our vision is to revolutionize personal productivity in the digital age by:

- **Decentralizing Data**: Moving note-taking from centralized servers to a global, distributed blockchain
- **Ensuring Ownership**: Empowering users to have complete control and ownership over their digital thoughts and information
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of notes that cannot be altered or deleted by third parties
- **Enhancing Privacy**: Leveraging blockchain security to protect personal information from unauthorized access
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, not by company promises
- **Improving Accessibility**: Allowing users to access saved cities anywhere

We envision a lightweight decentralized application where blockchain ensures trust and persistence, while external APIs handle real-time weather computation.

## Key Features

### 1. **Simple City Management**
- Add favorite cities with a single function call
- Store city name and country information
- Automatic unique ID generation for each city
- Persistent storage on the Stellar blockchain

### 2. **Efficient Data Retrieval**
- Fetch all saved cities in one call
- Structured data format for frontend integration
- Quick access to full city list
- Real-time synchronization with blockchain state

### 3. **Secure Deletion**
- Remove cities using their unique ID
- Permanent deletion from contract storage
- Updated city list after removal
- Efficient and clean state management

### 4. **Transparency and Security**
- All stored cities are verifiable on-chain
- Immutable record of user actions
- No centralized database dependency
- Tamper-resistant storage system

### 5. **Stellar Network Integration**
- Built using Soroban smart contracts
- Low-cost and high-speed blockchain execution
- Scalable architecture for global usage
- Compatible with Stellar ecosystem tools

## Contract Details

- Contract Address: CCFCM3XGWUFCCLLKF2REEUIZFNEW3JYSV2CBPGWKD2LN6LQJKJ3ZSTLB
  ![Deployment](deploy.png)

## Future Scope

### Short-Term Enhancements

1. **Weather API Integration**: Display real-time weather using Open-Meteo
2. **City Search Feature**: Search and add cities easily
3. **UI Improvements**: Simple frontend for better user experience
4. **Auto Location Detection**: Suggest nearby cities automatically

### Medium-Term Development

5. **Weather Alerts System**
    - Notify users of extreme weather conditions
    - Store alert preferences on-chain
6. **Cross-Device Sync**
    - Access saved cities across devices using blockchain
7. **Weather History Tracking**
    - Link historical weather data to saved cities
8. **Multi-API Support**
    - Use multiple weather data providers

### Long-Term Vision

9. **Global Weather Network**
    - Community-driven weather insights system
10. **Decentralized Weather Dashboard**
    - Fully on-chain preference system with UI
11. **AI Weather Predictions**
    - Smart forecasting using historical trends
12. **Cross-Chain Expansion**
    - Extend support to other blockchain networks
13. **Decentralized Identity Integration**
    - Link user profiles with DID systems

### Enterprise Features

15. **Smart City Analytics**: Aggregate city preference insights
16. **Disaster Monitoring Support**: Track high-risk locations
17. **Research Integration**: Provide structured weather location data
18. **Multi-Region Support**: Global localization and scaling

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network
- External Weather API

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

- 'add_city()' - Add a new favorite city
- 'get_cities()' - Retrieve all saved cities
- 'delete_city()' - Remove a city by ID

Then connect a simple frontend or API layer to fetch real-time weather data for those cities.

---

**Stellar Weather DApp** - Securing Blockchain and Global Weather Data
